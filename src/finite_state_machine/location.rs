
// Some states here
struct Stalling;

use cgmath::Vector4;
struct Moving {
    // Quaternion describing the position of the center
    start: SphericalRotation<f32>,
    // Quaternion describing the goal position
    goal: SphericalRotation<f32>,
    // Alpha coefficient between 0 and 1
    alpha: f32,
    // Start time
    t0: f32,
}
impl Moving {
    #[inline]
    fn w0() -> f32 {
        5_f32
    }

    fn new(start: SphericalRotation<f32>, goal: SphericalRotation<f32>) -> Moving {
        console::log_1(&format!("Welcome state Moving").into());
        let t0 = utils::get_current_time();

        let alpha = 0_f32;
        Moving {
            start,
            goal,
            t0,
            alpha
        }
    }
}
use cgmath::Vector3;

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
        // Time elapsed since the beginning of the inertia
        let t = (utils::get_current_time() - self.t0)/1000_f32;
        
        // Undamped angular frequency of the oscillator
        // From wiki: https://en.wikipedia.org/wiki/Harmonic_oscillator
        //
        // In a damped harmonic oscillator system: w0 = sqrt(k / m)
        // where: 
        // * k is the stiffness of the ressort
        // * m is its mass
        let alpha = 1_f32 + (0_f32 - 1_f32) * (Moving::w0() * t + 1_f32) * ((-Moving::w0() * t).exp());
        let p = self.start.slerp(&self.goal, alpha);

        viewport.set_rotation(&p);
        viewport.displacement::<P>(sphere, catalog, grid);

        self.alpha = alpha;
    }
}

use web_sys::console;
use crate::event_manager::MouseLeftButtonDoublePressed;
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
        if let Some(pos_screen_space) = events.get::<MouseLeftButtonDoublePressed>() {
            if let Some(pos_world_space) = P::screen_to_world_space(&pos_screen_space, &viewport) {
                let start = *viewport.get_rotation();
                let goal = SphericalRotation::from_sky_position(&pos_world_space);
                Some(Moving::new(start, goal))
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

use crate::utils;
use crate::event_manager::{
 MouseLeftButtonPressed,
 MouseWheelDown,
 MouseWheelUp
};
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
        // Priority to the user events
        // such as move
        if let Some(_) = events.get::<MouseLeftButtonPressed>() {
            Some(Stalling {})
        // or zooming/unzooming events
        } else if let Some(_) = events.get::<MouseWheelDown>() {
            Some(Stalling {})
        } else if let Some(_) = events.get::<MouseWheelUp>() {
            Some(Stalling {})
        } else {
            let eps = (s.alpha - 1_f32).abs();
            if eps < 1e-3 {
                Some(Stalling{})
            } else {
                None
            }
        }
    }
}

pub enum MoveSphere {
    Stalling(Stalling),
    Moving(Moving),
}

impl FiniteStateMachine for MoveSphere {
    fn init() -> Self {
        console::log_1(&format!("Welcome starting state Inertia").into());
        MoveSphere::Stalling(Stalling {})
    }
}

impl MoveSphere {
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
            MoveSphere::Stalling(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
            MoveSphere::Moving(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
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
            MoveSphere::Stalling(stalling) => {
                // Checks the Stalling -> Moving condition
                if let Some(e) = stalling.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = MoveSphere::Moving(e);
                }
            },
            MoveSphere::Moving(moving) => {
                // Checks the Moving -> Stalling condition
                if let Some(e) = moving.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = MoveSphere::Stalling(e);
                }
            },
        }
    }
}