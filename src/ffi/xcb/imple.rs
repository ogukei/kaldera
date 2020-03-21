
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
    fn handle(&self) -> *mut xcb_connection_t {
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
}