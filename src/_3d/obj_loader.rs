use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::math::{Vec2, Vec3};

#[derive(Debug, Clone, Default)]
pub struct ObjMaterial {
    pub name: String,
    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
    pub transmittance: [f32; 3],
    pub emission: [f32; 3],
    pub shininess: f32,
    pub ior: f32,
    pub dissolve: f32,
    pub illum: i32,
    pub ambient_texture: String,
    pub diffuse_texture: String,
    pub specular_texture: String,
    pub normal_texture: String,
}

impl ObjMaterial {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ambient: [0.2, 0.2, 0.2],
            diffuse: [0.8, 0.8, 0.8],
            specular: [1.0, 1.0, 1.0],
            transmittance: [0.0, 0.0, 0.0],
            emission: [0.0, 0.0, 0.0],
            shininess: 0.0,
            ior: 1.0,
            dissolve: 1.0,
            illum: 1,
            ambient_texture: String::new(),
            diffuse_texture: String::new(),
            specular_texture: String::new(),
            normal_texture: String::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObjMesh {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub indices: Vec<u32>,
    pub material_ids: Vec<i32>,
}

impl ObjMesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

#[derive(Debug, Clone)]
pub struct ObjShape {
    pub name: String,
    pub mesh: ObjMesh,
}

impl ObjShape {
    pub fn new(name: String) -> Self {
        Self {
            name,
            mesh: ObjMesh::new(),
        }
    }
}

pub struct ObjLoader {
    shapes: Vec<ObjShape>,
    materials: Vec<ObjMaterial>,
    material_map: HashMap<String, usize>,
}

impl ObjLoader {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            materials: Vec::new(),
            material_map: HashMap::new(),
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        
        let base_path = Path::new(path)
            .parent()
            .map(|p| p.to_str().unwrap_or(""))
            .unwrap_or("");
        
        Self::load_from_reader(reader, base_path)
    }

