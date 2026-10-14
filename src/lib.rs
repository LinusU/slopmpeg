// PL_MPEG - MPEG1 Video decoder, MP2 Audio decoder, MPEG-PS demuxer
// SPDX-License-Identifier: MIT
//
// Dominic Szablewski - https://phoboslab.org

#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

extern "C" {
    pub type __sFILEX;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> usize;
    fn fseek(_: *mut FILE, _: ::core::ffi::c_long, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_off_t = __int64_t;
pub type int16_t = i16;
pub type int64_t = i64;
pub type size_t = usize;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type uint8_t = u8;
pub type uint16_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_t {
    pub demux: *mut plm_demux_t,
    pub time: ::core::ffi::c_double,
    pub has_ended: ::core::ffi::c_int,
    pub r#loop: ::core::ffi::c_int,
    pub has_decoders: ::core::ffi::c_int,
    pub video_enabled: ::core::ffi::c_int,
    pub video_packet_type: ::core::ffi::c_int,
    pub video_buffer: *mut plm_buffer_t,
    pub video_decoder: *mut plm_video_t,
    pub audio_enabled: ::core::ffi::c_int,
    pub audio_stream_index: ::core::ffi::c_int,
    pub audio_packet_type: ::core::ffi::c_int,
    pub audio_lead_time: ::core::ffi::c_double,
    pub audio_buffer: *mut plm_buffer_t,
    pub audio_decoder: *mut plm_audio_t,
    pub video_decode_callback: plm_video_decode_callback,
    pub video_decode_callback_user_data: *mut ::core::ffi::c_void,
    pub audio_decode_callback: plm_audio_decode_callback,
    pub audio_decode_callback_user_data: *mut ::core::ffi::c_void,
}
pub type plm_audio_decode_callback =
    Option<unsafe extern "C" fn(*mut plm_t, *mut plm_samples_t, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_samples_t {
    pub time: ::core::ffi::c_double,
    pub count: ::core::ffi::c_uint,
    pub interleaved: [::core::ffi::c_float; 2304],
}
pub type plm_video_decode_callback =
    Option<unsafe extern "C" fn(*mut plm_t, *mut plm_frame_t, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_frame_t {
    pub time: ::core::ffi::c_double,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub y: plm_plane_t,
    pub cr: plm_plane_t,
    pub cb: plm_plane_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_plane_t {
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_audio_t {
    pub time: ::core::ffi::c_double,
    pub samples_decoded: ::core::ffi::c_int,
    pub samplerate_index: ::core::ffi::c_int,
    pub bitrate_index: ::core::ffi::c_int,
    pub version: ::core::ffi::c_int,
    pub layer: ::core::ffi::c_int,
    pub mode: ::core::ffi::c_int,
    pub bound: ::core::ffi::c_int,
    pub v_pos: ::core::ffi::c_int,
    pub next_frame_data_size: ::core::ffi::c_int,
    pub has_header: ::core::ffi::c_int,
    pub buffer: *mut plm_buffer_t,
    pub destroy_buffer_when_done: ::core::ffi::c_int,
    pub allocation: [[*const plm_quantizer_spec_t; 32]; 2],
    pub scale_factor_info: [[uint8_t; 32]; 2],
    pub scale_factor: [[[::core::ffi::c_int; 3]; 32]; 2],
    pub sample: [[[::core::ffi::c_int; 3]; 32]; 2],
    pub samples: plm_samples_t,
    pub D: [::core::ffi::c_float; 1024],
    pub V: [[::core::ffi::c_float; 1024]; 2],
    pub U: [::core::ffi::c_float; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_quantizer_spec_t {
    pub levels: ::core::ffi::c_ushort,
    pub group: ::core::ffi::c_uchar,
    pub bits: ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_buffer_t {
    pub bit_index: size_t,
    pub capacity: size_t,
    pub length: size_t,
    pub total_size: size_t,
    pub discard_read_bytes: ::core::ffi::c_int,
    pub has_ended: ::core::ffi::c_int,
    pub free_when_done: ::core::ffi::c_int,
    pub close_when_done: ::core::ffi::c_int,
    pub fh: *mut FILE,
    pub load_callback: plm_buffer_load_callback,
    pub seek_callback: plm_buffer_seek_callback,
    pub tell_callback: plm_buffer_tell_callback,
    pub load_callback_user_data: *mut ::core::ffi::c_void,
    pub bytes: *mut uint8_t,
    pub mode: plm_buffer_mode,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct plm_buffer_mode(pub ::core::ffi::c_uint);
impl plm_buffer_mode {
    pub const PLM_BUFFER_MODE_FILE: Self = Self(0);
    pub const PLM_BUFFER_MODE_FIXED_MEM: Self = Self(1);
    pub const PLM_BUFFER_MODE_RING: Self = Self(2);
    pub const PLM_BUFFER_MODE_APPEND: Self = Self(3);
}
pub type plm_buffer_tell_callback =
    Option<unsafe extern "C" fn(*mut plm_buffer_t, *mut ::core::ffi::c_void) -> size_t>;
pub type plm_buffer_seek_callback =
    Option<unsafe extern "C" fn(*mut plm_buffer_t, size_t, *mut ::core::ffi::c_void) -> ()>;
pub type plm_buffer_load_callback =
    Option<unsafe extern "C" fn(*mut plm_buffer_t, *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_video_t {
    pub framerate: ::core::ffi::c_double,
    pub pixel_aspect_ratio: ::core::ffi::c_double,
    pub time: ::core::ffi::c_double,
    pub frames_decoded: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_size: ::core::ffi::c_int,
    pub luma_width: ::core::ffi::c_int,
    pub luma_height: ::core::ffi::c_int,
    pub chroma_width: ::core::ffi::c_int,
    pub chroma_height: ::core::ffi::c_int,
    pub start_code: ::core::ffi::c_int,
    pub picture_type: ::core::ffi::c_int,
    pub motion_forward: plm_video_motion_t,
    pub motion_backward: plm_video_motion_t,
    pub has_sequence_header: ::core::ffi::c_int,
    pub quantizer_scale: ::core::ffi::c_int,
    pub slice_begin: ::core::ffi::c_int,
    pub macroblock_address: ::core::ffi::c_int,
    pub mb_row: ::core::ffi::c_int,
    pub mb_col: ::core::ffi::c_int,
    pub macroblock_type: ::core::ffi::c_int,
    pub macroblock_intra: ::core::ffi::c_int,
    pub dc_predictor: [::core::ffi::c_int; 3],
    pub buffer: *mut plm_buffer_t,
    pub destroy_buffer_when_done: ::core::ffi::c_int,
    pub frame_current: plm_frame_t,
    pub frame_forward: plm_frame_t,
    pub frame_backward: plm_frame_t,
    pub frames_data: *mut uint8_t,
    pub block_data: [::core::ffi::c_int; 64],
    pub intra_quant_matrix: [uint8_t; 64],
    pub non_intra_quant_matrix: [uint8_t; 64],
    pub has_reference_frame: ::core::ffi::c_int,
    pub assume_no_b_frames: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_video_motion_t {
    pub full_px: ::core::ffi::c_int,
    pub is_set: ::core::ffi::c_int,
    pub r_size: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub v: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_demux_t {
    pub buffer: *mut plm_buffer_t,
    pub destroy_buffer_when_done: ::core::ffi::c_int,
    pub system_clock_ref: ::core::ffi::c_double,
    pub last_file_size: size_t,
    pub last_decoded_pts: ::core::ffi::c_double,
    pub start_time: ::core::ffi::c_double,
    pub duration: ::core::ffi::c_double,
    pub start_code: ::core::ffi::c_int,
    pub has_pack_header: ::core::ffi::c_int,
    pub has_system_header: ::core::ffi::c_int,
    pub has_headers: ::core::ffi::c_int,
    pub num_audio_streams: ::core::ffi::c_int,
    pub num_video_streams: ::core::ffi::c_int,
    pub current_packet: plm_packet_t,
    pub next_packet: plm_packet_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_packet_t {
    pub r#type: ::core::ffi::c_int,
    pub pts: ::core::ffi::c_double,
    pub length: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_vlc_uint_t {
    pub index: int16_t,
    pub value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plm_vlc_t {
    pub index: int16_t,
    pub value: int16_t,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PLM_PACKET_INVALID_TS: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const PLM_AUDIO_SAMPLES_PER_FRAME: ::core::ffi::c_int = 1152 as ::core::ffi::c_int;
pub const PLM_BUFFER_DEFAULT_SIZE: ::core::ffi::c_int =
    128 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
static mut PLM_DEMUX_PACKET_PRIVATE: ::core::ffi::c_int = 0xbd as ::core::ffi::c_int;
static mut PLM_DEMUX_PACKET_AUDIO_1: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
static mut PLM_DEMUX_PACKET_AUDIO_4: ::core::ffi::c_int = 0xc3 as ::core::ffi::c_int;
static mut PLM_DEMUX_PACKET_VIDEO_1: ::core::ffi::c_int = 0xe0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn plm_create_with_filename(
    mut filename: *const ::core::ffi::c_char,
) -> *mut plm_t {
    let mut buffer: *mut plm_buffer_t = plm_buffer_create_with_filename(filename);
    if buffer.is_null() {
        return ::core::ptr::null_mut::<plm_t>();
    }
    return plm_create_with_buffer(buffer, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn plm_create_with_file(
    mut fh: *mut FILE,
    mut close_when_done: ::core::ffi::c_int,
) -> *mut plm_t {
    let mut buffer: *mut plm_buffer_t = plm_buffer_create_with_file(fh, close_when_done);
    return plm_create_with_buffer(buffer, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn plm_create_with_memory(
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut free_when_done: ::core::ffi::c_int,
) -> *mut plm_t {
    let mut buffer: *mut plm_buffer_t =
        plm_buffer_create_with_memory(bytes, length, free_when_done);
    return plm_create_with_buffer(buffer, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn plm_create_with_buffer(
    mut buffer: *mut plm_buffer_t,
    mut destroy_when_done: ::core::ffi::c_int,
) -> *mut plm_t {
    let mut self_0: *mut plm_t = malloc(::core::mem::size_of::<plm_t>()) as *mut plm_t;
    memset(
        self_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<plm_t>(),
    );
    (*self_0).demux = plm_demux_create(buffer, destroy_when_done);
    (*self_0).video_enabled = TRUE;
    (*self_0).audio_enabled = TRUE;
    plm_init_decoders(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_init_decoders(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    if (*self_0).has_decoders != 0 {
        return TRUE;
    }
    if plm_demux_has_headers((*self_0).demux) == 0 {
        return FALSE;
    }
    if plm_demux_get_num_video_streams((*self_0).demux) > 0 as ::core::ffi::c_int {
        if (*self_0).video_enabled != 0 {
            (*self_0).video_packet_type = PLM_DEMUX_PACKET_VIDEO_1;
        }
        if (*self_0).video_decoder.is_null() {
            (*self_0).video_buffer =
                plm_buffer_create_with_capacity(PLM_BUFFER_DEFAULT_SIZE as size_t);
            plm_buffer_set_load_callback(
                (*self_0).video_buffer,
                Some(
                    plm_read_video_packet
                        as unsafe extern "C" fn(*mut plm_buffer_t, *mut ::core::ffi::c_void) -> (),
                ),
                self_0 as *mut ::core::ffi::c_void,
            );
            (*self_0).video_decoder = plm_video_create_with_buffer((*self_0).video_buffer, TRUE);
        }
    }
    if plm_demux_get_num_audio_streams((*self_0).demux) > 0 as ::core::ffi::c_int {
        if (*self_0).audio_enabled != 0 {
            (*self_0).audio_packet_type = PLM_DEMUX_PACKET_AUDIO_1 + (*self_0).audio_stream_index;
        }
        if (*self_0).audio_decoder.is_null() {
            (*self_0).audio_buffer =
                plm_buffer_create_with_capacity(PLM_BUFFER_DEFAULT_SIZE as size_t);
            plm_buffer_set_load_callback(
                (*self_0).audio_buffer,
                Some(
                    plm_read_audio_packet
                        as unsafe extern "C" fn(*mut plm_buffer_t, *mut ::core::ffi::c_void) -> (),
                ),
                self_0 as *mut ::core::ffi::c_void,
            );
            (*self_0).audio_decoder = plm_audio_create_with_buffer((*self_0).audio_buffer, TRUE);
        }
    }
    (*self_0).has_decoders = TRUE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_destroy(mut self_0: *mut plm_t) {
    if !(*self_0).video_decoder.is_null() {
        plm_video_destroy((*self_0).video_decoder);
    }
    if !(*self_0).audio_decoder.is_null() {
        plm_audio_destroy((*self_0).audio_decoder);
    }
    plm_demux_destroy((*self_0).demux);
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_audio_enabled(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return (*self_0).audio_enabled;
}
#[no_mangle]
pub unsafe extern "C" fn plm_has_headers(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    if plm_demux_has_headers((*self_0).demux) == 0 {
        return FALSE;
    }
    if plm_init_decoders(self_0) == 0 {
        return FALSE;
    }
    if !(*self_0).video_decoder.is_null() && plm_video_has_header((*self_0).video_decoder) == 0
        || !(*self_0).audio_decoder.is_null() && plm_audio_has_header((*self_0).audio_decoder) == 0
    {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_probe(
    mut self_0: *mut plm_t,
    mut probesize: size_t,
) -> ::core::ffi::c_int {
    let mut found_streams: ::core::ffi::c_int = plm_demux_probe((*self_0).demux, probesize);
    if found_streams == 0 {
        return FALSE;
    }
    (*self_0).has_decoders = FALSE;
    (*self_0).video_packet_type = 0 as ::core::ffi::c_int;
    (*self_0).audio_packet_type = 0 as ::core::ffi::c_int;
    return plm_init_decoders(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_audio_enabled(
    mut self_0: *mut plm_t,
    mut enabled: ::core::ffi::c_int,
) {
    (*self_0).audio_enabled = enabled;
    if enabled == 0 {
        (*self_0).audio_packet_type = 0 as ::core::ffi::c_int;
        return;
    }
    (*self_0).audio_packet_type =
        if plm_init_decoders(self_0) != 0 && !(*self_0).audio_decoder.is_null() {
            PLM_DEMUX_PACKET_AUDIO_1 + (*self_0).audio_stream_index
        } else {
            0 as ::core::ffi::c_int
        };
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_audio_stream(
    mut self_0: *mut plm_t,
    mut stream_index: ::core::ffi::c_int,
) {
    if stream_index < 0 as ::core::ffi::c_int || stream_index > 3 as ::core::ffi::c_int {
        return;
    }
    (*self_0).audio_stream_index = stream_index;
    plm_set_audio_enabled(self_0, (*self_0).audio_enabled);
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_video_enabled(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return (*self_0).video_enabled;
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_video_enabled(
    mut self_0: *mut plm_t,
    mut enabled: ::core::ffi::c_int,
) {
    (*self_0).video_enabled = enabled;
    if enabled == 0 {
        (*self_0).video_packet_type = 0 as ::core::ffi::c_int;
        return;
    }
    (*self_0).video_packet_type =
        if plm_init_decoders(self_0) != 0 && !(*self_0).video_decoder.is_null() {
            PLM_DEMUX_PACKET_VIDEO_1
        } else {
            0 as ::core::ffi::c_int
        };
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_num_video_streams(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return plm_demux_get_num_video_streams((*self_0).demux);
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_width(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return if plm_init_decoders(self_0) != 0 && !(*self_0).video_decoder.is_null() {
        plm_video_get_width((*self_0).video_decoder)
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_height(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return if plm_init_decoders(self_0) != 0 && !(*self_0).video_decoder.is_null() {
        plm_video_get_height((*self_0).video_decoder)
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_framerate(mut self_0: *mut plm_t) -> ::core::ffi::c_double {
    return if plm_init_decoders(self_0) != 0 && !(*self_0).video_decoder.is_null() {
        plm_video_get_framerate((*self_0).video_decoder)
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_pixel_aspect_ratio(
    mut self_0: *mut plm_t,
) -> ::core::ffi::c_double {
    return if plm_init_decoders(self_0) != 0 && !(*self_0).video_decoder.is_null() {
        plm_video_get_pixel_aspect_ratio((*self_0).video_decoder)
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_num_audio_streams(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return plm_demux_get_num_audio_streams((*self_0).demux);
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_samplerate(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return if plm_init_decoders(self_0) != 0 && !(*self_0).audio_decoder.is_null() {
        plm_audio_get_samplerate((*self_0).audio_decoder)
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_audio_lead_time(mut self_0: *mut plm_t) -> ::core::ffi::c_double {
    return (*self_0).audio_lead_time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_audio_lead_time(
    mut self_0: *mut plm_t,
    mut lead_time: ::core::ffi::c_double,
) {
    (*self_0).audio_lead_time = lead_time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_time(mut self_0: *mut plm_t) -> ::core::ffi::c_double {
    return (*self_0).time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_duration(mut self_0: *mut plm_t) -> ::core::ffi::c_double {
    return plm_demux_get_duration((*self_0).demux, PLM_DEMUX_PACKET_VIDEO_1);
}
#[no_mangle]
pub unsafe extern "C" fn plm_rewind(mut self_0: *mut plm_t) {
    if !(*self_0).video_decoder.is_null() {
        plm_video_rewind((*self_0).video_decoder);
    }
    if !(*self_0).audio_decoder.is_null() {
        plm_audio_rewind((*self_0).audio_decoder);
    }
    plm_demux_rewind((*self_0).demux);
    (*self_0).time = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*self_0).has_ended = FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_get_loop(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return (*self_0).r#loop;
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_loop(mut self_0: *mut plm_t, mut r#loop: ::core::ffi::c_int) {
    (*self_0).r#loop = r#loop;
}
#[no_mangle]
pub unsafe extern "C" fn plm_has_ended(mut self_0: *mut plm_t) -> ::core::ffi::c_int {
    return (*self_0).has_ended;
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_video_decode_callback(
    mut self_0: *mut plm_t,
    mut fp: plm_video_decode_callback,
    mut user: *mut ::core::ffi::c_void,
) {
    (*self_0).video_decode_callback = fp;
    (*self_0).video_decode_callback_user_data = user;
}
#[no_mangle]
pub unsafe extern "C" fn plm_set_audio_decode_callback(
    mut self_0: *mut plm_t,
    mut fp: plm_audio_decode_callback,
    mut user: *mut ::core::ffi::c_void,
) {
    (*self_0).audio_decode_callback = fp;
    (*self_0).audio_decode_callback_user_data = user;
}
#[no_mangle]
pub unsafe extern "C" fn plm_decode(mut self_0: *mut plm_t, mut tick: ::core::ffi::c_double) {
    if plm_init_decoders(self_0) == 0 {
        return;
    }
    let mut decode_video: ::core::ffi::c_int = ((*self_0).video_decode_callback.is_some()
        && (*self_0).video_packet_type != 0)
        as ::core::ffi::c_int;
    let mut decode_audio: ::core::ffi::c_int = ((*self_0).audio_decode_callback.is_some()
        && (*self_0).audio_packet_type != 0)
        as ::core::ffi::c_int;
    if decode_video == 0 && decode_audio == 0 {
        return;
    }
    let mut did_decode: ::core::ffi::c_int = FALSE;
    let mut decode_video_failed: ::core::ffi::c_int = FALSE;
    let mut decode_audio_failed: ::core::ffi::c_int = FALSE;
    let mut video_target_time: ::core::ffi::c_double = (*self_0).time + tick;
    let mut audio_target_time: ::core::ffi::c_double =
        (*self_0).time + tick + (*self_0).audio_lead_time;
    loop {
        did_decode = FALSE;
        if decode_video != 0 && plm_video_get_time((*self_0).video_decoder) < video_target_time {
            let mut frame: *mut plm_frame_t = plm_video_decode((*self_0).video_decoder);
            if !frame.is_null() {
                (*self_0)
                    .video_decode_callback
                    .expect("non-null function pointer")(
                    self_0,
                    frame,
                    (*self_0).video_decode_callback_user_data,
                );
                did_decode = TRUE;
            } else {
                decode_video_failed = TRUE;
            }
        }
        if decode_audio != 0 && plm_audio_get_time((*self_0).audio_decoder) < audio_target_time {
            let mut samples: *mut plm_samples_t = plm_audio_decode((*self_0).audio_decoder);
            if !samples.is_null() {
                (*self_0)
                    .audio_decode_callback
                    .expect("non-null function pointer")(
                    self_0,
                    samples,
                    (*self_0).audio_decode_callback_user_data,
                );
                did_decode = TRUE;
            } else {
                decode_audio_failed = TRUE;
            }
        }
        if did_decode == 0 {
            break;
        }
    }
    if (decode_video == 0 || decode_video_failed != 0)
        && (decode_audio == 0 || decode_audio_failed != 0)
        && plm_demux_has_ended((*self_0).demux) != 0
    {
        plm_handle_end(self_0);
        return;
    }
    (*self_0).time += tick;
}
#[no_mangle]
pub unsafe extern "C" fn plm_decode_video(mut self_0: *mut plm_t) -> *mut plm_frame_t {
    if plm_init_decoders(self_0) == 0 {
        return ::core::ptr::null_mut::<plm_frame_t>();
    }
    if (*self_0).video_packet_type == 0 {
        return ::core::ptr::null_mut::<plm_frame_t>();
    }
    let mut frame: *mut plm_frame_t = plm_video_decode((*self_0).video_decoder);
    if !frame.is_null() {
        (*self_0).time = (*frame).time;
    } else if plm_demux_has_ended((*self_0).demux) != 0 {
        plm_handle_end(self_0);
    }
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn plm_decode_audio(mut self_0: *mut plm_t) -> *mut plm_samples_t {
    if plm_init_decoders(self_0) == 0 {
        return ::core::ptr::null_mut::<plm_samples_t>();
    }
    if (*self_0).audio_packet_type == 0 {
        return ::core::ptr::null_mut::<plm_samples_t>();
    }
    let mut samples: *mut plm_samples_t = plm_audio_decode((*self_0).audio_decoder);
    if !samples.is_null() {
        (*self_0).time = (*samples).time;
    } else if plm_demux_has_ended((*self_0).demux) != 0 {
        plm_handle_end(self_0);
    }
    return samples;
}
#[no_mangle]
pub unsafe extern "C" fn plm_handle_end(mut self_0: *mut plm_t) {
    if (*self_0).r#loop != 0 {
        plm_rewind(self_0);
    } else {
        (*self_0).has_ended = TRUE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_read_video_packet(
    mut buffer: *mut plm_buffer_t,
    mut user: *mut ::core::ffi::c_void,
) {
    let mut self_0: *mut plm_t = user as *mut plm_t;
    plm_read_packets(self_0, (*self_0).video_packet_type);
}
#[no_mangle]
pub unsafe extern "C" fn plm_read_audio_packet(
    mut buffer: *mut plm_buffer_t,
    mut user: *mut ::core::ffi::c_void,
) {
    let mut self_0: *mut plm_t = user as *mut plm_t;
    plm_read_packets(self_0, (*self_0).audio_packet_type);
}
#[no_mangle]
pub unsafe extern "C" fn plm_read_packets(
    mut self_0: *mut plm_t,
    mut requested_type: ::core::ffi::c_int,
) {
    let mut packet: *mut plm_packet_t = ::core::ptr::null_mut::<plm_packet_t>();
    loop {
        packet = plm_demux_decode((*self_0).demux);
        if packet.is_null() {
            break;
        }
        if (*packet).r#type == (*self_0).video_packet_type {
            plm_buffer_write((*self_0).video_buffer, (*packet).data, (*packet).length);
        } else if (*packet).r#type == (*self_0).audio_packet_type {
            plm_buffer_write((*self_0).audio_buffer, (*packet).data, (*packet).length);
        }
        if (*packet).r#type == requested_type {
            return;
        }
    }
    if plm_demux_has_ended((*self_0).demux) != 0 {
        if !(*self_0).video_buffer.is_null() {
            plm_buffer_signal_end((*self_0).video_buffer);
        }
        if !(*self_0).audio_buffer.is_null() {
            plm_buffer_signal_end((*self_0).audio_buffer);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_seek_frame(
    mut self_0: *mut plm_t,
    mut time: ::core::ffi::c_double,
    mut seek_exact: ::core::ffi::c_int,
) -> *mut plm_frame_t {
    if plm_init_decoders(self_0) == 0 {
        return ::core::ptr::null_mut::<plm_frame_t>();
    }
    if (*self_0).video_packet_type == 0 {
        return ::core::ptr::null_mut::<plm_frame_t>();
    }
    let mut r#type: ::core::ffi::c_int = (*self_0).video_packet_type;
    let mut start_time: ::core::ffi::c_double = plm_demux_get_start_time((*self_0).demux, r#type);
    let mut duration: ::core::ffi::c_double = plm_demux_get_duration((*self_0).demux, r#type);
    if time < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        time = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else if time > duration {
        time = duration;
    }
    let mut packet: *mut plm_packet_t = plm_demux_seek((*self_0).demux, time, r#type, TRUE);
    if packet.is_null() {
        return ::core::ptr::null_mut::<plm_frame_t>();
    }
    let mut previous_audio_packet_type: ::core::ffi::c_int = (*self_0).audio_packet_type;
    (*self_0).audio_packet_type = 0 as ::core::ffi::c_int;
    plm_video_rewind((*self_0).video_decoder);
    plm_video_set_time((*self_0).video_decoder, (*packet).pts - start_time);
    plm_buffer_write((*self_0).video_buffer, (*packet).data, (*packet).length);
    let mut frame: *mut plm_frame_t = plm_video_decode((*self_0).video_decoder);
    if seek_exact != 0 {
        while !frame.is_null() && (*frame).time < time {
            frame = plm_video_decode((*self_0).video_decoder);
        }
    }
    (*self_0).audio_packet_type = previous_audio_packet_type;
    if !frame.is_null() {
        (*self_0).time = (*frame).time;
    }
    (*self_0).has_ended = FALSE;
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn plm_seek(
    mut self_0: *mut plm_t,
    mut time: ::core::ffi::c_double,
    mut seek_exact: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut frame: *mut plm_frame_t = plm_seek_frame(self_0, time, seek_exact);
    if frame.is_null() {
        return FALSE;
    }
    if (*self_0).video_decode_callback.is_some() {
        (*self_0)
            .video_decode_callback
            .expect("non-null function pointer")(
            self_0,
            frame,
            (*self_0).video_decode_callback_user_data,
        );
    }
    if (*self_0).audio_packet_type == 0 {
        return TRUE;
    }
    let mut start_time: ::core::ffi::c_double =
        plm_demux_get_start_time((*self_0).demux, (*self_0).video_packet_type);
    plm_audio_rewind((*self_0).audio_decoder);
    let mut packet: *mut plm_packet_t = ::core::ptr::null_mut::<plm_packet_t>();
    loop {
        packet = plm_demux_decode((*self_0).demux);
        if packet.is_null() {
            break;
        }
        if (*packet).r#type == (*self_0).video_packet_type {
            plm_buffer_write((*self_0).video_buffer, (*packet).data, (*packet).length);
        } else {
            if !((*packet).r#type == (*self_0).audio_packet_type
                && (*packet).pts - start_time > (*self_0).time)
            {
                continue;
            }
            plm_audio_set_time((*self_0).audio_decoder, (*packet).pts - start_time);
            plm_buffer_write((*self_0).audio_buffer, (*packet).data, (*packet).length);
            plm_decode(self_0, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
            break;
        }
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_create_with_filename(
    mut filename: *const ::core::ffi::c_char,
) -> *mut plm_buffer_t {
    let mut fh: *mut FILE = fopen(filename, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if fh.is_null() {
        return ::core::ptr::null_mut::<plm_buffer_t>();
    }
    return plm_buffer_create_with_file(fh, TRUE);
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_create_with_file(
    mut fh: *mut FILE,
    mut close_when_done: ::core::ffi::c_int,
) -> *mut plm_buffer_t {
    let mut self_0: *mut plm_buffer_t =
        plm_buffer_create_with_capacity(PLM_BUFFER_DEFAULT_SIZE as size_t);
    (*self_0).fh = fh;
    (*self_0).close_when_done = close_when_done;
    (*self_0).mode = plm_buffer_mode::PLM_BUFFER_MODE_FILE;
    (*self_0).discard_read_bytes = TRUE;
    fseek((*self_0).fh, 0 as ::core::ffi::c_long, SEEK_END);
    (*self_0).total_size = ftell((*self_0).fh) as size_t;
    fseek((*self_0).fh, 0 as ::core::ffi::c_long, SEEK_SET);
    (*self_0).load_callback = Some(
        plm_buffer_load_file_callback
            as unsafe extern "C" fn(*mut plm_buffer_t, *mut ::core::ffi::c_void) -> (),
    ) as plm_buffer_load_callback;
    (*self_0).seek_callback = Some(
        plm_buffer_seek_file_callback
            as unsafe extern "C" fn(*mut plm_buffer_t, size_t, *mut ::core::ffi::c_void) -> (),
    ) as plm_buffer_seek_callback;
    (*self_0).tell_callback = Some(
        plm_buffer_tell_file_callback
            as unsafe extern "C" fn(*mut plm_buffer_t, *mut ::core::ffi::c_void) -> size_t,
    ) as plm_buffer_tell_callback;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_create_with_callbacks(
    mut load_callback: plm_buffer_load_callback,
    mut seek_callback: plm_buffer_seek_callback,
    mut tell_callback: plm_buffer_tell_callback,
    mut length: size_t,
    mut user: *mut ::core::ffi::c_void,
) -> *mut plm_buffer_t {
    let mut self_0: *mut plm_buffer_t =
        plm_buffer_create_with_capacity(PLM_BUFFER_DEFAULT_SIZE as size_t);
    (*self_0).mode = plm_buffer_mode::PLM_BUFFER_MODE_FILE;
    (*self_0).total_size = length;
    (*self_0).load_callback = load_callback;
    (*self_0).seek_callback = seek_callback;
    (*self_0).tell_callback = tell_callback;
    (*self_0).load_callback_user_data = user;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_create_with_memory(
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut free_when_done: ::core::ffi::c_int,
) -> *mut plm_buffer_t {
    let mut self_0: *mut plm_buffer_t =
        malloc(::core::mem::size_of::<plm_buffer_t>()) as *mut plm_buffer_t;
    memset(
        self_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<plm_buffer_t>(),
    );
    (*self_0).capacity = length;
    (*self_0).length = length;
    (*self_0).total_size = length;
    (*self_0).free_when_done = free_when_done;
    (*self_0).bytes = bytes;
    (*self_0).mode = plm_buffer_mode::PLM_BUFFER_MODE_FIXED_MEM;
    (*self_0).discard_read_bytes = FALSE;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_create_with_capacity(
    mut capacity: size_t,
) -> *mut plm_buffer_t {
    let mut self_0: *mut plm_buffer_t =
        malloc(::core::mem::size_of::<plm_buffer_t>()) as *mut plm_buffer_t;
    memset(
        self_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<plm_buffer_t>(),
    );
    (*self_0).capacity = capacity;
    (*self_0).free_when_done = TRUE;
    (*self_0).bytes = malloc(capacity) as *mut uint8_t;
    (*self_0).mode = plm_buffer_mode::PLM_BUFFER_MODE_RING;
    (*self_0).discard_read_bytes = TRUE;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_create_for_appending(
    mut initial_capacity: size_t,
) -> *mut plm_buffer_t {
    let mut self_0: *mut plm_buffer_t = plm_buffer_create_with_capacity(initial_capacity);
    (*self_0).mode = plm_buffer_mode::PLM_BUFFER_MODE_APPEND;
    (*self_0).discard_read_bytes = FALSE;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_destroy(mut self_0: *mut plm_buffer_t) {
    if !(*self_0).fh.is_null() && (*self_0).close_when_done != 0 {
        fclose((*self_0).fh);
    }
    if (*self_0).free_when_done != 0 {
        free((*self_0).bytes as *mut ::core::ffi::c_void);
    }
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_get_size(mut self_0: *mut plm_buffer_t) -> size_t {
    return if (*self_0).mode.0 == plm_buffer_mode::PLM_BUFFER_MODE_FILE.0 {
        (*self_0).total_size
    } else {
        (*self_0).length
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_get_remaining(mut self_0: *mut plm_buffer_t) -> size_t {
    return (*self_0)
        .length
        .wrapping_sub((*self_0).bit_index >> 3 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_write(
    mut self_0: *mut plm_buffer_t,
    mut bytes: *mut uint8_t,
    mut length: size_t,
) -> size_t {
    if (*self_0).mode.0 == plm_buffer_mode::PLM_BUFFER_MODE_FIXED_MEM.0 {
        return 0 as size_t;
    }
    if (*self_0).discard_read_bytes != 0 {
        plm_buffer_discard_read_bytes(self_0);
        if (*self_0).mode.0 == plm_buffer_mode::PLM_BUFFER_MODE_RING.0 {
            (*self_0).total_size = 0 as size_t;
        }
    }
    let mut bytes_available: size_t = (*self_0).capacity.wrapping_sub((*self_0).length);
    if bytes_available < length {
        let mut new_size: size_t = (*self_0).capacity;
        loop {
            new_size = new_size.wrapping_mul(2 as size_t);
            if new_size.wrapping_sub((*self_0).length) >= length {
                break;
            }
        }
        (*self_0).bytes =
            realloc((*self_0).bytes as *mut ::core::ffi::c_void, new_size) as *mut uint8_t;
        (*self_0).capacity = new_size;
    }
    memcpy(
        (*self_0).bytes.offset((*self_0).length as isize) as *mut ::core::ffi::c_void,
        bytes as *const ::core::ffi::c_void,
        length,
    );
    (*self_0).length = (*self_0).length.wrapping_add(length);
    (*self_0).has_ended = FALSE;
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_signal_end(mut self_0: *mut plm_buffer_t) {
    (*self_0).total_size = (*self_0).length;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_set_load_callback(
    mut self_0: *mut plm_buffer_t,
    mut fp: plm_buffer_load_callback,
    mut user: *mut ::core::ffi::c_void,
) {
    (*self_0).load_callback = fp;
    (*self_0).load_callback_user_data = user;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_rewind(mut self_0: *mut plm_buffer_t) {
    plm_buffer_seek(self_0, 0 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_seek(mut self_0: *mut plm_buffer_t, mut pos: size_t) {
    (*self_0).has_ended = FALSE;
    if (*self_0).seek_callback.is_some() {
        (*self_0).seek_callback.expect("non-null function pointer")(
            self_0,
            pos,
            (*self_0).load_callback_user_data,
        );
        (*self_0).bit_index = 0 as size_t;
        (*self_0).length = 0 as size_t;
    } else if (*self_0).mode.0 == plm_buffer_mode::PLM_BUFFER_MODE_RING.0 {
        if pos != 0 as size_t {
            return;
        }
        (*self_0).bit_index = 0 as size_t;
        (*self_0).length = 0 as size_t;
        (*self_0).total_size = 0 as size_t;
    } else if pos < (*self_0).length {
        (*self_0).bit_index = pos << 3 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_tell(mut self_0: *mut plm_buffer_t) -> size_t {
    return if (*self_0).tell_callback.is_some() {
        (*self_0).tell_callback.expect("non-null function pointer")(
            self_0,
            (*self_0).load_callback_user_data,
        )
        .wrapping_add((*self_0).bit_index >> 3 as ::core::ffi::c_int)
        .wrapping_sub((*self_0).length)
    } else {
        (*self_0).bit_index >> 3 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_discard_read_bytes(mut self_0: *mut plm_buffer_t) {
    let mut byte_pos: size_t = (*self_0).bit_index >> 3 as ::core::ffi::c_int;
    if byte_pos == (*self_0).length {
        (*self_0).bit_index = 0 as size_t;
        (*self_0).length = 0 as size_t;
    } else if byte_pos > 0 as size_t {
        memmove(
            (*self_0).bytes as *mut ::core::ffi::c_void,
            (*self_0).bytes.offset(byte_pos as isize) as *const ::core::ffi::c_void,
            (*self_0).length.wrapping_sub(byte_pos),
        );
        (*self_0).bit_index = (*self_0)
            .bit_index
            .wrapping_sub(byte_pos << 3 as ::core::ffi::c_int);
        (*self_0).length = (*self_0).length.wrapping_sub(byte_pos);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_load_file_callback(
    mut self_0: *mut plm_buffer_t,
    mut user: *mut ::core::ffi::c_void,
) {
    if (*self_0).discard_read_bytes != 0 {
        plm_buffer_discard_read_bytes(self_0);
    }
    let mut bytes_available: size_t = (*self_0).capacity.wrapping_sub((*self_0).length);
    let mut bytes_read: size_t = fread(
        (*self_0).bytes.offset((*self_0).length as isize) as *mut ::core::ffi::c_void,
        1 as size_t,
        bytes_available,
        (*self_0).fh,
    );
    (*self_0).length = (*self_0).length.wrapping_add(bytes_read);
    if bytes_read == 0 as size_t {
        (*self_0).has_ended = TRUE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_seek_file_callback(
    mut self_0: *mut plm_buffer_t,
    mut offset: size_t,
    mut user: *mut ::core::ffi::c_void,
) {
    fseek((*self_0).fh, offset as ::core::ffi::c_long, SEEK_SET);
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_tell_file_callback(
    mut self_0: *mut plm_buffer_t,
    mut user: *mut ::core::ffi::c_void,
) -> size_t {
    return ftell((*self_0).fh) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_has_ended(mut self_0: *mut plm_buffer_t) -> ::core::ffi::c_int {
    return (*self_0).has_ended;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_has(
    mut self_0: *mut plm_buffer_t,
    mut count: size_t,
) -> ::core::ffi::c_int {
    if ((*self_0).length << 3 as ::core::ffi::c_int).wrapping_sub((*self_0).bit_index) >= count {
        return TRUE;
    }
    if (*self_0).load_callback.is_some() {
        (*self_0).load_callback.expect("non-null function pointer")(
            self_0,
            (*self_0).load_callback_user_data,
        );
        if ((*self_0).length << 3 as ::core::ffi::c_int).wrapping_sub((*self_0).bit_index) >= count
        {
            return TRUE;
        }
    }
    if (*self_0).total_size != 0 as size_t && (*self_0).length == (*self_0).total_size {
        (*self_0).has_ended = TRUE;
    }
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_read(
    mut self_0: *mut plm_buffer_t,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if plm_buffer_has(self_0, count as size_t) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while count != 0 {
        let mut current_byte: ::core::ffi::c_int = *(*self_0)
            .bytes
            .offset(((*self_0).bit_index >> 3 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        let mut remaining: ::core::ffi::c_int =
            (8 as size_t).wrapping_sub((*self_0).bit_index & 7 as size_t) as ::core::ffi::c_int;
        let mut read: ::core::ffi::c_int = if remaining < count { remaining } else { count };
        let mut shift: ::core::ffi::c_int = remaining - read;
        let mut mask: ::core::ffi::c_int =
            0xff as ::core::ffi::c_int >> 8 as ::core::ffi::c_int - read;
        value = value << read | (current_byte & mask << shift) >> shift;
        (*self_0).bit_index = (*self_0).bit_index.wrapping_add(read as size_t);
        count -= read;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_align(mut self_0: *mut plm_buffer_t) {
    (*self_0).bit_index = ((*self_0).bit_index.wrapping_add(7 as size_t)
        >> 3 as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_skip(mut self_0: *mut plm_buffer_t, mut count: size_t) {
    if plm_buffer_has(self_0, count) != 0 {
        (*self_0).bit_index = (*self_0).bit_index.wrapping_add(count);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_skip_bytes(
    mut self_0: *mut plm_buffer_t,
    mut v: uint8_t,
) -> ::core::ffi::c_int {
    plm_buffer_align(self_0);
    let mut skipped: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while plm_buffer_has(self_0, 8 as size_t) != 0
        && *(*self_0)
            .bytes
            .offset(((*self_0).bit_index >> 3 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == v as ::core::ffi::c_int
    {
        (*self_0).bit_index = (*self_0).bit_index.wrapping_add(8 as size_t);
        skipped += 1;
    }
    return skipped;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_next_start_code(
    mut self_0: *mut plm_buffer_t,
) -> ::core::ffi::c_int {
    plm_buffer_align(self_0);
    while plm_buffer_has(
        self_0,
        ((5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as size_t,
    ) != 0
    {
        let mut byte_index: size_t = (*self_0).bit_index >> 3 as ::core::ffi::c_int;
        if *(*self_0).bytes.offset(byte_index as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            && *(*self_0)
                .bytes
                .offset(byte_index.wrapping_add(1 as size_t) as isize)
                as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            && *(*self_0)
                .bytes
                .offset(byte_index.wrapping_add(2 as size_t) as isize)
                as ::core::ffi::c_int
                == 0x1 as ::core::ffi::c_int
        {
            (*self_0).bit_index = byte_index.wrapping_add(4 as size_t) << 3 as ::core::ffi::c_int;
            return *(*self_0)
                .bytes
                .offset(byte_index.wrapping_add(3 as size_t) as isize)
                as ::core::ffi::c_int;
        }
        (*self_0).bit_index = (*self_0).bit_index.wrapping_add(8 as size_t);
    }
    return -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_find_start_code(
    mut self_0: *mut plm_buffer_t,
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        current = plm_buffer_next_start_code(self_0);
        if current == code || current == -1 as ::core::ffi::c_int {
            return current;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_has_start_code(
    mut self_0: *mut plm_buffer_t,
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut previous_bit_index: size_t = (*self_0).bit_index;
    let mut previous_discard_read_bytes: ::core::ffi::c_int = (*self_0).discard_read_bytes;
    (*self_0).discard_read_bytes = FALSE;
    let mut current: ::core::ffi::c_int = plm_buffer_find_start_code(self_0, code);
    (*self_0).bit_index = previous_bit_index;
    (*self_0).discard_read_bytes = previous_discard_read_bytes;
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_peek_non_zero(
    mut self_0: *mut plm_buffer_t,
    mut bit_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if plm_buffer_has(self_0, bit_count as size_t) == 0 {
        return FALSE;
    }
    let mut val: ::core::ffi::c_int = plm_buffer_read(self_0, bit_count);
    (*self_0).bit_index = (*self_0).bit_index.wrapping_sub(bit_count as size_t);
    return (val != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_read_vlc(
    mut self_0: *mut plm_buffer_t,
    mut table: *const plm_vlc_t,
) -> int16_t {
    let mut state: plm_vlc_t = plm_vlc_t {
        index: 0 as int16_t,
        value: 0 as int16_t,
    };
    loop {
        state = *table.offset(
            (state.index as ::core::ffi::c_int + plm_buffer_read(self_0, 1 as ::core::ffi::c_int))
                as isize,
        );
        if state.index as ::core::ffi::c_int <= 0 as ::core::ffi::c_int {
            break;
        }
    }
    return state.value;
}
#[no_mangle]
pub unsafe extern "C" fn plm_buffer_read_vlc_uint(
    mut self_0: *mut plm_buffer_t,
    mut table: *const plm_vlc_uint_t,
) -> uint16_t {
    return plm_buffer_read_vlc(self_0, table as *const plm_vlc_t) as uint16_t;
}
static mut PLM_START_PACK: ::core::ffi::c_int = 0xba as ::core::ffi::c_int;
static mut PLM_START_SYSTEM: ::core::ffi::c_int = 0xbb as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn plm_demux_create(
    mut buffer: *mut plm_buffer_t,
    mut destroy_when_done: ::core::ffi::c_int,
) -> *mut plm_demux_t {
    let mut self_0: *mut plm_demux_t =
        malloc(::core::mem::size_of::<plm_demux_t>()) as *mut plm_demux_t;
    memset(
        self_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<plm_demux_t>(),
    );
    (*self_0).buffer = buffer;
    (*self_0).destroy_buffer_when_done = destroy_when_done;
    (*self_0).start_time = PLM_PACKET_INVALID_TS as ::core::ffi::c_double;
    (*self_0).duration = PLM_PACKET_INVALID_TS as ::core::ffi::c_double;
    (*self_0).start_code = -1 as ::core::ffi::c_int;
    plm_demux_has_headers(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_destroy(mut self_0: *mut plm_demux_t) {
    if (*self_0).destroy_buffer_when_done != 0 {
        plm_buffer_destroy((*self_0).buffer);
    }
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_has_headers(mut self_0: *mut plm_demux_t) -> ::core::ffi::c_int {
    if (*self_0).has_headers != 0 {
        return TRUE;
    }
    if (*self_0).has_pack_header == 0 {
        if (*self_0).start_code != PLM_START_PACK
            && plm_buffer_find_start_code((*self_0).buffer, PLM_START_PACK)
                == -1 as ::core::ffi::c_int
        {
            return FALSE;
        }
        (*self_0).start_code = PLM_START_PACK;
        if plm_buffer_has((*self_0).buffer, 64 as size_t) == 0 {
            return FALSE;
        }
        (*self_0).start_code = -1 as ::core::ffi::c_int;
        if plm_buffer_read((*self_0).buffer, 4 as ::core::ffi::c_int) != 0x2 as ::core::ffi::c_int {
            return FALSE;
        }
        (*self_0).system_clock_ref = plm_demux_decode_time(self_0);
        plm_buffer_skip((*self_0).buffer, 1 as size_t);
        plm_buffer_skip((*self_0).buffer, 22 as size_t);
        plm_buffer_skip((*self_0).buffer, 1 as size_t);
        (*self_0).has_pack_header = TRUE;
    }
    if (*self_0).has_system_header == 0 {
        if (*self_0).start_code != PLM_START_SYSTEM
            && plm_buffer_find_start_code((*self_0).buffer, PLM_START_SYSTEM)
                == -1 as ::core::ffi::c_int
        {
            return FALSE;
        }
        (*self_0).start_code = PLM_START_SYSTEM;
        if plm_buffer_has((*self_0).buffer, 56 as size_t) == 0 {
            return FALSE;
        }
        (*self_0).start_code = -1 as ::core::ffi::c_int;
        plm_buffer_skip((*self_0).buffer, 16 as size_t);
        plm_buffer_skip((*self_0).buffer, 24 as size_t);
        (*self_0).num_audio_streams = plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
        plm_buffer_skip((*self_0).buffer, 5 as size_t);
        (*self_0).num_video_streams = plm_buffer_read((*self_0).buffer, 5 as ::core::ffi::c_int);
        (*self_0).has_system_header = TRUE;
    }
    (*self_0).has_headers = TRUE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_probe(
    mut self_0: *mut plm_demux_t,
    mut probesize: size_t,
) -> ::core::ffi::c_int {
    let mut previous_pos: ::core::ffi::c_int =
        plm_buffer_tell((*self_0).buffer) as ::core::ffi::c_int;
    let mut video_stream: ::core::ffi::c_int = FALSE;
    let mut audio_streams: [::core::ffi::c_int; 4] = [FALSE, FALSE, FALSE, FALSE];
    loop {
        (*self_0).start_code = plm_buffer_next_start_code((*self_0).buffer);
        if (*self_0).start_code == PLM_DEMUX_PACKET_VIDEO_1 {
            video_stream = TRUE;
        } else if (*self_0).start_code >= PLM_DEMUX_PACKET_AUDIO_1
            && (*self_0).start_code <= PLM_DEMUX_PACKET_AUDIO_4
        {
            audio_streams[((*self_0).start_code - PLM_DEMUX_PACKET_AUDIO_1) as usize] = TRUE;
        }
        if !((*self_0).start_code != -1 as ::core::ffi::c_int
            && plm_buffer_tell((*self_0).buffer).wrapping_sub(previous_pos as size_t) < probesize)
        {
            break;
        }
    }
    (*self_0).num_video_streams = if video_stream != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*self_0).num_audio_streams = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        if audio_streams[i as usize] != 0 {
            (*self_0).num_audio_streams += 1;
        }
        i += 1;
    }
    plm_demux_buffer_seek(self_0, previous_pos as size_t);
    return ((*self_0).num_video_streams != 0 || (*self_0).num_audio_streams != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_get_num_video_streams(
    mut self_0: *mut plm_demux_t,
) -> ::core::ffi::c_int {
    return if plm_demux_has_headers(self_0) != 0 {
        (*self_0).num_video_streams
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_get_num_audio_streams(
    mut self_0: *mut plm_demux_t,
) -> ::core::ffi::c_int {
    return if plm_demux_has_headers(self_0) != 0 {
        (*self_0).num_audio_streams
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_rewind(mut self_0: *mut plm_demux_t) {
    plm_buffer_rewind((*self_0).buffer);
    (*self_0).current_packet.length = 0 as size_t;
    (*self_0).next_packet.length = 0 as size_t;
    (*self_0).start_code = -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_has_ended(mut self_0: *mut plm_demux_t) -> ::core::ffi::c_int {
    return plm_buffer_has_ended((*self_0).buffer);
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_buffer_seek(mut self_0: *mut plm_demux_t, mut pos: size_t) {
    plm_buffer_seek((*self_0).buffer, pos);
    (*self_0).current_packet.length = 0 as size_t;
    (*self_0).next_packet.length = 0 as size_t;
    (*self_0).start_code = -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_get_start_time(
    mut self_0: *mut plm_demux_t,
    mut r#type: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    if (*self_0).start_time != PLM_PACKET_INVALID_TS as ::core::ffi::c_double {
        return (*self_0).start_time;
    }
    let mut previous_pos: ::core::ffi::c_int =
        plm_buffer_tell((*self_0).buffer) as ::core::ffi::c_int;
    let mut previous_start_code: ::core::ffi::c_int = (*self_0).start_code;
    plm_demux_rewind(self_0);
    loop {
        let mut packet: *mut plm_packet_t = plm_demux_decode(self_0);
        if packet.is_null() {
            break;
        }
        if (*packet).r#type == r#type {
            (*self_0).start_time = (*packet).pts;
        }
        if (*self_0).start_time != PLM_PACKET_INVALID_TS as ::core::ffi::c_double {
            break;
        }
    }
    plm_demux_buffer_seek(self_0, previous_pos as size_t);
    (*self_0).start_code = previous_start_code;
    return (*self_0).start_time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_get_duration(
    mut self_0: *mut plm_demux_t,
    mut r#type: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut file_size: size_t = plm_buffer_get_size((*self_0).buffer);
    if (*self_0).duration != PLM_PACKET_INVALID_TS as ::core::ffi::c_double
        && (*self_0).last_file_size == file_size
    {
        return (*self_0).duration;
    }
    let mut previous_pos: size_t = plm_buffer_tell((*self_0).buffer);
    let mut previous_start_code: ::core::ffi::c_int = (*self_0).start_code;
    let mut start_range: ::core::ffi::c_long =
        (64 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_long;
    let mut max_range: ::core::ffi::c_long =
        (4096 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_long;
    let mut range: ::core::ffi::c_long = start_range;
    while range <= max_range {
        let mut seek_pos: ::core::ffi::c_long =
            file_size.wrapping_sub(range as size_t) as ::core::ffi::c_long;
        if seek_pos < 0 as ::core::ffi::c_long {
            seek_pos = 0 as ::core::ffi::c_long;
            range = max_range;
        }
        plm_demux_buffer_seek(self_0, seek_pos as size_t);
        (*self_0).current_packet.length = 0 as size_t;
        let mut last_pts: ::core::ffi::c_double = PLM_PACKET_INVALID_TS as ::core::ffi::c_double;
        let mut packet: *mut plm_packet_t = ::core::ptr::null_mut::<plm_packet_t>();
        loop {
            packet = plm_demux_decode(self_0);
            if packet.is_null() {
                break;
            }
            if (*packet).pts != PLM_PACKET_INVALID_TS as ::core::ffi::c_double
                && (*packet).r#type == r#type
            {
                last_pts = (*packet).pts;
            }
        }
        if last_pts != PLM_PACKET_INVALID_TS as ::core::ffi::c_double {
            (*self_0).duration = last_pts - plm_demux_get_start_time(self_0, r#type);
            break;
        } else {
            range *= 2 as ::core::ffi::c_long;
        }
    }
    plm_demux_buffer_seek(self_0, previous_pos);
    (*self_0).start_code = previous_start_code;
    (*self_0).last_file_size = file_size;
    return (*self_0).duration;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_seek(
    mut self_0: *mut plm_demux_t,
    mut seek_time: ::core::ffi::c_double,
    mut r#type: ::core::ffi::c_int,
    mut force_intra: ::core::ffi::c_int,
) -> *mut plm_packet_t {
    if plm_demux_has_headers(self_0) == 0 {
        return ::core::ptr::null_mut::<plm_packet_t>();
    }
    let mut duration: ::core::ffi::c_double = plm_demux_get_duration(self_0, r#type);
    let mut file_size: ::core::ffi::c_long =
        plm_buffer_get_size((*self_0).buffer) as ::core::ffi::c_long;
    let mut byterate: ::core::ffi::c_long =
        (file_size as ::core::ffi::c_double / duration) as ::core::ffi::c_long;
    let mut cur_time: ::core::ffi::c_double = (*self_0).last_decoded_pts;
    let mut scan_span: ::core::ffi::c_double = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    if seek_time > duration {
        seek_time = duration;
    } else if seek_time < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        seek_time = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    seek_time += (*self_0).start_time;
    let mut retry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while retry < 32 as ::core::ffi::c_int {
        let mut found_packet_with_pts: ::core::ffi::c_int = FALSE;
        let mut found_packet_in_range: ::core::ffi::c_int = FALSE;
        let mut last_valid_packet_start: ::core::ffi::c_long = -1 as ::core::ffi::c_long;
        let mut first_packet_time: ::core::ffi::c_double =
            PLM_PACKET_INVALID_TS as ::core::ffi::c_double;
        let mut cur_pos: ::core::ffi::c_long =
            plm_buffer_tell((*self_0).buffer) as ::core::ffi::c_long;
        let mut offset: ::core::ffi::c_long = ((seek_time - cur_time - scan_span)
            * byterate as ::core::ffi::c_double)
            as ::core::ffi::c_long;
        let mut seek_pos: ::core::ffi::c_long = cur_pos + offset;
        if seek_pos < 0 as ::core::ffi::c_long {
            seek_pos = 0 as ::core::ffi::c_long;
        } else if seek_pos > file_size - 256 as ::core::ffi::c_long {
            seek_pos = file_size - 256 as ::core::ffi::c_long;
        }
        plm_demux_buffer_seek(self_0, seek_pos as size_t);
        while plm_buffer_find_start_code((*self_0).buffer, r#type) != -1 as ::core::ffi::c_int {
            let mut packet_start: ::core::ffi::c_long =
                plm_buffer_tell((*self_0).buffer) as ::core::ffi::c_long;
            let mut packet: *mut plm_packet_t = plm_demux_decode_packet(self_0, r#type);
            if packet.is_null() || (*packet).pts == PLM_PACKET_INVALID_TS as ::core::ffi::c_double {
                continue;
            }
            if (*packet).pts > seek_time || (*packet).pts < seek_time - scan_span {
                found_packet_with_pts = TRUE;
                byterate = ((seek_pos - cur_pos) as ::core::ffi::c_double
                    / ((*packet).pts - cur_time)) as ::core::ffi::c_long;
                cur_time = (*packet).pts;
                break;
            } else {
                if found_packet_in_range == 0 {
                    found_packet_in_range = TRUE;
                    first_packet_time = (*packet).pts;
                }
                if force_intra != 0 {
                    let mut i: size_t = 0 as size_t;
                    while i < (*packet).length.wrapping_sub(6 as size_t) {
                        if *(*packet).data.offset(i as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && *(*packet).data.offset(i.wrapping_add(1 as size_t) as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            && *(*packet).data.offset(i.wrapping_add(2 as size_t) as isize)
                                as ::core::ffi::c_int
                                == 0x1 as ::core::ffi::c_int
                            && *(*packet).data.offset(i.wrapping_add(3 as size_t) as isize)
                                as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            if *(*packet).data.offset(i.wrapping_add(5 as size_t) as isize)
                                as ::core::ffi::c_int
                                & 0x38 as ::core::ffi::c_int
                                == 8 as ::core::ffi::c_int
                            {
                                last_valid_packet_start = packet_start;
                            }
                            break;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                } else {
                    last_valid_packet_start = packet_start;
                }
            }
        }
        if last_valid_packet_start != -1 as ::core::ffi::c_long {
            plm_demux_buffer_seek(self_0, last_valid_packet_start as size_t);
            return plm_demux_decode_packet(self_0, r#type);
        } else if found_packet_in_range != 0 {
            scan_span *= 2 as ::core::ffi::c_int as ::core::ffi::c_double;
            seek_time = first_packet_time;
        } else if found_packet_with_pts == 0 {
            byterate = ((seek_pos - cur_pos) as ::core::ffi::c_double / (duration - cur_time))
                as ::core::ffi::c_long;
            cur_time = duration;
        }
        retry += 1;
    }
    return ::core::ptr::null_mut::<plm_packet_t>();
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_decode(mut self_0: *mut plm_demux_t) -> *mut plm_packet_t {
    if plm_demux_has_headers(self_0) == 0 {
        return ::core::ptr::null_mut::<plm_packet_t>();
    }
    if (*self_0).current_packet.length != 0 {
        let mut bits_till_next_packet: size_t =
            (*self_0).current_packet.length << 3 as ::core::ffi::c_int;
        if plm_buffer_has((*self_0).buffer, bits_till_next_packet) == 0 {
            return ::core::ptr::null_mut::<plm_packet_t>();
        }
        plm_buffer_skip((*self_0).buffer, bits_till_next_packet);
        (*self_0).current_packet.length = 0 as size_t;
    }
    if (*self_0).next_packet.length != 0 {
        return plm_demux_get_packet(self_0);
    }
    if (*self_0).start_code != -1 as ::core::ffi::c_int {
        return plm_demux_decode_packet(self_0, (*self_0).start_code);
    }
    loop {
        (*self_0).start_code = plm_buffer_next_start_code((*self_0).buffer);
        if (*self_0).start_code == PLM_DEMUX_PACKET_VIDEO_1
            || (*self_0).start_code == PLM_DEMUX_PACKET_PRIVATE
            || (*self_0).start_code >= PLM_DEMUX_PACKET_AUDIO_1
                && (*self_0).start_code <= PLM_DEMUX_PACKET_AUDIO_4
        {
            return plm_demux_decode_packet(self_0, (*self_0).start_code);
        }
        if (*self_0).start_code == -1 as ::core::ffi::c_int {
            break;
        }
    }
    return ::core::ptr::null_mut::<plm_packet_t>();
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_decode_time(
    mut self_0: *mut plm_demux_t,
) -> ::core::ffi::c_double {
    let mut clock: int64_t = (plm_buffer_read((*self_0).buffer, 3 as ::core::ffi::c_int)
        << 30 as ::core::ffi::c_int) as int64_t;
    plm_buffer_skip((*self_0).buffer, 1 as size_t);
    clock |= (plm_buffer_read((*self_0).buffer, 15 as ::core::ffi::c_int)
        << 15 as ::core::ffi::c_int) as int64_t;
    plm_buffer_skip((*self_0).buffer, 1 as size_t);
    clock |= plm_buffer_read((*self_0).buffer, 15 as ::core::ffi::c_int) as int64_t;
    plm_buffer_skip((*self_0).buffer, 1 as size_t);
    return clock as ::core::ffi::c_double / 90000.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_decode_packet(
    mut self_0: *mut plm_demux_t,
    mut r#type: ::core::ffi::c_int,
) -> *mut plm_packet_t {
    if plm_buffer_has(
        (*self_0).buffer,
        ((16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as size_t,
    ) == 0
    {
        return ::core::ptr::null_mut::<plm_packet_t>();
    }
    (*self_0).start_code = -1 as ::core::ffi::c_int;
    (*self_0).next_packet.r#type = r#type;
    (*self_0).next_packet.length =
        plm_buffer_read((*self_0).buffer, 16 as ::core::ffi::c_int) as size_t;
    (*self_0).next_packet.length = (*self_0)
        .next_packet
        .length
        .wrapping_sub(plm_buffer_skip_bytes((*self_0).buffer, 0xff as uint8_t) as size_t);
    if plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int) == 0x1 as ::core::ffi::c_int {
        plm_buffer_skip((*self_0).buffer, 16 as size_t);
        (*self_0).next_packet.length = (*self_0).next_packet.length.wrapping_sub(2 as size_t);
    }
    let mut pts_dts_marker: ::core::ffi::c_int =
        plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int);
    if pts_dts_marker == 0x3 as ::core::ffi::c_int {
        (*self_0).next_packet.pts = plm_demux_decode_time(self_0);
        (*self_0).last_decoded_pts = (*self_0).next_packet.pts;
        plm_buffer_skip((*self_0).buffer, 40 as size_t);
        (*self_0).next_packet.length = (*self_0).next_packet.length.wrapping_sub(10 as size_t);
    } else if pts_dts_marker == 0x2 as ::core::ffi::c_int {
        (*self_0).next_packet.pts = plm_demux_decode_time(self_0);
        (*self_0).last_decoded_pts = (*self_0).next_packet.pts;
        (*self_0).next_packet.length = (*self_0).next_packet.length.wrapping_sub(5 as size_t);
    } else if pts_dts_marker == 0 as ::core::ffi::c_int {
        (*self_0).next_packet.pts = PLM_PACKET_INVALID_TS as ::core::ffi::c_double;
        plm_buffer_skip((*self_0).buffer, 4 as size_t);
        (*self_0).next_packet.length = (*self_0).next_packet.length.wrapping_sub(1 as size_t);
    } else {
        return ::core::ptr::null_mut::<plm_packet_t>();
    }
    return plm_demux_get_packet(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn plm_demux_get_packet(mut self_0: *mut plm_demux_t) -> *mut plm_packet_t {
    if plm_buffer_has(
        (*self_0).buffer,
        (*self_0).next_packet.length << 3 as ::core::ffi::c_int,
    ) == 0
    {
        return ::core::ptr::null_mut::<plm_packet_t>();
    }
    (*self_0).current_packet.data = (*(*self_0).buffer)
        .bytes
        .offset(((*(*self_0).buffer).bit_index >> 3 as ::core::ffi::c_int) as isize);
    (*self_0).current_packet.length = (*self_0).next_packet.length;
    (*self_0).current_packet.r#type = (*self_0).next_packet.r#type;
    (*self_0).current_packet.pts = (*self_0).next_packet.pts;
    (*self_0).next_packet.length = 0 as size_t;
    return &raw mut (*self_0).current_packet;
}
static mut PLM_VIDEO_PICTURE_TYPE_INTRA: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut PLM_VIDEO_PICTURE_TYPE_PREDICTIVE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut PLM_VIDEO_PICTURE_TYPE_B: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut PLM_START_SEQUENCE: ::core::ffi::c_int = 0xb3 as ::core::ffi::c_int;
static mut PLM_START_SLICE_FIRST: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
static mut PLM_START_SLICE_LAST: ::core::ffi::c_int = 0xaf as ::core::ffi::c_int;
static mut PLM_START_PICTURE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut PLM_START_EXTENSION: ::core::ffi::c_int = 0xb5 as ::core::ffi::c_int;
static mut PLM_START_USER_DATA: ::core::ffi::c_int = 0xb2 as ::core::ffi::c_int;
static mut PLM_VIDEO_PIXEL_ASPECT_RATIO: [::core::ffi::c_float; 14] = [
    1.0000f32, 0.6735f32, 0.7031f32, 0.7615f32, 0.8055f32, 0.8437f32, 0.8935f32, 0.9157f32,
    0.9815f32, 1.0255f32, 1.0695f32, 1.0950f32, 1.1575f32, 1.2051f32,
];
static mut PLM_VIDEO_PICTURE_RATE: [::core::ffi::c_double; 16] = [
    0.000f64, 23.976f64, 24.000f64, 25.000f64, 29.970f64, 30.000f64, 50.000f64, 59.940f64,
    60.000f64, 0.000f64, 0.000f64, 0.000f64, 0.000f64, 0.000f64, 0.000f64, 0.000f64,
];
static mut PLM_VIDEO_ZIG_ZAG: [uint8_t; 64] = [
    0 as uint8_t,
    1 as uint8_t,
    8 as uint8_t,
    16 as uint8_t,
    9 as uint8_t,
    2 as uint8_t,
    3 as uint8_t,
    10 as uint8_t,
    17 as uint8_t,
    24 as uint8_t,
    32 as uint8_t,
    25 as uint8_t,
    18 as uint8_t,
    11 as uint8_t,
    4 as uint8_t,
    5 as uint8_t,
    12 as uint8_t,
    19 as uint8_t,
    26 as uint8_t,
    33 as uint8_t,
    40 as uint8_t,
    48 as uint8_t,
    41 as uint8_t,
    34 as uint8_t,
    27 as uint8_t,
    20 as uint8_t,
    13 as uint8_t,
    6 as uint8_t,
    7 as uint8_t,
    14 as uint8_t,
    21 as uint8_t,
    28 as uint8_t,
    35 as uint8_t,
    42 as uint8_t,
    49 as uint8_t,
    56 as uint8_t,
    57 as uint8_t,
    50 as uint8_t,
    43 as uint8_t,
    36 as uint8_t,
    29 as uint8_t,
    22 as uint8_t,
    15 as uint8_t,
    23 as uint8_t,
    30 as uint8_t,
    37 as uint8_t,
    44 as uint8_t,
    51 as uint8_t,
    58 as uint8_t,
    59 as uint8_t,
    52 as uint8_t,
    45 as uint8_t,
    38 as uint8_t,
    31 as uint8_t,
    39 as uint8_t,
    46 as uint8_t,
    53 as uint8_t,
    60 as uint8_t,
    61 as uint8_t,
    54 as uint8_t,
    47 as uint8_t,
    55 as uint8_t,
    62 as uint8_t,
    63 as uint8_t,
];
static mut PLM_VIDEO_INTRA_QUANT_MATRIX: [uint8_t; 64] = [
    8 as uint8_t,
    16 as uint8_t,
    19 as uint8_t,
    22 as uint8_t,
    26 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    34 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    22 as uint8_t,
    24 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    34 as uint8_t,
    37 as uint8_t,
    19 as uint8_t,
    22 as uint8_t,
    26 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    34 as uint8_t,
    34 as uint8_t,
    38 as uint8_t,
    22 as uint8_t,
    22 as uint8_t,
    26 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    34 as uint8_t,
    37 as uint8_t,
    40 as uint8_t,
    22 as uint8_t,
    26 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    32 as uint8_t,
    35 as uint8_t,
    40 as uint8_t,
    48 as uint8_t,
    26 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    32 as uint8_t,
    35 as uint8_t,
    40 as uint8_t,
    48 as uint8_t,
    58 as uint8_t,
    26 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    34 as uint8_t,
    38 as uint8_t,
    46 as uint8_t,
    56 as uint8_t,
    69 as uint8_t,
    27 as uint8_t,
    29 as uint8_t,
    35 as uint8_t,
    38 as uint8_t,
    46 as uint8_t,
    56 as uint8_t,
    69 as uint8_t,
    83 as uint8_t,
];
static mut PLM_VIDEO_NON_INTRA_QUANT_MATRIX: [uint8_t; 64] = [
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
    16 as uint8_t,
];
static mut PLM_VIDEO_PREMULTIPLIER_MATRIX: [uint8_t; 64] = [
    32 as uint8_t,
    44 as uint8_t,
    42 as uint8_t,
    38 as uint8_t,
    32 as uint8_t,
    25 as uint8_t,
    17 as uint8_t,
    9 as uint8_t,
    44 as uint8_t,
    62 as uint8_t,
    58 as uint8_t,
    52 as uint8_t,
    44 as uint8_t,
    35 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    42 as uint8_t,
    58 as uint8_t,
    55 as uint8_t,
    49 as uint8_t,
    42 as uint8_t,
    33 as uint8_t,
    23 as uint8_t,
    12 as uint8_t,
    38 as uint8_t,
    52 as uint8_t,
    49 as uint8_t,
    44 as uint8_t,
    38 as uint8_t,
    30 as uint8_t,
    20 as uint8_t,
    10 as uint8_t,
    32 as uint8_t,
    44 as uint8_t,
    42 as uint8_t,
    38 as uint8_t,
    32 as uint8_t,
    25 as uint8_t,
    17 as uint8_t,
    9 as uint8_t,
    25 as uint8_t,
    35 as uint8_t,
    33 as uint8_t,
    30 as uint8_t,
    25 as uint8_t,
    20 as uint8_t,
    14 as uint8_t,
    7 as uint8_t,
    17 as uint8_t,
    24 as uint8_t,
    23 as uint8_t,
    20 as uint8_t,
    17 as uint8_t,
    14 as uint8_t,
    9 as uint8_t,
    5 as uint8_t,
    9 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    10 as uint8_t,
    9 as uint8_t,
    7 as uint8_t,
    5 as uint8_t,
    2 as uint8_t,
];
static mut PLM_VIDEO_MACROBLOCK_ADDRESS_INCREMENT: [plm_vlc_t; 80] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 1 as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 3 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 2 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 5 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 4 as int16_t,
    },
    plm_vlc_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((9 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 7 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 6 as int16_t,
    },
    plm_vlc_t {
        index: ((10 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((11 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((12 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((13 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((14 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((15 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((16 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((17 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((18 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((19 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 9 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 8 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((20 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((21 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((22 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((23 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 15 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 14 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 13 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 12 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 11 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 10 as int16_t,
    },
    plm_vlc_t {
        index: ((24 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((25 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((26 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((28 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((29 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((31 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((33 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((34 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((35 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((36 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((37 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((38 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((39 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 21 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 20 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 19 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 18 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 17 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 16 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 35 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 34 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 33 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 32 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 31 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 30 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 29 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 28 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 27 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 26 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 25 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 24 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 23 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 22 as int16_t,
    },
];
static mut PLM_VIDEO_MACROBLOCK_TYPE_INTRA: [plm_vlc_t; 4] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x1 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x11 as int16_t,
    },
];
static mut PLM_VIDEO_MACROBLOCK_TYPE_PREDICTIVE: [plm_vlc_t; 14] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0xa as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x2 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x8 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x12 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x1a as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x1 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x11 as int16_t,
    },
];
static mut PLM_VIDEO_MACROBLOCK_TYPE_B: [plm_vlc_t; 22] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0xc as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0xe as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x4 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x6 as int16_t,
    },
    plm_vlc_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x8 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0xa as int16_t,
    },
    plm_vlc_t {
        index: ((9 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((10 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x1e as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x1 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x11 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x16 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0x1a as int16_t,
    },
];
static mut PLM_VIDEO_MACROBLOCK_TYPE: [*const plm_vlc_t; 4] = unsafe {
    [
        ::core::ptr::null::<plm_vlc_t>(),
        &raw const PLM_VIDEO_MACROBLOCK_TYPE_INTRA as *const plm_vlc_t,
        &raw const PLM_VIDEO_MACROBLOCK_TYPE_PREDICTIVE as *const plm_vlc_t,
        &raw const PLM_VIDEO_MACROBLOCK_TYPE_B as *const plm_vlc_t,
    ]
};
static mut PLM_VIDEO_CODE_BLOCK_PATTERN: [plm_vlc_t; 126] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((9 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((10 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((11 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((12 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((13 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 60 as int16_t,
    },
    plm_vlc_t {
        index: ((14 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((15 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((16 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((17 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((18 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((19 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((20 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((21 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((22 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((23 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 32 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 16 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 8 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 4 as int16_t,
    },
    plm_vlc_t {
        index: ((24 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((25 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((26 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((28 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((29 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((31 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 62 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 2 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 61 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 1 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 56 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 52 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 44 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 28 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 40 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 20 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 48 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 12 as int16_t,
    },
    plm_vlc_t {
        index: ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((33 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((34 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((35 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((36 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((37 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((38 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((39 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((41 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((42 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((43 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 63 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 3 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 36 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 24 as int16_t,
    },
    plm_vlc_t {
        index: ((44 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((45 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((46 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((47 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((48 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((49 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((50 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((51 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((52 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((53 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((54 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((55 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((56 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((57 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((58 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((59 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 34 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 18 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 10 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 6 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 33 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 17 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 9 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 5 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((60 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((61 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((62 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 58 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 54 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 46 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 30 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 57 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 53 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 45 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 29 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 38 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 26 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 37 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 25 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 43 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 23 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 51 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 15 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 42 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 22 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 50 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 14 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 41 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 21 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 49 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 13 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 35 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 19 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 11 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 7 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 39 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 27 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 59 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 55 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 47 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 31 as int16_t,
    },
];
static mut PLM_VIDEO_MOTION: [plm_vlc_t; 68] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 1 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -1 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 2 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -2 as int16_t,
    },
    plm_vlc_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((9 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 3 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -3 as int16_t,
    },
    plm_vlc_t {
        index: ((10 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((11 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((12 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((13 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((14 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((15 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((16 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((17 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((18 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 4 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -4 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((19 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((20 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((21 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 7 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -7 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 6 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -6 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 5 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -5 as int16_t,
    },
    plm_vlc_t {
        index: ((22 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((23 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((24 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((25 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((26 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((28 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((29 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((31 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((33 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 10 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -10 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 9 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -9 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 8 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -8 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 16 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -16 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 15 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -15 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 14 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -14 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 13 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -13 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 12 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -12 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 11 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: -11 as int16_t,
    },
];
static mut PLM_VIDEO_DCT_SIZE_LUMINANCE: [plm_vlc_t; 18] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 1 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 2 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 3 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 4 as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 5 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 6 as int16_t,
    },
    plm_vlc_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 7 as int16_t,
    },
    plm_vlc_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 8 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
];
static mut PLM_VIDEO_DCT_SIZE_CHROMINANCE: [plm_vlc_t; 18] = [
    plm_vlc_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 1 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 2 as int16_t,
    },
    plm_vlc_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 3 as int16_t,
    },
    plm_vlc_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 4 as int16_t,
    },
    plm_vlc_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 5 as int16_t,
    },
    plm_vlc_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 6 as int16_t,
    },
    plm_vlc_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 7 as int16_t,
    },
    plm_vlc_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as int16_t,
    },
    plm_vlc_t {
        index: 0 as int16_t,
        value: 8 as int16_t,
    },
    plm_vlc_t {
        index: -1 as int16_t,
        value: 0 as int16_t,
    },
];
static mut PLM_VIDEO_DCT_SIZE: [*const plm_vlc_t; 3] = unsafe {
    [
        &raw const PLM_VIDEO_DCT_SIZE_LUMINANCE as *const plm_vlc_t,
        &raw const PLM_VIDEO_DCT_SIZE_CHROMINANCE as *const plm_vlc_t,
        &raw const PLM_VIDEO_DCT_SIZE_CHROMINANCE as *const plm_vlc_t,
    ]
};
static mut PLM_VIDEO_DCT_COEFF: [plm_vlc_uint_t; 224] = [
    plm_vlc_uint_t {
        index: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((2 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((3 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((4 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((5 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((6 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x101 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((7 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((8 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((9 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((10 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x2 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x201 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((11 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((12 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((13 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((14 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((15 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x3 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x401 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x301 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((16 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xffff as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((17 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((18 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x701 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x601 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x102 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x501 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((19 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((20 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((21 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((22 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x202 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x901 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x4 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x801 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((23 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((24 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((25 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((26 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((27 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((28 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((29 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((30 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xd01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x6 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xc01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xb01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x302 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x103 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x5 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xa01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((31 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((32 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((33 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((34 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((35 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((36 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((37 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((38 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((39 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((40 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((41 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((42 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((43 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((44 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((45 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((46 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1001 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x502 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x7 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x203 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x104 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xf01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xe01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x402 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((47 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((48 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((49 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((50 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((51 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((52 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((53 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((54 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((55 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((56 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((57 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((58 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((59 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((60 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((61 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((62 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: -1 as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((63 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((64 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((65 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((66 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((67 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((68 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((69 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((70 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((71 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((72 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((73 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((74 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((75 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((76 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((77 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xb as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x802 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x403 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xa as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x204 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x702 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1501 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1401 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x9 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1301 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1201 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x105 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x303 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x8 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x602 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1101 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((78 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((79 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((80 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((81 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((82 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((83 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((84 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((85 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((86 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((87 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((88 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((89 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((90 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((91 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xa02 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x902 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x503 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x304 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x205 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x107 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x106 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xf as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xe as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xd as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xc as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1a01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1901 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1801 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1701 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1601 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((92 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((93 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((94 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((95 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((96 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((97 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((98 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((99 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((100 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((101 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((102 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((103 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1f as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1e as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1d as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1c as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1b as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1a as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x19 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x18 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x17 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x16 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x15 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x14 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x13 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x12 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x11 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((104 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((105 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((106 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((107 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((108 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((109 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((110 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: ((111 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int16_t,
        value: 0 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x28 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x27 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x26 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x25 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x24 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x23 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x22 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x21 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x20 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10e as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10d as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10c as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10b as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10a as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x109 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x108 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x112 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x111 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x110 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x10f as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x603 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1002 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xf02 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xe02 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xd02 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xc02 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0xb02 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1f01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1e01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1d01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1c01 as uint16_t,
    },
    plm_vlc_uint_t {
        index: 0 as int16_t,
        value: 0x1b01 as uint16_t,
    },
];
#[inline]
unsafe extern "C" fn plm_clamp(mut n: ::core::ffi::c_int) -> uint8_t {
    if n > 255 as ::core::ffi::c_int {
        n = 255 as ::core::ffi::c_int;
    } else if n < 0 as ::core::ffi::c_int {
        n = 0 as ::core::ffi::c_int;
    }
    return n as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_create_with_buffer(
    mut buffer: *mut plm_buffer_t,
    mut destroy_when_done: ::core::ffi::c_int,
) -> *mut plm_video_t {
    let mut self_0: *mut plm_video_t =
        malloc(::core::mem::size_of::<plm_video_t>()) as *mut plm_video_t;
    memset(
        self_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<plm_video_t>(),
    );
    (*self_0).buffer = buffer;
    (*self_0).destroy_buffer_when_done = destroy_when_done;
    (*self_0).start_code = plm_buffer_find_start_code((*self_0).buffer, PLM_START_SEQUENCE);
    if (*self_0).start_code != -1 as ::core::ffi::c_int {
        plm_video_decode_sequence_header(self_0);
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_destroy(mut self_0: *mut plm_video_t) {
    if (*self_0).destroy_buffer_when_done != 0 {
        plm_buffer_destroy((*self_0).buffer);
    }
    if (*self_0).has_sequence_header != 0 {
        free((*self_0).frames_data as *mut ::core::ffi::c_void);
    }
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_get_framerate(
    mut self_0: *mut plm_video_t,
) -> ::core::ffi::c_double {
    return if plm_video_has_header(self_0) != 0 {
        (*self_0).framerate
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_get_pixel_aspect_ratio(
    mut self_0: *mut plm_video_t,
) -> ::core::ffi::c_double {
    return if plm_video_has_header(self_0) != 0 {
        (*self_0).pixel_aspect_ratio
    } else {
        0 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_get_width(mut self_0: *mut plm_video_t) -> ::core::ffi::c_int {
    return if plm_video_has_header(self_0) != 0 {
        (*self_0).width
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_get_height(mut self_0: *mut plm_video_t) -> ::core::ffi::c_int {
    return if plm_video_has_header(self_0) != 0 {
        (*self_0).height
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_set_no_delay(
    mut self_0: *mut plm_video_t,
    mut no_delay: ::core::ffi::c_int,
) {
    (*self_0).assume_no_b_frames = no_delay;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_get_time(mut self_0: *mut plm_video_t) -> ::core::ffi::c_double {
    return (*self_0).time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_set_time(
    mut self_0: *mut plm_video_t,
    mut time: ::core::ffi::c_double,
) {
    (*self_0).frames_decoded = ((*self_0).framerate * time) as ::core::ffi::c_int;
    (*self_0).time = time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_rewind(mut self_0: *mut plm_video_t) {
    plm_buffer_rewind((*self_0).buffer);
    (*self_0).time = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*self_0).frames_decoded = 0 as ::core::ffi::c_int;
    (*self_0).has_reference_frame = FALSE;
    (*self_0).start_code = -1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_has_ended(mut self_0: *mut plm_video_t) -> ::core::ffi::c_int {
    return plm_buffer_has_ended((*self_0).buffer);
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode(mut self_0: *mut plm_video_t) -> *mut plm_frame_t {
    if plm_video_has_header(self_0) == 0 {
        return ::core::ptr::null_mut::<plm_frame_t>();
    }
    let mut frame: *mut plm_frame_t = ::core::ptr::null_mut::<plm_frame_t>();
    loop {
        if (*self_0).start_code != PLM_START_PICTURE {
            (*self_0).start_code = plm_buffer_find_start_code((*self_0).buffer, PLM_START_PICTURE);
            if (*self_0).start_code == -1 as ::core::ffi::c_int {
                if (*self_0).has_reference_frame != 0
                    && (*self_0).assume_no_b_frames == 0
                    && plm_buffer_has_ended((*self_0).buffer) != 0
                    && ((*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_INTRA
                        || (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_PREDICTIVE)
                {
                    (*self_0).has_reference_frame = FALSE;
                    frame = &raw mut (*self_0).frame_backward;
                    break;
                } else {
                    return ::core::ptr::null_mut::<plm_frame_t>();
                }
            }
        }
        if plm_buffer_has_start_code((*self_0).buffer, PLM_START_PICTURE)
            == -1 as ::core::ffi::c_int
            && plm_buffer_has_ended((*self_0).buffer) == 0
        {
            return ::core::ptr::null_mut::<plm_frame_t>();
        }
        plm_buffer_discard_read_bytes((*self_0).buffer);
        plm_video_decode_picture(self_0);
        if (*self_0).assume_no_b_frames != 0 {
            frame = &raw mut (*self_0).frame_backward;
        } else if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_B {
            frame = &raw mut (*self_0).frame_current;
        } else if (*self_0).has_reference_frame != 0 {
            frame = &raw mut (*self_0).frame_forward;
        } else {
            (*self_0).has_reference_frame = TRUE;
        }
        if !frame.is_null() {
            break;
        }
    }
    (*frame).time = (*self_0).time;
    (*self_0).frames_decoded += 1;
    (*self_0).time = (*self_0).frames_decoded as ::core::ffi::c_double / (*self_0).framerate;
    return frame;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_has_header(mut self_0: *mut plm_video_t) -> ::core::ffi::c_int {
    if (*self_0).has_sequence_header != 0 {
        return TRUE;
    }
    if (*self_0).start_code != PLM_START_SEQUENCE {
        (*self_0).start_code = plm_buffer_find_start_code((*self_0).buffer, PLM_START_SEQUENCE);
    }
    if (*self_0).start_code == -1 as ::core::ffi::c_int {
        return FALSE;
    }
    if plm_video_decode_sequence_header(self_0) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_sequence_header(
    mut self_0: *mut plm_video_t,
) -> ::core::ffi::c_int {
    let mut max_header_size: ::core::ffi::c_int = 64 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int * 64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
    if plm_buffer_has((*self_0).buffer, max_header_size as size_t) == 0 {
        return FALSE;
    }
    (*self_0).width = plm_buffer_read((*self_0).buffer, 12 as ::core::ffi::c_int);
    (*self_0).height = plm_buffer_read((*self_0).buffer, 12 as ::core::ffi::c_int);
    if (*self_0).width <= 0 as ::core::ffi::c_int || (*self_0).height <= 0 as ::core::ffi::c_int {
        return FALSE;
    }
    let mut pixel_aspect_ratio_code: ::core::ffi::c_int = 0;
    pixel_aspect_ratio_code = plm_buffer_read((*self_0).buffer, 4 as ::core::ffi::c_int);
    pixel_aspect_ratio_code -= 1 as ::core::ffi::c_int;
    if pixel_aspect_ratio_code < 0 as ::core::ffi::c_int {
        pixel_aspect_ratio_code = 0 as ::core::ffi::c_int;
    }
    let mut par_last: ::core::ffi::c_int = ::core::mem::size_of::<[::core::ffi::c_float; 14]>()
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_float>())
        .wrapping_sub(1usize) as ::core::ffi::c_int;
    if pixel_aspect_ratio_code > par_last {
        pixel_aspect_ratio_code = par_last;
    }
    (*self_0).pixel_aspect_ratio =
        PLM_VIDEO_PIXEL_ASPECT_RATIO[pixel_aspect_ratio_code as usize] as ::core::ffi::c_double;
    (*self_0).framerate =
        PLM_VIDEO_PICTURE_RATE[plm_buffer_read((*self_0).buffer, 4 as ::core::ffi::c_int) as usize];
    plm_buffer_skip(
        (*self_0).buffer,
        (18 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int
            + 10 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as size_t,
    );
    if plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int) != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            let mut idx: ::core::ffi::c_int = PLM_VIDEO_ZIG_ZAG[i as usize] as ::core::ffi::c_int;
            (*self_0).intra_quant_matrix[idx as usize] =
                plm_buffer_read((*self_0).buffer, 8 as ::core::ffi::c_int) as uint8_t;
            i += 1;
        }
    } else {
        memcpy(
            &raw mut (*self_0).intra_quant_matrix as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const PLM_VIDEO_INTRA_QUANT_MATRIX as *const uint8_t as *const ::core::ffi::c_void,
            64 as size_t,
        );
    }
    if plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int) != 0 {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 64 as ::core::ffi::c_int {
            let mut idx_0: ::core::ffi::c_int =
                PLM_VIDEO_ZIG_ZAG[i_0 as usize] as ::core::ffi::c_int;
            (*self_0).non_intra_quant_matrix[idx_0 as usize] =
                plm_buffer_read((*self_0).buffer, 8 as ::core::ffi::c_int) as uint8_t;
            i_0 += 1;
        }
    } else {
        memcpy(
            &raw mut (*self_0).non_intra_quant_matrix as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const PLM_VIDEO_NON_INTRA_QUANT_MATRIX as *const uint8_t
                as *const ::core::ffi::c_void,
            64 as size_t,
        );
    }
    (*self_0).mb_width = (*self_0).width + 15 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
    (*self_0).mb_height = (*self_0).height + 15 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int;
    (*self_0).mb_size = (*self_0).mb_width * (*self_0).mb_height;
    (*self_0).luma_width = (*self_0).mb_width << 4 as ::core::ffi::c_int;
    (*self_0).luma_height = (*self_0).mb_height << 4 as ::core::ffi::c_int;
    (*self_0).chroma_width = (*self_0).mb_width << 3 as ::core::ffi::c_int;
    (*self_0).chroma_height = (*self_0).mb_height << 3 as ::core::ffi::c_int;
    let mut luma_plane_size: size_t = ((*self_0).luma_width * (*self_0).luma_height) as size_t;
    let mut chroma_plane_size: size_t =
        ((*self_0).chroma_width * (*self_0).chroma_height) as size_t;
    let mut frame_data_size: size_t =
        luma_plane_size.wrapping_add((2 as size_t).wrapping_mul(chroma_plane_size));
    (*self_0).frames_data = malloc(frame_data_size.wrapping_mul(3 as size_t)) as *mut uint8_t;
    plm_video_init_frame(
        self_0,
        &raw mut (*self_0).frame_current,
        (*self_0)
            .frames_data
            .offset(frame_data_size.wrapping_mul(0 as size_t) as isize),
    );
    plm_video_init_frame(
        self_0,
        &raw mut (*self_0).frame_forward,
        (*self_0)
            .frames_data
            .offset(frame_data_size.wrapping_mul(1 as size_t) as isize),
    );
    plm_video_init_frame(
        self_0,
        &raw mut (*self_0).frame_backward,
        (*self_0)
            .frames_data
            .offset(frame_data_size.wrapping_mul(2 as size_t) as isize),
    );
    (*self_0).has_sequence_header = TRUE;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_init_frame(
    mut self_0: *mut plm_video_t,
    mut frame: *mut plm_frame_t,
    mut base: *mut uint8_t,
) {
    let mut luma_plane_size: size_t = ((*self_0).luma_width * (*self_0).luma_height) as size_t;
    let mut chroma_plane_size: size_t =
        ((*self_0).chroma_width * (*self_0).chroma_height) as size_t;
    (*frame).width = (*self_0).width as ::core::ffi::c_uint;
    (*frame).height = (*self_0).height as ::core::ffi::c_uint;
    (*frame).y.width = (*self_0).luma_width as ::core::ffi::c_uint;
    (*frame).y.height = (*self_0).luma_height as ::core::ffi::c_uint;
    (*frame).y.data = base;
    (*frame).cr.width = (*self_0).chroma_width as ::core::ffi::c_uint;
    (*frame).cr.height = (*self_0).chroma_height as ::core::ffi::c_uint;
    (*frame).cr.data = base.offset(luma_plane_size as isize);
    (*frame).cb.width = (*self_0).chroma_width as ::core::ffi::c_uint;
    (*frame).cb.height = (*self_0).chroma_height as ::core::ffi::c_uint;
    (*frame).cb.data = base
        .offset(luma_plane_size as isize)
        .offset(chroma_plane_size as isize);
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_picture(mut self_0: *mut plm_video_t) {
    plm_buffer_skip((*self_0).buffer, 10 as size_t);
    (*self_0).picture_type = plm_buffer_read((*self_0).buffer, 3 as ::core::ffi::c_int);
    plm_buffer_skip((*self_0).buffer, 16 as size_t);
    if (*self_0).picture_type <= 0 as ::core::ffi::c_int
        || (*self_0).picture_type > PLM_VIDEO_PICTURE_TYPE_B
    {
        return;
    }
    if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_PREDICTIVE
        || (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_B
    {
        (*self_0).motion_forward.full_px =
            plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int);
        let mut f_code: ::core::ffi::c_int =
            plm_buffer_read((*self_0).buffer, 3 as ::core::ffi::c_int);
        if f_code == 0 as ::core::ffi::c_int {
            return;
        }
        (*self_0).motion_forward.r_size = f_code - 1 as ::core::ffi::c_int;
    }
    if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_B {
        (*self_0).motion_backward.full_px =
            plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int);
        let mut f_code_0: ::core::ffi::c_int =
            plm_buffer_read((*self_0).buffer, 3 as ::core::ffi::c_int);
        if f_code_0 == 0 as ::core::ffi::c_int {
            return;
        }
        (*self_0).motion_backward.r_size = f_code_0 - 1 as ::core::ffi::c_int;
    }
    let mut frame_temp: plm_frame_t = (*self_0).frame_forward;
    if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_INTRA
        || (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_PREDICTIVE
    {
        (*self_0).frame_forward = (*self_0).frame_backward;
    }
    loop {
        (*self_0).start_code = plm_buffer_next_start_code((*self_0).buffer);
        if !((*self_0).start_code == PLM_START_EXTENSION
            || (*self_0).start_code == PLM_START_USER_DATA)
        {
            break;
        }
    }
    while (*self_0).start_code >= PLM_START_SLICE_FIRST
        && (*self_0).start_code <= PLM_START_SLICE_LAST
    {
        plm_video_decode_slice(self_0, (*self_0).start_code & 0xff as ::core::ffi::c_int);
        if (*self_0).macroblock_address >= (*self_0).mb_size - 1 as ::core::ffi::c_int {
            break;
        }
        (*self_0).start_code = plm_buffer_next_start_code((*self_0).buffer);
    }
    if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_INTRA
        || (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_PREDICTIVE
    {
        (*self_0).frame_backward = (*self_0).frame_current;
        (*self_0).frame_current = frame_temp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_slice(
    mut self_0: *mut plm_video_t,
    mut slice: ::core::ffi::c_int,
) {
    (*self_0).slice_begin = TRUE;
    (*self_0).macroblock_address =
        (slice - 1 as ::core::ffi::c_int) * (*self_0).mb_width - 1 as ::core::ffi::c_int;
    (*self_0).motion_forward.h = 0 as ::core::ffi::c_int;
    (*self_0).motion_backward.h = (*self_0).motion_forward.h;
    (*self_0).motion_forward.v = 0 as ::core::ffi::c_int;
    (*self_0).motion_backward.v = (*self_0).motion_forward.v;
    (*self_0).dc_predictor[0usize] = 128 as ::core::ffi::c_int;
    (*self_0).dc_predictor[1usize] = 128 as ::core::ffi::c_int;
    (*self_0).dc_predictor[2usize] = 128 as ::core::ffi::c_int;
    (*self_0).quantizer_scale = plm_buffer_read((*self_0).buffer, 5 as ::core::ffi::c_int);
    while plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int) != 0 {
        plm_buffer_skip((*self_0).buffer, 8 as size_t);
    }
    loop {
        plm_video_decode_macroblock(self_0);
        if !((*self_0).macroblock_address < (*self_0).mb_size - 1 as ::core::ffi::c_int
            && plm_buffer_peek_non_zero((*self_0).buffer, 23 as ::core::ffi::c_int) != 0)
        {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_macroblock(mut self_0: *mut plm_video_t) {
    let mut increment: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut t: ::core::ffi::c_int = plm_buffer_read_vlc(
        (*self_0).buffer,
        &raw const PLM_VIDEO_MACROBLOCK_ADDRESS_INCREMENT as *const plm_vlc_t,
    ) as ::core::ffi::c_int;
    while t == 34 as ::core::ffi::c_int {
        t = plm_buffer_read_vlc(
            (*self_0).buffer,
            &raw const PLM_VIDEO_MACROBLOCK_ADDRESS_INCREMENT as *const plm_vlc_t,
        ) as ::core::ffi::c_int;
    }
    while t == 35 as ::core::ffi::c_int {
        increment += 33 as ::core::ffi::c_int;
        t = plm_buffer_read_vlc(
            (*self_0).buffer,
            &raw const PLM_VIDEO_MACROBLOCK_ADDRESS_INCREMENT as *const plm_vlc_t,
        ) as ::core::ffi::c_int;
    }
    increment += t;
    if (*self_0).slice_begin != 0 {
        (*self_0).slice_begin = FALSE;
        (*self_0).macroblock_address += increment;
    } else {
        if (*self_0).macroblock_address + increment >= (*self_0).mb_size {
            return;
        }
        if increment > 1 as ::core::ffi::c_int {
            (*self_0).dc_predictor[0usize] = 128 as ::core::ffi::c_int;
            (*self_0).dc_predictor[1usize] = 128 as ::core::ffi::c_int;
            (*self_0).dc_predictor[2usize] = 128 as ::core::ffi::c_int;
            if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_PREDICTIVE {
                (*self_0).motion_forward.h = 0 as ::core::ffi::c_int;
                (*self_0).motion_forward.v = 0 as ::core::ffi::c_int;
            }
        }
        while increment > 1 as ::core::ffi::c_int {
            (*self_0).macroblock_address += 1;
            (*self_0).mb_row = (*self_0).macroblock_address / (*self_0).mb_width;
            (*self_0).mb_col = (*self_0).macroblock_address % (*self_0).mb_width;
            plm_video_predict_macroblock(self_0);
            increment -= 1;
        }
        (*self_0).macroblock_address += 1;
    }
    (*self_0).mb_row = (*self_0).macroblock_address / (*self_0).mb_width;
    (*self_0).mb_col = (*self_0).macroblock_address % (*self_0).mb_width;
    if (*self_0).mb_col >= (*self_0).mb_width || (*self_0).mb_row >= (*self_0).mb_height {
        return;
    }
    let mut table: *const plm_vlc_t = PLM_VIDEO_MACROBLOCK_TYPE[(*self_0).picture_type as usize];
    (*self_0).macroblock_type = plm_buffer_read_vlc((*self_0).buffer, table) as ::core::ffi::c_int;
    (*self_0).macroblock_intra = (*self_0).macroblock_type & 0x1 as ::core::ffi::c_int;
    (*self_0).motion_forward.is_set = (*self_0).macroblock_type & 0x8 as ::core::ffi::c_int;
    (*self_0).motion_backward.is_set = (*self_0).macroblock_type & 0x4 as ::core::ffi::c_int;
    if (*self_0).macroblock_type & 0x10 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        (*self_0).quantizer_scale = plm_buffer_read((*self_0).buffer, 5 as ::core::ffi::c_int);
    }
    if (*self_0).macroblock_intra != 0 {
        (*self_0).motion_forward.h = 0 as ::core::ffi::c_int;
        (*self_0).motion_backward.h = (*self_0).motion_forward.h;
        (*self_0).motion_forward.v = 0 as ::core::ffi::c_int;
        (*self_0).motion_backward.v = (*self_0).motion_forward.v;
    } else {
        (*self_0).dc_predictor[0usize] = 128 as ::core::ffi::c_int;
        (*self_0).dc_predictor[1usize] = 128 as ::core::ffi::c_int;
        (*self_0).dc_predictor[2usize] = 128 as ::core::ffi::c_int;
        plm_video_decode_motion_vectors(self_0);
        plm_video_predict_macroblock(self_0);
    }
    let mut cbp: ::core::ffi::c_int =
        if (*self_0).macroblock_type & 0x2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            plm_buffer_read_vlc(
                (*self_0).buffer,
                &raw const PLM_VIDEO_CODE_BLOCK_PATTERN as *const plm_vlc_t,
            ) as ::core::ffi::c_int
        } else if (*self_0).macroblock_intra != 0 {
            0x3f as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    let mut block: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mask: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
    while block < 6 as ::core::ffi::c_int {
        if cbp & mask != 0 as ::core::ffi::c_int {
            plm_video_decode_block(self_0, block);
        }
        mask >>= 1 as ::core::ffi::c_int;
        block += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_motion_vectors(mut self_0: *mut plm_video_t) {
    if (*self_0).motion_forward.is_set != 0 {
        let mut r_size: ::core::ffi::c_int = (*self_0).motion_forward.r_size;
        (*self_0).motion_forward.h =
            plm_video_decode_motion_vector(self_0, r_size, (*self_0).motion_forward.h);
        (*self_0).motion_forward.v =
            plm_video_decode_motion_vector(self_0, r_size, (*self_0).motion_forward.v);
    } else if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_PREDICTIVE {
        (*self_0).motion_forward.h = 0 as ::core::ffi::c_int;
        (*self_0).motion_forward.v = 0 as ::core::ffi::c_int;
    }
    if (*self_0).motion_backward.is_set != 0 {
        let mut r_size_0: ::core::ffi::c_int = (*self_0).motion_backward.r_size;
        (*self_0).motion_backward.h =
            plm_video_decode_motion_vector(self_0, r_size_0, (*self_0).motion_backward.h);
        (*self_0).motion_backward.v =
            plm_video_decode_motion_vector(self_0, r_size_0, (*self_0).motion_backward.v);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_motion_vector(
    mut self_0: *mut plm_video_t,
    mut r_size: ::core::ffi::c_int,
    mut motion: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fscale: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << r_size;
    let mut m_code: ::core::ffi::c_int = plm_buffer_read_vlc(
        (*self_0).buffer,
        &raw const PLM_VIDEO_MOTION as *const plm_vlc_t,
    ) as ::core::ffi::c_int;
    let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut d: ::core::ffi::c_int = 0;
    if m_code != 0 as ::core::ffi::c_int && fscale != 1 as ::core::ffi::c_int {
        r = plm_buffer_read((*self_0).buffer, r_size);
        d = ((abs(m_code) - 1 as ::core::ffi::c_int) << r_size) + r + 1 as ::core::ffi::c_int;
        if m_code < 0 as ::core::ffi::c_int {
            d = -d;
        }
    } else {
        d = m_code;
    }
    motion += d;
    if motion > (fscale << 4 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int {
        motion -= fscale << 5 as ::core::ffi::c_int;
    } else if motion
        < ((-fscale as ::core::ffi::c_uint) << 4 as ::core::ffi::c_int) as ::core::ffi::c_int
    {
        motion += fscale << 5 as ::core::ffi::c_int;
    }
    return motion;
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_predict_macroblock(mut self_0: *mut plm_video_t) {
    let mut fw_h: ::core::ffi::c_int = (*self_0).motion_forward.h;
    let mut fw_v: ::core::ffi::c_int = (*self_0).motion_forward.v;
    if (*self_0).motion_forward.full_px != 0 {
        fw_h <<= 1 as ::core::ffi::c_int;
        fw_v <<= 1 as ::core::ffi::c_int;
    }
    if (*self_0).picture_type == PLM_VIDEO_PICTURE_TYPE_B {
        let mut bw_h: ::core::ffi::c_int = (*self_0).motion_backward.h;
        let mut bw_v: ::core::ffi::c_int = (*self_0).motion_backward.v;
        if (*self_0).motion_backward.full_px != 0 {
            bw_h <<= 1 as ::core::ffi::c_int;
            bw_v <<= 1 as ::core::ffi::c_int;
        }
        if (*self_0).motion_forward.is_set != 0 {
            plm_video_copy_macroblock(self_0, &raw mut (*self_0).frame_forward, fw_h, fw_v);
            if (*self_0).motion_backward.is_set != 0 {
                plm_video_interpolate_macroblock(
                    self_0,
                    &raw mut (*self_0).frame_backward,
                    bw_h,
                    bw_v,
                );
            }
        } else {
            plm_video_copy_macroblock(self_0, &raw mut (*self_0).frame_backward, bw_h, bw_v);
        }
    } else {
        plm_video_copy_macroblock(self_0, &raw mut (*self_0).frame_forward, fw_h, fw_v);
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_copy_macroblock(
    mut self_0: *mut plm_video_t,
    mut s: *mut plm_frame_t,
    mut motion_h: ::core::ffi::c_int,
    mut motion_v: ::core::ffi::c_int,
) {
    let mut d: *mut plm_frame_t = &raw mut (*self_0).frame_current;
    plm_video_process_macroblock(
        self_0,
        (*s).y.data,
        (*d).y.data,
        motion_h,
        motion_v,
        16 as ::core::ffi::c_int,
        FALSE,
    );
    plm_video_process_macroblock(
        self_0,
        (*s).cr.data,
        (*d).cr.data,
        motion_h / 2 as ::core::ffi::c_int,
        motion_v / 2 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        FALSE,
    );
    plm_video_process_macroblock(
        self_0,
        (*s).cb.data,
        (*d).cb.data,
        motion_h / 2 as ::core::ffi::c_int,
        motion_v / 2 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        FALSE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_interpolate_macroblock(
    mut self_0: *mut plm_video_t,
    mut s: *mut plm_frame_t,
    mut motion_h: ::core::ffi::c_int,
    mut motion_v: ::core::ffi::c_int,
) {
    let mut d: *mut plm_frame_t = &raw mut (*self_0).frame_current;
    plm_video_process_macroblock(
        self_0,
        (*s).y.data,
        (*d).y.data,
        motion_h,
        motion_v,
        16 as ::core::ffi::c_int,
        TRUE,
    );
    plm_video_process_macroblock(
        self_0,
        (*s).cr.data,
        (*d).cr.data,
        motion_h / 2 as ::core::ffi::c_int,
        motion_v / 2 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        TRUE,
    );
    plm_video_process_macroblock(
        self_0,
        (*s).cb.data,
        (*d).cb.data,
        motion_h / 2 as ::core::ffi::c_int,
        motion_v / 2 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        TRUE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_process_macroblock(
    mut self_0: *mut plm_video_t,
    mut s: *mut uint8_t,
    mut d: *mut uint8_t,
    mut motion_h: ::core::ffi::c_int,
    mut motion_v: ::core::ffi::c_int,
    mut block_size: ::core::ffi::c_int,
    mut interpolate: ::core::ffi::c_int,
) {
    let mut dw: ::core::ffi::c_int = (*self_0).mb_width * block_size;
    let mut hp: ::core::ffi::c_int = motion_h >> 1 as ::core::ffi::c_int;
    let mut vp: ::core::ffi::c_int = motion_v >> 1 as ::core::ffi::c_int;
    let mut odd_h: ::core::ffi::c_int =
        (motion_h & 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut odd_v: ::core::ffi::c_int =
        (motion_v & 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut si: ::core::ffi::c_uint = (((*self_0).mb_row * block_size + vp) * dw
        + (*self_0).mb_col * block_size
        + hp) as ::core::ffi::c_uint;
    let mut di: ::core::ffi::c_uint =
        (((*self_0).mb_row * dw + (*self_0).mb_col) * block_size) as ::core::ffi::c_uint;
    let mut max_address: ::core::ffi::c_uint = (dw
        * ((*self_0).mb_height * block_size - block_size + 1 as ::core::ffi::c_int)
        - block_size) as ::core::ffi::c_uint;
    if si > max_address || di > max_address {
        return;
    }
    match interpolate << 2 as ::core::ffi::c_int | odd_h << 1 as ::core::ffi::c_int | odd_v {
        0 => {
            let mut dest_scan: ::core::ffi::c_int = dw - block_size;
            let mut source_scan: ::core::ffi::c_int = dw - block_size;
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < block_size {
                let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x < block_size {
                    *d.offset(di as isize) = *s.offset(si as isize);
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x += 1;
                }
                si = si.wrapping_add(source_scan as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan as ::core::ffi::c_uint);
                y += 1;
            }
        }
        1 => {
            let mut dest_scan_0: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_0: ::core::ffi::c_int = dw - block_size;
            let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_0 < block_size {
                let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_0 < block_size {
                    *d.offset(di as isize) = (*s.offset(si as isize) as ::core::ffi::c_int
                        + *s.offset(si.wrapping_add(dw as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_0 += 1;
                }
                si = si.wrapping_add(source_scan_0 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_0 as ::core::ffi::c_uint);
                y_0 += 1;
            }
        }
        2 => {
            let mut dest_scan_1: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_1: ::core::ffi::c_int = dw - block_size;
            let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_1 < block_size {
                let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_1 < block_size {
                    *d.offset(di as isize) = (*s.offset(si as isize) as ::core::ffi::c_int
                        + *s.offset(si.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_1 += 1;
                }
                si = si.wrapping_add(source_scan_1 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_1 as ::core::ffi::c_uint);
                y_1 += 1;
            }
        }
        3 => {
            let mut dest_scan_2: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_2: ::core::ffi::c_int = dw - block_size;
            let mut y_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_2 < block_size {
                let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_2 < block_size {
                    *d.offset(di as isize) = (*s.offset(si as isize) as ::core::ffi::c_int
                        + *s.offset(si.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int
                        + *s.offset(si.wrapping_add(dw as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int
                        + *s.offset(
                            si.wrapping_add(dw as ::core::ffi::c_uint)
                                .wrapping_add(1 as ::core::ffi::c_uint)
                                as isize,
                        ) as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_2 += 1;
                }
                si = si.wrapping_add(source_scan_2 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_2 as ::core::ffi::c_uint);
                y_2 += 1;
            }
        }
        4 => {
            let mut dest_scan_3: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_3: ::core::ffi::c_int = dw - block_size;
            let mut y_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_3 < block_size {
                let mut x_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_3 < block_size {
                    *d.offset(di as isize) = (*d.offset(di as isize) as ::core::ffi::c_int
                        + *s.offset(si as isize) as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_3 += 1;
                }
                si = si.wrapping_add(source_scan_3 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_3 as ::core::ffi::c_uint);
                y_3 += 1;
            }
        }
        5 => {
            let mut dest_scan_4: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_4: ::core::ffi::c_int = dw - block_size;
            let mut y_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_4 < block_size {
                let mut x_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_4 < block_size {
                    *d.offset(di as isize) = (*d.offset(di as isize) as ::core::ffi::c_int
                        + (*s.offset(si as isize) as ::core::ffi::c_int
                            + *s.offset(si.wrapping_add(dw as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_4 += 1;
                }
                si = si.wrapping_add(source_scan_4 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_4 as ::core::ffi::c_uint);
                y_4 += 1;
            }
        }
        6 => {
            let mut dest_scan_5: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_5: ::core::ffi::c_int = dw - block_size;
            let mut y_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_5 < block_size {
                let mut x_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_5 < block_size {
                    *d.offset(di as isize) = (*d.offset(di as isize) as ::core::ffi::c_int
                        + (*s.offset(si as isize) as ::core::ffi::c_int
                            + *s.offset(si.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_5 += 1;
                }
                si = si.wrapping_add(source_scan_5 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_5 as ::core::ffi::c_uint);
                y_5 += 1;
            }
        }
        7 => {
            let mut dest_scan_6: ::core::ffi::c_int = dw - block_size;
            let mut source_scan_6: ::core::ffi::c_int = dw - block_size;
            let mut y_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_6 < block_size {
                let mut x_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_6 < block_size {
                    *d.offset(di as isize) = (*d.offset(di as isize) as ::core::ffi::c_int
                        + (*s.offset(si as isize) as ::core::ffi::c_int
                            + *s.offset(si.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_int
                            + *s.offset(si.wrapping_add(dw as ::core::ffi::c_uint) as isize)
                                as ::core::ffi::c_int
                            + *s.offset(
                                si.wrapping_add(dw as ::core::ffi::c_uint)
                                    .wrapping_add(1 as ::core::ffi::c_uint)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int)
                        as uint8_t;
                    si = si.wrapping_add(1);
                    di = di.wrapping_add(1);
                    x_6 += 1;
                }
                si = si.wrapping_add(source_scan_6 as ::core::ffi::c_uint);
                di = di.wrapping_add(dest_scan_6 as ::core::ffi::c_uint);
                y_6 += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_decode_block(
    mut self_0: *mut plm_video_t,
    mut block: ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut quant_matrix: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if (*self_0).macroblock_intra != 0 {
        let mut predictor: ::core::ffi::c_int = 0;
        let mut dct_size: ::core::ffi::c_int = 0;
        let mut plane_index: ::core::ffi::c_int = if block > 3 as ::core::ffi::c_int {
            block - 3 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        predictor = (*self_0).dc_predictor[plane_index as usize];
        dct_size = plm_buffer_read_vlc((*self_0).buffer, PLM_VIDEO_DCT_SIZE[plane_index as usize])
            as ::core::ffi::c_int;
        if dct_size > 0 as ::core::ffi::c_int {
            let mut differential: ::core::ffi::c_int = plm_buffer_read((*self_0).buffer, dct_size);
            if differential & (1 as ::core::ffi::c_int) << dct_size - 1 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                (*self_0).block_data[0usize] = predictor + differential;
            } else {
                (*self_0).block_data[0usize] = predictor
                    + (-((1 as ::core::ffi::c_int) << dct_size)
                        | differential + 1 as ::core::ffi::c_int);
            }
        } else {
            (*self_0).block_data[0usize] = predictor;
        }
        (*self_0).dc_predictor[plane_index as usize] = (*self_0).block_data[0usize];
        (*self_0).block_data[0usize] <<= 3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
        quant_matrix = &raw mut (*self_0).intra_quant_matrix as *mut uint8_t;
        n = 1 as ::core::ffi::c_int;
    } else {
        quant_matrix = &raw mut (*self_0).non_intra_quant_matrix as *mut uint8_t;
    }
    let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut run: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut coeff: uint16_t = plm_buffer_read_vlc_uint(
            (*self_0).buffer,
            &raw const PLM_VIDEO_DCT_COEFF as *const plm_vlc_uint_t,
        );
        if coeff as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int
            && n > 0 as ::core::ffi::c_int
            && plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
        {
            break;
        }
        if coeff as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int {
            run = plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
            level = plm_buffer_read((*self_0).buffer, 8 as ::core::ffi::c_int);
            if level == 0 as ::core::ffi::c_int {
                level = plm_buffer_read((*self_0).buffer, 8 as ::core::ffi::c_int);
            } else if level == 128 as ::core::ffi::c_int {
                level = plm_buffer_read((*self_0).buffer, 8 as ::core::ffi::c_int)
                    - 256 as ::core::ffi::c_int;
            } else if level > 128 as ::core::ffi::c_int {
                level = level - 256 as ::core::ffi::c_int;
            }
        } else {
            run = coeff as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
            level = coeff as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
            if plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int) != 0 {
                level = -level;
            }
        }
        n += run;
        if n < 0 as ::core::ffi::c_int || n >= 64 as ::core::ffi::c_int {
            return;
        }
        let mut de_zig_zagged: ::core::ffi::c_int =
            PLM_VIDEO_ZIG_ZAG[n as usize] as ::core::ffi::c_int;
        n += 1;
        level = ((level as ::core::ffi::c_uint) << 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if (*self_0).macroblock_intra == 0 {
            level += if level < 0 as ::core::ffi::c_int {
                -1 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        }
        level = level
            * (*self_0).quantizer_scale
            * *quant_matrix.offset(de_zig_zagged as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int;
        if level & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            level -= if level > 0 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                -1 as ::core::ffi::c_int
            };
        }
        if level > 2047 as ::core::ffi::c_int {
            level = 2047 as ::core::ffi::c_int;
        } else if level < -2048 as ::core::ffi::c_int {
            level = -2048 as ::core::ffi::c_int;
        }
        (*self_0).block_data[de_zig_zagged as usize] =
            level * PLM_VIDEO_PREMULTIPLIER_MATRIX[de_zig_zagged as usize] as ::core::ffi::c_int;
    }
    let mut d: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dw: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = 0;
    if block < 4 as ::core::ffi::c_int {
        d = (*self_0).frame_current.y.data;
        dw = (*self_0).luma_width;
        di = (*self_0).mb_row * (*self_0).luma_width + (*self_0).mb_col << 4 as ::core::ffi::c_int;
        if block & 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            di += 8 as ::core::ffi::c_int;
        }
        if block & 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            di += (*self_0).luma_width << 3 as ::core::ffi::c_int;
        }
    } else {
        d = if block == 4 as ::core::ffi::c_int {
            (*self_0).frame_current.cb.data
        } else {
            (*self_0).frame_current.cr.data
        };
        dw = (*self_0).chroma_width;
        di = ((*self_0).mb_row * (*self_0).luma_width << 2 as ::core::ffi::c_int)
            + ((*self_0).mb_col << 3 as ::core::ffi::c_int);
    }
    let mut s: *mut ::core::ffi::c_int = &raw mut (*self_0).block_data as *mut ::core::ffi::c_int;
    let mut si: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*self_0).macroblock_intra != 0 {
        if n == 1 as ::core::ffi::c_int {
            let mut clamped: ::core::ffi::c_int =
                plm_clamp(*s.offset(0isize) + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            let mut dest_scan: ::core::ffi::c_int = dw - 8 as ::core::ffi::c_int;
            let mut source_scan: ::core::ffi::c_int =
                8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < 8 as ::core::ffi::c_int {
                let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x < 8 as ::core::ffi::c_int {
                    *d.offset(di as isize) = clamped as uint8_t;
                    si += 1;
                    di += 1;
                    x += 1;
                }
                si += source_scan;
                di += dest_scan;
                y += 1;
            }
            *s.offset(0isize) = 0 as ::core::ffi::c_int;
        } else {
            plm_video_idct(s);
            let mut dest_scan_0: ::core::ffi::c_int = dw - 8 as ::core::ffi::c_int;
            let mut source_scan_0: ::core::ffi::c_int =
                8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
            let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_0 < 8 as ::core::ffi::c_int {
                let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_0 < 8 as ::core::ffi::c_int {
                    *d.offset(di as isize) = plm_clamp(*s.offset(si as isize));
                    si += 1;
                    di += 1;
                    x_0 += 1;
                }
                si += source_scan_0;
                di += dest_scan_0;
                y_0 += 1;
            }
            memset(
                &raw mut (*self_0).block_data as *mut ::core::ffi::c_int
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<[::core::ffi::c_int; 64]>(),
            );
        }
    } else if n == 1 as ::core::ffi::c_int {
        let mut value: ::core::ffi::c_int =
            *s.offset(0isize) + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        let mut dest_scan_1: ::core::ffi::c_int = dw - 8 as ::core::ffi::c_int;
        let mut source_scan_1: ::core::ffi::c_int =
            8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_1 < 8 as ::core::ffi::c_int {
            let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_1 < 8 as ::core::ffi::c_int {
                *d.offset(di as isize) =
                    plm_clamp(*d.offset(di as isize) as ::core::ffi::c_int + value);
                si += 1;
                di += 1;
                x_1 += 1;
            }
            si += source_scan_1;
            di += dest_scan_1;
            y_1 += 1;
        }
        *s.offset(0isize) = 0 as ::core::ffi::c_int;
    } else {
        plm_video_idct(s);
        let mut dest_scan_2: ::core::ffi::c_int = dw - 8 as ::core::ffi::c_int;
        let mut source_scan_2: ::core::ffi::c_int =
            8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int;
        let mut y_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_2 < 8 as ::core::ffi::c_int {
            let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_2 < 8 as ::core::ffi::c_int {
                *d.offset(di as isize) = plm_clamp(
                    *d.offset(di as isize) as ::core::ffi::c_int + *s.offset(si as isize),
                );
                si += 1;
                di += 1;
                x_2 += 1;
            }
            si += source_scan_2;
            di += dest_scan_2;
            y_2 += 1;
        }
        memset(
            &raw mut (*self_0).block_data as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_int; 64]>(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_video_idct(mut block: *mut ::core::ffi::c_int) {
    let mut b1: ::core::ffi::c_int = 0;
    let mut b3: ::core::ffi::c_int = 0;
    let mut b4: ::core::ffi::c_int = 0;
    let mut b6: ::core::ffi::c_int = 0;
    let mut b7: ::core::ffi::c_int = 0;
    let mut tmp1: ::core::ffi::c_int = 0;
    let mut tmp2: ::core::ffi::c_int = 0;
    let mut m0: ::core::ffi::c_int = 0;
    let mut x0: ::core::ffi::c_int = 0;
    let mut x1: ::core::ffi::c_int = 0;
    let mut x2: ::core::ffi::c_int = 0;
    let mut x3: ::core::ffi::c_int = 0;
    let mut x4: ::core::ffi::c_int = 0;
    let mut y3: ::core::ffi::c_int = 0;
    let mut y4: ::core::ffi::c_int = 0;
    let mut y5: ::core::ffi::c_int = 0;
    let mut y6: ::core::ffi::c_int = 0;
    let mut y7: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        b1 = *block.offset((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        b3 = *block.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
            + *block.offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        b4 = *block.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
            - *block.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        tmp1 = *block.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
            + *block.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        tmp2 = *block.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
            + *block.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        b6 = *block.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
            - *block.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        b7 = tmp1 + tmp2;
        m0 = *block.offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize);
        x4 = (b6 * 473 as ::core::ffi::c_int - b4 * 196 as ::core::ffi::c_int
            + 128 as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int)
            - b7;
        x0 = x4
            - ((tmp1 - tmp2) * 362 as ::core::ffi::c_int + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int);
        x1 = m0 - b1;
        x2 = ((*block.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
            - *block.offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize))
            * 362 as ::core::ffi::c_int
            + 128 as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int)
            - b3;
        x3 = m0 + b1;
        y3 = x1 + x2;
        y4 = x3 + b3;
        y5 = x1 - x2;
        y6 = x3 - b3;
        y7 = -x0
            - (b4 * 473 as ::core::ffi::c_int
                + b6 * 196 as ::core::ffi::c_int
                + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int);
        *block.offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = b7 + y4;
        *block.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = x4 + y3;
        *block.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = y5 - x0;
        *block.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = y6 - y7;
        *block.offset((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = y6 + y7;
        *block.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = x0 + y5;
        *block.offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = y3 - x4;
        *block.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) = y4 - b7;
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 64 as ::core::ffi::c_int {
        b1 = *block.offset((4 as ::core::ffi::c_int + i_0) as isize);
        b3 = *block.offset((2 as ::core::ffi::c_int + i_0) as isize)
            + *block.offset((6 as ::core::ffi::c_int + i_0) as isize);
        b4 = *block.offset((5 as ::core::ffi::c_int + i_0) as isize)
            - *block.offset((3 as ::core::ffi::c_int + i_0) as isize);
        tmp1 = *block.offset((1 as ::core::ffi::c_int + i_0) as isize)
            + *block.offset((7 as ::core::ffi::c_int + i_0) as isize);
        tmp2 = *block.offset((3 as ::core::ffi::c_int + i_0) as isize)
            + *block.offset((5 as ::core::ffi::c_int + i_0) as isize);
        b6 = *block.offset((1 as ::core::ffi::c_int + i_0) as isize)
            - *block.offset((7 as ::core::ffi::c_int + i_0) as isize);
        b7 = tmp1 + tmp2;
        m0 = *block.offset((0 as ::core::ffi::c_int + i_0) as isize);
        x4 = (b6 * 473 as ::core::ffi::c_int - b4 * 196 as ::core::ffi::c_int
            + 128 as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int)
            - b7;
        x0 = x4
            - ((tmp1 - tmp2) * 362 as ::core::ffi::c_int + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int);
        x1 = m0 - b1;
        x2 = ((*block.offset((2 as ::core::ffi::c_int + i_0) as isize)
            - *block.offset((6 as ::core::ffi::c_int + i_0) as isize))
            * 362 as ::core::ffi::c_int
            + 128 as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int)
            - b3;
        x3 = m0 + b1;
        y3 = x1 + x2;
        y4 = x3 + b3;
        y5 = x1 - x2;
        y6 = x3 - b3;
        y7 = -x0
            - (b4 * 473 as ::core::ffi::c_int
                + b6 * 196 as ::core::ffi::c_int
                + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int);
        *block.offset((0 as ::core::ffi::c_int + i_0) as isize) =
            b7 + y4 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((1 as ::core::ffi::c_int + i_0) as isize) =
            x4 + y3 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((2 as ::core::ffi::c_int + i_0) as isize) =
            y5 - x0 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((3 as ::core::ffi::c_int + i_0) as isize) =
            y6 - y7 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((4 as ::core::ffi::c_int + i_0) as isize) =
            y6 + y7 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((5 as ::core::ffi::c_int + i_0) as isize) =
            x0 + y5 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((6 as ::core::ffi::c_int + i_0) as isize) =
            y3 - x4 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        *block.offset((7 as ::core::ffi::c_int + i_0) as isize) =
            y4 - b7 + 128 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int;
        i_0 += 8 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_frame_to_rgb(
    mut frame: *mut plm_frame_t,
    mut dest: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut cols: ::core::ffi::c_int =
        ((*frame).width >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int =
        ((*frame).height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut yw: ::core::ffi::c_int = (*frame).y.width as ::core::ffi::c_int;
    let mut cw: ::core::ffi::c_int = (*frame).cb.width as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        let mut c_index: ::core::ffi::c_int = row * cw;
        let mut y_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * yw;
        let mut d_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * stride;
        let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while col < cols {
            let mut y: ::core::ffi::c_int = 0;
            let mut cr: ::core::ffi::c_int = *(*frame).cr.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut cb: ::core::ffi::c_int = *(*frame).cb.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int =
                cr * 104597 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            let mut g: ::core::ffi::c_int = cb * 25674 as ::core::ffi::c_int
                + cr * 53278 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            let mut b: ::core::ffi::c_int =
                cb * 132201 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame).y.data.offset((y_index + yw) as isize) as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + stride + 0 as ::core::ffi::c_int) as isize) = plm_clamp(y + r);
            *dest.offset((d_index + stride + 1 as ::core::ffi::c_int) as isize) = plm_clamp(y - g);
            *dest.offset((d_index + stride + 2 as ::core::ffi::c_int) as isize) = plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + yw + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset(
                (d_index + stride + 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + r);
            *dest.offset(
                (d_index + stride + 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y - g);
            *dest.offset(
                (d_index + stride + 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + b);
            c_index += 1 as ::core::ffi::c_int;
            y_index += 2 as ::core::ffi::c_int;
            d_index += 2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_frame_to_bgr(
    mut frame: *mut plm_frame_t,
    mut dest: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut cols: ::core::ffi::c_int =
        ((*frame).width >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int =
        ((*frame).height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut yw: ::core::ffi::c_int = (*frame).y.width as ::core::ffi::c_int;
    let mut cw: ::core::ffi::c_int = (*frame).cb.width as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        let mut c_index: ::core::ffi::c_int = row * cw;
        let mut y_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * yw;
        let mut d_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * stride;
        let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while col < cols {
            let mut y: ::core::ffi::c_int = 0;
            let mut cr: ::core::ffi::c_int = *(*frame).cr.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut cb: ::core::ffi::c_int = *(*frame).cb.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int =
                cr * 104597 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            let mut g: ::core::ffi::c_int = cb * 25674 as ::core::ffi::c_int
                + cr * 53278 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            let mut b: ::core::ffi::c_int =
                cb * 132201 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame).y.data.offset((y_index + yw) as isize) as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + stride + 2 as ::core::ffi::c_int) as isize) = plm_clamp(y + r);
            *dest.offset((d_index + stride + 1 as ::core::ffi::c_int) as isize) = plm_clamp(y - g);
            *dest.offset((d_index + stride + 0 as ::core::ffi::c_int) as isize) = plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + yw + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset(
                (d_index + stride + 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + r);
            *dest.offset(
                (d_index + stride + 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y - g);
            *dest.offset(
                (d_index + stride + 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + b);
            c_index += 1 as ::core::ffi::c_int;
            y_index += 2 as ::core::ffi::c_int;
            d_index += 2 as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_frame_to_rgba(
    mut frame: *mut plm_frame_t,
    mut dest: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut cols: ::core::ffi::c_int =
        ((*frame).width >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int =
        ((*frame).height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut yw: ::core::ffi::c_int = (*frame).y.width as ::core::ffi::c_int;
    let mut cw: ::core::ffi::c_int = (*frame).cb.width as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        let mut c_index: ::core::ffi::c_int = row * cw;
        let mut y_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * yw;
        let mut d_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * stride;
        let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while col < cols {
            let mut y: ::core::ffi::c_int = 0;
            let mut cr: ::core::ffi::c_int = *(*frame).cr.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut cb: ::core::ffi::c_int = *(*frame).cb.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int =
                cr * 104597 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            let mut g: ::core::ffi::c_int = cb * 25674 as ::core::ffi::c_int
                + cr * 53278 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            let mut b: ::core::ffi::c_int =
                cb * 132201 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame).y.data.offset((y_index + yw) as isize) as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + stride + 0 as ::core::ffi::c_int) as isize) = plm_clamp(y + r);
            *dest.offset((d_index + stride + 1 as ::core::ffi::c_int) as isize) = plm_clamp(y - g);
            *dest.offset((d_index + stride + 2 as ::core::ffi::c_int) as isize) = plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + yw + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + r);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y - g);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + b);
            c_index += 1 as ::core::ffi::c_int;
            y_index += 2 as ::core::ffi::c_int;
            d_index += 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_frame_to_bgra(
    mut frame: *mut plm_frame_t,
    mut dest: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut cols: ::core::ffi::c_int =
        ((*frame).width >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int =
        ((*frame).height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut yw: ::core::ffi::c_int = (*frame).y.width as ::core::ffi::c_int;
    let mut cw: ::core::ffi::c_int = (*frame).cb.width as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        let mut c_index: ::core::ffi::c_int = row * cw;
        let mut y_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * yw;
        let mut d_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * stride;
        let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while col < cols {
            let mut y: ::core::ffi::c_int = 0;
            let mut cr: ::core::ffi::c_int = *(*frame).cr.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut cb: ::core::ffi::c_int = *(*frame).cb.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int =
                cr * 104597 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            let mut g: ::core::ffi::c_int = cb * 25674 as ::core::ffi::c_int
                + cr * 53278 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            let mut b: ::core::ffi::c_int =
                cb * 132201 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame).y.data.offset((y_index + yw) as isize) as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + stride + 2 as ::core::ffi::c_int) as isize) = plm_clamp(y + r);
            *dest.offset((d_index + stride + 1 as ::core::ffi::c_int) as isize) = plm_clamp(y - g);
            *dest.offset((d_index + stride + 0 as ::core::ffi::c_int) as isize) = plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + yw + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + r);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y - g);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + b);
            c_index += 1 as ::core::ffi::c_int;
            y_index += 2 as ::core::ffi::c_int;
            d_index += 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_frame_to_argb(
    mut frame: *mut plm_frame_t,
    mut dest: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut cols: ::core::ffi::c_int =
        ((*frame).width >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int =
        ((*frame).height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut yw: ::core::ffi::c_int = (*frame).y.width as ::core::ffi::c_int;
    let mut cw: ::core::ffi::c_int = (*frame).cb.width as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        let mut c_index: ::core::ffi::c_int = row * cw;
        let mut y_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * yw;
        let mut d_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * stride;
        let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while col < cols {
            let mut y: ::core::ffi::c_int = 0;
            let mut cr: ::core::ffi::c_int = *(*frame).cr.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut cb: ::core::ffi::c_int = *(*frame).cb.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int =
                cr * 104597 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            let mut g: ::core::ffi::c_int = cb * 25674 as ::core::ffi::c_int
                + cr * 53278 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            let mut b: ::core::ffi::c_int =
                cb * 132201 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame).y.data.offset((y_index + yw) as isize) as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + stride + 1 as ::core::ffi::c_int) as isize) = plm_clamp(y + r);
            *dest.offset((d_index + stride + 2 as ::core::ffi::c_int) as isize) = plm_clamp(y - g);
            *dest.offset((d_index + stride + 3 as ::core::ffi::c_int) as isize) = plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + yw + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + r);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y - g);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + b);
            c_index += 1 as ::core::ffi::c_int;
            y_index += 2 as ::core::ffi::c_int;
            d_index += 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
            col += 1;
        }
        row += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plm_frame_to_abgr(
    mut frame: *mut plm_frame_t,
    mut dest: *mut uint8_t,
    mut stride: ::core::ffi::c_int,
) {
    let mut cols: ::core::ffi::c_int =
        ((*frame).width >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut rows: ::core::ffi::c_int =
        ((*frame).height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut yw: ::core::ffi::c_int = (*frame).y.width as ::core::ffi::c_int;
    let mut cw: ::core::ffi::c_int = (*frame).cb.width as ::core::ffi::c_int;
    let mut row: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while row < rows {
        let mut c_index: ::core::ffi::c_int = row * cw;
        let mut y_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * yw;
        let mut d_index: ::core::ffi::c_int = row * 2 as ::core::ffi::c_int * stride;
        let mut col: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while col < cols {
            let mut y: ::core::ffi::c_int = 0;
            let mut cr: ::core::ffi::c_int = *(*frame).cr.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut cb: ::core::ffi::c_int = *(*frame).cb.data.offset(c_index as isize)
                as ::core::ffi::c_int
                - 128 as ::core::ffi::c_int;
            let mut r: ::core::ffi::c_int =
                cr * 104597 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            let mut g: ::core::ffi::c_int = cb * 25674 as ::core::ffi::c_int
                + cr * 53278 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            let mut b: ::core::ffi::c_int =
                cb * 132201 as ::core::ffi::c_int >> 16 as ::core::ffi::c_int;
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + r);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                plm_clamp(y - g);
            *dest.offset((d_index + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                plm_clamp(y + b);
            y = (*(*frame).y.data.offset((y_index + yw) as isize) as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset((d_index + stride + 3 as ::core::ffi::c_int) as isize) = plm_clamp(y + r);
            *dest.offset((d_index + stride + 2 as ::core::ffi::c_int) as isize) = plm_clamp(y - g);
            *dest.offset((d_index + stride + 1 as ::core::ffi::c_int) as isize) = plm_clamp(y + b);
            y = (*(*frame)
                .y
                .data
                .offset((y_index + yw + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - 16 as ::core::ffi::c_int)
                * 76309 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + r);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y - g);
            *dest.offset(
                (d_index + stride + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ) = plm_clamp(y + b);
            c_index += 1 as ::core::ffi::c_int;
            y_index += 2 as ::core::ffi::c_int;
            d_index += 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
            col += 1;
        }
        row += 1;
    }
}
static mut PLM_AUDIO_FRAME_SYNC: ::core::ffi::c_int = 0x7ff as ::core::ffi::c_int;
static mut PLM_AUDIO_MPEG_1: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
static mut PLM_AUDIO_LAYER_II: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
static mut PLM_AUDIO_MODE_JOINT_STEREO: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
static mut PLM_AUDIO_MODE_MONO: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
static mut PLM_AUDIO_SAMPLE_RATE: [::core::ffi::c_ushort; 8] = [
    44100 as ::core::ffi::c_ushort,
    48000 as ::core::ffi::c_ushort,
    32000 as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_ushort,
    22050 as ::core::ffi::c_ushort,
    24000 as ::core::ffi::c_ushort,
    16000 as ::core::ffi::c_ushort,
    0 as ::core::ffi::c_ushort,
];
static mut PLM_AUDIO_BIT_RATE: [::core::ffi::c_short; 28] = [
    32 as ::core::ffi::c_short,
    48 as ::core::ffi::c_short,
    56 as ::core::ffi::c_short,
    64 as ::core::ffi::c_short,
    80 as ::core::ffi::c_short,
    96 as ::core::ffi::c_short,
    112 as ::core::ffi::c_short,
    128 as ::core::ffi::c_short,
    160 as ::core::ffi::c_short,
    192 as ::core::ffi::c_short,
    224 as ::core::ffi::c_short,
    256 as ::core::ffi::c_short,
    320 as ::core::ffi::c_short,
    384 as ::core::ffi::c_short,
    8 as ::core::ffi::c_short,
    16 as ::core::ffi::c_short,
    24 as ::core::ffi::c_short,
    32 as ::core::ffi::c_short,
    40 as ::core::ffi::c_short,
    48 as ::core::ffi::c_short,
    56 as ::core::ffi::c_short,
    64 as ::core::ffi::c_short,
    80 as ::core::ffi::c_short,
    96 as ::core::ffi::c_short,
    112 as ::core::ffi::c_short,
    128 as ::core::ffi::c_short,
    144 as ::core::ffi::c_short,
    160 as ::core::ffi::c_short,
];
static mut PLM_AUDIO_SCALEFACTOR_BASE: [::core::ffi::c_int; 3] = [
    0x2000000 as ::core::ffi::c_int,
    0x1965fea as ::core::ffi::c_int,
    0x1428a30 as ::core::ffi::c_int,
];
static mut PLM_AUDIO_SYNTHESIS_WINDOW: [::core::ffi::c_float; 512] = [
    0.0f32,
    -0.5f32,
    -0.5f32,
    -0.5f32,
    -0.5f32,
    -0.5f32,
    -0.5f32,
    -1.0f32,
    -1.0f32,
    -1.0f32,
    -1.0f32,
    -1.5f32,
    -1.5f32,
    -2.0f32,
    -2.0f32,
    -2.5f32,
    -2.5f32,
    -3.0f32,
    -3.5f32,
    -3.5f32,
    -4.0f32,
    -4.5f32,
    -5.0f32,
    -5.5f32,
    -6.5f32,
    -7.0f32,
    -8.0f32,
    -8.5f32,
    -9.5f32,
    -10.5f32,
    -12.0f32,
    -13.0f32,
    -14.5f32,
    -15.5f32,
    -17.5f32,
    -19.0f32,
    -20.5f32,
    -22.5f32,
    -24.5f32,
    -26.5f32,
    -29.0f32,
    -31.5f32,
    -34.0f32,
    -36.5f32,
    -39.5f32,
    -42.5f32,
    -45.5f32,
    -48.5f32,
    -52.0f32,
    -55.5f32,
    -58.5f32,
    -62.5f32,
    -66.0f32,
    -69.5f32,
    -73.5f32,
    -77.0f32,
    -80.5f32,
    -84.5f32,
    -88.0f32,
    -91.5f32,
    -95.0f32,
    -98.0f32,
    -101.0f32,
    -104.0f32,
    106.5f32,
    109.0f32,
    111.0f32,
    112.5f32,
    113.5f32,
    114.0f32,
    114.0f32,
    113.5f32,
    112.0f32,
    110.5f32,
    107.5f32,
    104.0f32,
    100.0f32,
    94.5f32,
    88.5f32,
    81.5f32,
    73.0f32,
    63.5f32,
    53.0f32,
    41.5f32,
    28.5f32,
    14.5f32,
    -1.0f32,
    -18.0f32,
    -36.0f32,
    -55.5f32,
    -76.5f32,
    -98.5f32,
    -122.0f32,
    -147.0f32,
    -173.5f32,
    -200.5f32,
    -229.5f32,
    -259.5f32,
    -290.5f32,
    -322.5f32,
    -355.5f32,
    -389.5f32,
    -424.0f32,
    -459.5f32,
    -495.5f32,
    -532.0f32,
    -568.5f32,
    -605.0f32,
    -641.5f32,
    -678.0f32,
    -714.0f32,
    -749.0f32,
    -783.5f32,
    -817.0f32,
    -849.0f32,
    -879.5f32,
    -908.5f32,
    -935.0f32,
    -959.5f32,
    -981.0f32,
    -1000.5f32,
    -1016.0f32,
    -1028.5f32,
    -1037.5f32,
    -1042.5f32,
    -1043.5f32,
    -1040.0f32,
    -1031.5f32,
    1018.5f32,
    1000.0f32,
    976.0f32,
    946.5f32,
    911.0f32,
    869.5f32,
    822.0f32,
    767.5f32,
    707.0f32,
    640.0f32,
    565.5f32,
    485.0f32,
    397.0f32,
    302.5f32,
    201.0f32,
    92.5f32,
    -22.5f32,
    -144.0f32,
    -272.5f32,
    -407.0f32,
    -547.5f32,
    -694.0f32,
    -846.0f32,
    -1003.0f32,
    -1165.0f32,
    -1331.5f32,
    -1502.0f32,
    -1675.5f32,
    -1852.5f32,
    -2031.5f32,
    -2212.5f32,
    -2394.0f32,
    -2576.5f32,
    -2758.5f32,
    -2939.5f32,
    -3118.5f32,
    -3294.5f32,
    -3467.5f32,
    -3635.5f32,
    -3798.5f32,
    -3955.0f32,
    -4104.5f32,
    -4245.5f32,
    -4377.5f32,
    -4499.0f32,
    -4609.5f32,
    -4708.0f32,
    -4792.5f32,
    -4863.5f32,
    -4919.0f32,
    -4958.0f32,
    -4979.5f32,
    -4983.0f32,
    -4967.5f32,
    -4931.5f32,
    -4875.0f32,
    -4796.0f32,
    -4694.5f32,
    -4569.5f32,
    -4420.0f32,
    -4246.0f32,
    -4046.0f32,
    -3820.0f32,
    -3567.0f32,
    3287.0f32,
    2979.5f32,
    2644.0f32,
    2280.5f32,
    1888.0f32,
    1467.5f32,
    1018.5f32,
    541.0f32,
    35.0f32,
    -499.0f32,
    -1061.0f32,
    -1650.0f32,
    -2266.5f32,
    -2909.0f32,
    -3577.0f32,
    -4270.0f32,
    -4987.5f32,
    -5727.5f32,
    -6490.0f32,
    -7274.0f32,
    -8077.5f32,
    -8899.5f32,
    -9739.0f32,
    -10594.5f32,
    -11464.5f32,
    -12347.0f32,
    -13241.0f32,
    -14144.5f32,
    -15056.0f32,
    -15973.5f32,
    -16895.5f32,
    -17820.0f32,
    -18744.5f32,
    -19668.0f32,
    -20588.0f32,
    -21503.0f32,
    -22410.5f32,
    -23308.5f32,
    -24195.0f32,
    -25068.5f32,
    -25926.5f32,
    -26767.0f32,
    -27589.0f32,
    -28389.0f32,
    -29166.5f32,
    -29919.0f32,
    -30644.5f32,
    -31342.0f32,
    -32009.5f32,
    -32645.0f32,
    -33247.0f32,
    -33814.5f32,
    -34346.0f32,
    -34839.5f32,
    -35295.0f32,
    -35710.0f32,
    -36084.5f32,
    -36417.5f32,
    -36707.5f32,
    -36954.0f32,
    -37156.5f32,
    -37315.0f32,
    -37428.0f32,
    -37496.0f32,
    37519.0f32,
    37496.0f32,
    37428.0f32,
    37315.0f32,
    37156.5f32,
    36954.0f32,
    36707.5f32,
    36417.5f32,
    36084.5f32,
    35710.0f32,
    35295.0f32,
    34839.5f32,
    34346.0f32,
    33814.5f32,
    33247.0f32,
    32645.0f32,
    32009.5f32,
    31342.0f32,
    30644.5f32,
    29919.0f32,
    29166.5f32,
    28389.0f32,
    27589.0f32,
    26767.0f32,
    25926.5f32,
    25068.5f32,
    24195.0f32,
    23308.5f32,
    22410.5f32,
    21503.0f32,
    20588.0f32,
    19668.0f32,
    18744.5f32,
    17820.0f32,
    16895.5f32,
    15973.5f32,
    15056.0f32,
    14144.5f32,
    13241.0f32,
    12347.0f32,
    11464.5f32,
    10594.5f32,
    9739.0f32,
    8899.5f32,
    8077.5f32,
    7274.0f32,
    6490.0f32,
    5727.5f32,
    4987.5f32,
    4270.0f32,
    3577.0f32,
    2909.0f32,
    2266.5f32,
    1650.0f32,
    1061.0f32,
    499.0f32,
    -35.0f32,
    -541.0f32,
    -1018.5f32,
    -1467.5f32,
    -1888.0f32,
    -2280.5f32,
    -2644.0f32,
    -2979.5f32,
    3287.0f32,
    3567.0f32,
    3820.0f32,
    4046.0f32,
    4246.0f32,
    4420.0f32,
    4569.5f32,
    4694.5f32,
    4796.0f32,
    4875.0f32,
    4931.5f32,
    4967.5f32,
    4983.0f32,
    4979.5f32,
    4958.0f32,
    4919.0f32,
    4863.5f32,
    4792.5f32,
    4708.0f32,
    4609.5f32,
    4499.0f32,
    4377.5f32,
    4245.5f32,
    4104.5f32,
    3955.0f32,
    3798.5f32,
    3635.5f32,
    3467.5f32,
    3294.5f32,
    3118.5f32,
    2939.5f32,
    2758.5f32,
    2576.5f32,
    2394.0f32,
    2212.5f32,
    2031.5f32,
    1852.5f32,
    1675.5f32,
    1502.0f32,
    1331.5f32,
    1165.0f32,
    1003.0f32,
    846.0f32,
    694.0f32,
    547.5f32,
    407.0f32,
    272.5f32,
    144.0f32,
    22.5f32,
    -92.5f32,
    -201.0f32,
    -302.5f32,
    -397.0f32,
    -485.0f32,
    -565.5f32,
    -640.0f32,
    -707.0f32,
    -767.5f32,
    -822.0f32,
    -869.5f32,
    -911.0f32,
    -946.5f32,
    -976.0f32,
    -1000.0f32,
    1018.5f32,
    1031.5f32,
    1040.0f32,
    1043.5f32,
    1042.5f32,
    1037.5f32,
    1028.5f32,
    1016.0f32,
    1000.5f32,
    981.0f32,
    959.5f32,
    935.0f32,
    908.5f32,
    879.5f32,
    849.0f32,
    817.0f32,
    783.5f32,
    749.0f32,
    714.0f32,
    678.0f32,
    641.5f32,
    605.0f32,
    568.5f32,
    532.0f32,
    495.5f32,
    459.5f32,
    424.0f32,
    389.5f32,
    355.5f32,
    322.5f32,
    290.5f32,
    259.5f32,
    229.5f32,
    200.5f32,
    173.5f32,
    147.0f32,
    122.0f32,
    98.5f32,
    76.5f32,
    55.5f32,
    36.0f32,
    18.0f32,
    1.0f32,
    -14.5f32,
    -28.5f32,
    -41.5f32,
    -53.0f32,
    -63.5f32,
    -73.0f32,
    -81.5f32,
    -88.5f32,
    -94.5f32,
    -100.0f32,
    -104.0f32,
    -107.5f32,
    -110.5f32,
    -112.0f32,
    -113.5f32,
    -114.0f32,
    -114.0f32,
    -113.5f32,
    -112.5f32,
    -111.0f32,
    -109.0f32,
    106.5f32,
    104.0f32,
    101.0f32,
    98.0f32,
    95.0f32,
    91.5f32,
    88.0f32,
    84.5f32,
    80.5f32,
    77.0f32,
    73.5f32,
    69.5f32,
    66.0f32,
    62.5f32,
    58.5f32,
    55.5f32,
    52.0f32,
    48.5f32,
    45.5f32,
    42.5f32,
    39.5f32,
    36.5f32,
    34.0f32,
    31.5f32,
    29.0f32,
    26.5f32,
    24.5f32,
    22.5f32,
    20.5f32,
    19.0f32,
    17.5f32,
    15.5f32,
    14.5f32,
    13.0f32,
    12.0f32,
    10.5f32,
    9.5f32,
    8.5f32,
    8.0f32,
    7.0f32,
    6.5f32,
    5.5f32,
    5.0f32,
    4.5f32,
    4.0f32,
    3.5f32,
    3.5f32,
    3.0f32,
    2.5f32,
    2.5f32,
    2.0f32,
    2.0f32,
    1.5f32,
    1.5f32,
    1.0f32,
    1.0f32,
    1.0f32,
    1.0f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.5f32,
    0.5f32,
];
static mut PLM_AUDIO_QUANT_LUT_STEP_1: [[uint8_t; 16]; 2] = [
    [
        0 as uint8_t,
        0 as uint8_t,
        1 as uint8_t,
        1 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        0,
        0,
    ],
    [
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        1 as uint8_t,
        1 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        2 as uint8_t,
        0,
        0,
    ],
];
pub const PLM_AUDIO_QUANT_TAB_A: ::core::ffi::c_int =
    27 as ::core::ffi::c_int | 64 as ::core::ffi::c_int;
pub const PLM_AUDIO_QUANT_TAB_B: ::core::ffi::c_int =
    30 as ::core::ffi::c_int | 64 as ::core::ffi::c_int;
pub const PLM_AUDIO_QUANT_TAB_C: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PLM_AUDIO_QUANT_TAB_D: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut QUANT_LUT_STEP_2: [[uint8_t; 3]; 3] = [
    [
        PLM_AUDIO_QUANT_TAB_C as uint8_t,
        PLM_AUDIO_QUANT_TAB_C as uint8_t,
        PLM_AUDIO_QUANT_TAB_D as uint8_t,
    ],
    [
        PLM_AUDIO_QUANT_TAB_A as uint8_t,
        PLM_AUDIO_QUANT_TAB_A as uint8_t,
        PLM_AUDIO_QUANT_TAB_A as uint8_t,
    ],
    [
        PLM_AUDIO_QUANT_TAB_B as uint8_t,
        PLM_AUDIO_QUANT_TAB_A as uint8_t,
        PLM_AUDIO_QUANT_TAB_B as uint8_t,
    ],
];
static mut PLM_AUDIO_QUANT_LUT_STEP_3: [[uint8_t; 32]; 3] = [
    [
        0x44 as uint8_t,
        0x44 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0x43 as uint8_t,
        0x43 as uint8_t,
        0x43 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x42 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x31 as uint8_t,
        0x20 as uint8_t,
        0x20 as uint8_t,
        0x20 as uint8_t,
        0x20 as uint8_t,
        0x20 as uint8_t,
        0x20 as uint8_t,
        0x20 as uint8_t,
        0,
        0,
    ],
    [
        0x45 as uint8_t,
        0x45 as uint8_t,
        0x45 as uint8_t,
        0x45 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x34 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0x24 as uint8_t,
        0,
        0,
    ],
];
static mut PLM_AUDIO_QUANT_LUT_STEP_4: [[uint8_t; 16]; 6] = [
    [
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        17 as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        3 as uint8_t,
        4 as uint8_t,
        5 as uint8_t,
        6 as uint8_t,
        17 as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        3 as uint8_t,
        4 as uint8_t,
        5 as uint8_t,
        6 as uint8_t,
        7 as uint8_t,
        8 as uint8_t,
        9 as uint8_t,
        10 as uint8_t,
        11 as uint8_t,
        12 as uint8_t,
        13 as uint8_t,
        14 as uint8_t,
        17 as uint8_t,
    ],
    [
        0 as uint8_t,
        1 as uint8_t,
        3 as uint8_t,
        5 as uint8_t,
        6 as uint8_t,
        7 as uint8_t,
        8 as uint8_t,
        9 as uint8_t,
        10 as uint8_t,
        11 as uint8_t,
        12 as uint8_t,
        13 as uint8_t,
        14 as uint8_t,
        15 as uint8_t,
        16 as uint8_t,
        17 as uint8_t,
    ],
    [
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        4 as uint8_t,
        5 as uint8_t,
        6 as uint8_t,
        7 as uint8_t,
        8 as uint8_t,
        9 as uint8_t,
        10 as uint8_t,
        11 as uint8_t,
        12 as uint8_t,
        13 as uint8_t,
        14 as uint8_t,
        15 as uint8_t,
        17 as uint8_t,
    ],
    [
        0 as uint8_t,
        1 as uint8_t,
        2 as uint8_t,
        3 as uint8_t,
        4 as uint8_t,
        5 as uint8_t,
        6 as uint8_t,
        7 as uint8_t,
        8 as uint8_t,
        9 as uint8_t,
        10 as uint8_t,
        11 as uint8_t,
        12 as uint8_t,
        13 as uint8_t,
        14 as uint8_t,
        15 as uint8_t,
    ],
];
static mut PLM_AUDIO_QUANT_TAB: [plm_quantizer_spec_t; 17] = [
    plm_quantizer_spec_t {
        levels: 3 as ::core::ffi::c_ushort,
        group: 1 as ::core::ffi::c_uchar,
        bits: 5 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 5 as ::core::ffi::c_ushort,
        group: 1 as ::core::ffi::c_uchar,
        bits: 7 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 7 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 3 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 9 as ::core::ffi::c_ushort,
        group: 1 as ::core::ffi::c_uchar,
        bits: 10 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 15 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 4 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 31 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 5 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 63 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 6 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 127 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 7 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 255 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 8 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 511 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 9 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 1023 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 10 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 2047 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 11 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 4095 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 12 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 8191 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 13 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 16383 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 14 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 32767 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 15 as ::core::ffi::c_uchar,
    },
    plm_quantizer_spec_t {
        levels: 65535 as ::core::ffi::c_ushort,
        group: 0 as ::core::ffi::c_uchar,
        bits: 16 as ::core::ffi::c_uchar,
    },
];
#[no_mangle]
pub unsafe extern "C" fn plm_audio_create_with_buffer(
    mut buffer: *mut plm_buffer_t,
    mut destroy_when_done: ::core::ffi::c_int,
) -> *mut plm_audio_t {
    let mut self_0: *mut plm_audio_t =
        malloc(::core::mem::size_of::<plm_audio_t>()) as *mut plm_audio_t;
    memset(
        self_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<plm_audio_t>(),
    );
    (*self_0).samples.count = PLM_AUDIO_SAMPLES_PER_FRAME as ::core::ffi::c_uint;
    (*self_0).buffer = buffer;
    (*self_0).destroy_buffer_when_done = destroy_when_done;
    (*self_0).samplerate_index = 3 as ::core::ffi::c_int;
    memcpy(
        &raw mut (*self_0).D as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
        &raw const PLM_AUDIO_SYNTHESIS_WINDOW as *const ::core::ffi::c_float
            as *const ::core::ffi::c_void,
        (512 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
    );
    memcpy(
        (&raw mut (*self_0).D as *mut ::core::ffi::c_float)
            .offset(512 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw const PLM_AUDIO_SYNTHESIS_WINDOW as *const ::core::ffi::c_float
            as *const ::core::ffi::c_void,
        (512 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>()),
    );
    (*self_0).next_frame_data_size = plm_audio_decode_header(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_destroy(mut self_0: *mut plm_audio_t) {
    if (*self_0).destroy_buffer_when_done != 0 {
        plm_buffer_destroy((*self_0).buffer);
    }
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_has_header(mut self_0: *mut plm_audio_t) -> ::core::ffi::c_int {
    if (*self_0).has_header != 0 {
        return TRUE;
    }
    (*self_0).next_frame_data_size = plm_audio_decode_header(self_0);
    return (*self_0).has_header;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_get_samplerate(
    mut self_0: *mut plm_audio_t,
) -> ::core::ffi::c_int {
    return if plm_audio_has_header(self_0) != 0 {
        PLM_AUDIO_SAMPLE_RATE[(*self_0).samplerate_index as usize] as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_get_time(mut self_0: *mut plm_audio_t) -> ::core::ffi::c_double {
    return (*self_0).time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_set_time(
    mut self_0: *mut plm_audio_t,
    mut time: ::core::ffi::c_double,
) {
    (*self_0).samples_decoded = (time
        * PLM_AUDIO_SAMPLE_RATE[(*self_0).samplerate_index as usize] as ::core::ffi::c_double)
        as ::core::ffi::c_int;
    (*self_0).time = time;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_rewind(mut self_0: *mut plm_audio_t) {
    plm_buffer_rewind((*self_0).buffer);
    (*self_0).time = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*self_0).samples_decoded = 0 as ::core::ffi::c_int;
    (*self_0).next_frame_data_size = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_has_ended(mut self_0: *mut plm_audio_t) -> ::core::ffi::c_int {
    return plm_buffer_has_ended((*self_0).buffer);
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_decode(mut self_0: *mut plm_audio_t) -> *mut plm_samples_t {
    if (*self_0).next_frame_data_size == 0 {
        if plm_buffer_has((*self_0).buffer, 48 as size_t) == 0 {
            return ::core::ptr::null_mut::<plm_samples_t>();
        }
        (*self_0).next_frame_data_size = plm_audio_decode_header(self_0);
    }
    if (*self_0).next_frame_data_size == 0 as ::core::ffi::c_int
        || plm_buffer_has(
            (*self_0).buffer,
            ((*self_0).next_frame_data_size << 3 as ::core::ffi::c_int) as size_t,
        ) == 0
    {
        return ::core::ptr::null_mut::<plm_samples_t>();
    }
    plm_audio_decode_frame(self_0);
    (*self_0).next_frame_data_size = 0 as ::core::ffi::c_int;
    (*self_0).samples.time = (*self_0).time;
    (*self_0).samples_decoded += PLM_AUDIO_SAMPLES_PER_FRAME;
    (*self_0).time = (*self_0).samples_decoded as ::core::ffi::c_double
        / PLM_AUDIO_SAMPLE_RATE[(*self_0).samplerate_index as usize] as ::core::ffi::c_double;
    return &raw mut (*self_0).samples;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_find_frame_sync(
    mut self_0: *mut plm_audio_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    i = (*(*self_0).buffer).bit_index >> 3 as ::core::ffi::c_int;
    while i < (*(*self_0).buffer).length.wrapping_sub(1 as size_t) {
        if *(*(*self_0).buffer).bytes.offset(i as isize) as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
            && *(*(*self_0).buffer)
                .bytes
                .offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                & 0xfe as ::core::ffi::c_int
                == 0xfc as ::core::ffi::c_int
        {
            (*(*self_0).buffer).bit_index =
                (i.wrapping_add(1 as size_t) << 3 as ::core::ffi::c_int).wrapping_add(3 as size_t);
            return TRUE;
        }
        i = i.wrapping_add(1);
    }
    (*(*self_0).buffer).bit_index = i.wrapping_add(1 as size_t) << 3 as ::core::ffi::c_int;
    return FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_decode_header(
    mut self_0: *mut plm_audio_t,
) -> ::core::ffi::c_int {
    if plm_buffer_has((*self_0).buffer, 48 as size_t) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    plm_buffer_skip_bytes((*self_0).buffer, 0 as uint8_t);
    let mut sync: ::core::ffi::c_int = plm_buffer_read((*self_0).buffer, 11 as ::core::ffi::c_int);
    if sync != PLM_AUDIO_FRAME_SYNC && plm_audio_find_frame_sync(self_0) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*self_0).version = plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int);
    (*self_0).layer = plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int);
    let mut hasCRC: ::core::ffi::c_int =
        (plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int) == 0) as ::core::ffi::c_int;
    if (*self_0).version != PLM_AUDIO_MPEG_1 || (*self_0).layer != PLM_AUDIO_LAYER_II {
        return 0 as ::core::ffi::c_int;
    }
    let mut bitrate_index: ::core::ffi::c_int =
        plm_buffer_read((*self_0).buffer, 4 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
    if bitrate_index > 13 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut samplerate_index: ::core::ffi::c_int =
        plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int);
    if samplerate_index == 3 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    let mut padding: ::core::ffi::c_int =
        plm_buffer_read((*self_0).buffer, 1 as ::core::ffi::c_int);
    plm_buffer_skip((*self_0).buffer, 1 as size_t);
    let mut mode: ::core::ffi::c_int = plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int);
    if (*self_0).has_header != 0
        && ((*self_0).bitrate_index != bitrate_index
            || (*self_0).samplerate_index != samplerate_index
            || (*self_0).mode != mode)
    {
        return 0 as ::core::ffi::c_int;
    }
    (*self_0).bitrate_index = bitrate_index;
    (*self_0).samplerate_index = samplerate_index;
    (*self_0).mode = mode;
    (*self_0).has_header = TRUE;
    if mode == PLM_AUDIO_MODE_JOINT_STEREO {
        (*self_0).bound = (plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int;
    } else {
        plm_buffer_skip((*self_0).buffer, 2 as size_t);
        (*self_0).bound = if mode == PLM_AUDIO_MODE_MONO {
            0 as ::core::ffi::c_int
        } else {
            32 as ::core::ffi::c_int
        };
    }
    plm_buffer_skip((*self_0).buffer, 4 as size_t);
    if hasCRC != 0 {
        plm_buffer_skip((*self_0).buffer, 16 as size_t);
    }
    let mut bitrate: ::core::ffi::c_int =
        PLM_AUDIO_BIT_RATE[(*self_0).bitrate_index as usize] as ::core::ffi::c_int;
    let mut samplerate: ::core::ffi::c_int =
        PLM_AUDIO_SAMPLE_RATE[(*self_0).samplerate_index as usize] as ::core::ffi::c_int;
    let mut frame_size: ::core::ffi::c_int =
        144000 as ::core::ffi::c_int * bitrate / samplerate + padding;
    return frame_size
        - if hasCRC != 0 {
            6 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_decode_frame(mut self_0: *mut plm_audio_t) {
    let mut tab3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sblimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tab1: ::core::ffi::c_int = if (*self_0).mode == PLM_AUDIO_MODE_MONO {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    let mut tab2: ::core::ffi::c_int = PLM_AUDIO_QUANT_LUT_STEP_1[tab1 as usize]
        [(*self_0).bitrate_index as usize]
        as ::core::ffi::c_int;
    tab3 =
        QUANT_LUT_STEP_2[tab2 as usize][(*self_0).samplerate_index as usize] as ::core::ffi::c_int;
    sblimit = tab3 & 63 as ::core::ffi::c_int;
    tab3 >>= 6 as ::core::ffi::c_int;
    if (*self_0).bound > sblimit {
        (*self_0).bound = sblimit;
    }
    let mut sb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sb < (*self_0).bound {
        (*self_0).allocation[0usize][sb as usize] = plm_audio_read_allocation(self_0, sb, tab3);
        (*self_0).allocation[1usize][sb as usize] = plm_audio_read_allocation(self_0, sb, tab3);
        sb += 1;
    }
    let mut sb_0: ::core::ffi::c_int = (*self_0).bound;
    while sb_0 < sblimit {
        (*self_0).allocation[1usize][sb_0 as usize] = plm_audio_read_allocation(self_0, sb_0, tab3);
        (*self_0).allocation[0usize][sb_0 as usize] = (*self_0).allocation[1usize][sb_0 as usize];
        sb_0 += 1;
    }
    let mut channels: ::core::ffi::c_int = if (*self_0).mode == PLM_AUDIO_MODE_MONO {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    let mut sb_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sb_1 < sblimit {
        let mut ch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ch < channels {
            if !(*self_0).allocation[ch as usize][sb_1 as usize].is_null() {
                (*self_0).scale_factor_info[ch as usize][sb_1 as usize] =
                    plm_buffer_read((*self_0).buffer, 2 as ::core::ffi::c_int) as uint8_t;
            }
            ch += 1;
        }
        if (*self_0).mode == PLM_AUDIO_MODE_MONO {
            (*self_0).scale_factor_info[1usize][sb_1 as usize] =
                (*self_0).scale_factor_info[0usize][sb_1 as usize];
        }
        sb_1 += 1;
    }
    let mut sb_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while sb_2 < sblimit {
        let mut ch_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ch_0 < channels {
            if !(*self_0).allocation[ch_0 as usize][sb_2 as usize].is_null() {
                let mut sf: *mut ::core::ffi::c_int =
                    &raw mut *(&raw mut *(&raw mut (*self_0).scale_factor
                        as *mut [[::core::ffi::c_int; 3]; 32])
                        .offset(ch_0 as isize)
                        as *mut [::core::ffi::c_int; 3])
                        .offset(sb_2 as isize) as *mut ::core::ffi::c_int;
                match (*self_0).scale_factor_info[ch_0 as usize][sb_2 as usize]
                    as ::core::ffi::c_int
                {
                    0 => {
                        *sf.offset(0isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                        *sf.offset(1isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                        *sf.offset(2isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                    }
                    1 => {
                        *sf.offset(1isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                        *sf.offset(0isize) = *sf.offset(1isize);
                        *sf.offset(2isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                    }
                    2 => {
                        *sf.offset(2isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                        *sf.offset(1isize) = *sf.offset(2isize);
                        *sf.offset(0isize) = *sf.offset(1isize);
                    }
                    3 => {
                        *sf.offset(0isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                        *sf.offset(2isize) =
                            plm_buffer_read((*self_0).buffer, 6 as ::core::ffi::c_int);
                        *sf.offset(1isize) = *sf.offset(2isize);
                    }
                    _ => {}
                }
            }
            ch_0 += 1;
        }
        if (*self_0).mode == PLM_AUDIO_MODE_MONO {
            (*self_0).scale_factor[1usize][sb_2 as usize][0usize] =
                (*self_0).scale_factor[0usize][sb_2 as usize][0usize];
            (*self_0).scale_factor[1usize][sb_2 as usize][1usize] =
                (*self_0).scale_factor[0usize][sb_2 as usize][1usize];
            (*self_0).scale_factor[1usize][sb_2 as usize][2usize] =
                (*self_0).scale_factor[0usize][sb_2 as usize][2usize];
        }
        sb_2 += 1;
    }
    let mut out_pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut part: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while part < 3 as ::core::ffi::c_int {
        let mut granule: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while granule < 4 as ::core::ffi::c_int {
            let mut sb_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while sb_3 < (*self_0).bound {
                plm_audio_read_samples(self_0, 0 as ::core::ffi::c_int, sb_3, part);
                plm_audio_read_samples(self_0, 1 as ::core::ffi::c_int, sb_3, part);
                sb_3 += 1;
            }
            let mut sb_4: ::core::ffi::c_int = (*self_0).bound;
            while sb_4 < sblimit {
                plm_audio_read_samples(self_0, 0 as ::core::ffi::c_int, sb_4, part);
                (*self_0).sample[1usize][sb_4 as usize][0usize] =
                    (*self_0).sample[0usize][sb_4 as usize][0usize];
                (*self_0).sample[1usize][sb_4 as usize][1usize] =
                    (*self_0).sample[0usize][sb_4 as usize][1usize];
                (*self_0).sample[1usize][sb_4 as usize][2usize] =
                    (*self_0).sample[0usize][sb_4 as usize][2usize];
                sb_4 += 1;
            }
            let mut sb_5: ::core::ffi::c_int = sblimit;
            while sb_5 < 32 as ::core::ffi::c_int {
                (*self_0).sample[0usize][sb_5 as usize][0usize] = 0 as ::core::ffi::c_int;
                (*self_0).sample[0usize][sb_5 as usize][1usize] = 0 as ::core::ffi::c_int;
                (*self_0).sample[0usize][sb_5 as usize][2usize] = 0 as ::core::ffi::c_int;
                (*self_0).sample[1usize][sb_5 as usize][0usize] = 0 as ::core::ffi::c_int;
                (*self_0).sample[1usize][sb_5 as usize][1usize] = 0 as ::core::ffi::c_int;
                (*self_0).sample[1usize][sb_5 as usize][2usize] = 0 as ::core::ffi::c_int;
                sb_5 += 1;
            }
            let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p < 3 as ::core::ffi::c_int {
                (*self_0).v_pos =
                    (*self_0).v_pos - 64 as ::core::ffi::c_int & 1023 as ::core::ffi::c_int;
                let mut ch_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while ch_1 < 2 as ::core::ffi::c_int {
                    plm_audio_idct36(
                        &raw mut *(&raw mut (*self_0).sample as *mut [[::core::ffi::c_int; 3]; 32])
                            .offset(ch_1 as isize)
                            as *mut [::core::ffi::c_int; 3],
                        p,
                        &raw mut *(&raw mut (*self_0).V as *mut [::core::ffi::c_float; 1024])
                            .offset(ch_1 as isize)
                            as *mut ::core::ffi::c_float,
                        (*self_0).v_pos,
                    );
                    memset(
                        &raw mut (*self_0).U as *mut ::core::ffi::c_float
                            as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<[::core::ffi::c_float; 32]>(),
                    );
                    let mut d_index: ::core::ffi::c_int =
                        512 as ::core::ffi::c_int - ((*self_0).v_pos >> 1 as ::core::ffi::c_int);
                    let mut v_index: ::core::ffi::c_int =
                        (*self_0).v_pos % 128 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
                    while v_index < 1024 as ::core::ffi::c_int {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < 32 as ::core::ffi::c_int {
                            let c2rust_fresh0 = d_index;
                            d_index += 1;
                            let c2rust_fresh1 = v_index;
                            v_index += 1;
                            (*self_0).U[i as usize] += (*self_0).D[c2rust_fresh0 as usize]
                                * (*self_0).V[ch_1 as usize][c2rust_fresh1 as usize];
                            i += 1;
                        }
                        v_index += 128 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
                        d_index += 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
                    }
                    d_index -= 512 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
                    v_index = 128 as ::core::ffi::c_int - 32 as ::core::ffi::c_int
                        + 1024 as ::core::ffi::c_int
                        - v_index;
                    while v_index < 1024 as ::core::ffi::c_int {
                        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_0 < 32 as ::core::ffi::c_int {
                            let c2rust_fresh2 = d_index;
                            d_index += 1;
                            let c2rust_fresh3 = v_index;
                            v_index += 1;
                            (*self_0).U[i_0 as usize] += (*self_0).D[c2rust_fresh2 as usize]
                                * (*self_0).V[ch_1 as usize][c2rust_fresh3 as usize];
                            i_0 += 1;
                        }
                        v_index += 128 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
                        d_index += 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int;
                    }
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j < 32 as ::core::ffi::c_int {
                        (*self_0).samples.interleaved
                            [((out_pos + j << 1 as ::core::ffi::c_int) + ch_1) as usize] =
                            (*self_0).U[j as usize] / -1090519040.0f32;
                        j += 1;
                    }
                    ch_1 += 1;
                }
                out_pos += 32 as ::core::ffi::c_int;
                p += 1;
            }
            granule += 1;
        }
        part += 1;
    }
    plm_buffer_align((*self_0).buffer);
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_read_allocation(
    mut self_0: *mut plm_audio_t,
    mut sb: ::core::ffi::c_int,
    mut tab3: ::core::ffi::c_int,
) -> *const plm_quantizer_spec_t {
    let mut tab4: ::core::ffi::c_int =
        PLM_AUDIO_QUANT_LUT_STEP_3[tab3 as usize][sb as usize] as ::core::ffi::c_int;
    let mut qtab: ::core::ffi::c_int = PLM_AUDIO_QUANT_LUT_STEP_4
        [(tab4 & 15 as ::core::ffi::c_int) as usize]
        [plm_buffer_read((*self_0).buffer, tab4 >> 4 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    return if qtab != 0 {
        (&raw const PLM_AUDIO_QUANT_TAB as *const plm_quantizer_spec_t)
            .offset((qtab - 1 as ::core::ffi::c_int) as isize)
    } else {
        ::core::ptr::null::<plm_quantizer_spec_t>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_read_samples(
    mut self_0: *mut plm_audio_t,
    mut ch: ::core::ffi::c_int,
    mut sb: ::core::ffi::c_int,
    mut part: ::core::ffi::c_int,
) {
    let mut q: *const plm_quantizer_spec_t = (*self_0).allocation[ch as usize][sb as usize];
    let mut sf: ::core::ffi::c_int =
        (*self_0).scale_factor[ch as usize][sb as usize][part as usize];
    let mut sample: *mut ::core::ffi::c_int =
        &raw mut *(&raw mut *(&raw mut (*self_0).sample as *mut [[::core::ffi::c_int; 3]; 32])
            .offset(ch as isize) as *mut [::core::ffi::c_int; 3])
            .offset(sb as isize) as *mut ::core::ffi::c_int;
    let mut val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if q.is_null() {
        *sample.offset(2isize) = 0 as ::core::ffi::c_int;
        *sample.offset(1isize) = *sample.offset(2isize);
        *sample.offset(0isize) = *sample.offset(1isize);
        return;
    }
    if sf == 63 as ::core::ffi::c_int {
        sf = 0 as ::core::ffi::c_int;
    } else {
        let mut shift: ::core::ffi::c_int = sf / 3 as ::core::ffi::c_int | 0 as ::core::ffi::c_int;
        sf = PLM_AUDIO_SCALEFACTOR_BASE[(sf % 3 as ::core::ffi::c_int) as usize]
            + ((1 as ::core::ffi::c_int) << shift >> 1 as ::core::ffi::c_int)
            >> shift;
    }
    let mut adj: ::core::ffi::c_int = (*q).levels as ::core::ffi::c_int;
    if (*q).group != 0 {
        val = plm_buffer_read((*self_0).buffer, (*q).bits as ::core::ffi::c_int);
        *sample.offset(0isize) = val % adj;
        val /= adj;
        *sample.offset(1isize) = val % adj;
        *sample.offset(2isize) = val / adj;
    } else {
        *sample.offset(0isize) = plm_buffer_read((*self_0).buffer, (*q).bits as ::core::ffi::c_int);
        *sample.offset(1isize) = plm_buffer_read((*self_0).buffer, (*q).bits as ::core::ffi::c_int);
        *sample.offset(2isize) = plm_buffer_read((*self_0).buffer, (*q).bits as ::core::ffi::c_int);
    }
    let mut scale: ::core::ffi::c_int =
        65536 as ::core::ffi::c_int / (adj + 1 as ::core::ffi::c_int);
    adj = (adj + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
    val = (adj - *sample.offset(0isize)) * scale;
    *sample.offset(0isize) = val * (sf >> 12 as ::core::ffi::c_int)
        + (val * (sf & 4095 as ::core::ffi::c_int) + 2048 as ::core::ffi::c_int
            >> 12 as ::core::ffi::c_int)
        >> 12 as ::core::ffi::c_int;
    val = (adj - *sample.offset(1isize)) * scale;
    *sample.offset(1isize) = val * (sf >> 12 as ::core::ffi::c_int)
        + (val * (sf & 4095 as ::core::ffi::c_int) + 2048 as ::core::ffi::c_int
            >> 12 as ::core::ffi::c_int)
        >> 12 as ::core::ffi::c_int;
    val = (adj - *sample.offset(2isize)) * scale;
    *sample.offset(2isize) = val * (sf >> 12 as ::core::ffi::c_int)
        + (val * (sf & 4095 as ::core::ffi::c_int) + 2048 as ::core::ffi::c_int
            >> 12 as ::core::ffi::c_int)
        >> 12 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plm_audio_idct36(
    mut s: *mut [::core::ffi::c_int; 3],
    mut ss: ::core::ffi::c_int,
    mut d: *mut ::core::ffi::c_float,
    mut dp: ::core::ffi::c_int,
) {
    let mut t01: ::core::ffi::c_float = 0.;
    let mut t02: ::core::ffi::c_float = 0.;
    let mut t03: ::core::ffi::c_float = 0.;
    let mut t04: ::core::ffi::c_float = 0.;
    let mut t05: ::core::ffi::c_float = 0.;
    let mut t06: ::core::ffi::c_float = 0.;
    let mut t07: ::core::ffi::c_float = 0.;
    let mut t08: ::core::ffi::c_float = 0.;
    let mut t09: ::core::ffi::c_float = 0.;
    let mut t10: ::core::ffi::c_float = 0.;
    let mut t11: ::core::ffi::c_float = 0.;
    let mut t12: ::core::ffi::c_float = 0.;
    let mut t13: ::core::ffi::c_float = 0.;
    let mut t14: ::core::ffi::c_float = 0.;
    let mut t15: ::core::ffi::c_float = 0.;
    let mut t16: ::core::ffi::c_float = 0.;
    let mut t17: ::core::ffi::c_float = 0.;
    let mut t18: ::core::ffi::c_float = 0.;
    let mut t19: ::core::ffi::c_float = 0.;
    let mut t20: ::core::ffi::c_float = 0.;
    let mut t21: ::core::ffi::c_float = 0.;
    let mut t22: ::core::ffi::c_float = 0.;
    let mut t23: ::core::ffi::c_float = 0.;
    let mut t24: ::core::ffi::c_float = 0.;
    let mut t25: ::core::ffi::c_float = 0.;
    let mut t26: ::core::ffi::c_float = 0.;
    let mut t27: ::core::ffi::c_float = 0.;
    let mut t28: ::core::ffi::c_float = 0.;
    let mut t29: ::core::ffi::c_float = 0.;
    let mut t30: ::core::ffi::c_float = 0.;
    let mut t31: ::core::ffi::c_float = 0.;
    let mut t32: ::core::ffi::c_float = 0.;
    let mut t33: ::core::ffi::c_float = 0.;
    t01 = ((*s.offset(0isize))[ss as usize] + (*s.offset(31isize))[ss as usize])
        as ::core::ffi::c_float;
    t02 = ((*s.offset(0isize))[ss as usize] - (*s.offset(31isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.500602998235f32;
    t03 = ((*s.offset(1isize))[ss as usize] + (*s.offset(30isize))[ss as usize])
        as ::core::ffi::c_float;
    t04 = ((*s.offset(1isize))[ss as usize] - (*s.offset(30isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.505470959898f32;
    t05 = ((*s.offset(2isize))[ss as usize] + (*s.offset(29isize))[ss as usize])
        as ::core::ffi::c_float;
    t06 = ((*s.offset(2isize))[ss as usize] - (*s.offset(29isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.515447309923f32;
    t07 = ((*s.offset(3isize))[ss as usize] + (*s.offset(28isize))[ss as usize])
        as ::core::ffi::c_float;
    t08 = ((*s.offset(3isize))[ss as usize] - (*s.offset(28isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.53104259109f32;
    t09 = ((*s.offset(4isize))[ss as usize] + (*s.offset(27isize))[ss as usize])
        as ::core::ffi::c_float;
    t10 = ((*s.offset(4isize))[ss as usize] - (*s.offset(27isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.553103896034f32;
    t11 = ((*s.offset(5isize))[ss as usize] + (*s.offset(26isize))[ss as usize])
        as ::core::ffi::c_float;
    t12 = ((*s.offset(5isize))[ss as usize] - (*s.offset(26isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.582934968206f32;
    t13 = ((*s.offset(6isize))[ss as usize] + (*s.offset(25isize))[ss as usize])
        as ::core::ffi::c_float;
    t14 = ((*s.offset(6isize))[ss as usize] - (*s.offset(25isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.622504123036f32;
    t15 = ((*s.offset(7isize))[ss as usize] + (*s.offset(24isize))[ss as usize])
        as ::core::ffi::c_float;
    t16 = ((*s.offset(7isize))[ss as usize] - (*s.offset(24isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.674808341455f32;
    t17 = ((*s.offset(8isize))[ss as usize] + (*s.offset(23isize))[ss as usize])
        as ::core::ffi::c_float;
    t18 = ((*s.offset(8isize))[ss as usize] - (*s.offset(23isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.744536271002f32;
    t19 = ((*s.offset(9isize))[ss as usize] + (*s.offset(22isize))[ss as usize])
        as ::core::ffi::c_float;
    t20 = ((*s.offset(9isize))[ss as usize] - (*s.offset(22isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.839349645416f32;
    t21 = ((*s.offset(10isize))[ss as usize] + (*s.offset(21isize))[ss as usize])
        as ::core::ffi::c_float;
    t22 = ((*s.offset(10isize))[ss as usize] - (*s.offset(21isize))[ss as usize])
        as ::core::ffi::c_float
        * 0.972568237862f32;
    t23 = ((*s.offset(11isize))[ss as usize] + (*s.offset(20isize))[ss as usize])
        as ::core::ffi::c_float;
    t24 = ((*s.offset(11isize))[ss as usize] - (*s.offset(20isize))[ss as usize])
        as ::core::ffi::c_float
        * 1.16943993343f32;
    t25 = ((*s.offset(12isize))[ss as usize] + (*s.offset(19isize))[ss as usize])
        as ::core::ffi::c_float;
    t26 = ((*s.offset(12isize))[ss as usize] - (*s.offset(19isize))[ss as usize])
        as ::core::ffi::c_float
        * 1.48416461631f32;
    t27 = ((*s.offset(13isize))[ss as usize] + (*s.offset(18isize))[ss as usize])
        as ::core::ffi::c_float;
    t28 = ((*s.offset(13isize))[ss as usize] - (*s.offset(18isize))[ss as usize])
        as ::core::ffi::c_float
        * 2.05778100995f32;
    t29 = ((*s.offset(14isize))[ss as usize] + (*s.offset(17isize))[ss as usize])
        as ::core::ffi::c_float;
    t30 = ((*s.offset(14isize))[ss as usize] - (*s.offset(17isize))[ss as usize])
        as ::core::ffi::c_float
        * 3.40760841847f32;
    t31 = ((*s.offset(15isize))[ss as usize] + (*s.offset(16isize))[ss as usize])
        as ::core::ffi::c_float;
    t32 = ((*s.offset(15isize))[ss as usize] - (*s.offset(16isize))[ss as usize])
        as ::core::ffi::c_float
        * 10.1900081235f32;
    t33 = t01 + t31;
    t31 = (t01 - t31) * 0.502419286188f32;
    t01 = t03 + t29;
    t29 = (t03 - t29) * 0.52249861494f32;
    t03 = t05 + t27;
    t27 = (t05 - t27) * 0.566944034816f32;
    t05 = t07 + t25;
    t25 = (t07 - t25) * 0.64682178336f32;
    t07 = t09 + t23;
    t23 = (t09 - t23) * 0.788154623451f32;
    t09 = t11 + t21;
    t21 = (t11 - t21) * 1.06067768599f32;
    t11 = t13 + t19;
    t19 = (t13 - t19) * 1.72244709824f32;
    t13 = t15 + t17;
    t17 = (t15 - t17) * 5.10114861869f32;
    t15 = t33 + t13;
    t13 = (t33 - t13) * 0.509795579104f32;
    t33 = t01 + t11;
    t01 = (t01 - t11) * 0.601344886935f32;
    t11 = t03 + t09;
    t09 = (t03 - t09) * 0.899976223136f32;
    t03 = t05 + t07;
    t07 = (t05 - t07) * 2.56291544774f32;
    t05 = t15 + t03;
    t15 = (t15 - t03) * 0.541196100146f32;
    t03 = t33 + t11;
    t11 = (t33 - t11) * 1.30656296488f32;
    t33 = t05 + t03;
    t05 = (t05 - t03) * 0.707106781187f32;
    t03 = t15 + t11;
    t15 = (t15 - t11) * 0.707106781187f32;
    t03 += t15;
    t11 = t13 + t07;
    t13 = (t13 - t07) * 0.541196100146f32;
    t07 = t01 + t09;
    t09 = (t01 - t09) * 1.30656296488f32;
    t01 = t11 + t07;
    t07 = (t11 - t07) * 0.707106781187f32;
    t11 = t13 + t09;
    t13 = (t13 - t09) * 0.707106781187f32;
    t11 += t13;
    t01 += t11;
    t11 += t07;
    t07 += t13;
    t09 = t31 + t17;
    t31 = (t31 - t17) * 0.509795579104f32;
    t17 = t29 + t19;
    t29 = (t29 - t19) * 0.601344886935f32;
    t19 = t27 + t21;
    t21 = (t27 - t21) * 0.899976223136f32;
    t27 = t25 + t23;
    t23 = (t25 - t23) * 2.56291544774f32;
    t25 = t09 + t27;
    t09 = (t09 - t27) * 0.541196100146f32;
    t27 = t17 + t19;
    t19 = (t17 - t19) * 1.30656296488f32;
    t17 = t25 + t27;
    t27 = (t25 - t27) * 0.707106781187f32;
    t25 = t09 + t19;
    t19 = (t09 - t19) * 0.707106781187f32;
    t25 += t19;
    t09 = t31 + t23;
    t31 = (t31 - t23) * 0.541196100146f32;
    t23 = t29 + t21;
    t21 = (t29 - t21) * 1.30656296488f32;
    t29 = t09 + t23;
    t23 = (t09 - t23) * 0.707106781187f32;
    t09 = t31 + t21;
    t31 = (t31 - t21) * 0.707106781187f32;
    t09 += t31;
    t29 += t09;
    t09 += t23;
    t23 += t31;
    t17 += t29;
    t29 += t25;
    t25 += t09;
    t09 += t27;
    t27 += t23;
    t23 += t19;
    t19 += t31;
    t21 = t02 + t32;
    t02 = (t02 - t32) * 0.502419286188f32;
    t32 = t04 + t30;
    t04 = (t04 - t30) * 0.52249861494f32;
    t30 = t06 + t28;
    t28 = (t06 - t28) * 0.566944034816f32;
    t06 = t08 + t26;
    t08 = (t08 - t26) * 0.64682178336f32;
    t26 = t10 + t24;
    t10 = (t10 - t24) * 0.788154623451f32;
    t24 = t12 + t22;
    t22 = (t12 - t22) * 1.06067768599f32;
    t12 = t14 + t20;
    t20 = (t14 - t20) * 1.72244709824f32;
    t14 = t16 + t18;
    t16 = (t16 - t18) * 5.10114861869f32;
    t18 = t21 + t14;
    t14 = (t21 - t14) * 0.509795579104f32;
    t21 = t32 + t12;
    t32 = (t32 - t12) * 0.601344886935f32;
    t12 = t30 + t24;
    t24 = (t30 - t24) * 0.899976223136f32;
    t30 = t06 + t26;
    t26 = (t06 - t26) * 2.56291544774f32;
    t06 = t18 + t30;
    t18 = (t18 - t30) * 0.541196100146f32;
    t30 = t21 + t12;
    t12 = (t21 - t12) * 1.30656296488f32;
    t21 = t06 + t30;
    t30 = (t06 - t30) * 0.707106781187f32;
    t06 = t18 + t12;
    t12 = (t18 - t12) * 0.707106781187f32;
    t06 += t12;
    t18 = t14 + t26;
    t26 = (t14 - t26) * 0.541196100146f32;
    t14 = t32 + t24;
    t24 = (t32 - t24) * 1.30656296488f32;
    t32 = t18 + t14;
    t14 = (t18 - t14) * 0.707106781187f32;
    t18 = t26 + t24;
    t24 = (t26 - t24) * 0.707106781187f32;
    t18 += t24;
    t32 += t18;
    t18 += t14;
    t26 = t14 + t24;
    t14 = t02 + t16;
    t02 = (t02 - t16) * 0.509795579104f32;
    t16 = t04 + t20;
    t04 = (t04 - t20) * 0.601344886935f32;
    t20 = t28 + t22;
    t22 = (t28 - t22) * 0.899976223136f32;
    t28 = t08 + t10;
    t10 = (t08 - t10) * 2.56291544774f32;
    t08 = t14 + t28;
    t14 = (t14 - t28) * 0.541196100146f32;
    t28 = t16 + t20;
    t20 = (t16 - t20) * 1.30656296488f32;
    t16 = t08 + t28;
    t28 = (t08 - t28) * 0.707106781187f32;
    t08 = t14 + t20;
    t20 = (t14 - t20) * 0.707106781187f32;
    t08 += t20;
    t14 = t02 + t10;
    t02 = (t02 - t10) * 0.541196100146f32;
    t10 = t04 + t22;
    t22 = (t04 - t22) * 1.30656296488f32;
    t04 = t14 + t10;
    t10 = (t14 - t10) * 0.707106781187f32;
    t14 = t02 + t22;
    t02 = (t02 - t22) * 0.707106781187f32;
    t14 += t02;
    t04 += t14;
    t14 += t10;
    t10 += t02;
    t16 += t04;
    t04 += t08;
    t08 += t14;
    t14 += t28;
    t28 += t10;
    t10 += t20;
    t20 += t02;
    t21 += t16;
    t16 += t32;
    t32 += t04;
    t04 += t06;
    t06 += t08;
    t08 += t18;
    t18 += t14;
    t14 += t30;
    t30 += t28;
    t28 += t26;
    t26 += t10;
    t10 += t12;
    t12 += t20;
    t20 += t24;
    t24 += t02;
    *d.offset((dp + 48 as ::core::ffi::c_int) as isize) = -t33;
    *d.offset((dp + 47 as ::core::ffi::c_int) as isize) = -t21;
    *d.offset((dp + 49 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 47 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 46 as ::core::ffi::c_int) as isize) = -t17;
    *d.offset((dp + 50 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 46 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 45 as ::core::ffi::c_int) as isize) = -t16;
    *d.offset((dp + 51 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 45 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 44 as ::core::ffi::c_int) as isize) = -t01;
    *d.offset((dp + 52 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 44 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 43 as ::core::ffi::c_int) as isize) = -t32;
    *d.offset((dp + 53 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 43 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 42 as ::core::ffi::c_int) as isize) = -t29;
    *d.offset((dp + 54 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 42 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 41 as ::core::ffi::c_int) as isize) = -t04;
    *d.offset((dp + 55 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 41 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 40 as ::core::ffi::c_int) as isize) = -t03;
    *d.offset((dp + 56 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 40 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 39 as ::core::ffi::c_int) as isize) = -t06;
    *d.offset((dp + 57 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 39 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 38 as ::core::ffi::c_int) as isize) = -t25;
    *d.offset((dp + 58 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 38 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 37 as ::core::ffi::c_int) as isize) = -t08;
    *d.offset((dp + 59 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 37 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 36 as ::core::ffi::c_int) as isize) = -t11;
    *d.offset((dp + 60 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 36 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 35 as ::core::ffi::c_int) as isize) = -t18;
    *d.offset((dp + 61 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 35 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 34 as ::core::ffi::c_int) as isize) = -t09;
    *d.offset((dp + 62 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 34 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 33 as ::core::ffi::c_int) as isize) = -t14;
    *d.offset((dp + 63 as ::core::ffi::c_int) as isize) =
        *d.offset((dp + 33 as ::core::ffi::c_int) as isize);
    *d.offset((dp + 32 as ::core::ffi::c_int) as isize) = -t05;
    *d.offset((dp + 0 as ::core::ffi::c_int) as isize) = t05;
    *d.offset((dp + 31 as ::core::ffi::c_int) as isize) = -t30;
    *d.offset((dp + 1 as ::core::ffi::c_int) as isize) = t30;
    *d.offset((dp + 30 as ::core::ffi::c_int) as isize) = -t27;
    *d.offset((dp + 2 as ::core::ffi::c_int) as isize) = t27;
    *d.offset((dp + 29 as ::core::ffi::c_int) as isize) = -t28;
    *d.offset((dp + 3 as ::core::ffi::c_int) as isize) = t28;
    *d.offset((dp + 28 as ::core::ffi::c_int) as isize) = -t07;
    *d.offset((dp + 4 as ::core::ffi::c_int) as isize) = t07;
    *d.offset((dp + 27 as ::core::ffi::c_int) as isize) = -t26;
    *d.offset((dp + 5 as ::core::ffi::c_int) as isize) = t26;
    *d.offset((dp + 26 as ::core::ffi::c_int) as isize) = -t23;
    *d.offset((dp + 6 as ::core::ffi::c_int) as isize) = t23;
    *d.offset((dp + 25 as ::core::ffi::c_int) as isize) = -t10;
    *d.offset((dp + 7 as ::core::ffi::c_int) as isize) = t10;
    *d.offset((dp + 24 as ::core::ffi::c_int) as isize) = -t15;
    *d.offset((dp + 8 as ::core::ffi::c_int) as isize) = t15;
    *d.offset((dp + 23 as ::core::ffi::c_int) as isize) = -t12;
    *d.offset((dp + 9 as ::core::ffi::c_int) as isize) = t12;
    *d.offset((dp + 22 as ::core::ffi::c_int) as isize) = -t19;
    *d.offset((dp + 10 as ::core::ffi::c_int) as isize) = t19;
    *d.offset((dp + 21 as ::core::ffi::c_int) as isize) = -t20;
    *d.offset((dp + 11 as ::core::ffi::c_int) as isize) = t20;
    *d.offset((dp + 20 as ::core::ffi::c_int) as isize) = -t13;
    *d.offset((dp + 12 as ::core::ffi::c_int) as isize) = t13;
    *d.offset((dp + 19 as ::core::ffi::c_int) as isize) = -t24;
    *d.offset((dp + 13 as ::core::ffi::c_int) as isize) = t24;
    *d.offset((dp + 18 as ::core::ffi::c_int) as isize) = -t31;
    *d.offset((dp + 14 as ::core::ffi::c_int) as isize) = t31;
    *d.offset((dp + 17 as ::core::ffi::c_int) as isize) = -t02;
    *d.offset((dp + 15 as ::core::ffi::c_int) as isize) = t02;
    *d.offset((dp + 16 as ::core::ffi::c_int) as isize) = 0.0f32;
}
