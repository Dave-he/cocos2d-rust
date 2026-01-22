use crate::math::{Vec3, Mat4};
use crate::base::Ref;
use crate::renderer::Texture;

#[derive(Debug)]
pub struct Mesh {
    name: String,
    vertex_data: Vec<f32>,
    index_data: Vec<u16>,
    vertex_count: u32,
    index_count: u32,
    aabb: AABB,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            name: String::new(),
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            vertex_count: 0,
            index_count: 0,
            aabb: AABB::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn get_index_count(&self) -> u32 {
        self.index_count
    }

    pub fn get_vertex_data(&self) -> &Vec<f32> {
        &self.vertex_data
    }

    pub fn get_index_data(&self) -> &Vec<u16> {
        &self.index_data
    }

    pub fn get_aabb(&self) -> &AABB {
        &self.aabb
    }

    pub fn set_vertex_data(&mut self, data: Vec<f32>) {
        self.vertex_data = data;
        self.vertex_count = (data.len() / 8) as u32;
    }

    pub fn set_index_data(&mut self, data: Vec<u16>) {
        self.index_data = data;
        self.index_count = data.len() as u32;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new() -> AABB {
        AABB {
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
        }
    }

    pub fn from_min_max(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }

    pub fn get_min(&self) -> Vec3 {
        self.min
    }

    pub fn get_max(&self) -> Vec3 {
        self.max
    }

    pub fn get_center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn get_half_extents(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    pub fn is_empty(&self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
    }

    pub fn reset(&mut self) {
        self.min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        self.max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
    }

    pub fn update_min_max(&mut self, points: &[Vec3]) {
        for point in points {
            if point.x < self.min.x { self.min.x = point.x; }
            if point.y < self.min.y { self.min.y = point.y; }
            if point.z < self.min.z { self.min.z = point.z; }
            if point.x > self.max.x { self.max.x = point.x; }
            if point.y > self.max.y { self.max.y = point.y; }
            if point.z > self.max.z { self.max.z = point.z; }
        }
    }

    pub fn contains_point(&self, point: &Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
            && point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn intersects_aabb(&self, aabb: &AABB) -> bool {
        self.min.x <= aabb.max.x && self.max.x >= aabb.min.x
            && self.min.y <= aabb.max.y && self.max.y >= aabb.min.y
            && self.min.z <= aabb.max.z && self.max.z >= aabb.min.z
    }
}

#[derive(Debug)]
pub struct MeshIndexData {
    index_buffer_id: u32,
    index_count: u32,
    index_format: IndexFormat,
}

impl MeshIndexData {
    pub fn new() -> MeshIndexData {
        MeshIndexData {
            index_buffer_id: 0,
            index_count: 0,
            index_format: IndexFormat::U16,
        }
    }

    pub fn get_index_count(&self) -> u32 {
        self.index_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    U16,
    U32,
}

#[derive(Debug)]
pub struct MeshVertexData {
    vertex_buffer_id: u32,
    vertex_count: u32,
    vertex_size: u32,
}

impl MeshVertexData {
    pub fn new() -> MeshVertexData {
        MeshVertexData {
            vertex_buffer_id: 0,
            vertex_count: 0,
            vertex_size: 0,
        }
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn get_vertex_size(&self) -> u32 {
        self.vertex_size
    }
}

#[derive(Debug)]
pub struct MeshSkin {
    bones: Vec<Ref<Bone3D>>,
    bone_indices: Vec<i32>,
    bone_weights: Vec<f32>,
    bind_pose_inverses: Vec<Mat4>,
}

impl MeshSkin {
    pub fn new() -> MeshSkin {
        MeshSkin {
            bones: Vec::new(),
            bone_indices: Vec::new(),
            bone_weights: Vec::new(),
            bind_pose_inverses: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, bone: Ref<Bone3D>) {
        self.bones.push(bone);
    }

    pub fn get_bones(&self) -> &Vec<Ref<Bone3D>> {
        &self.bones
    }

    pub fn set_bone_indices_and_weights(&mut self, indices: Vec<i32>, weights: Vec<f32>) {
        self.bone_indices = indices;
        self.bone_weights = weights;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttrib {
    POSITION,
    NORMAL,
    TANGENT,
    TEX_COORD,
    TEX_COORD1,
    TEX_COORD2,
    TEX_COORD3,
    TEX_COORD4,
    TEX_COORD5,
    TEX_COORD6,
    TEX_COORD7,
    TEX_COORD8,
    COLOR,
    COLOR1,
    COLOR2,
    COLOR3,
    COLOR4,
    BLEND_INDICES,
    BLEND_WEIGHT,
}

impl VertexAttrib {
    pub fn get_size(&self) -> u32 {
        match self {
            VertexAttrib::POSITION => 3,
            VertexAttrib::NORMAL => 3,
            VertexAttrib::TANGENT => 4,
            VertexAttrib::TEX_COORD => 2,
            VertexAttrib::TEX_COORD1 => 2,
            VertexAttrib::TEX_COORD2 => 2,
            VertexAttrib::TEX_COORD3 => 2,
            VertexAttrib::TEX_COORD4 => 2,
            VertexAttrib::TEX_COORD5 => 2,
            VertexAttrib::TEX_COORD6 => 2,
            VertexAttrib::TEX_COORD7 => 2,
            VertexAttrib::TEX_COORD8 => 2,
            VertexAttrib::COLOR => 4,
            VertexAttrib::COLOR1 => 4,
            VertexAttrib::COLOR2 => 4,
            VertexAttrib::COLOR3 => 4,
            VertexAttrib::COLOR4 => 4,
            VertexAttrib::BLEND_INDICES => 4,
            VertexAttrib::BLEND_WEIGHT => 4,
        }
    }

    pub fn get_type(&self) -> VertexAttribType {
        VertexAttribType::FLOAT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttribType {
    FLOAT,
    DOUBLE,
    BYTE,
    UBYTE,
    SHORT,
    USHORT,
    INT,
    UINT,
}

#[derive(Debug)]
pub struct VertexAttribBinding {
    vertex_buffer: Ref<MeshVertexData>,
    attribs: Vec<VertexAttribBindingInfo>,
}

#[derive(Debug, Clone)]
pub struct VertexAttribBindingInfo {
    attrib: VertexAttrib,
    size: u32,
    offset: u32,
}

impl VertexAttribBinding {
    pub fn new() -> VertexAttribBinding {
        VertexAttribBinding {
            vertex_buffer: Ref::new(MeshVertexData::new()),
            attribs: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct BillBoard {
    mode: BillBoardMode,
    eye_vector: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillBoardMode {
    VIEW_POINT_ORIENTED,
    VIEW_PLANE_ORIENTED,
}

impl BillBoard {
    pub fn new() -> BillBoard {
        BillBoard {
            mode: BillBoardMode::VIEW_POINT_ORIENTED,
            eye_vector: Vec3::ZERO,
        }
    }

    pub fn get_mode(&self) -> BillBoardMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: BillBoardMode) {
        self.mode = mode;
    }
}
