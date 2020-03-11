
// Some states here
struct Stalling;

use cgmath::Vector4;
struct Moving {
    // World position corresponding to the move
    world_pos: Vector4<f32>,
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
 Renderable,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {}
}

use cgmath::Rad;
// Move the renderables between two world position on the sky
fn move_renderables<P: Projection>(
 // Previous world position
 x: &Vector4<f32>,
 // Current world position
 y: &Vector4<f32>,
 // Renderables
 sphere: &mut Renderable<HiPSSphere>,
 catalog: &mut Renderable<Catalog>,
 grid: &mut Renderable<ProjetedGrid>,
 // Viewport
 viewport: &mut ViewPort,
) -> (Vector3<f32>, Rad<f32>) {
    let model_mat = sphere.get_model_mat();

    let x = (model_mat * x).truncate();
    let y = (model_mat * y).truncate();

    let axis = x.cross(y)
        .normalize();
    let d = math::angular_distance_xyz(x, y);

    sphere.apply_rotation(-axis, d);

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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) {
        if let Some(screen_pos) = events.get::<MouseMove>() {
            if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {
                // Check whether the world pos is different
                // Do not do the transition if the user move to the
                // same position. In principle, a new mouse move event
                // is launched if the position changed
                if world_pos != self.world_pos {
                    let (axis, d) = move_renderables::<P>(
                        &self.world_pos,
                        &world_pos,
                        sphere, catalog, grid,
                        viewport
                    );

                    // Update the previous position
                    self.world_pos = world_pos;
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
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

        //console::log_1(&format!("dtheta {:?}", theta).into());
        sphere.apply_rotation(-self.axis, theta);
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(screen_pos) = events.get::<MouseLeftButtonPressed>() {
            if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {
                console::log_1(&format!("Welcome state Moving").into());
                let time_move = utils::get_current_time();
                let angular_dist = Rad(0_f32);
                let axis = Vector3::new(0_f32, 0_f32, 0_f32);
                Some(Moving {
                    world_pos,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(screen_pos) = events.get::<MouseLeftButtonReleased>() {
            let t = utils::get_current_time() - s.time_move;
            if t < 10_f32 {
                return None;
            }

            if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {
                // Check whether the mouse has moved
                if world_pos != s.world_pos {
                    // If so perform the last sphere rotation
                    // before diving in the Stalling state
                    move_renderables::<P>(
                        &s.world_pos,
                        &world_pos,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(screen_pos) = events.get::<MouseLeftButtonReleased>() {
            let t = utils::get_current_time() - s.time_move;
            // Jump into the inertia mode if the mouse has been released in following 10ms after the last move
            if t <= 10_f32 {
                if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {
                    // Check whether the mouse has moved
                    if world_pos != s.world_pos {
                        // If so perform the last sphere rotation
                        // before diving in the Stalling state
                        move_renderables::<P>(
                            &s.world_pos,
                            &world_pos,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E> {
        if let Some(screen_pos) = events.get::<MouseLeftButtonPressed>() {
            if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {
                console::log_1(&format!("Welcome state Moving").into());
                let time_move = utils::get_current_time();
                let angular_dist = Rad(0_f32);
                let axis = Vector3::new(0_f32, 0_f32, 0_f32);
                Some(Moving {
                    world_pos,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
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
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
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