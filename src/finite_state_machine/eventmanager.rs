trait Event {
    const KEY_NAME: &'static str;
    type Data;

    fn to(data: Self::Data) -> EventType;
    fn get(event: &EventType) -> &Self::Data;
}

struct MouseLeftButtonPressed;
struct MouseLeftButtonReleased;
struct MouseMove;
struct KeyboardPressed;

impl Event for MouseLeftButtonPressed {
    type Data = (f32, f32);
    const KEY_NAME: &'static str = "MouseLeftButtonPressed";
    
    fn to(data: Self::Data) -> EventType {
        EventType::MouseLeftButtonPressed(data)
    }
    fn get(event: &EventType) -> &Self::Data {
        match event {
            EventType::MouseLeftButtonPressed(data) => data,
            _ => unreachable!()
        }
    }
}
impl Event for MouseLeftButtonReleased {
    type Data = (f32, f32);
    const KEY_NAME: &'static str = "MouseLeftButtonReleased";
    
    fn to(data: Self::Data) -> EventType {
        EventType::MouseLeftButtonReleased(data)
    }
    fn get(event: &EventType) -> &Self::Data {
        match event {
            EventType::MouseLeftButtonReleased(data) => data,
            _ => unreachable!()
        }
    }
}
impl Event for MouseMove {
    type Data = (f32, f32);
    const KEY_NAME: &'static str = "MouseMove";
    
    fn to(data: Self::Data) -> EventType {
        EventType::MouseMove(data)
    }
    fn get(event: &EventType) -> &Self::Data {
        match event {
            EventType::MouseMove(data) => data,
            _ => unreachable!()
        }
    }
}
impl Event for KeyboardPressed {
    type Data = &'static str;
    const KEY_NAME: &'static str = "KeyboardPressed";
    
    fn to(data: Self::Data) -> EventType {
        EventType::KeyboardPressed(data)
    }
    fn get(event: &EventType) -> &Self::Data {
        match event {
            EventType::KeyboardPressed(data) => data,
            _ => unreachable!()
        }
    }
}

// An enum of the different possible user interactions
enum EventType {
    MouseLeftButtonPressed((f32, f32)),
    MouseLeftButtonReleased((f32, f32)),
    MouseMove((f32, f32)),
    KeyboardPressed(&'static str),
}

use std::collections::HashMap;
struct EventManager(HashMap<&'static str, Option<EventType>>);

impl EventManager {
    pub fn new() -> EventManager {
        let mut manager = EventManager(HashMap::new());

        manager.insert_new_event::<MouseLeftButtonPressed>();
        manager.insert_new_event::<MouseLeftButtonReleased>();
        manager.insert_new_event::<MouseMove>();
        manager.insert_new_event::<KeyboardPressed>();

        manager
    }
    
    // Private method for inserting new events in the manager
    fn insert_new_event<E: Event>(&mut self) {
        self.0.insert(E::KEY_NAME, None);
    }

    pub fn enable<E: Event>(&mut self, data: E::Data) {
        let val = E::cast(data);

        let v = self.0
            .get_mut(E::KEY_NAME)
            .unwrap();
        *v = Some(val);
    }

    pub fn disable<E: Event>(&mut self) {
        let v = self.0
            .get_mut(E::KEY_NAME)
            .unwrap();
        *v = None;
    }

    pub fn get<E: Event>(&self) -> Option<&E::Data> {
        if let Some(event) = self.0.get(E::KEY_NAME).unwrap() {
            Some(E::get(event))
        } else {
            None
        }
    }
}