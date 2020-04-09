
// Some states here
struct Stalling;

use cgmath::Vector4;
struct Zooming {
    // Initial field of view (rad)
    z0: Rad<f32>,
    // Goal field of view (rad)
    zf: Rad<f32>,
    // Current quantity of zoom (rad)
    z: Rad<f32>,
    // The time when the zooming begins (in ms)
    t0: f32,
}
use cgmath::Vector3;
struct Unzooming {
    // Initial field of view (rad)
    z0: Rad<f32>,
    // Goal field of view (rad)
    zf: Rad<f32>,
    // Current quantity of zoom (rad)
    z: Rad<f32>,
    // The time when the unzooming begins (in ms)
    t0: f32,
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
// Move the renderables between two world position on the sky
fn move_renderables<P: Projection>(
 // Previous world position
 x: &Vector4<f32>,
 // Current world position
 y: &Vector4<f32>,
 // Renderables
 sphere: &mut HiPSSphere,
 catalog: &mut Catalog,
 grid: &mut ProjetedGrid,
 // Viewport
 viewport: &mut ViewPort,
) -> (Vector3<f32>, Rad<f32>) {
    let model_mat = viewport.get_model_mat();

    let x = (model_mat * x).truncate();
    let y = (model_mat * y).truncate();

    let axis = x.cross(y)
        .normalize();
    let d = math::angular_distance_xyz(x, y);

    viewport.apply_rotation(-axis, d);

    // Update all the renderables
    viewport.displacement::<P>(sphere, catalog, grid);
    (axis, d)
}

use crate::event_manager::MouseMove;
use crate::math;
use cgmath::InnerSpace;

impl Zooming {
    #[inline]
    pub fn w0() -> f32 {
        15_f32
    }
}

use crate::utils;
impl State for Zooming {
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
        let z = self.zf + (self.z0 - self.zf) * (Zooming::w0() * t + 1_f32) * ((-Zooming::w0() * t).exp());

        //console::log_1(&format!("dtheta {:?}", theta).into());
        //sphere.apply_rotation(-self.axis, theta);
        //viewport.displacement::<P>(sphere, catalog, grid);
        viewport.zoom::<P>(z, sphere, catalog, grid);

        self.z = z;
    }
}
impl Unzooming {
    #[inline]
    pub fn w0() -> f32 {
        15_f32
    }
}

impl State for Unzooming {
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
        let z = self.zf + (self.z0 - self.zf) * (Unzooming::w0() * t + 1_f32) * ((-Unzooming::w0() * t).exp());

        //console::log_1(&format!("dtheta {:?}", theta).into());
        //sphere.apply_rotation(-self.axis, theta);
        //viewport.displacement::<P>(sphere, catalog, grid);
        viewport.unzoom::<P>(z, sphere, catalog, grid);

        self.z = z;
    }
}

use crate::event_manager::NUM_WHEEL_PER_DEPTH;
fn fov<P: Projection>(wheel_idx: i32) -> Rad<f32> {
    let exp = (wheel_idx as f32) / (NUM_WHEEL_PER_DEPTH as f32);
    let fov = P::aperture_start() / 2_f32.powf(exp);

    fov.into()
}

