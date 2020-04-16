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
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
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
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
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
        sphere: &mut HiPSSphere,
        catalog: &mut Catalog,
        grid: &mut ProjetedGrid,
        // Viewport
        viewport: &mut ViewPort,
        // User events
        events: &EventManager
    ) -> Option<E>
    where T<Self, E>: Transition<S=Self, E=E> {
        T::<Self, E>::condition::<P>(&self, sphere, catalog, grid, viewport, events)
    }
}

pub trait FiniteStateMachine {
    fn init() -> Self;
}

mod moving;
pub use moving::UserMoveSphere;
mod zooming;
pub use zooming::UserZoom;
mod location;
pub use location::MoveSphere;