    pub fn load_from_reader<R: BufRead>(reader: R, base_path: &str) -> Result<Self, String> {
        let mut loader = Self::new();
        
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut texcoords = Vec::new();
        
        let mut current_shape = ObjShape::new(String::from("default"));
        let mut current_material_id: i32 = -1;
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error reading line: {}", e))?;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "v" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        positions.push(Vec3::new(x, y, z));
                    }
                }
                "vn" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        normals.push(Vec3::new(x, y, z));
                    }
                }
                "vt" => {
                    if parts.len() >= 3 {
                        let u = parts[1].parse::<f32>().unwrap_or(0.0);
                        let v = parts[2].parse::<f32>().unwrap_or(0.0);
                        texcoords.push(Vec2::new(u, v));
                    }
                }
                "f" => {
                    if parts.len() >= 4 {
                        loader.parse_face(
                            &parts[1..],
                            &positions,
                            &normals,
                            &texcoords,
                            &mut current_shape,
                            current_material_id,
                        );
                    }
                }
                "o" | "g" => {
                    if !current_shape.mesh.positions.is_empty() {
                        loader.shapes.push(current_shape.clone());
                    }
                    let name = parts.get(1).unwrap_or(&"unnamed").to_string();
                    current_shape = ObjShape::new(name);
                }
                "mtllib" => {
                    if parts.len() >= 2 {
                        let mtl_path = if base_path.is_empty() {
                            parts[1].to_string()
                        } else {
                            format!("{}/{}", base_path, parts[1])
                        };
                        let _ = loader.load_mtl(&mtl_path);
                    }
                }
                "usemtl" => {
                    if parts.len() >= 2 {
                        let material_name = parts[1];
                        current_material_id = loader
                            .material_map
                            .get(material_name)
                            .map(|&id| id as i32)
                            .unwrap_or(-1);
                    }
                }
                _ => {}
            }
        }
        
        if !current_shape.mesh.positions.is_empty() {
            loader.shapes.push(current_shape);
        }
        
        Ok(loader)
    }

    fn parse_face(
        &self,
        face_parts: &[&str],
        positions: &[Vec3],
        normals: &[Vec3],
        texcoords: &[Vec2],
        shape: &mut ObjShape,
        material_id: i32,
    ) {
        let mut indices = Vec::new();
        
        for part in face_parts {
            let components: Vec<&str> = part.split('/').collect();
            
            if !components.is_empty() {
                if let Ok(v_idx) = components[0].parse::<i32>() {
                    let idx = if v_idx < 0 {
                        (positions.len() as i32 + v_idx) as usize
                    } else {
                        (v_idx - 1) as usize
                    };
                    
                    if idx < positions.len() {
                        let pos = positions[idx];
                        shape.mesh.positions.push(pos.x);
                        shape.mesh.positions.push(pos.y);
                        shape.mesh.positions.push(pos.z);
                    }
                }
                
                if components.len() > 1 && !components[1].is_empty() {
                    if let Ok(vt_idx) = components[1].parse::<i32>() {
                        let idx = if vt_idx < 0 {
                            (texcoords.len() as i32 + vt_idx) as usize
                        } else {
                            (vt_idx - 1) as usize
                        };
                        
                        if idx < texcoords.len() {
                            let tc = texcoords[idx];
                            shape.mesh.texcoords.push(tc.x);
                            shape.mesh.texcoords.push(tc.y);
                        }
                    }
                }
                
                if components.len() > 2 && !components[2].is_empty() {
                    if let Ok(vn_idx) = components[2].parse::<i32>() {
                        let idx = if vn_idx < 0 {
                            (normals.len() as i32 + vn_idx) as usize
                        } else {
                            (vn_idx - 1) as usize
                        };
                        
                        if idx < normals.len() {
                            let n = normals[idx];
                            shape.mesh.normals.push(n.x);
                            shape.mesh.normals.push(n.y);
                            shape.mesh.normals.push(n.z);
                        }
                    }
                }
                
                indices.push((shape.mesh.positions.len() / 3 - 1) as u32);
            }
        }
        
        if indices.len() >= 3 {
            for i in 1..indices.len() - 1 {
                shape.mesh.indices.push(indices[0]);
                shape.mesh.indices.push(indices[i]);
                shape.mesh.indices.push(indices[i + 1]);
                shape.mesh.material_ids.push(material_id);
                shape.mesh.material_ids.push(material_id);
                shape.mesh.material_ids.push(material_id);
            }
        }
    }

    fn load_mtl(&mut self, path: &str) -> Result<(), String> {
        let file = File::open(path).map_err(|e| format!("Failed to open MTL file: {}", e))?;
        let reader = BufReader::new(file);
        
        let mut current_material: Option<ObjMaterial> = None;
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("Error reading MTL line: {}", e))?;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            match parts[0] {
                "newmtl" => {
                    if let Some(mat) = current_material.take() {
                        self.material_map.insert(mat.name.clone(), self.materials.len());
                        self.materials.push(mat);
                    }
                    if parts.len() >= 2 {
                        current_material = Some(ObjMaterial::new(parts[1].to_string()));
                    }
                }
                "Ka" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.ambient[0] = parts[1].parse().unwrap_or(0.0);
                            mat.ambient[1] = parts[2].parse().unwrap_or(0.0);
                            mat.ambient[2] = parts[3].parse().unwrap_or(0.0);
                        }
                    }
                }
                "Kd" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.diffuse[0] = parts[1].parse().unwrap_or(0.0);
                            mat.diffuse[1] = parts[2].parse().unwrap_or(0.0);
                            mat.diffuse[2] = parts[3].parse().unwrap_or(0.0);
                        }
                    }
                }
                "Ks" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.specular[0] = parts[1].parse().unwrap_or(0.0);
                            mat.specular[1] = parts[2].parse().unwrap_or(0.0);
                            mat.specular[2] = parts[3].parse().unwrap_or(0.0);
                        }
                    }
                }
                "Ns" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            mat.shininess = parts[1].parse().unwrap_or(0.0);
                        }
                    }
                }
                "Ni" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            mat.ior = parts[1].parse().unwrap_or(1.0);
                        }
                    }
                }
                "d" | "Tr" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            let value = parts[1].parse().unwrap_or(1.0);
                            mat.dissolve = if parts[0] == "Tr" { 1.0 - value } else { value };
                        }
                    }
                }
                "illum" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            mat.illum = parts[1].parse().unwrap_or(1);
                        }
                    }
                }
                "map_Ka" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            mat.ambient_texture = parts[1].to_string();
                        }
                    }
                }
                "map_Kd" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            mat.diffuse_texture = parts[1].to_string();
                        }
                    }
                }
                "map_Ks" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2 {
                            mat.specular_texture = parts[1].to_string();
                        }
                    }
                }
                "map_Bump" | "bump" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 2
 {
                            mat.normal_texture = parts[1].to_string();
                        }
                    }
                }
                _ => {}
            }
        }
        
        if let Some(mat) = current_material {
            self.material_map.insert(mat.name.clone(), self.materials.len());
            self.materials.push(mat);
        }
        
        Ok(())
    }

    pub fn get_shapes(&self) -> &[ObjShape] {
        &self.shapes
    }

    pub fn get_materials(&self) -> &[ObjMaterial] {
        &self.materials
    }

    pub fn shape_count(&self) -> usize {
        self.shapes.len()
    }

    pub fn material_count(&self) -> usize {
        self.materials.len()
    }
}

impl Default for ObjLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_material_creation() {
        let mat = ObjMaterial::new("test_material".to_string());
        assert_eq!(mat.name, "test_material");
        assert_eq!(mat.dissolve, 1.0);
        assert_eq!(mat.ior, 1.0);
    }

    #[test]
    fn test_obj_mesh_creation() {
        let mesh = ObjMesh::new();
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.index_count(), 0);
    }

    #[test]
    fn test_obj_shape_creation() {
        let shape = ObjShape::new("test_shape".to_string());
        assert_eq!(shape.name, "test_shape");
        assert_eq!(shape.mesh.vertex_count(), 0);
    }

    #[test]
    fn test_obj_loader_creation() {
        let loader = ObjLoader::new();
        assert_eq!(loader.shape_count(), 0);
        assert_eq!(loader.material_count(), 0);
    }

    #[test]
    fn test_obj_mesh_vertex_count() {
        let mut mesh = ObjMesh::new();
        mesh.positions = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert_eq!(mesh.vertex_count(), 2);
    }

    #[test]
    fn test_obj_mesh_index_count() {
        let mut mesh = ObjMesh::new();
        mesh.indices = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(mesh.index_count(), 6);
    }

    #[test]
    fn test_material_default_values() {
        let mat = ObjMaterial::new("default".to_string());
        assert_eq!(mat.ambient, [0.2, 0.2, 0.2]);
        assert_eq!(mat.diffuse, [0.8, 0.8, 0.8]);
        assert_eq!(mat.specular, [1.0, 1.0, 1.0]);
        assert_eq!(mat.shininess, 0.0);
        assert_eq!(mat.illum, 1);
    }

    #[test]
    fn test_obj_loader_default() {
        let loader = ObjLoader::default();
        assert_eq!(loader.shape_count(), 0);
        assert_eq!(loader.material_count(), 0);
    }
}
