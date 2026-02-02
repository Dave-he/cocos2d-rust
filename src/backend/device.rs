use glow::HasContext;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    VERTEX,
    INDEX,
}

impl BufferType {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            BufferType::VERTEX => glow::ARRAY_BUFFER,
            BufferType::INDEX => glow::ELEMENT_ARRAY_BUFFER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    STATIC,
    DYNAMIC,
}

impl BufferUsage {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            BufferUsage::STATIC => glow::STATIC_DRAW,
            BufferUsage::DYNAMIC => glow::DYNAMIC_DRAW,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BufferObject {
    id: u32,
    buffer_type: BufferType,
    size: usize,
    usage: BufferUsage,
    native: Option<glow::NativeBuffer>,
}

impl BufferObject {
    pub fn new() -> BufferObject {
        BufferObject {
            id: 0,
            buffer_type: BufferType::VERTEX,
            size: 0,
            usage: BufferUsage::STATIC,
            native: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_native(&self) -> Option<glow::NativeBuffer> {
        self.native
    }
}

// #[derive(Debug)]
pub struct GraphicsDevice {
    pub context: Rc<glow::Context>,
    capabilities: DeviceCapabilities,
    profiles: HashMap<String, DeviceProfile>,
    programs: HashMap<u32, ShaderProgram>,
    shaders: HashMap<u32, ShaderObject>, // Individual shader objects (Vertex/Fragment)
    buffers: HashMap<u32, BufferObject>,
    textures: HashMap<u32, TextureObject>,
    framebuffers: HashMap<u32, FramebufferObject>,
    next_id: u32,
}

impl std::fmt::Debug for GraphicsDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphicsDevice")
            .field("capabilities", &self.capabilities)
            .field("profiles", &self.profiles)
            .finish()
    }
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

#[derive(Debug, Clone)]
pub struct ShaderObject {
    id: u32,
    shader_type: u32, // glow::VERTEX_SHADER or glow::FRAGMENT_SHADER
    native: Option<glow::NativeShader>,
}

impl ShaderObject {
    pub fn get_native(&self) -> Option<glow::NativeShader> {
        self.native
    }
}

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    id: u32,
    vertex_shader: u32,
    fragment_shader: u32,
    uniforms: HashMap<String, i32>,
    native_uniforms: HashMap<i32, glow::NativeUniformLocation>,
    native: Option<glow::NativeProgram>,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        ShaderProgram {
            id: 0,
            vertex_shader: 0,
            fragment_shader: 0,
            uniforms: HashMap::new(),
            native_uniforms: HashMap::new(),
            native: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<i32> {
        self.uniforms.get(name).cloned()
    }

    pub fn get_native(&self) -> Option<glow::NativeProgram> {
        self.native
    }
}

#[derive(Debug, Clone)]
pub struct TextureObject {
    id: u32,
    target: u32,
    width: u32,
    height: u32,
    format: u32,
    native: Option<glow::NativeTexture>,
}

impl TextureObject {
    pub fn new() -> TextureObject {
        TextureObject {
            id: 0,
            target: 0,
            width: 0,
            height: 0,
            format: 0,
            native: None,
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

    pub fn get_native(&self) -> Option<glow::NativeTexture> {
        self.native
    }
}

#[derive(Debug, Clone)]
pub struct FramebufferObject {
    id: u32,
    color_buffer: u32,
    depth_buffer: u32,
    stencil_buffer: u32,
    width: u32,
    height: u32,
    native: Option<glow::NativeFramebuffer>,
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
            native: None,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_native(&self) -> Option<glow::NativeFramebuffer> {
        self.native
    }
}

impl GraphicsDevice {
    pub fn new(context: Rc<glow::Context>) -> GraphicsDevice {
        GraphicsDevice {
            context,
            capabilities: DeviceCapabilities::new(),
            profiles: HashMap::new(),
            programs: HashMap::new(),
            shaders: HashMap::new(),
            buffers: HashMap::new(),
            textures: HashMap::new(),
            framebuffers: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn get_capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    pub fn create_program(&mut self) -> ShaderProgram {
        let native = unsafe { self.context.create_program().ok() };
        let id = self.next_id;
        self.next_id += 1;

        let program = ShaderProgram {
            id,
            vertex_shader: 0,
            fragment_shader: 0,
            uniforms: HashMap::new(),
            native_uniforms: HashMap::new(),
            native,
        };
        self.programs.insert(id, program.clone());
        program
    }

    pub fn create_shader(&mut self, shader_type: u32) -> u32 {
        let native = unsafe { self.context.create_shader(shader_type).ok() };
        let id = self.next_id;
        self.next_id += 1;

        let shader = ShaderObject {
            id,
            shader_type,
            native,
        };
        self.shaders.insert(id, shader);
        id
    }

    pub fn create_buffer(&mut self) -> BufferObject {
        let native = unsafe { self.context.create_buffer().ok() };
        let id = self.next_id;
        self.next_id += 1;

        let buffer = BufferObject {
            id,
            buffer_type: BufferType::VERTEX,
            size: 0,
            usage: BufferUsage::STATIC,
            native,
        };
        self.buffers.insert(id, buffer.clone());
        buffer
    }

    pub fn create_texture(&mut self) -> TextureObject {
        let native = unsafe { self.context.create_texture().ok() };
        let id = self.next_id;
        self.next_id += 1;

        let texture = TextureObject {
            id,
            target: glow::TEXTURE_2D,
            width: 0,
            height: 0,
            format: glow::RGBA,
            native,
        };
        self.textures.insert(id, texture.clone());
        texture
    }

    pub fn create_framebuffer(&mut self) -> FramebufferObject {
        let native = unsafe { self.context.create_framebuffer().ok() };
        let id = self.next_id;
        self.next_id += 1;

        let framebuffer = FramebufferObject {
            id,
            color_buffer: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            width: 0,
            height: 0,
            native,
        };
        self.framebuffers.insert(id, framebuffer.clone());
        framebuffer
    }

    pub fn delete_program(&mut self, program_id: u32) {
        if let Some(program) = self.programs.remove(&program_id) {
            if let Some(native) = program.native {
                unsafe { self.context.delete_program(native) };
            }
        }
    }

    pub fn delete_shader(&mut self, shader_id: u32) {
        if let Some(shader) = self.shaders.remove(&shader_id) {
            if let Some(native) = shader.native {
                unsafe { self.context.delete_shader(native) };
            }
        }
    }

    pub fn delete_buffer(&mut self, buffer: &mut BufferObject) {
        if let Some(buf) = self.buffers.remove(&buffer.id) {
            if let Some(native) = buf.native {
                unsafe { self.context.delete_buffer(native) };
            }
        }
        buffer.id = 0;
    }

    pub fn delete_texture(&mut self, texture: &mut TextureObject) {
        if let Some(tex) = self.textures.remove(&texture.id) {
            if let Some(native) = tex.native {
                unsafe { self.context.delete_texture(native) };
            }
        }
        texture.id = 0;
    }

    pub fn delete_framebuffer(&mut self, framebuffer: &mut FramebufferObject) {
        if let Some(fb) = self.framebuffers.remove(&framebuffer.id) {
            if let Some(native) = fb.native {
                unsafe { self.context.delete_framebuffer(native) };
            }
        }
        framebuffer.id = 0;
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: u32, height: u32) {
        unsafe { self.context.viewport(x, y, width as i32, height as i32) };
    }

    pub fn set_scissor_test(&self, enabled: bool) {
        unsafe {
            if enabled {
                self.context.enable(glow::SCISSOR_TEST);
            } else {
                self.context.disable(glow::SCISSOR_TEST);
            }
        }
    }

    pub fn clear(&self, mask: u32) {
        unsafe { self.context.clear(mask) };
    }

    pub fn get_error(&self) -> u32 {
        unsafe { self.context.get_error() }
    }

    pub fn get_program(&self, id: u32) -> Option<&ShaderProgram> {
        self.programs.get(&id)
    }

    pub fn get_program_mut(&mut self, id: u32) -> Option<&mut ShaderProgram> {
        self.programs.get_mut(&id)
    }

    pub fn get_shader(&self, id: u32) -> Option<&ShaderObject> {
        self.shaders.get(&id)
    }

    pub fn get_buffer(&self, id: u32) -> Option<&BufferObject> {
        self.buffers.get(&id)
    }

    pub fn get_texture(&self, id: u32) -> Option<&TextureObject> {
        self.textures.get(&id)
    }

    pub fn get_framebuffer(&self, id: u32) -> Option<&FramebufferObject> {
        self.framebuffers.get(&id)
    }

    pub fn get_native_uniform(
        &self,
        program_id: u32,
        location: i32,
    ) -> Option<glow::NativeUniformLocation> {
        if let Some(shader) = self.programs.get(&program_id) {
            shader.native_uniforms.get(&location).cloned()
        } else {
            None
        }
    }

    pub fn cache_uniforms(&mut self, program_id: u32) {
        if let Some(shader) = self.programs.get_mut(&program_id) {
            if let Some(native_program) = shader.native {
                unsafe {
                    let count = self.context.get_active_uniforms(native_program);
                    for i in 0..count {
                        if let Some(info) = self.context.get_active_uniform(native_program, i) {
                            if let Some(location) = self
                                .context
                                .get_uniform_location(native_program, &info.name)
                            {
                                let id = i as i32;
                                shader.uniforms.insert(info.name, id);
                                shader.native_uniforms.insert(id, location);
                            }
                        }
                    }
                }
            }
        }
    }
}
