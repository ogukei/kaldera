
use std::sync::Arc;
use std::sync::Mutex;

use crate::ffi::xcb::*;
use crate::cores::{InputEvent, InputKeyEvent};

use xcb_event_mask_t::*;

struct XcbInputInterpreterState {
    x_last: Option<i16>,
    y_last: Option<i16>,
    keys: [bool; 256],
}

impl Default for XcbInputInterpreterState {
    fn default() -> Self {
        Self {
            x_last: None,
            y_last: None,
            keys: [false; 256],
        }
    }
}

struct KeyCodes {
    pub w: u8,
    pub a: u8,
    pub s: u8,
    pub d: u8,
    pub shift_left: u8,
    pub control_left: u8,
}

impl KeyCodes {
    fn new(keymap: &Arc<XcbKeymap>) -> Self {
        KeyCodes {
            w: keymap.code_of_key(XcbKey::W).unwrap_or(0),
            a: keymap.code_of_key(XcbKey::A).unwrap_or(0),
            s: keymap.code_of_key(XcbKey::S).unwrap_or(0),
            d: keymap.code_of_key(XcbKey::D).unwrap_or(0),
            shift_left: keymap.code_of_key(XcbKey::ShiftLeft).unwrap_or(0),
            control_left: keymap.code_of_key(XcbKey::ControlLeft).unwrap_or(0),
        }
    }
}

pub struct XcbInputInterpreter {
    window: Arc<XcbWindow>,
    state: Arc<Mutex<XcbInputInterpreterState>>,
    key_codes: KeyCodes,
}

impl XcbInputInterpreter {
    pub fn new(window: &Arc<XcbWindow>) -> Self {
        let keymap = window.connection().generate_keymap().unwrap();
        let key_codes = KeyCodes::new(&keymap);
        Self {
            window: Arc::clone(window),
            state: Arc::new(Mutex::new(Default::default())),
            key_codes,
        }
    }

    pub fn next(&self) -> Option<Vec<InputEvent>> {
        let events = self.window.events();
        let keys = self.keys(events.as_ref());
        let motions = self.motions(events.as_ref());
        let events: Vec<InputEvent> = keys.into_iter().chain(motions).collect();
        if events.is_empty() {
            None
        } else {
            Some(events)
        }
    }

    fn keys(&self, events: Option<&Vec<XcbEvent>>) -> Option<InputEvent> {
        let event_types: Vec<&XcbEventType> = events.iter()
            .flat_map(|v| v.iter())
            .filter_map(|v| v.event_type())
            .collect();
        let mut state = self.state.lock().unwrap();
        let events = event_types.iter()
            .filter_map(|v| match v {
                XcbEventType::KeyPress(event) => Some((event.detail, true)),
                XcbEventType::KeyRelease(event) => Some((event.detail, false)),
                _ => None,
            });
        for (key, press) in events {
            state.keys[key as usize] = press;
        }
        let forward = 1.0 * (if state.keys[self.key_codes.w as usize] { 1.0 } else { 0.0 });
        let backward = -1.0 * (if state.keys[self.key_codes.s as usize] { 1.0 } else { 0.0 });
        let right = 1.0 * (if state.keys[self.key_codes.a as usize] { 1.0 } else { 0.0 });
        let left = -1.0 * (if state.keys[self.key_codes.d as usize] { 1.0 } else { 0.0 });
        let x = right + left;
        let y = forward + backward;
        if x == 0.0 && y == 0.0 {
            None
        } else {
            let is_shift = state.keys[self.key_codes.shift_left as usize];
            let is_control = state.keys[self.key_codes.control_left as usize];
            let event = InputKeyEvent { x, y, is_shift, is_control };
            Some(InputEvent::Key(event))
        }
    }

    fn motions(&self, events: Option<&Vec<XcbEvent>>) -> Option<InputEvent> {
        let events = if let Some(events) = events { events } else { return None };
        let event_types: Vec<&XcbEventType> = events.iter()
            .filter_map(|v| v.event_type())
            .collect();
        let mut state = self.state.lock().unwrap();
        let motion = event_types.iter()
            .filter_map(|v| match v {
                XcbEventType::MotionNotify(event) => Some(event),
                _ => None,
            })
            .fold((state.x_last, state.y_last, 0i64, 0i64), |acc, event| {
                if (event.state & XCB_EVENT_MASK_BUTTON_1_MOTION as u16) == 0 {
                    return (Some(event.event_x), Some(event.event_y), acc.2, acc.3)
                }
                let (x_last, y_last, acc_x, acc_y) = acc;
                let dx = x_last
                    .map(|x| event.event_x - x)
                    .map(|dx| dx as i64)
                    .map(|dx| dx + acc_x)
                    .unwrap_or(0);
                let dy = y_last
                    .map(|y| event.event_y - y)
                    .map(|dy| dy as i64)
                    .map(|dy| dy + acc_y)
                    .unwrap_or(0);
                (Some(event.event_x), Some(event.event_y), dx, dy)
            });
        let (x_last, y_last, dx, dy) = motion;
        state.x_last = x_last;
        state.y_last = y_last;
        if dx == 0 && dy == 0 {
            None
        } else {
            Some(InputEvent::MoveDelta(dx as f32, dy as f32))
        }
    }
}
