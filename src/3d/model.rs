use crate::base::Ref;
use crate::math::{Vec3, Mat4};
use super::mesh::{Mesh, AABB, MeshSkin};

#[derive(Debug)]
pub struct Sprite3D {
    mesh: Option<Ref<Mesh>>,
    skin: Option<Ref<MeshSkin>>,
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

    pub fn get_mesh(&self) -> Option<&Ref<Mesh>> {
        self.mesh.as_ref()
    }

    pub fn set_mesh(&mut self, mesh: Ref<Mesh>) {
        self.mesh = Some(mesh);
    }

    pub fn get_skin(&self) -> Option<&Ref<MeshSkin>> {
        self.skin.as_ref()
    }

    pub fn set_skin(&mut self, skin: Ref<MeshSkin>) {
        self.skin = Some(skin);
    }

    pub fn get_aabb(&self) -> &AABB {
        &self.aabb
    }
}

#[derive(Debug)]
pub struct Model {
    meshes: Vec<Ref<Mesh>>,
    materials: Vec<Ref<()>>,
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

    pub fn add_mesh(&mut self, mesh: Ref<Mesh>) {
        self.meshes.push(mesh);
    }

    pub fn get_meshes(&self) -> &Vec<Ref<Mesh>> {
        &self.meshes
    }

    pub fn get_aabb(&self) -> &AABB {
        &self.aabb
    }
}
