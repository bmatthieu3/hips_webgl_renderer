//mod inertia;
//mod mousemove;
use crate::renderable::Renderable;
use crate::renderable::{
 HiPSSphere,
 Catalog,
 ProjetedGrid
};

use crate::event_manager::EventManager;

use crate::viewport::ViewPort;
use crate::projection::Projection;

// A generic structure that will implement Transition
// for various state (S, E) tuples
struct T<S, E>
where S: State,
      E: State {
    s: std::marker::PhantomData<S>,
    e: std::marker::PhantomData<E>
}

// The transition trait with two associated type:
// - a starting state of type S
// - an ending state of type E
trait Transition {
    type S: State;
    type E: State;

    fn condition<P: Projection>(
        s: &Self::S,
        // Renderables
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<Self::E>;
}

trait State: std::marker::Sized {
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
    );

    // A method checking if the transition from Self to (E: State) is valid
    // If so, returns the ending state. If not returns None.
    // This method is only defined if the transition between Self and E exists
    fn check<E: State, P: Projection>(&mut self,
        // Renderables
        sphere: &mut Renderable<HiPSSphere>,
        catalog: &mut Renderable<Catalog>,
        grid: &mut Renderable<ProjetedGrid>,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<E>
    where T<Self, E>: Transition<S=Self, E=E> {
        T::<Self, E>::condition::<P>(&self, sphere, catalog, grid, viewport, events)
    }
}

// Some states here
struct Stalling;

use cgmath::Vector4;
struct Moving {
    // World position corresponding to the move
    world_pos: Vector4<f32>,
    time_move: f32,

    angular_dist: Rad<f32>,
}
struct Inertia;

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
) -> Rad<f32> {
    let model_mat = sphere.get_model_mat();

    let x = (model_mat * x).truncate();
    let y = (model_mat * y).truncate();

    let axis = x.cross(y)
        .normalize();
    let d = math::angular_distance_xyz(x, y);

    sphere.apply_rotation(-axis, d);

    // Update all the renderables
    viewport.displacement::<P>(sphere, catalog, grid);
    d
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
                    let angular_dist = move_renderables::<P>(
                        &self.world_pos,
                        &world_pos,
                        sphere, catalog, grid,
                        viewport
                    );

                    // Update the previous position
                    self.world_pos = world_pos;
                    self.time_move = utils::get_current_time();
                    self.angular_dist = angular_dist;
                }
            }
        }
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

    }
}

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
                println!("Welcome state Moving");
                let time_move = utils::get_current_time();
                let angular_dist = Rad(0_f32);
                Some(Moving {
                    world_pos,
                    time_move,
                    angular_dist,
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
            if t > 100_f32 {
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
            println!("Welcome state Stalling");
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
            if t <= 100_f32 {
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
                println!("Welcome state Inertia");
                Some(Inertia {})
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub trait FiniteStateMachine {
    fn init() -> Self;
}

pub enum UserMoveSphere {
    Stalling(Stalling),
    Moving(Moving),
}

impl FiniteStateMachine for UserMoveSphere {
    fn init() -> Self {
        println!("Welcome to starting state STALLING");
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
                }
            },
        }
    }
}