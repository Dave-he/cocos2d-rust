use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::math::{Mat4, Vec2, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    UByte4,
    Short2,
    Short4,
}

impl VertexFormat {
    pub fn size_bytes(&self) -> usize {
        match self {
            VertexFormat::Float => 4,
            VertexFormat::Float2 => 8,
            VertexFormat::Float3 => 12,
            VertexFormat::Float4 => 16,
            VertexFormat::Byte4 => 4,
            VertexFormat::UByte4 => 4,
            VertexFormat::Short2 => 4,
            VertexFormat::Short4 => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttrib {
    Position,
    Normal,
    Color,
    TexCoord,
    TexCoord1,
    TexCoord2,
    TexCoord3,
    BlendWeight,
    BlendIndex,
}

#[derive(Debug, Clone)]
pub struct MeshVertexAttrib {
    pub vertex_attrib: VertexAttrib,
    pub format: VertexFormat,
    pub offset: usize,
}

impl MeshVertexAttrib {
    pub fn new(attrib: VertexAttrib, format: VertexFormat, offset: usize) -> Self {
        Self {
            vertex_attrib: attrib,
            format,
            offset,
        }
    }

    pub fn size_bytes(&self) -> usize {
        self.format.size_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertex_size_in_float: usize,
    pub vertex: Vec<f32>,
    pub attribs: Vec<MeshVertexAttrib>,
    pub num_index: usize,
    pub indices: Vec<u16>,
    pub indices_32: Vec<u32>,
    pub sub_mesh_id: String,
    pub sub_mesh_aabb_min: Vec3,
    pub sub_mesh_aabb_max: Vec3,
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            vertex_size_in_float: 0,
            vertex: Vec::new(),
            attribs: Vec::new(),
            num_index: 0,
            indices: Vec::new(),
            indices_32: Vec::new(),
            sub_mesh_id: String::new(),
            sub_mesh_aabb_min: Vec3::ZERO,
            sub_mesh_aabb_max: Vec3::ZERO,
        }
    }

    pub fn reset(&mut self) {
        self.vertex.clear();
        self.attribs.clear();
        self.indices.clear();
        self.indices_32.clear();
        self.sub_mesh_id.clear();
        self.num_index = 0;
        self.vertex_size_in_float = 0;
    }
}

#[derive(Debug, Clone)]
pub struct ModelData {
    pub sub_mesh_id: String,
    pub material_id: String,
    pub bones: Vec<String>,
    pub inv_bind_pose: Vec<Mat4>,
}

impl ModelData {
    pub fn new() -> Self {
        Self {
            sub_mesh_id: String::new(),
            material_id: String::new(),
            bones: Vec::new(),
            inv_bind_pose: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.bones.clear();
        self.inv_bind_pose.clear();
    }
}

impl Default for ModelData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct NodeData {
    pub id: String,
    pub transform: Mat4,
    pub model_node_datas: Vec<ModelData>,
    pub children: Vec<NodeData>,
}

impl NodeData {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            transform: Mat4::IDENTITY,
            model_node_datas: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.id.clear();
        self.transform = Mat4::IDENTITY;
        self.model_node_datas.clear();
        self.children.clear();
    }
}

impl Default for NodeData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MaterialData {
    pub id: String,
    pub textures: HashMap<String, String>,
    pub properties: HashMap<String, Vec<f32>>,
}

impl MaterialData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.id.clear();
        self.textures.clear();
        self.properties.clear();
    }
}

#[derive(Debug, Clone)]
pub struct SkinData {
    pub bones: Vec<String>,
    pub inv_bind_pose: Vec<Mat4>,
    pub root_bone: Option<String>,
}

impl SkinData {
    pub fn new() -> Self {
        Self {
            bones: Vec::new(),
            inv_bind_pose: Vec::new(),
            root_bone: None,
        }
    }

    pub fn reset(&mut self) {
        self.bones.clear();
        self.inv_bind_pose.clear();
        self.root_bone = None;
    }

    pub fn add_skin_bone_names(&mut self, bone: String) {
        self.bones.push(bone);
    }
}

impl Default for SkinData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Animation3DData {
    pub key_frame_data: HashMap<String, Vec<f32>>,
    pub duration: f32,
}

