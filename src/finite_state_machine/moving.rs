
// Some states here
struct Stalling;

use cgmath::Vector4;
struct Moving {
    // World position corresponding to the move
    pos_model_space: Vector4<f32>,
    time_move: f32,

    angular_dist: Rad<f32>,
    axis: Vector3<f32>
}
use cgmath::Vector3;
struct Inertia {
    // Init angular distance of rotation
    d0: Rad<f32>,
    // angular distance
    d: Rad<f32>,
    // The axis of rotation when the mouse has been released
    axis: Vector3<f32>,
    // The time when the inertia begins (in ms)
    t_start: f32,
}

use crate::finite_state_machine::{
    State,
    Transition,
    T,
    FiniteStateMachine
};
use crate::projection::Projection;
use crate::renderable::{
 HiPSSphere,
 Catalog,
 ProjetedGrid
};
use crate::event_manager::EventManager;
use crate::viewport::ViewPort;
impl State for Stalling {
    fn update<P: Projection>(&mut self,
        // Time of the previous frame
        dt: f32,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {}
}

use cgmath::Rad;
use crate::rotation::SphericalRotation;
// Move the renderables between two world position on the sky
pub fn move_renderables<P: Projection>(
 // Previous model position
 x: &Vector4<f32>,
 // Current model position
 y: &Vector4<f32>,
 // Renderables
 sphere: &mut HiPSSphere,
 catalog: &mut Catalog,
 grid: &mut ProjetedGrid,
 // Viewport
 viewport: &mut ViewPort,
) -> (Vector3<f32>, Rad<f32>) {
    //let inv_model_mat = viewport.get_inverted_model_mat();
    //viewport.set_rotation(&(&y).into());
    let r = viewport.get_rotation();
    let x = r.rotate(x).truncate();
    let y = r.rotate(y).truncate();
    let axis = x.cross(y)
        .normalize();
    let d = math::ang_between_vect(&x, &y);

    viewport.apply_rotation(-axis, d);

    // Update all the renderables
    viewport.displacement::<P>(sphere, catalog, grid);
    (axis, d)
}

use crate::event_manager::MouseMove;
use crate::math;
use cgmath::InnerSpace;
impl State for Moving {
    fn update<P: Projection>(&mut self,
        // Time of the previous frame
        dt: f32,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {
        if let Some(pos_screen_space) = events.get::<MouseMove>() {
            if let Some(pos_model_space) = P::screen_to_model_space(&pos_screen_space, &viewport) {
                // Check whether the world pos is different
                // Do not do the transition if the user move to the
                // same position. In principle, a new mouse move event
                // is launched if the position changed
                if pos_model_space != self.pos_model_space {
                    let (axis, d) = move_renderables::<P>(
                        &self.pos_model_space,
                        &pos_model_space,
                        sphere, catalog, grid,
                        viewport
                    );

                    // Update the previous position
                    self.pos_model_space = pos_model_space;
                    self.time_move = utils::get_current_time();
                    self.angular_dist = d;
                    self.axis = axis;
                }
            }
        }
    }
}

impl Inertia {
    #[inline]
    pub fn w0() -> f32 {
        5_f32
    }
}
impl State for Inertia {
    fn update<P: Projection>(&mut self,
        // Time of the previous frame
        dt: f32,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {
        // Time elapsed since the beginning of the inertia
        let t = (utils::get_current_time() - self.t_start)/1000_f32;
        // Undamped angular frequency of the oscillator
        // From wiki: https://en.wikipedia.org/wiki/Harmonic_oscillator
        //
        // In a damped harmonic oscillator system: w0 = sqrt(k / m)
        // where:
        // * k is the stiffness of the ressort
        // * m is its mass
        let theta = self.d0 * (Inertia::w0() * t + 1_f32) * ((-Inertia::w0() * t).exp());

        viewport.apply_rotation(-self.axis, theta);
        viewport.displacement::<P>(sphere, catalog, grid);

        self.d = theta;
    }
}
use web_sys::console;
use crate::event_manager::MouseLeftButtonPressed;
// Stalling -> Moving
impl Transition for T<Stalling, Moving> {
    type S = Stalling;
    type E = Moving;
   
