use crate::math::{Vec3, Mat4};
use crate::renderer::renderer::ViewPort;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraProjection {
    PERSPECTIVE,
    ORTHOGRAPHIC,
}

#[derive(Debug)]
pub struct Camera {
    projection: CameraProjection,
    fov_y: f32,
    aspect_ratio: f32,
    near_clip: f32,
    far_clip: f32,
    view_port: ViewPort,
    position: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    view_projection_matrix: Mat4,
    depth: f32,
    rendering_order: i32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            projection: CameraProjection::PERSPECTIVE,
            fov_y: 45.0,
            aspect_ratio: 1.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            view_port: ViewPort::new(0.0, 0.0, 1.0, 1.0),
            position: Vec3::new(0.0, 0.0, 10.0),
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            right: Vec3::new(1.0, 0.0, 0.0),
            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
            depth: 0.0,
            rendering_order: 0,
        }
    }

    pub fn create_perspective(fov_y: f32, aspect_ratio: f32, near_clip: f32, far_clip: f32) -> Camera {
        let mut camera = Camera::new();
        camera.fov_y = fov_y;
        camera.aspect_ratio = aspect_ratio;
        camera.near_clip = near_clip;
        camera.far_clip = far_clip;
        camera
    }

    pub fn create_orthographic(width: f32, height: f32, near_clip: f32, far_clip: f32) -> Camera {
        let mut camera = Camera::new();
        camera.projection = CameraProjection::ORTHOGRAPHIC;
        camera.aspect_ratio = width / height;
        camera.near_clip = near_clip;
        camera.far_clip = far_clip;
        camera
    }

    pub fn set_projection(&mut self, projection: CameraProjection) {
        self.projection = projection;
    }

    pub fn get_projection(&self) -> CameraProjection {
        self.projection
    }

    pub fn set_fov_y(&mut self, fov_y: f32) {
        self.fov_y = fov_y;
    }

    pub fn get_fov_y(&self) -> f32 {
        self.fov_y
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn set_near_clip(&mut self, near_clip: f32) {
        self.near_clip = near_clip;
    }

    pub fn get_near_clip(&self) -> f32 {
        self.near_clip
    }

    pub fn set_far_clip(&mut self, far_clip: f32) {
        self.far_clip = far_clip;
    }

    pub fn get_far_clip(&self) -> f32 {
        self.far_clip
    }

    pub fn set_view_port(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.view_port = ViewPort::new(x, y, width, height);
    }

    pub fn get_view_port(&self) -> &ViewPort {
        &self.view_port
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        self.forward = (target - self.position).normalize();
        self.right = self.forward.cross(up).normalize();
        self.up = self.right.cross(self.forward).normalize();
        self.update_matrices();
    }

    pub fn get_forward(&self) -> Vec3 {
        self.forward
    }

    pub fn get_up(&self) -> Vec3 {
        self.up
    }

    pub fn get_right(&self) -> Vec3 {
        self.right
    }

    pub fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn get_projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }

    pub fn get_view_projection_matrix(&self) -> &Mat4 {
        &self.view_projection_matrix
    }

    pub fn set_depth(&mut self, depth: f32) {
        self.depth = depth;
    }

    pub fn get_depth(&self) -> f32 {
        self.depth
    }

    pub fn set_rendering_order(&mut self, order: i32) {
        self.rendering_order = order;
    }

    pub fn get_rendering_order(&self) -> i32 {
        self.rendering_order
    }

    fn update_matrices(&mut self) {
        // Update view matrix
        let (rx, ry, rz) = (self.right.x, self.right.y, self.right.z);
        let (ux, uy, uz) = (self.up.x, self.up.y, self.up.z);
        let (fx, fy, fz) = (self.forward.x, self.forward.y, self.forward.z);
        let (px, py, pz) = (self.position.x, self.position.y, self.position.z);

        self.view_matrix = Mat4::new(
            rx, ux, -fx, 0.0,
            ry, uy, -fy, 0.0,
            rz, uz, -fz, 0.0,
            -(rx * px + ry * py + rz * pz),
            -(ux * px + uy * py + uz * pz),
            fx * px + fy * py + fz * pz,
            1.0
        );

        // Update projection matrix
        if self.projection == CameraProjection::PERSPECTIVE {
            let f = 1.0 / (self.fov_y * 0.5).tan();
            self.projection_matrix = Mat4::new(
                f / self.aspect_ratio, 0.0, 0.0, 0.0,
                0.0, f, 0.0, 0.0,
                0.0, 0.0, (self.far_clip + self.near_clip) / (self.near_clip - self.far_clip), -1.0,
                0.0, 0.0, (2.0 * self.far_clip * self.near_clip) / (self.near_clip - self.far_clip), 0.0
            );
        } else {
            let width = self.view_port.get_width();
            let height = self.view_port.get_height();
            self.projection_matrix = Mat4::new(
                2.0 / width, 0.0, 0.0, 0.0,
                0.0, 2.0 / height, 0.0, 0.0,
                0.0, 0.0, -2.0 / (self.far_clip - self.near_clip), 0.0,
                -1.0, -1.0, -(self.far_clip + self.near_clip) / (self.far_clip - self.near_clip), 1.0
            );
        }

        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }
}
