
#![allow(dead_code)]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use libc::{c_void, c_int, c_char, c_uint};

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
pub type xcb_window_t = u32;
pub type xcb_colormap_t = u32;
pub type xcb_visualid_t = u32;
pub type xcb_atom_t = u32;
pub type xcb_timestamp_t = u32;
pub type xcb_button_t = u8;
pub type xcb_keycode_t = u8;
pub type xcb_keysym_t = u32;
pub type xcb_font_t = u32;
pub type xcb_cursor_t = u32;

pub const XCB_COPY_FROM_PARENT: u64 = 0;
// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
pub const XCB_KEY_PRESS: u8 = 2;
pub const XCB_KEY_RELEASE: u8 = 3;
pub const XCB_BUTTON_PRESS: u8 = 4;
pub const XCB_BUTTON_RELEASE: u8 = 5;
pub const XCB_MOTION_NOTIFY: u8 = 6;
pub const XCB_CONFIGURE_NOTIFY: u8 = 22;

#[repr(C)]
pub struct xcb_connection_t { _private: [u8; 0] }

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

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub enum xcb_event_mask_t {
    XCB_EVENT_MASK_NO_EVENT = 0,
    XCB_EVENT_MASK_KEY_PRESS = 1,
    XCB_EVENT_MASK_KEY_RELEASE = 2,
    XCB_EVENT_MASK_BUTTON_PRESS = 4,
    XCB_EVENT_MASK_BUTTON_RELEASE = 8,
    XCB_EVENT_MASK_ENTER_WINDOW = 16,
    XCB_EVENT_MASK_LEAVE_WINDOW = 32,
    XCB_EVENT_MASK_POINTER_MOTION = 64,
    XCB_EVENT_MASK_POINTER_MOTION_HINT = 128,
    XCB_EVENT_MASK_BUTTON_1_MOTION = 256,
    XCB_EVENT_MASK_BUTTON_2_MOTION = 512,
    XCB_EVENT_MASK_BUTTON_3_MOTION = 1024,
    XCB_EVENT_MASK_BUTTON_4_MOTION = 2048,
    XCB_EVENT_MASK_BUTTON_5_MOTION = 4096,
    XCB_EVENT_MASK_BUTTON_MOTION = 8192,
    XCB_EVENT_MASK_KEYMAP_STATE = 16384,
    XCB_EVENT_MASK_EXPOSURE = 32768,
    XCB_EVENT_MASK_VISIBILITY_CHANGE = 65536,
    XCB_EVENT_MASK_STRUCTURE_NOTIFY = 131072,
    XCB_EVENT_MASK_RESIZE_REDIRECT = 262144,
    XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY = 524288,
    XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT = 1048576,
    XCB_EVENT_MASK_FOCUS_CHANGE = 2097152,
    XCB_EVENT_MASK_PROPERTY_CHANGE = 4194304,
    XCB_EVENT_MASK_COLOR_MAP_CHANGE = 8388608,
    XCB_EVENT_MASK_OWNER_GRAB_BUTTON = 16777216
}

