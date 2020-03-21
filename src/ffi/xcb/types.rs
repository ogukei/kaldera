
use libc::{c_void, c_int, c_char, c_uint};

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
pub type xcb_window_t = u32;
pub type xcb_colormap_t = u32;
pub type xcb_visualid_t = u32;
pub type xcb_atom_t = u32;

pub const XCB_COPY_FROM_PARENT: u64 = 0;

#[repr(C)]
pub struct xcb_connection_t { _private: [u8; 0] }

#[repr(C)]
pub struct xcb_setup_t { _private: [u8; 0] }

// @see https://xcb.freedesktop.org/manual/structxcb__screen__t.html
#[repr(C)]
pub struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: xcb_colormap_t,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: xcb_visualid_t,
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_screen_iterator_t {
    pub data: *mut xcb_screen_t,
    pub rem: c_int,
    pub index: c_int,
}

// @see https://xcb.freedesktop.org/manual/structxcb__void__cookie__t.html
// @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L199
#[repr(C)]
pub struct xcb_void_cookie_t {
    sequence: c_uint,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub enum xcb_window_class_t {
    XCB_WINDOW_CLASS_COPY_FROM_PARENT = 0,
    XCB_WINDOW_CLASS_INPUT_OUTPUT = 1,
    XCB_WINDOW_CLASS_INPUT_ONLY = 2,
}

#[repr(C)]
pub enum xcb_cw_t {
    XCB_CW_BACK_PIXMAP = 1,
    XCB_CW_BACK_PIXEL = 2,
    XCB_CW_BORDER_PIXMAP = 4,
    XCB_CW_BORDER_PIXEL = 8,
    XCB_CW_BIT_GRAVITY = 16,
    XCB_CW_WIN_GRAVITY = 32,
    XCB_CW_BACKING_STORE = 64,
    XCB_CW_BACKING_PLANES = 128,
    XCB_CW_BACKING_PIXEL = 256,
    XCB_CW_OVERRIDE_REDIRECT = 512,
    XCB_CW_SAVE_UNDER = 1024,
    XCB_CW_EVENT_MASK = 2048,
    XCB_CW_DONT_PROPAGATE = 4096,
    XCB_CW_COLORMAP = 8192,
    XCB_CW_CURSOR = 16384,
}

#[repr(C)]
pub enum xcb_prop_mode_t {
    XCB_PROP_MODE_REPLACE = 0,
    XCB_PROP_MODE_PREPEND = 1,
    XCB_PROP_MODE_APPEND = 2,
}

#[repr(C)]
pub enum xcb_atom_enum_t {
    XCB_ATOM_NONE = 0,
    XCB_ATOM_PRIMARY = 1,
    XCB_ATOM_SECONDARY = 2,
    XCB_ATOM_ARC = 3,
    XCB_ATOM_ATOM = 4,
    XCB_ATOM_BITMAP = 5,
    XCB_ATOM_CARDINAL = 6,
    XCB_ATOM_COLORMAP = 7,
    XCB_ATOM_CURSOR = 8,
    XCB_ATOM_CUT_BUFFER0 = 9,
    XCB_ATOM_CUT_BUFFER1 = 10,
    XCB_ATOM_CUT_BUFFER2 = 11,
    XCB_ATOM_CUT_BUFFER3 = 12,
    XCB_ATOM_CUT_BUFFER4 = 13,
    XCB_ATOM_CUT_BUFFER5 = 14,
    XCB_ATOM_CUT_BUFFER6 = 15,
    XCB_ATOM_CUT_BUFFER7 = 16,
    XCB_ATOM_DRAWABLE = 17,
    XCB_ATOM_FONT = 18,
    XCB_ATOM_INTEGER = 19,
    XCB_ATOM_PIXMAP = 20,
    XCB_ATOM_POINT = 21,
    XCB_ATOM_RECTANGLE = 22,
    XCB_ATOM_RESOURCE_MANAGER = 23,
    XCB_ATOM_RGB_COLOR_MAP = 24,
    XCB_ATOM_RGB_BEST_MAP = 25,
    XCB_ATOM_RGB_BLUE_MAP = 26,
    XCB_ATOM_RGB_DEFAULT_MAP = 27,
    XCB_ATOM_RGB_GRAY_MAP = 28,
    XCB_ATOM_RGB_GREEN_MAP = 29,
    XCB_ATOM_RGB_RED_MAP = 30,
    XCB_ATOM_STRING = 31,
    XCB_ATOM_VISUALID = 32,
    XCB_ATOM_WINDOW = 33,
    XCB_ATOM_WM_COMMAND = 34,
    XCB_ATOM_WM_HINTS = 35,
    XCB_ATOM_WM_CLIENT_MACHINE = 36,
    XCB_ATOM_WM_ICON_NAME = 37,
    XCB_ATOM_WM_ICON_SIZE = 38,
    XCB_ATOM_WM_NAME = 39,
    XCB_ATOM_WM_NORMAL_HINTS = 40,
    XCB_ATOM_WM_SIZE_HINTS = 41,
    XCB_ATOM_WM_ZOOM_HINTS = 42,
    XCB_ATOM_MIN_SPACE = 43,
    XCB_ATOM_NORM_SPACE = 44,
    XCB_ATOM_MAX_SPACE = 45,
    XCB_ATOM_END_SPACE = 46,
    XCB_ATOM_SUPERSCRIPT_X = 47,
    XCB_ATOM_SUPERSCRIPT_Y = 48,
    XCB_ATOM_SUBSCRIPT_X = 49,
    XCB_ATOM_SUBSCRIPT_Y = 50,
    XCB_ATOM_UNDERLINE_POSITION = 51,
    XCB_ATOM_UNDERLINE_THICKNESS = 52,
    XCB_ATOM_STRIKEOUT_ASCENT = 53,
    XCB_ATOM_STRIKEOUT_DESCENT = 54,
    XCB_ATOM_ITALIC_ANGLE = 55,
    XCB_ATOM_X_HEIGHT = 56,
    XCB_ATOM_QUAD_WIDTH = 57,
    XCB_ATOM_WEIGHT = 58,
    XCB_ATOM_POINT_SIZE = 59,
    XCB_ATOM_RESOLUTION = 60,
    XCB_ATOM_COPYRIGHT = 61,
    XCB_ATOM_NOTICE = 62,
    XCB_ATOM_FONT_NAME = 63,
    XCB_ATOM_FAMILY_NAME = 64,
    XCB_ATOM_FULL_NAME = 65,
    XCB_ATOM_CAP_HEIGHT = 66,
    XCB_ATOM_WM_CLASS = 67,
    XCB_ATOM_WM_TRANSIENT_FOR = 68
}

#[link(name = "xcb")]
extern "C" {
    // @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L567
    pub fn xcb_connect(
        displayname: *const c_char,
        screenp: *mut c_int,
    ) -> *mut xcb_connection_t;
    // @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L526
    pub fn xcb_disconnect(
        c: *mut xcb_connection_t,
    );
    // @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L468
    pub fn xcb_get_setup(
        c: *mut  xcb_connection_t,
    ) -> *const xcb_setup_t;
    // @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
    pub fn xcb_setup_roots_iterator(
        r: *const xcb_setup_t,
    ) -> xcb_screen_iterator_t;
    // @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
    pub fn xcb_screen_next(
        i: *mut xcb_screen_iterator_t
    );
    // @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L599
    pub fn xcb_generate_id(
        c: *mut xcb_connection_t,
    ) -> u32;
    // @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
    pub fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t;
    // @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
    pub fn xcb_map_window(
        c: *mut xcb_connection_t,
        window: xcb_window_t,
    ) -> xcb_void_cookie_t;
    // @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L246
    pub fn xcb_flush(
        c: *mut xcb_connection_t,
    ) -> c_int;
    // @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
    pub fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: u8,
        window: xcb_window_t,
        property: xcb_atom_t,
        tp: xcb_atom_t,
        format: u8,
        data_len: u32,
        data: *const c_void,
    ) -> xcb_void_cookie_t;
}