impl Animation3DData {
    pub fn new() -> Self {
        Self {
            key_frame_data: HashMap::new(),
            duration: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.key_frame_data.clear();
        self.duration = 0.0;
    }
}

impl Default for Animation3DData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BundleFormat {
    C3T,
    C3B,
    Unknown,
}

pub struct Bundle3D {
    format: BundleFormat,
    path: String,
    data: Vec<u8>,
    mesh_datas: Vec<MeshData>,
    material_datas: Vec<MaterialData>,
    node_datas: Vec<NodeData>,
    skin_datas: Vec<SkinData>,
    animation_datas: HashMap<String, Animation3DData>,
}

impl Bundle3D {
    pub fn new() -> Self {
        Self {
            format: BundleFormat::Unknown,
            path: String::new(),
            data: Vec::new(),
            mesh_datas: Vec::new(),
            material_datas: Vec::new(),
            node_datas: Vec::new(),
            skin_datas: Vec::new(),
            animation_datas: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str) -> Result<(), String> {
        self.path = path.to_string();
        
        let extension = Path::new(path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        
        self.format = match extension.to_lowercase().as_str() {
            "c3t" => BundleFormat::C3T,
            "c3b" => BundleFormat::C3B,
            _ => return Err(format!("Unsupported file format: {}", extension)),
        };
        
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut reader = BufReader::new(file);
        
        self.data.clear();
        reader
            .read_to_end(&mut self.data)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        match self.format {
            BundleFormat::C3T => self.parse_json(),
            BundleFormat::C3B => self.parse_binary(),
            BundleFormat::Unknown => Err("Unknown format".to_string()),
        }
    }

    fn parse_json(&mut self) -> Result<(), String> {
        let json_str = String::from_utf8(self.data.clone())
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        
        Ok(())
    }

    fn parse_binary(&mut self) -> Result<(), String> {
        if self.data.len() < 12 {
            return Err("File too small to be valid C3B".to_string());
        }
        
        let magic = &self.data[0..4];
        if magic != b"C3B\0" && magic != b"C3B " {
            return Err("Invalid C3B magic number".to_string());
        }
        
        Ok(())
    }

    pub fn load_mesh_datas(&mut self) -> Result<Vec<MeshData>, String> {
        Ok(self.mesh_datas.clone())
    }

    pub fn load_material_datas(&mut self) -> Result<Vec<MaterialData>, String> {
        Ok(self.material_datas.clone())
    }

    pub fn load_node_datas(&mut self) -> Result<Vec<NodeData>, String> {
        Ok(self.node_datas.clone())
    }

    pub fn load_skin_data(&self, id: &str) -> Option<&SkinData> {
        if id.is_empty() {
            self.skin_datas.first()
        } else {
            None
        }
    }

    pub fn load_animation_data(&self, id: &str) -> Option<&Animation3DData> {
        if id.is_empty() {
            self.animation_datas.values().next()
        } else {
            self.animation_datas.get(id)
        }
    }

    pub fn clear(&mut self) {
        self.format = BundleFormat::Unknown;
        self.path.clear();
        self.data.clear();
        self.mesh_datas.clear();
        self.material_datas.clear();
        self.node_datas.clear();
        self.skin_datas.clear();
        self.animation_datas.clear();
    }

    pub fn get_format(&self) -> BundleFormat {
        self.format
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl Default for Bundle3D {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_format_size() {
        assert_eq!(VertexFormat::Float.size_bytes(), 4);
        assert_eq!(VertexFormat::Float2.size_bytes(), 8);
        assert_eq!(VertexFormat::Float3.size_bytes(), 12);
        assert_eq!(VertexFormat::Float4.size_bytes(), 16);
    }

    #[test]
    fn test_mesh_vertex_attrib() {
        let attrib = MeshVertexAttrib::new(VertexAttrib::Position, VertexFormat::Float3, 0);
        assert_eq!(attrib.size_bytes(), 12);
    }

    #[test]
    fn test_mesh_data_creation() {
        let mesh = MeshData::new();
        assert_eq!(mesh.vertex.len(), 0);
        assert_eq!(mesh.indices.len(), 0);
    }

    #[test]
    fn test_mesh_data_reset() {
        let mut mesh = MeshData::new();
        mesh.vertex = vec![1.0, 2.0, 3.0];
        mesh.indices = vec![0, 1, 2];
        
        mesh.reset();
        assert_eq!(mesh.vertex.len(), 0);
        assert_eq!(mesh.indices.len(), 0);
    }

    #[test]
    fn test_model_data_creation() {
        let model = ModelData::new();
        assert!(model.bones.is_empty());
        assert!(model.inv_bind_pose.is_empty());
    }

    #[test]
    fn test_node_data_creation() {
        let node = NodeData::new();
        assert!(node.id.is_empty());
        assert_eq!(node.transform, Mat4::IDENTITY);
    }

    #[test]
    fn test_material_data() {
        let mut material = MaterialData::new();
        material.id = "test_material".to_string();
        material.textures.insert("diffuse".to_string(), "texture.png".to_string());
        
        assert_eq!(material.id, "test_material");
        assert_eq!(material.textures.get("diffuse").unwrap(), "texture.png");
    }

    #[test]
    fn test_skin_data() {
        let mut skin = SkinData::new();
        skin.add_skin_bone_names("bone1".to_string());
        skin.add_skin_bone_names("bone2".to_string());
        
        assert_eq!(skin.bones.len(), 2);
        assert_eq!(skin.bones[0], "bone1");
    }

    #[test]
    fn test_animation_data() {
        let mut anim = Animation3DData::new();
        anim.duration = 5.0;
        anim.key_frame_data.insert("rotation".to_string(), vec![0.0, 1.0, 2.0]);
        
        assert_eq!(anim.duration, 5.0);
        assert_eq!(anim.key_frame_data.len(), 1);
    }

    #[test]
    fn test_bundle3d_creation() {
        let bundle = Bundle3D::new();
        assert_eq!(bundle.get_format(), BundleFormat::Unknown);
        assert!(bundle.get_path().is_empty());
    }

    #[test]
    fn test_bundle3d_clear() {
        let mut bundle = Bundle3D::new();
        bundle.path = "test.c3b".to_string();
        bundle.format = BundleFormat::C3B;
        
        bundle.clear();
        assert_eq!(bundle.get_format(), BundleFormat::Unknown);
        assert!(bundle.get_path().is_empty());
    }

    #[test]
    fn test_bundle_format_detection() {
        let mut bundle = Bundle3D::new();
        
        let result = bundle.load("test.c3t");
        assert!(result.is_err());
        
        let result = bundle.load("test.c3b");
        assert!(result.is_err());
    }
}