// @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L137
#[repr(C)]
pub struct xcb_generic_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7],
    pub full_sequence: u32,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_button_press_event_t {
    pub response_type: u8,
    pub detail: xcb_button_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_motion_notify_event_t {
    pub response_type: u8,
    pub detail: u8,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_key_press_event_t {
    pub response_type: u8,
    pub detail: xcb_keycode_t,
    pub sequence: u16,
    pub time: xcb_timestamp_t,
    pub root: xcb_window_t,
    pub event: xcb_window_t,
    pub child: xcb_window_t,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
    pub pad0: u8,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_configure_notify_event_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub event: xcb_window_t,
    pub window: xcb_window_t,
    pub above_sibling: xcb_window_t,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: u8,
    pub pad1: u8,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_setup_t {
    pub status: u8,
    pub pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: xcb_keycode_t,
    pub max_keycode: xcb_keycode_t,
    pub pad1: [u8; 4],
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_get_keyboard_mapping_cookie_t {
    pub sequence: c_uint,
}

// @see https://xcb.freedesktop.org/manual/xproto_8h_source.html
#[repr(C)]
pub struct xcb_get_keyboard_mapping_request_t {
    pub major_opcode: u8,
    pub pad0: u8,
    pub length: u16,
    pub first_keycode: xcb_keycode_t,
    pub count: u8,
}

// @see http://manpages.ubuntu.com/manpages/bionic/man3/xcb_get_keyboard_mapping.3.html
#[repr(C)]
pub struct xcb_get_keyboard_mapping_reply_t {
    pub response_type: u8,
    pub keysyms_per_keycode: u8,
    pub sequence: u16,
    pub length: u32,
    pub pad0: [u8; 24],
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
    // @see https://github.com/freedesktop/xcb-libxcb/blob/ee9dfc9a7658e7fe75d27483bb5ed1ba4d1e2c86/src/xcb.h#L309
    pub fn xcb_poll_for_event(
        c: *mut xcb_connection_t,
    ) -> *mut xcb_generic_event_t;
    // @see http://manpages.ubuntu.com/manpages/bionic/man3/xcb_get_keyboard_mapping.3.html
    pub fn xcb_get_keyboard_mapping(
        c: *mut xcb_connection_t,
        first_keycode: xcb_keycode_t,
        count: u8,
    ) -> xcb_get_keyboard_mapping_cookie_t;
    // @see http://manpages.ubuntu.com/manpages/bionic/man3/xcb_get_keyboard_mapping.3.html
    pub fn xcb_get_keyboard_mapping_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_get_keyboard_mapping_cookie_t, 
        err: *mut *mut c_void,
    ) -> *mut xcb_get_keyboard_mapping_reply_t;
    // @see http://manpages.ubuntu.com/manpages/bionic/man3/xcb_get_keyboard_mapping.3.html
    pub fn xcb_get_keyboard_mapping_keysyms(
        reply: *mut xcb_get_keyboard_mapping_reply_t,
    ) -> *mut xcb_keysym_t;
    // @see http://manpages.ubuntu.com/manpages/bionic/man3/xcb_get_keyboard_mapping.3.html
    pub fn xcb_get_keyboard_mapping_keysyms_length(
        reply: *mut xcb_get_keyboard_mapping_reply_t,
    ) -> libc::c_int;
    // @see https://manpages.debian.org/testing/libxcb-doc/xcb_open_font.3.en.html
    pub fn xcb_open_font(
        conn: *mut xcb_connection_t, 
        fid: xcb_font_t, 
        name_len: u16, 
        name: *const c_char,
    ) -> xcb_void_cookie_t;
    // @see http://manpages.ubuntu.com/manpages/bionic/man3/xcb_create_glyph_cursor.3.html
    pub fn xcb_create_glyph_cursor(
        conn: *mut xcb_connection_t, 
        cid: xcb_cursor_t,
        source_font: xcb_font_t, 
        mask_font: xcb_font_t, 
        source_char: u16,
        mask_char: u16,
        fore_red: u16,
        fore_green: u16,
        fore_blue: u16,
        back_red: u16,
        back_green: u16,
        back_blue: u16,
    ) -> xcb_void_cookie_t;
    // @see http://manpages.ubuntu.com/manpages/xenial/man3/xcb_change_window_attributes.3.html
    pub fn xcb_change_window_attributes(
        conn: *mut xcb_connection_t,
        window: xcb_window_t,
        value_mask: u32,
        value_list: *const u32,
    ) -> xcb_void_cookie_t;
}

// @see https://www.cl.cam.ac.uk/~mgk25/ucs/keysymdef.h
pub const XK_space: xcb_keysym_t = 0x0020;
pub const XK_exclam: xcb_keysym_t = 0x0021;
pub const XK_quotedbl: xcb_keysym_t = 0x0022;
pub const XK_numbersign: xcb_keysym_t = 0x0023;
pub const XK_dollar: xcb_keysym_t = 0x0024;
pub const XK_percent: xcb_keysym_t = 0x0025;
pub const XK_ampersand: xcb_keysym_t = 0x0026;
pub const XK_apostrophe: xcb_keysym_t = 0x0027;
pub const XK_quoteright: xcb_keysym_t = 0x0027;
pub const XK_parenleft: xcb_keysym_t = 0x0028;
pub const XK_parenright: xcb_keysym_t = 0x0029;
pub const XK_asterisk: xcb_keysym_t = 0x002a;
pub const XK_plus: xcb_keysym_t = 0x002b;
pub const XK_comma: xcb_keysym_t = 0x002c;
pub const XK_minus: xcb_keysym_t = 0x002d;
pub const XK_period: xcb_keysym_t = 0x002e;
pub const XK_slash: xcb_keysym_t = 0x002f;
pub const XK_0: xcb_keysym_t = 0x0030;
pub const XK_1: xcb_keysym_t = 0x0031;
pub const XK_2: xcb_keysym_t = 0x0032;
pub const XK_3: xcb_keysym_t = 0x0033;
pub const XK_4: xcb_keysym_t = 0x0034;
pub const XK_5: xcb_keysym_t = 0x0035;
pub const XK_6: xcb_keysym_t = 0x0036;
pub const XK_7: xcb_keysym_t = 0x0037;
pub const XK_8: xcb_keysym_t = 0x0038;
pub const XK_9: xcb_keysym_t = 0x0039;
pub const XK_colon: xcb_keysym_t = 0x003a;
pub const XK_semicolon: xcb_keysym_t = 0x003b;
pub const XK_less: xcb_keysym_t = 0x003c;
pub const XK_equal: xcb_keysym_t = 0x003d;
pub const XK_greater: xcb_keysym_t = 0x003e;
pub const XK_question: xcb_keysym_t = 0x003f;
pub const XK_at: xcb_keysym_t = 0x0040;
pub const XK_A: xcb_keysym_t = 0x0041;
pub const XK_B: xcb_keysym_t = 0x0042;
pub const XK_C: xcb_keysym_t = 0x0043;
pub const XK_D: xcb_keysym_t = 0x0044;
pub const XK_E: xcb_keysym_t = 0x0045;
pub const XK_F: xcb_keysym_t = 0x0046;
pub const XK_G: xcb_keysym_t = 0x0047;
pub const XK_H: xcb_keysym_t = 0x0048;
pub const XK_I: xcb_keysym_t = 0x0049;
pub const XK_J: xcb_keysym_t = 0x004a;
pub const XK_K: xcb_keysym_t = 0x004b;
pub const XK_L: xcb_keysym_t = 0x004c;
pub const XK_M: xcb_keysym_t = 0x004d;
pub const XK_N: xcb_keysym_t = 0x004e;
pub const XK_O: xcb_keysym_t = 0x004f;
pub const XK_P: xcb_keysym_t = 0x0050;
pub const XK_Q: xcb_keysym_t = 0x0051;
pub const XK_R: xcb_keysym_t = 0x0052;
pub const XK_S: xcb_keysym_t = 0x0053;
pub const XK_T: xcb_keysym_t = 0x0054;
pub const XK_U: xcb_keysym_t = 0x0055;
pub const XK_V: xcb_keysym_t = 0x0056;
pub const XK_W: xcb_keysym_t = 0x0057;
pub const XK_X: xcb_keysym_t = 0x0058;
pub const XK_Y: xcb_keysym_t = 0x0059;
pub const XK_Z: xcb_keysym_t = 0x005a;
pub const XK_bracketleft: xcb_keysym_t = 0x005b;
pub const XK_backslash: xcb_keysym_t = 0x005c;
pub const XK_bracketright: xcb_keysym_t = 0x005d;
pub const XK_asciicircum: xcb_keysym_t = 0x005e;
pub const XK_underscore: xcb_keysym_t = 0x005f;
pub const XK_grave: xcb_keysym_t = 0x0060;
pub const XK_quoteleft: xcb_keysym_t = 0x0060;
pub const XK_a: xcb_keysym_t = 0x0061;
pub const XK_b: xcb_keysym_t = 0x0062;
pub const XK_c: xcb_keysym_t = 0x0063;
pub const XK_d: xcb_keysym_t = 0x0064;
pub const XK_e: xcb_keysym_t = 0x0065;
pub const XK_f: xcb_keysym_t = 0x0066;
pub const XK_g: xcb_keysym_t = 0x0067;
pub const XK_h: xcb_keysym_t = 0x0068;
pub const XK_i: xcb_keysym_t = 0x0069;
pub const XK_j: xcb_keysym_t = 0x006a;
pub const XK_k: xcb_keysym_t = 0x006b;
pub const XK_l: xcb_keysym_t = 0x006c;
pub const XK_m: xcb_keysym_t = 0x006d;
pub const XK_n: xcb_keysym_t = 0x006e;
pub const XK_o: xcb_keysym_t = 0x006f;
pub const XK_p: xcb_keysym_t = 0x0070;
pub const XK_q: xcb_keysym_t = 0x0071;
pub const XK_r: xcb_keysym_t = 0x0072;
pub const XK_s: xcb_keysym_t = 0x0073;
pub const XK_t: xcb_keysym_t = 0x0074;
pub const XK_u: xcb_keysym_t = 0x0075;
pub const XK_v: xcb_keysym_t = 0x0076;
pub const XK_w: xcb_keysym_t = 0x0077;
pub const XK_x: xcb_keysym_t = 0x0078;
pub const XK_y: xcb_keysym_t = 0x0079;
pub const XK_z: xcb_keysym_t = 0x007a;
pub const XK_braceleft: xcb_keysym_t = 0x007b;
pub const XK_bar: xcb_keysym_t = 0x007c;
pub const XK_braceright: xcb_keysym_t = 0x007d;
pub const XK_asciitilde: xcb_keysym_t = 0x007e;

pub const XK_Shift_L: xcb_keysym_t = 0xffe1;
pub const XK_Shift_R: xcb_keysym_t = 0xffe2;
pub const XK_Control_L: xcb_keysym_t = 0xffe3;
pub const XK_Control_R: xcb_keysym_t = 0xffe4;
pub const XK_Caps_Lock: xcb_keysym_t = 0xffe5;
pub const XK_Shift_Lock: xcb_keysym_t = 0xffe6;
pub const XK_Meta_L: xcb_keysym_t = 0xffe7;
pub const XK_Meta_R: xcb_keysym_t = 0xffe8;
pub const XK_Alt_L: xcb_keysym_t = 0xffe9;
pub const XK_Alt_R: xcb_keysym_t = 0xffea;
pub const XK_Super_L: xcb_keysym_t = 0xffeb;
pub const XK_Super_R: xcb_keysym_t = 0xffec;
pub const XK_Hyper_L: xcb_keysym_t = 0xffed;
pub const XK_Hyper_R: xcb_keysym_t = 0xffee;
