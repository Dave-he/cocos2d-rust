use crate::base::{Ref, RefPtr};
use crate::math::{Vec3, Mat4};
use super::mesh::{Mesh, AABB, MeshSkin};

#[derive(Debug)]
pub struct Sprite3D {
    mesh: Option<RefPtr<Mesh>>,
    skin: Option<RefPtr<MeshSkin>>,
    aabb: AABB,
}

impl Sprite3D {
    pub fn new() -> Sprite3D {
        Sprite3D {
            mesh: None,
            skin: None,
            aabb: AABB::new(),
        }
    }

    pub fn create(file_name: &str) -> Option<Sprite3D> {
        let mut sprite = Sprite3D::new();
        sprite.init(file_name);
        Some(sprite)
    }

    pub fn init(&mut self, file_name: &str) {
    }

    pub fn get_mesh(&self) -> Option<&RefPtr<Mesh>> {
        self.mesh.as_ref()
    }

    pub fn set_mesh(&mut self, mesh: RefPtr<Mesh>) {
        self.mesh = Some(mesh);
    }

    pub fn get_skin(&self) -> Option<&RefPtr<MeshSkin>> {
        self.skin.as_ref()
    }

    pub fn set_skin(&mut self, skin: RefPtr<MeshSkin>) {
        self.skin = Some(skin);
    }

    pub fn get_aabb(&self) -> &AABB {
        &self.aabb
    }
}

#[derive(Debug)]
pub struct Model {
    meshes: Vec<RefPtr<Mesh>>,
    materials: Vec<RefPtr<()>>,
    aabb: AABB,
}

impl Model {
    pub fn new() -> Model {
        Model {
            meshes: Vec::new(),
            materials: Vec::new(),
            aabb: AABB::new(),
        }
    }

    pub fn add_mesh(&mut self, mesh: RefPtr<Mesh>) {
        self.meshes.push(mesh);
    }

    pub fn get_meshes(&self) -> &Vec<RefPtr<Mesh>> {
        &self.meshes
    }

    pub fn get_aabb(&self) -> &AABB {
        &self.aabb
    }
}
