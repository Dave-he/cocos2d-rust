use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    VERTEX,
    INDEX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    STATIC,
    DYNAMIC,
}

#[derive(Debug)]
pub struct BufferObject {
    id: u32,
    buffer_type: BufferType,
    size: usize,
    usage: BufferUsage,
}

impl BufferObject {
    pub fn new() -> BufferObject {
        BufferObject {
            id: 0,
            buffer_type: BufferType::VERTEX,
            size: 0,
            usage: BufferUsage::STATIC,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct GraphicsDevice {
    capabilities: DeviceCapabilities,
    profiles: HashMap<String, DeviceProfile>,
    shaders: HashMap<u32, ShaderProgram>,
    buffers: HashMap<u32, BufferObject>,
    textures: HashMap<u32, TextureObject>,
    framebuffers: HashMap<u32, FramebufferObject>,
}

#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    max_texture_size: u32,
    max_texture_units: u32,
    max_vertex_attribs: u32,
    max_vertex_uniform_vectors: u32,
    max_fragment_uniform_vectors: u32,
    max_varying_vectors: u32,
    supports_s3tc: bool,
    supports_etc1: bool,
    supports_pvrtc: bool,
    supports_bgra8888: bool,
    supports_dxt: bool,
    supports_atc: bool,
}

impl DeviceCapabilities {
    pub fn new() -> DeviceCapabilities {
        DeviceCapabilities {
            max_texture_size: 4096,
            max_texture_units: 8,
            max_vertex_attribs: 16,
            max_vertex_uniform_vectors: 256,
            max_fragment_uniform_vectors: 224,
            max_varying_vectors: 16,
            supports_s3tc: false,
            supports_etc1: false,
            supports_pvrtc: false,
            supports_bgra8888: false,
            supports_dxt: false,
            supports_atc: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceProfile {
    name: String,
    max_texture_size: u32,
}

impl DeviceProfile {
    pub fn new(name: &str) -> DeviceProfile {
        DeviceProfile {
            name: name.to_string(),
            max_texture_size: 4096,
        }
    }
}

#[derive(Debug)]
pub struct ShaderProgram {
    id: u32,
    vertex_shader: u32,
    fragment_shader: u32,
    uniforms: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram {
            id: 0,
            vertex_shader: 0,
            fragment_shader: 0,
            uniforms: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        self.uniforms.get(name).cloned()
    }
}

#[derive(Debug)]
pub struct TextureObject {
    id: u32,
    target: u32,
    width: u32,
    height: u32,
    format: u32,
}

impl TextureObject {
    pub fn new() -> TextureObject {
        TextureObject {
            id: 0,
            target: 0,
            width: 0,
            height: 0,
            format: 0,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

#[derive(Debug)]
pub struct FramebufferObject {
    id: u32,
    color_buffer: u32,
    depth_buffer: u32,
    stencil_buffer: u32,
    width: u32,
    height: u32,
}

impl FramebufferObject {
    pub fn new() -> FramebufferObject {
        FramebufferObject {
            id: 0,
            color_buffer: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

impl GraphicsDevice {
    pub fn new() -> GraphicsDevice {
        GraphicsDevice {
            capabilities: DeviceCapabilities::new(),
            profiles: HashMap::new(),
            shaders: HashMap::new(),
            buffers: HashMap::new(),
            textures: HashMap::new(),
            framebuffers: HashMap::new(),
        }
    }

    pub fn get_capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    pub fn create_shader(&mut self) -> ShaderProgram {
        ShaderProgram::new()
    }

    pub fn create_buffer(&mut self) -> BufferObject {
        BufferObject::new()
    }

    pub fn create_texture(&mut self) -> TextureObject {
        TextureObject::new()
    }

    pub fn create_framebuffer(&mut self) -> FramebufferObject {
        FramebufferObject::new()
    }

    pub fn delete_shader(&mut self, program: &mut ShaderProgram) {
        program.id = 0;
    }

    pub fn delete_buffer(&mut self, buffer: &mut BufferObject) {
        buffer.id = 0;
    }

    pub fn delete_texture(&mut self, texture: &mut TextureObject) {
        texture.id = 0;
    }

    pub fn delete_framebuffer(&mut self, framebuffer: &mut FramebufferObject) {
        framebuffer.id = 0;
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32) {
    }

    pub fn set_scissor_test(&self, enabled: bool) {
    }

    pub fn clear(&self, mask: u32) {
    }

    pub fn get_error(&self) -> u32 {
        0
    }
}
