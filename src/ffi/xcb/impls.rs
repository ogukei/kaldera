
use super::types::*;

use std::sync::Arc;

use libc::{c_void, c_int};
use std::ptr;
use std::ffi::CString;

pub struct XcbConnection {
    handle: *mut xcb_connection_t,
    screen: *mut xcb_screen_t,
}

impl XcbConnection {
    pub fn new() -> Arc<XcbConnection> {
        let connection = unsafe { Self::init() };
        Arc::new(connection)
    }

    unsafe fn init() -> Self {
        let mut scr: c_int = 0;
        let connection = xcb_connect(ptr::null(), &mut scr);
        let setup = xcb_get_setup(connection);
        let mut iterator = xcb_setup_roots_iterator(setup);
        while scr > 0 {
            xcb_screen_next(&mut iterator);
            scr -= 1;
        }
        let screen = iterator.data;
        XcbConnection {
            handle: connection,
            screen: screen,
        }
    }

    #[inline]
    unsafe fn screen(&self) -> &xcb_screen_t {
        &*self.screen
    }

    #[inline]
    pub fn handle(&self) -> *mut xcb_connection_t {
        self.handle
    }
}

impl Drop for XcbConnection {
    fn drop(&mut self) {
        unsafe {
            xcb_disconnect(self.handle);
        }
    }
}

pub struct XcbWindow {
    connection: Arc<XcbConnection>,
    handle: xcb_window_t,
}

impl XcbWindow {
    pub fn new(connection: &Arc<XcbConnection>) -> Arc<XcbWindow> {
        let window = unsafe { Self::init(connection) };
        Arc::new(window)
    }

    unsafe fn init(connection: &Arc<XcbConnection>) -> Self {
        let conn = connection.handle();
        let screen = connection.screen();
        let window = xcb_generate_id(connection.handle());
        let value_mask: u32 = xcb_cw_t::XCB_CW_BACK_PIXEL as u32 | xcb_cw_t::XCB_CW_EVENT_MASK as u32;
        let mut value_list: [u32; 32] = [0u32; 32];
        value_list[0] = screen.black_pixel;
        let x: i16 = 0;
        let y: i16 = 0;
        let width: u16 = 400;
        let height: u16 = 400;
        value_list[1] = xcb_event_mask_t::default();
        xcb_create_window(conn,
            XCB_COPY_FROM_PARENT as u8,
            window,
            screen.root,
            x,
            y,
            width,
            height,
            0,
            xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual,
            value_mask,
            value_list.as_ptr() as *const c_void);
        Self::change_title_property(conn, window, CString::new("Karst").unwrap());
        xcb_map_window(conn, window);
        XcbWindow {
            connection: Arc::clone(connection),
            handle: window,
        }
    }

    pub fn change_title(&self, title: impl Into<String>) -> Result<(), std::ffi::NulError> {
        unsafe {
            Self::change_title_property(
                self.connection.handle(), 
                self.handle,
                CString::new(title.into())?);
        }
        Ok(())
    }

    unsafe fn change_title_property(conn: *mut xcb_connection_t, window: xcb_window_t, st: CString) {
        xcb_change_property(
            conn,
            xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
            window,
            xcb_atom_enum_t::XCB_ATOM_WM_NAME as u32,
            xcb_atom_enum_t::XCB_ATOM_STRING as u32,
            8,
            st.to_bytes().len() as u32, 
            st.as_ptr() as *const c_void,
        );
    }

    pub fn flush(&self) {
        unsafe {
            xcb_flush(self.connection.handle());
        }
    }

    pub fn connection(&self) -> &Arc<XcbConnection> {
        &self.connection
    }

    pub fn handle(&self) -> xcb_window_t {
        self.handle
    }

    pub fn events(&self) -> Option<Vec<XcbEvent>> {
        unsafe {
            let connection = self.connection();
            let event = xcb_poll_for_event(connection.handle());
            if event == std::ptr::null_mut() {
                // no events in most cases. 
                // returns immediately with no allocations
                return None
            }
            let event = XcbEvent::new(event, connection);
            let mut events = vec![event];
            loop {
                let event = xcb_poll_for_event(connection.handle());
                if event == std::ptr::null_mut() {
                    return Some(events)
                }
                let event = XcbEvent::new(event, connection);
                events.push(event);
            }
        }
    }
}

pub struct XcbEvent<'a> {
    connection: &'a Arc<XcbConnection>,
    handle: *mut xcb_generic_event_t,
    event_type: Option<XcbEventType<'a>>,
}

impl<'a> XcbEvent<'a> {
    fn new(handle: *mut xcb_generic_event_t, connection: &'a Arc<XcbConnection>) -> Self {
        unsafe {
            let event_type = handle.as_ref::<'a>()
                .map(|v| XcbEventType::new(v))
                .flatten();
            let event = XcbEvent {
                connection,
                handle,
                event_type,
            };
            event
        }
    }

    pub fn event_type(&self) -> Option<&XcbEventType> {
        self.event_type.as_ref()
    }
}

impl<'a> Drop for XcbEvent<'a> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.handle as *mut c_void);
        }
    }
}

pub enum XcbEventType<'a> {
    KeyPress(&'a xcb_key_press_event_t),
    KeyRelease(&'a xcb_key_press_event_t),
    ButtonPress(&'a xcb_button_press_event_t),
    ButtonRelease(&'a xcb_button_press_event_t),
    MotionNotify(&'a xcb_motion_notify_event_t),
    ConfigureNotify(&'a xcb_configure_notify_event_t),
}

impl<'a> XcbEventType<'a> {
    fn new(handle: &'a xcb_generic_event_t) -> Option<Self> {
        unsafe {
            let event_type = match handle.response_type & 0x7f {
                XCB_KEY_PRESS => Self::KeyPress(std::mem::transmute(handle)),
                XCB_KEY_RELEASE => Self::KeyRelease(std::mem::transmute(handle)),
                XCB_BUTTON_PRESS => Self::ButtonPress(std::mem::transmute(handle)),
                XCB_BUTTON_RELEASE => Self::ButtonRelease(std::mem::transmute(handle)),
                XCB_MOTION_NOTIFY => Self::MotionNotify(std::mem::transmute(handle)),
                XCB_CONFIGURE_NOTIFY => Self::ConfigureNotify(std::mem::transmute(handle)),
                _ => return None,
            };
            Some(event_type)
        }
    }
}

impl xcb_event_mask_t {
    fn default() -> u32 {
        Self::XCB_EVENT_MASK_EXPOSURE as u32
            | Self::XCB_EVENT_MASK_KEY_PRESS as u32
            | Self::XCB_EVENT_MASK_KEY_RELEASE as u32
            | Self::XCB_EVENT_MASK_BUTTON_PRESS as u32
            | Self::XCB_EVENT_MASK_BUTTON_RELEASE as u32
            | Self::XCB_EVENT_MASK_POINTER_MOTION as u32
            | Self::XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32
    }
}
