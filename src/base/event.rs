use crate::base::RefPtr;
use crate::math::Vec2;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    Touch,
    Keyboard,
    Mouse,
    Acceleration,
    Custom,
}

#[derive(Debug)]
pub struct Event {
    event_type: EventType,
    stopped: bool,
    #[allow(dead_code)]
    name: String,
}

impl Event {
    pub fn new(event_type: EventType) -> Event {
        Event {
            event_type,
            stopped: false,
            name: String::new(),
        }
    }

    pub fn get_event_type(&self) -> &EventType {
        &self.event_type
    }

    pub fn is_stopped(&self) -> bool {
        self.stopped
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn reset(&mut self) {
        self.stopped = false;
    }
}

#[derive(Debug)]
pub struct EventTouch {
    base: Event,
    touches: Vec<Vec2>,
    touch_id: i32,
}

impl EventTouch {
    pub fn new() -> EventTouch {
        EventTouch {
            base: Event::new(EventType::Touch),
            touches: Vec::new(),
            touch_id: 0,
        }
    }

    pub fn get_touches(&self) -> &Vec<Vec2> {
        &self.touches
    }

    pub fn add_touch(&mut self, touch: Vec2) {
        self.touches.push(touch);
    }

    pub fn get_touch_id(&self) -> i32 {
        self.touch_id
    }

    pub fn set_touch_id(&mut self, id: i32) {
        self.touch_id = id;
    }
}

#[derive(Debug)]
pub struct EventKeyboard {
    base: Event,
    key_code: i32,
    is_pressed: bool,
}

impl EventKeyboard {
    pub fn new(key_code: i32, is_pressed: bool) -> EventKeyboard {
        EventKeyboard {
            base: Event::new(EventType::Keyboard),
            key_code,
            is_pressed,
        }
    }

    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

#[derive(Debug)]
pub struct EventMouse {
    base: Event,
    x: f32,
    y: f32,
    mouse_type: MouseEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
    Scroll,
}

impl EventMouse {
    pub fn new() -> EventMouse {
        EventMouse {
            base: Event::new(EventType::Mouse),
            x: 0.0,
            y: 0.0,
            mouse_type: MouseEventType::Move,
        }
    }