    fn condition<P: Projection>(s: &Self::S,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(pos_screen_space) = events.get::<MouseLeftButtonPressed>() {
            if let Some(pos_model_space) = P::screen_to_model_space(&pos_screen_space, &viewport) {
                console::log_1(&format!("Welcome state Moving").into());
                let time_move = utils::get_current_time();
                let angular_dist = Rad(0_f32);
                let axis = Vector3::new(0_f32, 0_f32, 0_f32);
                Some(Moving {
                    pos_model_space,
                    time_move,
                    angular_dist,
                    axis
                })
            } else {
                // The clic is out of the projection, we keep stalling
                None
            }
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}

use crate::event_manager::MouseLeftButtonReleased; 
use crate::utils;
// Moving -> Stalling
impl Transition for T<Moving, Stalling> {
    type S = Moving;
    type E = Stalling;

    fn condition<P: Projection>(s: &Self::S,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(pos_screen_space) = events.get::<MouseLeftButtonReleased>() {
            let t = utils::get_current_time() - s.time_move;
            if t < 10_f32 {
                return None;
            }

            if let Some(pos_model_space) = P::screen_to_model_space(&pos_screen_space, &viewport) {
                // Check whether the mouse has moved
                if pos_model_space != s.pos_model_space {
                    // If so perform the last sphere rotation
                    // before diving in the Stalling state
                    move_renderables::<P>(
                        &s.pos_model_space,
                        &pos_model_space,
                        sphere, catalog, grid,
                        viewport
                    );
                }
            }
            console::log_1(&format!("Welcome state Stalling").into());
            Some(Stalling {})
        } else {
            None
        }
    }
}
// Moving -> Inertia
impl Transition for T<Moving, Inertia> {
    type S = Moving;
    type E = Inertia;

    fn condition<P: Projection>(s: &Self::S,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(pos_screen_space) = events.get::<MouseLeftButtonReleased>() {
            let t = utils::get_current_time() - s.time_move;
            // Jump into the inertia mode if the mouse has been released in following 10ms after the last move
            if t <= 10_f32 {
                if let Some(pos_model_space) = P::screen_to_model_space(&pos_screen_space, &viewport) {
                    // Check whether the mouse has moved
                    if pos_model_space != s.pos_model_space {
                        // If so perform the last sphere rotation
                        // before diving in the Stalling state
                        move_renderables::<P>(
                            &s.pos_model_space,
                            &pos_model_space,
                            sphere, catalog, grid,
                            viewport
                        );
                    }
                }
                console::log_1(&format!("Welcome state Inertia").into());
                let axis = s.axis;
                let d0 = s.angular_dist;
                let d = d0;
                let t_start = utils::get_current_time();
                Some(Inertia {
                    d0,
                    d,
                    axis,
                    t_start
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

// Inertia -> Stalling
impl Transition for T<Inertia, Stalling> {
    type S = Inertia;
    type E = Stalling;

    fn condition<P: Projection>(s: &Self::S,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if s.d < Rad(1e-5) {
            console::log_1(&format!("Welcome state Stalling").into());
            Some(Stalling {})
        } else {
            None
        }
    }
}

// Inertia -> Moving
impl Transition for T<Inertia, Moving> {
    type S = Inertia;
    type E = Moving;

    fn condition<P: Projection>(s: &Self::S,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(pos_screen_space) = events.get::<MouseLeftButtonPressed>() {
            if let Some(pos_model_space) = P::screen_to_model_space(pos_screen_space, &viewport) {
                console::log_1(&format!("Welcome state Moving").into());
                let time_move = utils::get_current_time();
                let angular_dist = Rad(0_f32);
                let axis = Vector3::new(0_f32, 0_f32, 0_f32);
                Some(Moving {
                    pos_model_space,
                    time_move,
                    angular_dist,
                    axis
                })
            } else {
                // The clic is out of the projection, we keep in the inertia state
                None
            }
        } else {
            // No left button pressed, we keep being in the inertia state
            None
        }
    }
}

pub enum UserMoveSphere {
    Stalling(Stalling),
    Moving(Moving),
    Inertia(Inertia)
}

impl FiniteStateMachine for UserMoveSphere {
    fn init() -> Self {
        console::log_1(&format!("Welcome starting state Inertia").into());
        UserMoveSphere::Stalling(Stalling {})
    }
}

impl UserMoveSphere {
    fn update<P: Projection>(&mut self,
        // Time of the previous frame
        dt: f32,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {
        match self {
            UserMoveSphere::Stalling(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
            UserMoveSphere::Moving(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
            UserMoveSphere::Inertia(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
        }
    }

    pub fn run<P: Projection>(
        &mut self,
        // Time of the previous frame
        dt: f32,
        // Renderables
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {
        // Update the current state
        self.update::<P>(dt,
            sphere, catalog, grid,
            viewport,
            events
        );

        // Checks whether conditions are valid after the update
        match self {
            UserMoveSphere::Stalling(stalling) => {
                // Checks the Stalling -> Moving condition
                if let Some(e) = stalling.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserMoveSphere::Moving(e);
                }
            },
            UserMoveSphere::Moving(moving) => {
                // Checks the Moving -> Stalling condition
                if let Some(e) = moving.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserMoveSphere::Stalling(e);
                // Checks the Moving -> Inertia condition
                } else if let Some(e) = moving.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserMoveSphere::Inertia(e);
                }
            },
            UserMoveSphere::Inertia(inertia) => {
                // Checks the Inertia -> Stalling condition
                if let Some(e) = inertia.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserMoveSphere::Stalling(e);
                // Checks the Inertia -> Moving condition
                } else if let Some(e) = inertia.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserMoveSphere::Moving(e);
                }
            },
        }
    }
}