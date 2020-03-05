//mod inertia;
mod mousemove;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;

trait State: std::marker::Sized {
    fn update(&mut self, dt: f32, sphere: &mut Renderable<HiPSSphere>);

    fn begin(&mut self, sphere: &mut Renderable<HiPSSphere>);
    fn end(&mut self, sphere: &mut Renderable<HiPSSphere>);

    // A method checking if the transition from Self to (E: State) is valid
    // If so, returns the ending state. If not returns None.
    // This method is only defined if the transition between Self and E exists
    fn check<E: State, P: Projection>(&self, viewport: &ViewPort, events: &EventManager) -> Option<E>
    where T<Self, E>: Transition<S=Self, E=E> {
        T::<Self, E>::condition::<P>(&self, viewport, events)
    }
}

// Some states here
struct Stalling;

use cgmath::Vector4;
struct Moving {
    // Starting world position
    previous_world_pos: Vector4<f32>,
}
struct Inertia;

impl State for Stalling {
    fn update(&mut self, dt: f32, sphere: &mut Renderable<HiPSSphere>) {}
}
impl State for Moving {
    fn update(&mut self, dt: f32, sphere: &mut Renderable<HiPSSphere>) {
        if let Some(start_world_pos) = P::screen_to_world_space(screen_pos, &self.viewport) {
            self.moving = Some(Move::new::<P>(start_world_pos, &self.hips_sphere));
        }
        if world_pos == self.start_world_pos {
            return;
        }
        let model_mat = sphere.get_model_mat();

        let start_model_pos = model_mat * self.start_world_pos;
        let start_model_pos = cgmath::Vector3::<f32>::new(start_model_pos.x, start_model_pos.y, start_model_pos.z);

        let model_pos = model_mat * world_pos;
        let model_pos = cgmath::Vector3::<f32>::new(model_pos.x, model_pos.y, model_pos.z);

        let axis = start_model_pos.cross(model_pos);
        self.x = math::angular_distance_xyz(start_model_pos, model_pos);

        self.axis = axis.normalize();

        hips_sphere.apply_rotation(-self.axis, self.x);

        self.start_world_pos = world_pos;

        // Update the time to the current move
        self.last_time = utils::get_current_time();
    }
}
impl State for Inertia {
    fn update(&mut self, dt: f32, sphere: &mut Renderable<HiPSSphere>) {}
}

use crate::event_manager::EventManager;
use crate::projection::Projection;
use crate::viewport::ViewPort;
// The transition trait with two associated type:
// - a starting state of type S
// - an ending state of type E
trait Transition {
    type S: State;
    type E: State;
   
    fn condition<P: Projection>(
        s: &Self::S,
        
        viewport: &ViewPort,
        events: &EventManager
    ) -> Option<Self::E>;
}

// A generic structure that will implement Transition
// for various state (S, E) tuples
struct T<S, E>
where S: State,
      E: State {
    s: std::marker::PhantomData<S>,
    e: std::marker::PhantomData<E>
}

use crate::event_manager::MouseLeftButtonPressed;
// Stalling -> Moving
impl Transition for T<Stalling, Moving> {
    type S = Stalling;
    type E = Moving;
   
    fn condition<P: Projection>(s: &Self::S, viewport: &ViewPort, events: &EventManager) -> Option<Self::E> {
        if let Some(screen_pos) = events.get::<MouseLeftButtonPressed>() {
            if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {
                println!("Welcome state Moving");
                Some(Moving {
                    previous_world_pos: world_pos
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

use crate::event_manager::MouseMove;
// Moving -> Moving
impl Transition for T<Moving, Moving> {
    type S = Moving;
    type E = Moving;

    fn condition<P: Projection>(s: &Self::S, viewport: &ViewPort, events: &EventManager) -> Option<Self::E> {
        if let Some(screen_pos) = events.get::<MouseMove>() {
            if let Some(world_pos) = P::screen_to_world_space(*screen_pos, &viewport) {


                println!("Welcome state Moving");
                Some(Moving {
                    previous_world_pos: world_pos
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
// B -> C
impl Transition for T<B, C> {
    type S = B;
    type E = C;
   
    fn condition(s: &Self::S) -> Option<Self::E> {
        println!("Welcome state C");
        Some(C {})
    }
}
// C -> A
impl Transition for T<C, A> {
    type S = C;
    type E = A;
   
    fn condition(s: &Self::S) -> Option<Self::E> {
        println!("Welcome state A");
        Some(A {})
    }
}

trait FiniteStateMachine {
    fn update(&mut self);
    fn run(&mut self);
}
/*
enum FSM {
    A(A),
    B(B),
    C(C)
}

impl FSM {
    fn new() -> FSM {
        println!("Welcome to starting state A");
        FSM::A(A {})
    }

    fn update(&mut self) {
        match self {
            FSM::A(a) => a.update(),
            FSM::B(b) => b.update(),
            FSM::C(c) => c.update(),
        }
    }
   
    fn run(&mut self) {
        // Update the current state
        self.update();
       
        // Checks whether conditions are valid after the update
        match self {
            FSM::A(a) => {
                // Checks the A -> B condition
                if let Some(e) = a.check() {
                    // If the condition going from A to B is valid
                    // Then we change the FSM current state to B
                    *self = FSM::B(e);
                }
            },
            FSM::B(b) => {
                // Checks the B -> A condition
                if let Some(e) = b.check() {
                    *self = FSM::A(e);
                // Checks the B -> C condition
                } else if let Some(e) = b.check() {
                    *self = FSM::C(e);
                }
            },
            FSM::C(c) => {
                // Checks the C -> A condition
                if let Some(e) = c.check() {
                    *self = FSM::A(e);
                }
            },
        }
    }
}

fn main() {
    let mut fsm = FSM::new();
   
    for _i in 1..10 {
        fsm.run();
    }
}
*/