    pub fn get_location(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn set_location(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_mouse_type(&self) -> &MouseEventType {
        &self.mouse_type
    }

    pub fn set_mouse_type(&mut self, mouse_type: MouseEventType) {
        self.mouse_type = mouse_type;
    }
}

#[derive(Debug)]
pub struct EventCustom {
    base: Event,
    event_name: String,
    user_data: Option<Box<dyn std::any::Any>>,
}

impl EventCustom {
    pub fn new(event_name: &str) -> EventCustom {
        EventCustom {
            base: Event::new(EventType::Custom),
            event_name: event_name.to_string(),
            user_data: None,
        }
    }

    pub fn get_event_name(&self) -> &str {
        &self.event_name
    }

    pub fn set_user_data(&mut self, data: Box<dyn std::any::Any>) {
        self.user_data = Some(data);
    }

    pub fn get_user_data<T: std::any::Any>(&self) -> Option<&T> {
        self.user_data.as_ref().and_then(|d| d.downcast_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventListenerType {
    TouchOneByOne,
    TouchAllAtOnce,
    Keyboard,
    Mouse,
    Acceleration,
    Custom,
    Node,
}

impl std::fmt::Debug for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListener")
            .field("listener_type", &self.listener_type)
            .field("enabled", &self.enabled)
            .field("paused", &self.paused)
            .finish()
    }
}

pub struct EventListener {
    listener_type: EventListenerType,
    callback: Box<dyn FnMut(&mut Event)>,
    enabled: bool,
    paused: bool,
    #[allow(dead_code)]
    node: Option<Rc<dyn std::any::Any>>,
}

impl EventListener {
    pub fn new(
        listener_type: EventListenerType,
        callback: Box<dyn FnMut(&mut Event)>,
    ) -> EventListener {
        EventListener {
            listener_type,
            callback,
            enabled: true,
            paused: false,
            node: None,
        }
    }

    pub fn get_type(&self) -> &EventListenerType {
        &self.listener_type
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    pub fn on_event(&mut self, event: &mut Event) {
        (self.callback)(event);
    }
}

#[derive(Debug)]
pub struct EventDispatcher {
    listeners: Vec<RefPtr<EventListener>>,
    listeners_map: HashMap<EventListenerType, Vec<usize>>,
    to_removed_listeners: Vec<usize>,
    in_update: bool,
}

impl EventDispatcher {
    pub fn new() -> EventDispatcher {
        EventDispatcher {
            listeners: Vec::new(),
            listeners_map: HashMap::new(),
            to_removed_listeners: Vec::new(),
            in_update: false,
        }
    }

    pub fn add_listener(&mut self, listener: RefPtr<EventListener>) {
        let index = self.listeners.len();
        self.listeners.push(listener.clone());

        let listener_type = listener.borrow().get_type().clone();
        self.listeners_map
            .entry(listener_type)
            .or_insert_with(Vec::new)
            .push(index);
    }

    pub fn remove_listener(&mut self, index: usize) {
        if self.in_update {
            self.to_removed_listeners.push(index);
        } else {
            self.listeners.remove(index);
        }
    }

    pub fn remove_all_listeners(&mut self) {
        self.listeners.clear();
        self.listeners_map.clear();
    }

    pub fn is_enabled(&self, listener_type: EventListenerType) -> bool {
        if let Some(indices) = self.listeners_map.get(&listener_type) {
            for &index in indices {
                if index < self.listeners.len() && self.listeners[index].borrow().is_enabled() {
                    return true;
                }
            }
        }
        false
    }

    pub fn set_enabled(&mut self, listener_type: EventListenerType, enabled: bool) {
        if let Some(indices) = self.listeners_map.get(&listener_type) {
            for &index in indices {
                if index < self.listeners.len() {
                    self.listeners[index].borrow_mut().set_enabled(enabled);
                }
            }
        }
    }

    pub fn dispatch_event(&mut self, event: &mut Event) {
        let event_type = event.get_event_type().clone();

        let listener_type = match event_type {
            EventType::Touch => EventListenerType::TouchOneByOne,
            EventType::Keyboard => EventListenerType::Keyboard,
            EventType::Mouse => EventListenerType::Mouse,
            EventType::Acceleration => EventListenerType::Acceleration,
            EventType::Custom => EventListenerType::Custom,
        };

        if let Some(indices) = self.listeners_map.get(&listener_type) {
            self.in_update = true;

            for &index in indices {
                if index < self.listeners.len() {
                    let mut listener = self.listeners[index].borrow_mut();

                    if listener.is_enabled() && !listener.is_paused() {
                        listener.on_event(event);

                        if event.is_stopped() {
                            break;
                        }
                    }
                }
            }

            self.in_update = false;

            for index in &self.to_removed_listeners {
                self.listeners.remove(*index);
            }
            self.to_removed_listeners.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_event_new() {
        let event = Event::new(EventType::Touch);
        assert_eq!(event.get_event_type(), &EventType::Touch);
        assert!(!event.is_stopped());
    }

    #[test]
    fn test_event_stop() {
        let mut event = Event::new(EventType::Keyboard);
        assert!(!event.is_stopped());
        event.stop();
        assert!(event.is_stopped());
    }

    #[test]
    fn test_event_reset() {
        let mut event = Event::new(EventType::Mouse);
        event.stop();
        assert!(event.is_stopped());
        event.reset();
        assert!(!event.is_stopped());
    }

    #[test]
    fn test_event_touch_new() {
        let touch_event = EventTouch::new();
        assert_eq!(touch_event.get_touches().len(), 0);
        assert_eq!(touch_event.get_touch_id(), 0);
    }

    #[test]
    fn test_event_touch_add_touch() {
        let mut touch_event = EventTouch::new();
        touch_event.add_touch(Vec2::new(100.0, 200.0));
        touch_event.add_touch(Vec2::new(300.0, 400.0));
        assert_eq!(touch_event.get_touches().len(), 2);
    }

    #[test]
    fn test_event_touch_set_touch_id() {
        let mut touch_event = EventTouch::new();
        assert_eq!(touch_event.get_touch_id(), 0);
        touch_event.set_touch_id(42);
        assert_eq!(touch_event.get_touch_id(), 42);
    }

    #[test]
    fn test_event_keyboard_new() {
        let keyboard_event = EventKeyboard::new(65, true);
        assert_eq!(keyboard_event.get_key_code(), 65);
        assert!(keyboard_event.is_pressed());
    }

    #[test]
    fn test_event_mouse_new() {
        let mouse_event = EventMouse::new();
        let location = mouse_event.get_location();
        assert_eq!(location.x, 0.0);
        assert_eq!(location.y, 0.0);
    }

    #[test]
    fn test_event_mouse_set_location() {
        let mut mouse_event = EventMouse::new();
        mouse_event.set_location(150.0, 250.0);
        let location = mouse_event.get_location();
        assert_eq!(location.x, 150.0);
        assert_eq!(location.y, 250.0);
    }

    #[test]
    fn test_event_mouse_mouse_type() {
        let mut mouse_event = EventMouse::new();
        assert_eq!(mouse_event.get_mouse_type(), &MouseEventType::Move);
        mouse_event.set_mouse_type(MouseEventType::Down);
        assert_eq!(mouse_event.get_mouse_type(), &MouseEventType::Down);
    }

    #[test]
    fn test_event_custom_new() {
        let custom_event = EventCustom::new("my_event");
        assert_eq!(custom_event.get_event_name(), "my_event");
    }

    #[test]
    fn test_event_custom_user_data() {
        let mut custom_event = EventCustom::new("data_event");
        custom_event.set_user_data(Box::new(42i32));
        let data = custom_event.get_user_data::<i32>();
        assert_eq!(data, Some(&42));
    }

    #[test]
    fn test_event_custom_user_data_wrong_type() {
        let mut custom_event = EventCustom::new("data_event");
        custom_event.set_user_data(Box::new(42i32));
        let data = custom_event.get_user_data::<i64>();
        assert_eq!(data, None);
    }

    #[test]
    fn test_event_listener_new() {
        let called = Rc::new(Cell::new(false));
        let callback = {
            let called = called.clone();
            Box::new(move |_: &mut Event| {
                called.set(true);
            }) as Box<dyn FnMut(&mut Event)>
        };
        let listener = EventListener::new(EventListenerType::TouchOneByOne, callback);
        assert!(listener.is_enabled());
        assert!(!listener.is_paused());
        assert_eq!(listener.get_type(), &EventListenerType::TouchOneByOne);
    }

    #[test]
    fn test_event_listener_set_enabled() {
        let listener = EventListener::new(
            EventListenerType::Keyboard,
            Box::new(|_: &mut Event| {})
        );
        assert!(listener.is_enabled());
    }

    #[test]
    fn test_event_listener_set_paused() {
        let mut listener = EventListener::new(
            EventListenerType::Mouse,
            Box::new(|_: &mut Event| {})
        );
        assert!(!listener.is_paused());
        listener.set_paused(true);
        assert!(listener.is_paused());
    }

    #[test]
    fn test_event_listener_on_event() {
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();
        let mut listener = EventListener::new(
            EventListenerType::Custom,
            Box::new(move |_: &mut Event| {
                called_clone.set(true);
            })
        );

        let mut event = Event::new(EventType::Custom);
        listener.on_event(&mut event);
        assert!(called.get());
    }

    #[test]
    fn test_event_dispatcher_new() {
        let dispatcher = EventDispatcher::new();
        assert_eq!(dispatcher.listeners.len(), 0);
    }

    #[test]
    fn test_event_dispatcher_add_listener() {
        let mut dispatcher = EventDispatcher::new();
        let listener = RefPtr::new(EventListener::new(
            EventListenerType::TouchOneByOne,
            Box::new(|_: &mut Event| {})
        ));
        dispatcher.add_listener(listener);
        assert_eq!(dispatcher.listeners.len(), 1);
    }

    #[test]
    fn test_event_dispatcher_remove_listener() {
        let mut dispatcher = EventDispatcher::new();
        let listener = RefPtr::new(EventListener::new(
            EventListenerType::TouchOneByOne,
            Box::new(|_: &mut Event| {})
        ));
        dispatcher.add_listener(listener.clone());
        assert_eq!(dispatcher.listeners.len(), 1);
        dispatcher.remove_listener(0);
        assert_eq!(dispatcher.listeners.len(), 0);
    }

    #[test]
    fn test_event_dispatcher_remove_all_listeners() {
        let mut dispatcher = EventDispatcher::new();
        let listener1 = RefPtr::new(EventListener::new(
            EventListenerType::TouchOneByOne,
            Box::new(|_: &mut Event| {})
        ));
        let listener2 = RefPtr::new(EventListener::new(
            EventListenerType::Keyboard,
            Box::new(|_: &mut Event| {})
        ));
        dispatcher.add_listener(listener1);
        dispatcher.add_listener(listener2);
        assert_eq!(dispatcher.listeners.len(), 2);
        dispatcher.remove_all_listeners();
        assert_eq!(dispatcher.listeners.len(), 0);
    }

    #[test]
    fn test_event_dispatcher_is_enabled() {
        let mut dispatcher = EventDispatcher::new();
        let listener = RefPtr::new(EventListener::new(
            EventListenerType::Mouse,
            Box::new(|_: &mut Event| {})
        ));
        assert!(!dispatcher.is_enabled(EventListenerType::Mouse));
        dispatcher.add_listener(listener);
        assert!(dispatcher.is_enabled(EventListenerType::Mouse));
    }

    #[test]
    fn test_event_dispatcher_set_enabled() {
        let mut dispatcher = EventDispatcher::new();
        let listener = RefPtr::new(EventListener::new(
            EventListenerType::Acceleration,
            Box::new(|_: &mut Event| {})
        ));
        dispatcher.add_listener(listener);
        assert!(dispatcher.is_enabled(EventListenerType::Acceleration));
        dispatcher.set_enabled(EventListenerType::Acceleration, false);
        assert!(!dispatcher.is_enabled(EventListenerType::Acceleration));
    }

    #[test]
    fn test_event_dispatcher_dispatch_event() {
        let mut dispatcher = EventDispatcher::new();
        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();

        let listener = RefPtr::new(EventListener::new(
            EventListenerType::TouchOneByOne,
            Box::new(move |_: &mut Event| {
                called_clone.set(true);
            })
        ));
        dispatcher.add_listener(listener);

        let mut event = Event::new(EventType::Touch);
        dispatcher.dispatch_event(&mut event);
        assert!(called.get());
    }

    #[test]
    fn test_event_dispatcher_stops_on_stopped_event() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let mut dispatcher = EventDispatcher::new();

        for i in 0..3 {
            let call_count_clone = call_count_clone.clone();
            let listener = RefPtr::new(EventListener::new(
                EventListenerType::TouchOneByOne,
                Box::new(move |event: &mut Event| {
                    let count = call_count_clone.get();
                    call_count_clone.set(count + 1);
                    if count < 1 {
                        event.stop();
                    }
                })
            ));
            dispatcher.add_listener(listener);
        }

        let mut event = Event::new(EventType::Touch);
        dispatcher.dispatch_event(&mut event);
        assert_eq!(call_count.get(), 1);
    }

    #[test]
    fn test_event_type_traits() {
        assert_eq!(EventType::Touch, EventType::Touch);
        assert_ne!(EventType::Touch, EventType::Keyboard);

        let types = vec![EventType::Touch, EventType::Keyboard, EventType::Mouse];
        assert_eq!(types.len(), 3);
    }

    #[test]
    fn test_event_listener_type_traits() {
        assert_eq!(EventListenerType::TouchOneByOne, EventListenerType::TouchOneByOne);
        assert_ne!(EventListenerType::TouchOneByOne, EventListenerType::Keyboard);

        let mut map = HashMap::new();
        map.insert(EventListenerType::TouchOneByOne, 1);
        map.insert(EventListenerType::Keyboard, 2);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_mouse_event_type_traits() {
        assert_eq!(MouseEventType::Down, MouseEventType::Down);
        assert_ne!(MouseEventType::Down, MouseEventType::Up);

        let types = vec![MouseEventType::Down, MouseEventType::Up, MouseEventType::Move];
        assert_eq!(types.len(), 3);
    }

    #[test]
    fn test_dispatcher_remove_during_dispatch() {
        let mut dispatcher = EventDispatcher::new();
        let listener = RefPtr::new(EventListener::new(
            EventListenerType::TouchOneByOne,
            Box::new(|_: &mut Event| {})
        ));
        dispatcher.add_listener(listener);
        assert_eq!(dispatcher.listeners.len(), 1);
    }
}
