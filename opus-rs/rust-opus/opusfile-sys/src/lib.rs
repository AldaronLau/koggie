#![allow(non_camel_case_types)]

extern crate libc;
extern crate ogg_sys;
extern crate opus_sys;

use ogg_sys::{ogg_packet, ogg_int64_t};
use opus_sys::{OpusMSDecoder, opus_int16, opus_int32, opus_uint32};

pub enum OggOpusFile { }

#[repr(C)]
#[derive(Copy)]
pub struct OpusHead {
    pub version: ::libc::c_int,
    pub channel_count: ::libc::c_int,
    pub pre_skip: ::libc::c_uint,
    pub input_sample_rate: opus_uint32,
    pub output_gain: ::libc::c_int,
    pub mapping_family: ::libc::c_int,
    pub stream_count: ::libc::c_int,
    pub coupled_count: ::libc::c_int,
    pub mapping: [::libc::c_uchar; 255usize],
}

impl ::std::clone::Clone for OpusHead {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for OpusHead {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct OpusTags {
    pub user_comments: *mut *mut ::libc::c_char,
    pub comment_lengths: *mut ::libc::c_int,
    pub comments: ::libc::c_int,
    pub vendor: *mut ::libc::c_char,
}

impl ::std::clone::Clone for OpusTags {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for OpusTags {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct OpusPictureTag {
    pub _type: opus_int32,
    pub mime_type: *mut ::libc::c_char,
    pub description: *mut ::libc::c_char,
    pub width: opus_uint32,
    pub height: opus_uint32,
    pub depth: opus_uint32,
    pub colors: opus_uint32,
    pub data_length: opus_uint32,
    pub data: *mut ::libc::c_uchar,
    pub format: ::libc::c_int,
}

impl ::std::clone::Clone for OpusPictureTag {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for OpusPictureTag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct OpusServerInfo {
    pub name: *mut ::libc::c_char,
    pub description: *mut ::libc::c_char,
    pub genre: *mut ::libc::c_char,
    pub url: *mut ::libc::c_char,
    pub server: *mut ::libc::c_char,
    pub content_type: *mut ::libc::c_char,
    pub bitrate_kbps: opus_int32,
    pub is_public: ::libc::c_int,
    pub is_ssl: ::libc::c_int,
}

impl ::std::clone::Clone for OpusServerInfo {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for OpusServerInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type op_read_func =
    ::std::option::Option<extern "C" fn(_stream: *mut ::libc::c_void,
                                        _ptr: *mut ::libc::c_uchar,
                                        _nbytes: ::libc::c_int)
                              -> ::libc::c_int>;
pub type op_seek_func =
    ::std::option::Option<extern "C" fn(_stream: *mut ::libc::c_void,
                                        _offset: ::libc::c_longlong,
                                        _whence: ::libc::c_int)
                              -> ::libc::c_int>;
pub type op_tell_func =
    ::std::option::Option<extern "C" fn(_stream: *mut ::libc::c_void)
                              -> ::libc::c_longlong>;
pub type op_close_func =
    ::std::option::Option<extern "C" fn(_stream: *mut ::libc::c_void)
                              -> ::libc::c_int>;

#[repr(C)]
#[derive(Copy)]
pub struct OpusFileCallbacks {
    pub read: op_read_func,
    pub seek: op_seek_func,
    pub tell: op_tell_func,
    pub close: op_close_func,
}

impl ::std::clone::Clone for OpusFileCallbacks {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for OpusFileCallbacks {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type op_decode_cb_func =
    ::std::option::Option<extern "C" fn(_ctx: *mut ::libc::c_void,
                                        _decoder: *mut OpusMSDecoder,
                                        _pcm: *mut ::libc::c_void,
                                        _op: *const ogg_packet,
                                        _nsamples: ::libc::c_int,
                                        _nchannels: ::libc::c_int,
                                        _format: ::libc::c_int,
                                        _li: ::libc::c_int) -> ::libc::c_int>;

extern "C" {
    pub fn opus_head_parse(_head: *mut OpusHead,
                           _data: *const ::libc::c_uchar, _len: ::libc::size_t)
     -> ::libc::c_int;
    pub fn opus_granule_sample(_head: *const OpusHead, _gp: ogg_int64_t)
     -> ogg_int64_t;
    pub fn opus_tags_parse(_tags: *mut OpusTags,
                           _data: *const ::libc::c_uchar, _len: ::libc::size_t)
     -> ::libc::c_int;
    pub fn opus_tags_copy(_dst: *mut OpusTags, _src: *const OpusTags)
     -> ::libc::c_int;
    pub fn opus_tags_init(_tags: *mut OpusTags) -> ();
    pub fn opus_tags_add(_tags: *mut OpusTags, _tag: *const ::libc::c_char,
                         _value: *const ::libc::c_char) -> ::libc::c_int;
    pub fn opus_tags_add_comment(_tags: *mut OpusTags,
                                 _comment: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn opus_tags_query(_tags: *const OpusTags,
                           _tag: *const ::libc::c_char, _count: ::libc::c_int)
     -> *const ::libc::c_char;
    pub fn opus_tags_query_count(_tags: *const OpusTags,
                                 _tag: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn opus_tags_get_track_gain(_tags: *const OpusTags,
                                    _gain_q8: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_tags_clear(_tags: *mut OpusTags) -> ();
    pub fn opus_tagcompare(_tag_name: *const ::libc::c_char,
                           _comment: *const ::libc::c_char) -> ::libc::c_int;
    pub fn opus_tagncompare(_tag_name: *const ::libc::c_char,
                            _tag_len: ::libc::c_int,
                            _comment: *const ::libc::c_char) -> ::libc::c_int;
    pub fn opus_picture_tag_parse(_pic: *mut OpusPictureTag,
                                  _tag: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn opus_picture_tag_init(_pic: *mut OpusPictureTag) -> ();
    pub fn opus_picture_tag_clear(_pic: *mut OpusPictureTag) -> ();
    pub fn opus_server_info_init(_info: *mut OpusServerInfo) -> ();
    pub fn opus_server_info_clear(_info: *mut OpusServerInfo) -> ();
    pub fn op_fopen(_cb: *mut OpusFileCallbacks, _path: *const ::libc::c_char,
                    _mode: *const ::libc::c_char) -> *mut ::libc::c_void;
    pub fn op_fdopen(_cb: *mut OpusFileCallbacks, _fd: ::libc::c_int,
                     _mode: *const ::libc::c_char) -> *mut ::libc::c_void;
    pub fn op_freopen(_cb: *mut OpusFileCallbacks,
                      _path: *const ::libc::c_char,
                      _mode: *const ::libc::c_char,
                      _stream: *mut ::libc::c_void) -> *mut ::libc::c_void;
    pub fn op_mem_stream_create(_cb: *mut OpusFileCallbacks,
                                _data: *const ::libc::c_uchar, _size: ::libc::size_t)
     -> *mut ::libc::c_void;
    // FIXME
    //pub fn op_url_stream_vcreate(_cb: *mut OpusFileCallbacks,
    //                             _url: *const ::libc::c_char, _ap: va_list)
    // -> *mut ::libc::c_void;
    pub fn op_url_stream_create(_cb: *mut OpusFileCallbacks,
                                _url: *const ::libc::c_char, ...)
     -> *mut ::libc::c_void;
    pub fn op_test(_head: *mut OpusHead,
                   _initial_data: *const ::libc::c_uchar,
                   _initial_bytes: ::libc::size_t) -> ::libc::c_int;
    pub fn op_open_file(_path: *const ::libc::c_char,
                        _error: *mut ::libc::c_int) -> *mut OggOpusFile;
    pub fn op_open_memory(_data: *const ::libc::c_uchar, _size: ::libc::size_t,
                          _error: *mut ::libc::c_int) -> *mut OggOpusFile;
    // FIXME
    //pub fn op_vopen_url(_url: *const ::libc::c_char,
    //                    _error: *mut ::libc::c_int, _ap: va_list)
    // -> *mut OggOpusFile;
    pub fn op_open_url(_url: *const ::libc::c_char,
                       _error: *mut ::libc::c_int, ...) -> *mut OggOpusFile;
    pub fn op_open_callbacks(_source: *mut ::libc::c_void,
                             _cb: *const OpusFileCallbacks,
                             _initial_data: *const ::libc::c_uchar,
                             _initial_bytes: ::libc::size_t,
                             _error: *mut ::libc::c_int) -> *mut OggOpusFile;
    pub fn op_test_file(_path: *const ::libc::c_char,
                        _error: *mut ::libc::c_int) -> *mut OggOpusFile;
    pub fn op_test_memory(_data: *const ::libc::c_uchar, _size: ::libc::size_t,
                          _error: *mut ::libc::c_int) -> *mut OggOpusFile;
    // FIXME
    //pub fn op_vtest_url(_url: *const ::libc::c_char,
    //                    _error: *mut ::libc::c_int, _ap: va_list)
    // -> *mut OggOpusFile;
    pub fn op_test_url(_url: *const ::libc::c_char,
                       _error: *mut ::libc::c_int, ...) -> *mut OggOpusFile;
    pub fn op_test_callbacks(_source: *mut ::libc::c_void,
                             _cb: *const OpusFileCallbacks,
                             _initial_data: *const ::libc::c_uchar,
                             _initial_bytes: ::libc::size_t,
                             _error: *mut ::libc::c_int) -> *mut OggOpusFile;
    pub fn op_test_open(_of: *mut OggOpusFile) -> ::libc::c_int;
    pub fn op_free(_of: *mut OggOpusFile) -> ();
    pub fn op_seekable(_of: *const OggOpusFile) -> ::libc::c_int;
    pub fn op_link_count(_of: *const OggOpusFile) -> ::libc::c_int;
    pub fn op_serialno(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> opus_uint32;
    pub fn op_channel_count(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> ::libc::c_int;
    pub fn op_raw_total(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> ::libc::c_longlong;
    pub fn op_pcm_total(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> ogg_int64_t;
    pub fn op_head(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> *const OpusHead;
    pub fn op_tags(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> *const OpusTags;
    pub fn op_current_link(_of: *const OggOpusFile) -> ::libc::c_int;
    pub fn op_bitrate(_of: *const OggOpusFile, _li: ::libc::c_int)
     -> opus_int32;
    pub fn op_bitrate_instant(_of: *mut OggOpusFile) -> opus_int32;
    pub fn op_raw_tell(_of: *const OggOpusFile) -> ::libc::c_longlong;
    pub fn op_pcm_tell(_of: *const OggOpusFile) -> ogg_int64_t;
    pub fn op_raw_seek(_of: *mut OggOpusFile,
                       _byte_offset: ::libc::c_longlong) -> ::libc::c_int;
    pub fn op_pcm_seek(_of: *mut OggOpusFile, _pcm_offset: ogg_int64_t)
     -> ::libc::c_int;
    pub fn op_set_decode_callback(_of: *mut OggOpusFile,
                                  _decode_cb: op_decode_cb_func,
                                  _ctx: *mut ::libc::c_void) -> ();
    pub fn op_set_gain_offset(_of: *mut OggOpusFile,
                              _gain_type: ::libc::c_int,
                              _gain_offset_q8: opus_int32) -> ::libc::c_int;
    pub fn op_set_dither_enabled(_of: *mut OggOpusFile,
                                 _enabled: ::libc::c_int) -> ();
    pub fn op_read(_of: *mut OggOpusFile, _pcm: *mut opus_int16,
                   _buf_size: ::libc::c_int, _li: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn op_read_float(_of: *mut OggOpusFile, _pcm: *mut ::libc::c_float,
                         _buf_size: ::libc::c_int, _li: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn op_read_stereo(_of: *mut OggOpusFile, _pcm: *mut opus_int16,
                          _buf_size: ::libc::c_int) -> ::libc::c_int;
    pub fn op_read_float_stereo(_of: *mut OggOpusFile,
                                _pcm: *mut ::libc::c_float,
                                _buf_size: ::libc::c_int) -> ::libc::c_int;
}
