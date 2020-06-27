
use std::sync::Arc;
use std::sync::Mutex;

use crate::ffi::xcb::*;
use crate::cores::InputEvent;

use xcb_event_mask_t::*;

struct XcbInputInterpreterState {
    x_last: Option<i16>,
    y_last: Option<i16>,
}

impl Default for XcbInputInterpreterState {
    fn default() -> Self {
        Self {
            x_last: None,
            y_last: None,
        }
    }
}

pub struct XcbInputInterpreter {
    window: Arc<XcbWindow>,
    state: Arc<Mutex<XcbInputInterpreterState>>,
}

impl XcbInputInterpreter {
    pub fn new(window: &Arc<XcbWindow>) -> Self {
        Self {
            window: Arc::clone(window),
            state: Arc::new(Mutex::new(Default::default()))
        }
    }

    pub fn next(&self) -> Option<InputEvent> {
        let events = self.window.events();
        let events = if let Some(events) = events {
            events
        } else {
            return None
        };
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
