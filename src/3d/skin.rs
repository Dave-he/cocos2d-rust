use crate::base::Ref;
use crate::math::{Vec3, Mat4, Quaternion};

#[derive(Debug)]
pub struct Bone3D {
    name: String,
    inverse_bind_pose: Mat4,
    local_pose: Mat4,
    global_pose: Mat4,
    position: Vec3,
    rotation: Quaternion,
    scale: Vec3,
    parent: Option<Ref<Bone3D>>,
    children: Vec<Ref<Bone3D>>,
}

impl Bone3D {
    pub fn new(name: &str) -> Bone3D {
        Bone3D {
            name: name.to_string(),
            inverse_bind_pose: Mat4::identity(),
            local_pose: Mat4::identity(),
            global_pose: Mat4::identity(),
            position: Vec3::ZERO,
            rotation: Quaternion::identity(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_inverse_bind_pose(&self) -> &Mat4 {
        &self.inverse_bind_pose
    }

    pub fn set_inverse_bind_pose(&mut self, matrix: Mat4) {
        self.inverse_bind_pose = matrix;
    }

    pub fn get_local_pose(&self) -> &Mat4 {
        &self.local_pose
    }

    pub fn get_global_pose(&self) -> &Mat4 {
        &self.global_pose
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> Quaternion {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
    }

    pub fn get_scale(&self) -> Vec3 {
        self.scale
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub fn add_child(&mut self, child: Ref<Bone3D>) {
        self.children.push(child);
    }

    pub fn get_children(&self) -> &Vec<Ref<Bone3D>> {
        &self.children
    }
}

#[derive(Debug)]
pub struct Skeleton3D {
    bones: Vec<Ref<Bone3D>>,
    bone_index_by_name: std::collections::HashMap<String, usize>,
    root_bones: Vec<Ref<Bone3D>>,
}

impl Skeleton3D {
    pub fn new() -> Skeleton3D {
        Skeleton3D {
            bones: Vec::new(),
            bone_index_by_name: std::collections::HashMap::new(),
            root_bones: Vec::new(),
        }
    }

    pub fn add_bone(&mut self, bone: Ref<Bone3D>) {
        let index = self.bones.len();
        self.bones.push(bone.clone());
        self.bone_index_by_name.insert(bone.get_name().to_string(), index);
    }

    pub fn get_bones(&self) -> &Vec<Ref<Bone3D>> {
        &self.bones
    }

    pub fn get_bone_by_name(&self, name: &str) -> Option<&Ref<Bone3D>> {
        if let Some(&index) = self.bone_index_by_name.get(name) {
            self.bones.get(index)
        } else {
            None
        }
    }

    pub fn get_root_bones(&self) -> &Vec<Ref<Bone3D>> {
        &self.root_bones
    }
}

#[derive(Debug)]
pub struct Skin {
    mesh: Ref<()>,
    skeleton: Option<Ref<Skeleton3D>>,
}

impl Skin {
    pub fn new() -> Skin {
        Skin {
            mesh: Ref::new(()),
            skeleton: None,
        }
    }

    pub fn get_skeleton(&self) -> Option<&Ref<Skeleton3D>> {
        self.skeleton.as_ref()
    }

    pub fn set_skeleton(&mut self, skeleton: Ref<Skeleton3D>) {
        self.skeleton = Some(skeleton);
    }
}
