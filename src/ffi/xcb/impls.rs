#![allow(dead_code)]

use super::types::*;

use std::sync::Arc;

use libc::{c_void, c_int};
use std::ptr;
use std::ffi::CString;
use std::collections::HashMap;

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

    pub fn generate_keymap(&self) -> Option<Arc<XcbKeymap>> {
        XcbKeymap::new(self)
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
    pub fn new(connection: &Arc<XcbConnection>, width: u16, height: u16) -> Arc<XcbWindow> {
        let window = unsafe { Self::init(connection, width, height) };
        Arc::new(window)
    }

    unsafe fn init(connection: &Arc<XcbConnection>, width: u16, height: u16) -> Self {
        let conn = connection.handle();
        let screen = connection.screen();
        let window = xcb_generate_id(connection.handle());
        let value_mask: u32 = xcb_cw_t::XCB_CW_BACK_PIXEL as u32 | xcb_cw_t::XCB_CW_EVENT_MASK as u32;
        let mut value_list: [u32; 32] = [0u32; 32];
        value_list[0] = screen.black_pixel;
        let x: i16 = 0;
        let y: i16 = 0;
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
        Self::change_title_property(conn, window, CString::new("Kaldera").unwrap());
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

pub enum XcbKey {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ShiftLeft,
    ControlLeft,
}

impl XcbKey {
    fn symbol(&self) -> xcb_keysym_t {
        match &self {
            Self::A => XK_a,
            Self::B => XK_b,
            Self::C => XK_c,
            Self::D => XK_d,
            Self::E => XK_e,
            Self::F => XK_f,
            Self::G => XK_g,
            Self::H => XK_h,
            Self::I => XK_i,
            Self::J => XK_j,
            Self::K => XK_k,
            Self::L => XK_l,
            Self::M => XK_m,
            Self::N => XK_n,
            Self::O => XK_o,
            Self::P => XK_p,
            Self::Q => XK_q,
            Self::R => XK_r,
            Self::S => XK_s,
            Self::T => XK_t,
            Self::U => XK_u,
            Self::V => XK_v,
            Self::W => XK_w,
            Self::X => XK_x,
            Self::Y => XK_y,
            Self::Z => XK_z,
            Self::ShiftLeft => XK_Shift_L,
            Self::ControlLeft => XK_Control_L,
        }
    }
}

pub struct XcbKeymap {
    symbol_to_code: HashMap<xcb_keysym_t, xcb_keycode_t>,
}

impl XcbKeymap {
    fn new(connection: &XcbConnection) -> Option<Arc<Self>> {
        unsafe {
            let conn = connection.handle();
            let setup = xcb_get_setup(conn).as_ref()?;
            // gets keyboard_mapping
            let code_offset = setup.min_keycode as isize;
            let code_count = (setup.max_keycode as isize) - code_offset + 1;
            let cookie = xcb_get_keyboard_mapping(conn,
                code_offset as xcb_keycode_t, 
                code_count as u8);
            let reply = xcb_get_keyboard_mapping_reply(conn, cookie, std::ptr::null_mut());
            // iterate through mapping and store symbol and code pairs
            let mut symbol_to_code: HashMap<xcb_keysym_t, xcb_keycode_t> = HashMap::new();
            if let Some(keyboard_mapping) = reply.as_ref() {
                let syms_per_code = keyboard_mapping.keysyms_per_keycode as isize;
                let syms = xcb_get_keyboard_mapping_keysyms(reply);
                for i_code_base in 0..code_count {
                    let syms = syms.offset(i_code_base * syms_per_code);
                    let code = (i_code_base + code_offset) as xcb_keycode_t;
                    for i_sym in 0..syms_per_code {
                        let symbol = syms.offset(i_sym).as_ref().unwrap();
                        let symbol = *symbol;
                        if symbol == 0 {
                            break
                        }
                        if symbol == XK_Super_L || symbol == XK_Super_R {
                            break
                        }
                        symbol_to_code.insert(symbol, code);
                    }
                }
            }
            libc::free(reply as *mut c_void);
            let keymap = Self {
                symbol_to_code,
            };
            Some(Arc::new(keymap))
        }
    }

    pub fn code_of_key(&self, key: XcbKey) -> Option<xcb_keycode_t> {
        self.symbol_to_code.get(&key.symbol()).copied()
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
