#![crate_id = "avfilter3"]
#![license = "MIT"]
#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
/* automatically generated by rust-bindgen */
extern crate libc;
extern crate avutil = "avutil52";
extern crate avcodec = "avcodec54";
use libc::{uint8_t,int64_t,uint64_t,int8_t};

pub type __int128_t = ::libc::c_void;
pub type __uint128_t = ::libc::c_void;
pub type __builtin_va_list = [__va_list_tag, ..1u];
pub type AVFilterContext = Struct_AVFilterContext;
pub type AVFilterLink = Struct_AVFilterLink;
pub type AVFilterPad = Struct_AVFilterPad;
pub enum Struct_AVFilterFormats { }
pub type AVFilterFormats = Struct_AVFilterFormats;
#[repr(C)]
pub struct Struct_AVFilterBuffer {
    pub data: [*mut uint8_t, ..8u],
    pub extended_data: *mut *mut uint8_t,
    pub linesize: [::libc::c_int, ..8u],
    pub _priv: *mut ::libc::c_void,
    pub free: ::std::option::Option<extern "C" fn
                                        (arg1: *mut Struct_AVFilterBuffer)>,
    pub format: ::libc::c_int,
    pub w: ::libc::c_int,
    pub h: ::libc::c_int,
    pub refcount: ::libc::c_uint,
}
pub type AVFilterBuffer = Struct_AVFilterBuffer;
#[repr(C)]
pub struct Struct_AVFilterBufferRefAudioProps {
    pub channel_layout: uint64_t,
    pub nb_samples: ::libc::c_int,
    pub sample_rate: ::libc::c_int,
    pub channels: ::libc::c_int,
}
pub type AVFilterBufferRefAudioProps = Struct_AVFilterBufferRefAudioProps;
#[repr(C)]
pub struct Struct_AVFilterBufferRefVideoProps {
    pub w: ::libc::c_int,
    pub h: ::libc::c_int,
    pub sample_aspect_ratio: avutil::AVRational,
    pub interlaced: ::libc::c_int,
    pub top_field_first: ::libc::c_int,
    pub pict_type: avutil::Enum_AVPictureType,
    pub key_frame: ::libc::c_int,
    pub qp_table_linesize: ::libc::c_int,
    pub qp_table_size: ::libc::c_int,
    pub qp_table: *mut int8_t,
}
pub type AVFilterBufferRefVideoProps = Struct_AVFilterBufferRefVideoProps;
#[repr(C)]
pub struct Struct_AVFilterBufferRef {
    pub buf: *mut AVFilterBuffer,
    pub data: [*mut uint8_t, ..8u],
    pub extended_data: *mut *mut uint8_t,
    pub linesize: [::libc::c_int, ..8u],
    pub video: *mut AVFilterBufferRefVideoProps,
    pub audio: *mut AVFilterBufferRefAudioProps,
    pub pts: int64_t,
    pub pos: int64_t,
    pub format: ::libc::c_int,
    pub perms: ::libc::c_int,
    pub _type: avutil::Enum_AVMediaType,
    pub metadata: *mut avutil::AVDictionary,
}
pub type AVFilterBufferRef = Struct_AVFilterBufferRef;
#[repr(C)]
pub struct Struct_AVFilterPad {
    pub name: *const ::libc::c_char,
    pub _type: avutil::Enum_AVMediaType,
    pub min_perms: ::libc::c_int,
    pub rej_perms: ::libc::c_int,
    pub start_frame: ::std::option::Option<extern "C" fn
                                               (arg1: *mut AVFilterLink,
                                                arg2: *mut AVFilterBufferRef)
                                               -> ::libc::c_int>,
    pub get_video_buffer: ::std::option::Option<extern "C" fn
                                                    (arg1: *mut AVFilterLink,
                                                     arg2: ::libc::c_int,
                                                     arg3: ::libc::c_int)
                                                    -> *mut avutil::AVFrame>,
    pub get_audio_buffer: ::std::option::Option<extern "C" fn
                                                    (arg1: *mut AVFilterLink,
                                                     arg2: ::libc::c_int)
                                                    -> *mut avutil::AVFrame>,
    pub end_frame: ::std::option::Option<extern "C" fn
                                             (arg1: *mut AVFilterLink)
                                             -> ::libc::c_int>,
    pub draw_slice: ::std::option::Option<extern "C" fn
                                              (arg1: *mut AVFilterLink,
                                               arg2: ::libc::c_int,
                                               arg3: ::libc::c_int,
                                               arg4: ::libc::c_int)
                                              -> ::libc::c_int>,
    pub filter_frame: ::std::option::Option<extern "C" fn
                                                (arg1: *mut AVFilterLink,
                                                 arg2: *mut avutil::AVFrame)
                                                -> ::libc::c_int>,
    pub poll_frame: ::std::option::Option<extern "C" fn
                                              (arg1: *mut AVFilterLink)
                                              -> ::libc::c_int>,
    pub request_frame: ::std::option::Option<extern "C" fn
                                                 (arg1: *mut AVFilterLink)
                                                 -> ::libc::c_int>,
    pub config_props: ::std::option::Option<extern "C" fn
                                                (arg1: *mut AVFilterLink)
                                                -> ::libc::c_int>,
    pub needs_fifo: ::libc::c_int,
    pub needs_writable: ::libc::c_int,
}
#[repr(C)]
pub struct Struct_AVFilter {
    pub name: *const ::libc::c_char,
    pub description: *const ::libc::c_char,
    pub inputs: *const AVFilterPad,
    pub outputs: *const AVFilterPad,
    pub priv_class: *const avutil::AVClass,
    pub flags: ::libc::c_int,
    pub init: ::std::option::Option<extern "C" fn(arg1: *mut AVFilterContext)
                                        -> ::libc::c_int>,
    pub init_dict: ::std::option::Option<extern "C" fn
                                             (arg1: *mut AVFilterContext,
                                              arg2: *mut *mut avutil::AVDictionary)
                                             -> ::libc::c_int>,
    pub uninit: ::std::option::Option<extern "C" fn
                                          (arg1: *mut AVFilterContext)>,
    pub query_formats: ::std::option::Option<extern "C" fn
                                                 (arg1: *mut AVFilterContext)
                                                 -> ::libc::c_int>,
    pub priv_size: ::libc::c_int,
    pub next: *mut Struct_AVFilter,
    pub process_command: ::std::option::Option<extern "C" fn
                                                   (arg1:
                                                        *mut AVFilterContext,
                                                    arg2:
                                                        *const ::libc::c_char,
                                                    arg3:
                                                        *const ::libc::c_char,
                                                    arg4: *mut ::libc::c_char,
                                                    arg5: ::libc::c_int,
                                                    arg6: ::libc::c_int)
                                                   -> ::libc::c_int>,
    pub init_opaque: ::std::option::Option<extern "C" fn
                                               (arg1: *mut AVFilterContext,
                                                arg2: *mut ::libc::c_void)
                                               -> ::libc::c_int>,
}
pub type AVFilter = Struct_AVFilter;
pub enum Struct_AVFilterInternal { }
pub type AVFilterInternal = Struct_AVFilterInternal;
#[repr(C)]
pub struct Struct_AVFilterContext {
    pub av_class: *const avutil::AVClass,
    pub filter: *const AVFilter,
    pub name: *mut ::libc::c_char,
    pub input_pads: *mut AVFilterPad,
    pub inputs: *mut *mut AVFilterLink,
    pub input_count: ::libc::c_uint,
    pub nb_inputs: ::libc::c_uint,
    pub output_pads: *mut AVFilterPad,
    pub outputs: *mut *mut AVFilterLink,
    pub output_count: ::libc::c_uint,
    pub nb_outputs: ::libc::c_uint,
    pub _priv: *mut ::libc::c_void,
    pub graph: *mut Struct_AVFilterGraph,
    pub thread_type: ::libc::c_int,
    pub internal: *mut AVFilterInternal,
    pub command_queue: *mut Struct_AVFilterCommand,
    pub enable_str: *mut ::libc::c_char,
    pub enable: *mut ::libc::c_void,
    pub var_values: *mut ::libc::c_double,
    pub is_disabled: ::libc::c_int,
}
pub enum Struct_AVFilterCommand { }
#[repr(C)]
pub struct Struct_AVFilterLink {
    pub src: *mut AVFilterContext,
    pub srcpad: *mut AVFilterPad,
    pub dst: *mut AVFilterContext,
    pub dstpad: *mut AVFilterPad,
    pub _type: avutil::Enum_AVMediaType,
    pub w: ::libc::c_int,
    pub h: ::libc::c_int,
    pub sample_aspect_ratio: avutil::AVRational,
    pub channel_layout: uint64_t,
    pub sample_rate: ::libc::c_int,
    pub format: ::libc::c_int,
    pub time_base: avutil::AVRational,
    pub in_formats: *mut AVFilterFormats,
    pub out_formats: *mut AVFilterFormats,
    pub in_samplerates: *mut AVFilterFormats,
    pub out_samplerates: *mut AVFilterFormats,
    pub in_channel_layouts: *mut Struct_AVFilterChannelLayouts,
    pub out_channel_layouts: *mut Struct_AVFilterChannelLayouts,
    pub request_samples: ::libc::c_int,
    pub init_state: Enum_Unnamed1,
    pub pool: *mut Struct_AVFilterPool,
    pub graph: *mut Struct_AVFilterGraph,
    pub current_pts: int64_t,
    pub age_index: ::libc::c_int,
    pub frame_rate: avutil::AVRational,
    pub partial_buf: *mut avutil::AVFrame,
    pub partial_buf_size: ::libc::c_int,
    pub min_samples: ::libc::c_int,
    pub max_samples: ::libc::c_int,
    pub cur_buf_copy: *mut AVFilterBufferRef,
    pub closed: ::libc::c_int,
    pub channels: ::libc::c_int,
    pub frame_requested: ::libc::c_uint,
    pub flags: ::libc::c_uint,
    pub frame_count: int64_t,
}
pub enum Struct_AVFilterChannelLayouts { }
pub type Enum_Unnamed1 = ::libc::c_uint;
pub static AVLINK_UNINIT: ::libc::c_uint = 0;
pub static AVLINK_STARTINIT: ::libc::c_uint = 1;
pub static AVLINK_INIT: ::libc::c_uint = 2;
pub enum Struct_AVFilterPool { }
pub enum Struct_AVFilterGraphInternal { }
pub type AVFilterGraphInternal = Struct_AVFilterGraphInternal;
pub type avfilter_action_func = ::libc::c_void;
pub type avfilter_execute_func = ::libc::c_void;
#[repr(C)]
pub struct Struct_AVFilterGraph {
    pub av_class: *const avutil::AVClass,
    pub filter_count_unused: ::libc::c_uint,
    pub filters: *mut *mut AVFilterContext,
    pub scale_sws_opts: *mut ::libc::c_char,
    pub resample_lavr_opts: *mut ::libc::c_char,
    pub nb_filters: ::libc::c_uint,
    pub thread_type: ::libc::c_int,
    pub nb_threads: ::libc::c_int,
    pub internal: *mut AVFilterGraphInternal,
    pub opaque: *mut ::libc::c_void,
    pub execute: *mut avfilter_execute_func,
    pub aresample_swr_opts: *mut ::libc::c_char,
    pub sink_links: *mut *mut AVFilterLink,
    pub sink_links_count: ::libc::c_int,
    pub disable_auto_convert: ::libc::c_uint,
}
pub type AVFilterGraph = Struct_AVFilterGraph;
pub type Enum_Unnamed2 = ::libc::c_int;
pub static AVFILTER_AUTO_CONVERT_ALL: ::libc::c_int = 0;
pub static AVFILTER_AUTO_CONVERT_NONE: ::libc::c_int = -1;
#[repr(C)]
pub struct Struct_AVFilterInOut {
    pub name: *mut ::libc::c_char,
    pub filter_ctx: *mut AVFilterContext,
    pub pad_idx: ::libc::c_int,
    pub next: *mut Struct_AVFilterInOut,
}
pub type AVFilterInOut = Struct_AVFilterInOut;
pub type __va_list_tag = Struct___va_list_tag;
#[repr(C)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}
#[link(name = "avfilter")]
extern "C" {
    pub fn avfilter_version() -> ::libc::c_uint;
    pub fn avfilter_configuration() -> *const ::libc::c_char;
    pub fn avfilter_license() -> *const ::libc::c_char;
    pub fn avfilter_copy_buffer_ref_props(dst: *mut AVFilterBufferRef,
                                          src: *mut AVFilterBufferRef);
    pub fn avfilter_ref_buffer(_ref: *mut AVFilterBufferRef,
                               pmask: ::libc::c_int) ->
     *mut AVFilterBufferRef;
    pub fn avfilter_unref_buffer(_ref: *mut AVFilterBufferRef);
    pub fn avfilter_unref_bufferp(_ref: *mut *mut AVFilterBufferRef);
    pub fn avfilter_ref_get_channels(_ref: *mut AVFilterBufferRef) ->
     ::libc::c_int;
    pub fn avfilter_pad_count(pads: *const AVFilterPad) -> ::libc::c_int;
    pub fn avfilter_pad_get_name(pads: *const AVFilterPad,
                                 pad_idx: ::libc::c_int) ->
     *const ::libc::c_char;
    pub fn avfilter_pad_get_type(pads: *const AVFilterPad,
                                 pad_idx: ::libc::c_int) -> avutil::Enum_AVMediaType;
    pub fn avfilter_link(src: *mut AVFilterContext, srcpad: ::libc::c_uint,
                         dst: *mut AVFilterContext, dstpad: ::libc::c_uint) ->
     ::libc::c_int;
    pub fn avfilter_link_free(link: *mut *mut AVFilterLink);
    pub fn avfilter_link_get_channels(link: *mut AVFilterLink) ->
     ::libc::c_int;
    pub fn avfilter_link_set_closed(link: *mut AVFilterLink,
                                    closed: ::libc::c_int);
    pub fn avfilter_config_links(filter: *mut AVFilterContext) ->
     ::libc::c_int;
    pub fn avfilter_get_video_buffer_ref_from_arrays(data:
                                                         [*mut uint8_t, ..4u],
                                                     linesize:
                                                         [::libc::c_int, ..4u],
                                                     perms: ::libc::c_int,
                                                     w: ::libc::c_int,
                                                     h: ::libc::c_int,
                                                     format:
                                                         avutil::Enum_AVPixelFormat)
     -> *mut AVFilterBufferRef;
    pub fn avfilter_get_audio_buffer_ref_from_arrays(data: *mut *mut uint8_t,
                                                     linesize: ::libc::c_int,
                                                     perms: ::libc::c_int,
                                                     nb_samples:
                                                         ::libc::c_int,
                                                     sample_fmt:
                                                         avutil::Enum_AVSampleFormat,
                                                     channel_layout: uint64_t)
     -> *mut AVFilterBufferRef;
    pub fn avfilter_get_audio_buffer_ref_from_arrays_channels(data:
                                                                  *mut *mut uint8_t,
                                                              linesize:
                                                                  ::libc::c_int,
                                                              perms:
                                                                  ::libc::c_int,
                                                              nb_samples:
                                                                  ::libc::c_int,
                                                              sample_fmt:
                                                                  avutil::Enum_AVSampleFormat,
                                                              channels:
                                                                  ::libc::c_int,
                                                              channel_layout:
                                                                  uint64_t) ->
     *mut AVFilterBufferRef;
    pub fn avfilter_process_command(filter: *mut AVFilterContext,
                                    cmd: *const ::libc::c_char,
                                    arg: *const ::libc::c_char,
                                    res: *mut ::libc::c_char,
                                    res_len: ::libc::c_int,
                                    flags: ::libc::c_int) -> ::libc::c_int;
    pub fn avfilter_register_all();
    pub fn avfilter_uninit();
    pub fn avfilter_register(filter: *mut AVFilter) -> ::libc::c_int;
    pub fn avfilter_get_by_name(name: *const ::libc::c_char) -> *mut AVFilter;
    pub fn avfilter_next(prev: *const AVFilter) -> *const AVFilter;
    pub fn av_filter_next(filter: *mut *mut AVFilter) -> *mut *mut AVFilter;
    pub fn avfilter_open(filter_ctx: *mut *mut AVFilterContext,
                         filter: *mut AVFilter,
                         inst_name: *const ::libc::c_char) -> ::libc::c_int;
    pub fn avfilter_init_filter(filter: *mut AVFilterContext,
                                args: *const ::libc::c_char,
                                opaque: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn avfilter_init_str(ctx: *mut AVFilterContext,
                             args: *const ::libc::c_char) -> ::libc::c_int;
    pub fn avfilter_init_dict(ctx: *mut AVFilterContext,
                              options: *mut *mut avutil::AVDictionary) ->
     ::libc::c_int;
    pub fn avfilter_free(filter: *mut AVFilterContext);
    pub fn avfilter_insert_filter(link: *mut AVFilterLink,
                                  filt: *mut AVFilterContext,
                                  filt_srcpad_idx: ::libc::c_uint,
                                  filt_dstpad_idx: ::libc::c_uint) ->
     ::libc::c_int;
    pub fn avfilter_copy_frame_props(dst: *mut AVFilterBufferRef,
                                     src: *const avutil::AVFrame) -> ::libc::c_int;
    pub fn avfilter_copy_buf_props(dst: *mut avutil::AVFrame,
                                   src: *const AVFilterBufferRef) ->
     ::libc::c_int;
    pub fn avfilter_get_class() -> *const avutil::AVClass;
    pub fn avfilter_graph_alloc() -> *mut AVFilterGraph;
    pub fn avfilter_graph_alloc_filter(graph: *mut AVFilterGraph,
                                       filter: *const AVFilter,
                                       name: *const ::libc::c_char) ->
     *mut AVFilterContext;
    pub fn avfilter_graph_get_filter(graph: *mut AVFilterGraph,
                                     name: *mut ::libc::c_char) ->
     *mut AVFilterContext;
    pub fn avfilter_graph_add_filter(graphctx: *mut AVFilterGraph,
                                     filter: *mut AVFilterContext) ->
     ::libc::c_int;
    pub fn avfilter_graph_create_filter(filt_ctx: *mut *mut AVFilterContext,
                                        filt: *const AVFilter,
                                        name: *const ::libc::c_char,
                                        args: *const ::libc::c_char,
                                        opaque: *mut ::libc::c_void,
                                        graph_ctx: *mut AVFilterGraph) ->
     ::libc::c_int;
    pub fn avfilter_graph_set_auto_convert(graph: *mut AVFilterGraph,
                                           flags: ::libc::c_uint);
    pub fn avfilter_graph_config(graphctx: *mut AVFilterGraph,
                                 log_ctx: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn avfilter_graph_free(graph: *mut *mut AVFilterGraph);
    pub fn avfilter_inout_alloc() -> *mut AVFilterInOut;
    pub fn avfilter_inout_free(inout: *mut *mut AVFilterInOut);
    pub fn avfilter_graph_parse(graph: *mut AVFilterGraph,
                                filters: *const ::libc::c_char,
                                inputs: *mut *mut AVFilterInOut,
                                outputs: *mut *mut AVFilterInOut,
                                log_ctx: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn avfilter_graph_parse_ptr(graph: *mut AVFilterGraph,
                                    filters: *const ::libc::c_char,
                                    inputs: *mut *mut AVFilterInOut,
                                    outputs: *mut *mut AVFilterInOut,
                                    log_ctx: *mut ::libc::c_void) ->
     ::libc::c_int;
    pub fn avfilter_graph_parse2(graph: *mut AVFilterGraph,
                                 filters: *const ::libc::c_char,
                                 inputs: *mut *mut AVFilterInOut,
                                 outputs: *mut *mut AVFilterInOut) ->
     ::libc::c_int;
    pub fn avfilter_graph_send_command(graph: *mut AVFilterGraph,
                                       target: *const ::libc::c_char,
                                       cmd: *const ::libc::c_char,
                                       arg: *const ::libc::c_char,
                                       res: *mut ::libc::c_char,
                                       res_len: ::libc::c_int,
                                       flags: ::libc::c_int) -> ::libc::c_int;
    pub fn avfilter_graph_queue_command(graph: *mut AVFilterGraph,
                                        target: *const ::libc::c_char,
                                        cmd: *const ::libc::c_char,
                                        arg: *const ::libc::c_char,
                                        flags: ::libc::c_int,
                                        ts: ::libc::c_double) ->
     ::libc::c_int;
    pub fn avfilter_graph_dump(graph: *mut AVFilterGraph,
                               options: *const ::libc::c_char) ->
     *mut ::libc::c_char;
    pub fn avfilter_graph_request_oldest(graph: *mut AVFilterGraph) ->
     ::libc::c_int;
}

pub fn version() -> uint{
    unsafe {
        avfilter_version() as uint
    }
}
pub fn license() -> String {
    unsafe {
        std::str::raw::from_c_str(avfilter_license())
    }
}
