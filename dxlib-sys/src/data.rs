use crate::consts::*;

#[repr(C)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct DVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
pub struct Matrix {
    pub m: [[f32; 4]; 4],
}

#[repr(C)]
pub struct DMatrix {
    pub m: [[f64; 4]; 4],
}

#[repr(C)]
pub struct Float2 {
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
pub struct Double2 {
    pub u: f64,
    pub v: f64,
}

#[repr(C)]
pub struct Double4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
pub struct DateData {
    pub year: i32,
    pub mon: i32,
    pub day: i32,
    pub hour: i32,
    pub min: i32,
    pub sec: i32,
}

#[repr(C)]
pub struct TouchInputPoint {
    pub device: u32,
    pub id: u32,
    pub position_x: i32,
    pub position_y: i32,
}

#[repr(C)]
pub struct TouchInputData {
    pub time: i64,
    pub point_num: i32,
    pub point: [TouchInputPoint; TOUCHINPUTPOINT_MAX],
}

#[repr(C)]
pub struct IpData {
    pub d1: u8,
    pub d2: u8,
    pub d3: u8,
    pub d4: u8,
}

#[repr(C)]
pub struct IpDataV6 {
    pub addr: IpV6Addr,
    pub scope_id: u32,
}

#[repr(C)]
pub union IpV6Addr {
    pub byte: [u8; 16],
    pub word: [u16; 8],
}

#[repr(C)]
pub struct FileInfo {
    pub name: [i8; 260],
    pub dir_flag: i32,
    pub size: i64,
    pub cration_time: DateData,
    pub last_write_time: DateData,
}

#[repr(C)]
pub struct StreamDataShredType2 {
    pub open: extern "C" fn(*mut u8, i32, i32, i32) -> u32,
    pub close: extern "C" fn(u32) -> i32,
    // TODO: 残りを埋める
}

#[repr(C)]
pub struct StreamDataShredType2W {
    pub open: extern "C" fn(*mut u8, i32, i32, i32) -> u32,
    pub close: extern "C" fn(u32) -> i32,
    // TODO: 残りを埋める
}

#[repr(C)]
pub struct ColorData {
    pub format: u8,
    pub channel_num: u8,
    pub channel_bit_depth: u8,
    pub float_type_flag: u8,
    pub pixel_byte: u8,
    pub color_bit_depth: u8,
    pub none_loc: u8,
    pub none_width: u8,
    pub red_width: u8,
    pub green_width: u8,
    pub blue_width: u8,
    pub alpha_width: u8,
    pub red_loc: u8,
    pub green_loc: u8,
    pub blue_loc: u8,
    pub alpha_loc: u8,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub alpha_mask: u32,
    pub none_mask: u32,
    pub max_palette_no: i32,
}

#[repr(C)]
pub struct BaseImage {
    pub colordata: ColorData,
    pub width: i32,
    pub height: i32,
    pub pitch: i32,
    pub graph_data: *const libc::c_void,
    pub min_map_count: i32,
    pub graph_data_count: i32,
}

#[repr(C)]
pub struct BitMapInfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    size_image: u32,
    x_pels_per_meter: i32,
    y_pels_per_meter: i32,
    clr_used: u32,
    clr_important: u32,
}