use web_sys::console;
use crate::event_manager::MouseWheelUp;
// Stalling -> Zooming
impl Transition for T<Stalling, Zooming> {
    type S = Stalling;
    type E = Zooming;
   
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
        if let Some(_) = events.get::<MouseWheelUp>() {
            console::log_1(&format!("Welcome state Zooming").into());
            let t0 = utils::get_current_time();

            let wheel_idx = viewport.get_wheel_idx();
            viewport.up_wheel_idx();

            let z0 = fov::<P>(wheel_idx);
            let zf = fov::<P>(wheel_idx + 1);
            let z = z0;
            Some(Zooming {
                t0,
                z0,
                zf,
                z
            })
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}
// Zooming -> Zooming
impl Transition for T<Zooming, Zooming> {
    type S = Zooming;
    type E = Zooming;
   
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
        if let Some(_) = events.get::<MouseWheelUp>() {
            console::log_1(&format!("Welcome state Zooming").into());
            let t0 = utils::get_current_time();

            let wheel_idx = viewport.get_wheel_idx();
            viewport.up_wheel_idx();

            let zf = fov::<P>(wheel_idx + 1);
            // Change the final zoom fov zf only
            let z0 = s.z;
            let z = z0;
            Some(Zooming {
                t0,
                zf,
                z,
                z0,
            })
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}
// Zooming -> Stalling
impl Transition for T<Zooming, Stalling> {
    type S = Zooming;
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
        if (s.z - s.zf).0.abs() < 1e-4 {
            console::log_1(&format!("Welcome state Stalling").into());
            Some(Stalling {})
        } else {
            None
        }
    }
}

use crate::event_manager::MouseWheelDown;
// Stalling -> Unzooming
impl Transition for T<Stalling, Unzooming> {
    type S = Stalling;
    type E = Unzooming;
   
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
        if let Some(_) = events.get::<MouseWheelDown>() {
            let t0 = utils::get_current_time();

            let wheel_idx = viewport.get_wheel_idx();
            if wheel_idx == 0 {
                None
            } else {
                console::log_1(&format!("Welcome state Unzooming").into());
                viewport.down_wheel_idx();

                let z0 = fov::<P>(wheel_idx);
                let zf = fov::<P>(wheel_idx - 1);
                let z = z0;
                Some(Unzooming {
                    t0,
                    z0,
                    zf,
                    z
                })
            }
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}
// Unzooming -> Unzooming
impl Transition for T<Unzooming, Unzooming> {
    type S = Unzooming;
    type E = Unzooming;
   
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
        if let Some(_) = events.get::<MouseWheelDown>() {
            console::log_1(&format!("Welcome state Unzooming").into());
            let t0 = utils::get_current_time();

            let wheel_idx = viewport.get_wheel_idx();
            if wheel_idx == 0 {
                None
            } else {
                viewport.down_wheel_idx();

                let zf = fov::<P>(wheel_idx - 1);
                // Change the final zoom fov zf only
                let z0 = s.z;
                let z = z0;
                Some(Unzooming {
                    t0,
                    z0,
                    zf,
                    z
                })
            }
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}
// Unzooming -> Stalling
impl Transition for T<Unzooming, Stalling> {
    type S = Unzooming;
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
        if (s.z - s.zf).0.abs() < 1e-4 {
            console::log_1(&format!("Welcome state Stalling").into());
            Some(Stalling {})
        } else {
            None
        }
    }
}

// Zooming -> Unzooming
impl Transition for T<Zooming, Unzooming> {
    type S = Zooming;
    type E = Unzooming;

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
        if let Some(_) = events.get::<MouseWheelDown>() {
            console::log_1(&format!("Welcome state Unzooming").into());
            let t0 = utils::get_current_time();

            let wheel_idx = viewport.get_wheel_idx();
            if wheel_idx == 0 {
                None
            } else {
                viewport.down_wheel_idx();

                let zf = fov::<P>(wheel_idx - 1);
                // Change the final zoom fov zf only
                let z0 = s.z;
                let z = z0;
                Some(Unzooming {
                    t0,
                    z0,
                    zf,
                    z
                })
            }
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}

// Unzooming -> Zooming
impl Transition for T<Unzooming, Zooming> {
    type S = Unzooming;
    type E = Zooming;
   
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
        if let Some(_) = events.get::<MouseWheelUp>() {
            console::log_1(&format!("Welcome state Zooming").into());
            let t0 = utils::get_current_time();

            let wheel_idx = viewport.get_wheel_idx();
            viewport.up_wheel_idx();

            let zf = fov::<P>(wheel_idx + 1);
            // Change the final zoom fov zf only
            let z0 = s.z;
            let z = z0;
            Some(Zooming {
                t0,
                zf,
                z,
                z0,
            })
        } else {
            // No left button pressed, we keep stalling
            None
        }
    }
}

pub enum UserZoom {
    Stalling(Stalling),
    Zooming(Zooming),
    Unzooming(Unzooming)
}

impl FiniteStateMachine for UserZoom {
    fn init() -> Self {
        console::log_1(&format!("Welcome starting state Stalling").into());
        UserZoom::Stalling(Stalling {})
    }
}

impl UserZoom {
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
            UserZoom::Stalling(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
            UserZoom::Zooming(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
            UserZoom::Unzooming(s) => s.update::<P>(dt, sphere, catalog, grid, viewport, events),
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
            UserZoom::Stalling(stalling) => {
                // Checks the Stalling -> Moving condition
                if let Some(e) = stalling.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Zooming(e);
                } else if let Some(e) = stalling.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Unzooming(e);
                }
            },
            UserZoom::Zooming(zooming) => {
                // Checks the Zooming -> Stalling condition
                if let Some(e) = zooming.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Stalling(e);
                // Checks the Zooming -> Zooming condition
                } else if let Some(e) = zooming.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Zooming(e);
                // Checks the Zooming -> Unzooming condition
                } else if let Some(e) = zooming.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Unzooming(e);
                }
            },
            UserZoom::Unzooming(unzooming) => {
                // Checks the Unzooming -> Stalling condition
                if let Some(e) = unzooming.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Stalling(e);
                // Checks the Unzooming -> Unzooming condition
                } else if let Some(e) = unzooming.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Unzooming(e);
                // Checks the Unzooming -> Zooming condition
                } else if let Some(e) = unzooming.check::<_, P>(sphere, catalog, grid, viewport, events) {
                    *self = UserZoom::Zooming(e);
                }
            },
        }
    }
}