#[repr(C)]
pub struct RgbQuad {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

#[repr(C)]
pub struct BitMapInfo {
    pub header: BitMapInfoHeader,
    pub colors: [RgbQuad; 1],
}

#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[repr(C)]
pub struct DInputJoyState {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rx: i32,
    pub ry: i32,
    pub rz: i32,
    pub slider: [i32; 2],
    pub pov: [u32; 4],
    pub buttons: [u8; 32],
}

#[repr(C)]
pub struct XInputState {
    pub buttons: [u8; 16],
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub thumb_lx: i16,
    pub thumb_ly: i16,
    pub thumb_rx: i16,
    pub thumb_ry: i16,
}

#[repr(C)]
pub struct ColorU8 {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

#[repr(C)]
pub struct Vertex2dShader {
    pub pos: Vector,
    pub rhw: f32,
    pub dif: ColorU8,
    pub spc: ColorU8,
    pub u: f32,
    pub v: f32,
    pub su: f32,
    pub sv: f32,
}

#[repr(C)]
pub struct Vertex3dShader {
    pub pos: Vector,
    pub spos: Float4,
    pub norm: Vector,
    pub tan: Vector,
    pub binorm: Vector,
    pub dif: ColorU8,
    pub spc: ColorU8,
    pub u: f32,
    pub v: f32,
    pub su: f32,
    pub sv: f32,
}

#[repr(C)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[repr(C)]
pub struct DisplayModeData {
    width: i32,
    height: i32,
    color_bit_depth: i32,
    refresh_rate: i32,
}

#[repr(C)]
pub struct ColorF32 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
pub struct Vertex2d {
    pub pos: Vector,
    pub rhw: f32,
    pub dif: ColorU8,
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

#[repr(C)]
pub struct Vertex3dCompat {
    pub pos: Vector,
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
pub struct Vertex3d {
    pub pos: Vector,
    pub norm: Vector,
    pub dif: ColorU8,
    pub spc: ColorU8,
    pub u: f32,
    pub v: f32,
    pub su: f32,
    pub sv: f32,
}

#[repr(C)]
pub struct CubeData {
    pub pos1: Vector,
    pub pos2: Vector,
    pub dif: ColorU8,
    pub spc: ColorU8,
}

#[repr(C)]
pub struct PointData {
    pub x: i32,
    pub y: i32,
    pub color: u32,
}

#[repr(C)]
pub struct LineData {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub color: u32,
    pub pal: i32,
}

#[repr(C)]
pub struct ImageFormatDesc {
    texture_flag: u8,
    // TODO: 残りを埋める
}

#[repr(C)]
pub struct DrawCharInfo {
    pub chr: [i8; 14],
    pub bytes: u32,
    pub draw_x: f32,
    pub draw_y: f32,
    pub size_x: f32,
    pub size_y: f32,
}

#[repr(C)]
pub struct SegmentSegmentResult {
    pub sega_segb_mindist_square: f32,
    pub sega_mindist_pos1_pos2_t: f32,
    pub segb_mindist_pos1_pos2_t: f32,
    pub sega_mindist_pos: Vector,
    pub segb_mindist_pos: Vector,
}

#[repr(C)]
pub struct SegmentSegmentResultDouble {
    pub sega_segb_mindist_square: f64,
    pub sega_mindist_pos1_pos2_t: f64,
    pub segb_mindist_pos1_pos2_t: f64,
    pub sega_mindist_pos: DVector,
    pub segb_mindist_pos: DVector,
}

#[repr(C)]
pub struct SegmentPointResult {
    pub seg_point_mindist_square: f32,
    pub seg_mindist_pos1_pos2_t: f32,
    pub sega_mindist_pos: Vector,
}

#[repr(C)]
pub struct SegmentPointResultDouble {
    pub seg_point_mindist_square: f64,
    pub seg_mindist_pos1_pos2_t: f64,
    pub seg_mindist_pos: DVector,
}

#[repr(C)]
pub struct SegmentTriangleResult {
    pub seg_tri_mindist_square: f32,
    pub seg_mindist_pos1_pos2_t: f32,
    pub seg_mindist_pos: Vector,
    pub tri_mindist_pos1_w: f32,
    pub tri_mindist_pos2_w: f32,
    pub tri_mindist_pos3_w: f32,
    pub tri_mindist_pos: Vector,
}

#[repr(C)]
pub struct SegmentTriangleResultDouble {
    pub seg_tri_mindist_square: f64,
    pub seg_mindist_pos1_pos2_t: f64,
    pub seg_mindist_pos: DVector,
    pub tri_mindist_pos1_w: f64,
    pub tri_mindist_pos2_w: f64,
    pub tri_mindist_pos3_w: f64,
    pub tri_mindist_pos: DVector,
}

#[repr(C)]
pub struct TrianglePointResult {
    pub tri_point_mindist_square: f32,
    pub tri_mindist_pos1_w: f32,
    pub tri_mindist_pos2_w: f32,
    pub tri_mindist_pos3_w: f32,
    pub tri_mindist_pos: Vector,
}

#[repr(C)]
pub struct TrianglePointResultDouble {
    pub tri_point_mindist_square: f64,
    pub tri_mindist_pos1_w: f64,
    pub tri_mindist_pos2_w: f64,
    pub tri_mindist_pos3_w: f64,
    pub tri_mindist_pos: DVector,
}

#[repr(C)]
pub struct PlanePointResult {
    pub point_plane_normal_side: i32,
    pub point_plane_mindist_square: f32,
    pub plane_mindist_pos: Vector,
}

#[repr(C)]
pub struct PlanePointResultDouble {
    pub point_plane_normal_side: i32,
    pub point_plane_mindist_square: f64,
    pub plane_mindist_pos: DVector,
}

#[repr(C)]
pub struct MaterialParam {
    pub diffuse: ColorF32,
    pub ambient: ColorF32,
    pub specular: ColorF32,
    pub emissive: ColorF32,
    pub power: f32,
}

#[repr(C)]
pub struct HitResultLine {
    pub hit_flag: i32,
    pub position: Vector,
}

#[repr(C)]
pub struct HitResultLineDouble {
    pub hit_flag: i32,
    pub position: DVector,
}

#[repr(C)]
pub struct StreamDataShred {
    pub tell: extern "C" fn(u32) -> i64,
    // TODO: 残りを埋める
}

#[repr(C)]
pub struct StreamData {
    pub read_shred: StreamDataShred,
    pub data_point: u32,
}

#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct WaveFormatEx {
    pub format_tag: u16,
    pub n_channels: u16,
    pub n_samples_per_sec: u32,
    pub n_avg_bytes_per_sec: u32,
    pub n_block_align: u16,
    pub bits_per_sample: u16,
    pub cb_size: u16,
}

#[repr(C)]
pub struct Sound3dReverbParam {
    pub wet_dry_mix: f32,
    pub reflection_delay: u32,
    pub reverb_delay: u8,
    pub rear_delay: u8,
    pub position_left: u8,
    pub position_right: u8,
    pub position_matrix_left: u8,
    pub position_matrix_right: u8,
    pub early_diffusion: u8,
    pub late_diffusion: u8,
    pub low_eq_gain: u8,
    pub low_eq_cutoff: u8,
    pub high_eq_gain: u8,
    pub high_eq_cutoff: u8,
    pub room_filter_freq: f32,
    pub room_filter_main: f32,
    pub room_filter_hf: f32,
    pub reflections_gain: f32,
    pub reverb_gain: f32,
    pub decay_time: f32,
    pub density: f32,
    pub room_size: f32,
}

#[repr(C)]
pub struct Mv1CollResultPoly {
    pub hit_flag: i32,
    pub hit_position: Vector,
    pub frame_index: i32,
    pub mesh_index: i32,
    pub polygon_index: i32,
    pub material_index: i32,
    pub position: [Vector; 3],
    pub normal: Vector,
    pub position_weight: [f32; 3],
    pub pos_max_weight_frame_index: [i32; 3],
}

#[repr(C)]
pub struct Mv1CollResultPolyDim {
    pub hit_num: i32,
    pub dim: *const Mv1CollResultPoly,
}

#[repr(C)]
pub struct Mv1RefVertex {
    pub position: Vector,
    pub normal: Vector,
    pub tex_coord: [Float2; 2],
    pub diffuse: ColorU8,
    pub specular: ColorU8,
    pub max_weight_frame_index: i32,
}

#[repr(C)]
pub struct Mv1RefPolygon {
    pub frame_index: u16,
    pub mesh_index: u16,
    pub material_index: u16,
    pub v_index_target: u16,
    pub v_index: [i32; 3],
    pub min_position: Vector,
    pub max_position: Vector,
}

#[repr(C)]
pub struct Mv1RefPolygonList {
    pub polygon_num: i32,
    pub vertex_num: i32,
    pub min_position: Vector,
    pub max_position: Vector,
    pub polygons: *const Mv1RefPolygon,
    pub vertexs: *const Mv1RefVertex,
}
