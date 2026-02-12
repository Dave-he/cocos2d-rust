/// 统一的错误处理系统
///
/// 基于 Cocos2d-x 问题分析，提供：
/// - 强类型错误处理（Result<T, E>）
/// - 丰富的错误上下文
/// - 错误链追踪
/// - 自定义错误类型
///
/// 解决 Cocos2d-x 的问题：
/// - NULL 指针错误 → Option<T>
/// - init() 永远返回 true → Result<T, E>
/// - 错误信息丢失 → Error trait 完整上下文

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::path::PathBuf;

/// 资源错误类型
#[derive(Debug)]
pub enum ResourceError {
    /// 文件未找到
    FileNotFound {
        path: PathBuf,
        reason: String,
    },
    /// 无效的文件格式
    InvalidFormat {
        path: PathBuf,
        expected: String,
        actual: String,
    },
    /// 加载失败
    LoadFailed {
        resource_type: String,
        path: PathBuf,
        source: Box<dyn StdError + Send + Sync>,
    },
    /// 解析错误
    ParseError {
        path: PathBuf,
        line: usize,
        message: String,
    },
    /// 内存不足
    OutOfMemory {
        requested: usize,
        available: usize,
    },
    /// IO 错误
    IoError(io::Error),
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceError::FileNotFound { path, reason } => {
                write!(f, "File not found: {:?}, reason: {}", path, reason)
            }
            ResourceError::InvalidFormat {
                path,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Invalid format for {:?}: expected {}, got {}",
                    path, expected, actual
                )
            }
            ResourceError::LoadFailed {
                resource_type,
                path,
                source,
            } => {
                write!(
                    f,
                    "Failed to load {} from {:?}: {}",
                    resource_type, path, source
                )
            }
            ResourceError::ParseError {
                path,
                line,
                message,
            } => {
                write!(f, "Parse error in {:?} at line {}: {}", path, line, message)
            }
            ResourceError::OutOfMemory {
                requested,
                available,
            } => {
                write!(
                    f,
                    "Out of memory: requested {} bytes, only {} available",
                    requested, available
                )
            }
            ResourceError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl StdError for ResourceError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            ResourceError::LoadFailed { source, .. } => Some(&**source as &dyn StdError),
            ResourceError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ResourceError {
    fn from(error: io::Error) -> Self {
        ResourceError::IoError(error)
    }
}

/// 渲染错误类型
#[derive(Debug)]
pub enum RenderError {
    /// 着色器编译错误
    ShaderCompileError {
        shader_type: String,
        source: String,
        error_log: String,
    },
    /// 着色器链接错误
    ShaderLinkError {
        program_id: u32,
        error_log: String,
    },
    /// 纹理创建错误
    TextureCreationError {
        width: u32,
        height: u32,
        format: String,
        reason: String,
    },
    /// 帧缓冲不完整
    FrameBufferIncomplete {
        fbo_id: u32,
        status: String,
    },
    /// OpenGL 错误
    OpenGLError {
        error_code: u32,
        function: String,
    },
    /// 不支持的功能
    UnsupportedFeature {
        feature: String,
        reason: String,
    },
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RenderError::ShaderCompileError {
                shader_type,
                source,
                error_log,
            } => {
                write!(
                    f,
                    "Shader compile error ({}):\nSource:\n{}\nError:\n{}",
                    shader_type, source, error_log
                )
            }
            RenderError::ShaderLinkError {
                program_id,
                error_log,
            } => {
                write!(
                    f,
                    "Shader link error (program {}): {}",
                    program_id, error_log
                )
            }
            RenderError::TextureCreationError {
                width,
                height,
                format,
                reason,
            } => {
                write!(
                    f,
                    "Texture creation failed ({}x{}, {}): {}",
                    width, height, format, reason
                )
            }
            RenderError::FrameBufferIncomplete { fbo_id, status } => {
                write!(f, "Framebuffer {} is incomplete: {}", fbo_id, status)
            }
            RenderError::OpenGLError {
                error_code,
                function,
            } => {
                write!(f, "OpenGL error 0x{:X} in {}", error_code, function)
            }
            RenderError::UnsupportedFeature { feature, reason } => {
                write!(f, "Unsupported feature '{}': {}", feature, reason)
            }
        }
    }
}

impl StdError for RenderError {}

/// 场景错误类型
#[derive(Debug)]
pub enum SceneError {
    /// 节点未找到
    NodeNotFound {
        node_id: usize,
        parent_id: Option<usize>,
    },
    /// 循环引用
    CircularReference {
        node_id: usize,
        parent_id: usize,
    },
    /// 场景未激活
    SceneNotActive {
        scene_name: String,
    },
    /// 无效的节点操作
    InvalidNodeOperation {
        operation: String,
        reason: String,
    },
}

impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SceneError::NodeNotFound { node_id, parent_id } => {
                if let Some(parent) = parent_id {
                    write!(f, "Node {} not found in parent {}", node_id, parent)
                } else {
                    write!(f, "Node {} not found", node_id)
                }
            }
            SceneError::CircularReference { node_id, parent_id } => {
                write!(
                    f,
                    "Circular reference detected: node {} → parent {}",
                    node_id, parent_id
                )
            }
            SceneError::SceneNotActive { scene_name } => {
                write!(f, "Scene '{}' is not active", scene_name)
            }
            SceneError::InvalidNodeOperation { operation, reason } => {
                write!(f, "Invalid node operation '{}': {}", operation, reason)
            }
        }
    }
}

impl StdError for SceneError {}

/// 物理引擎错误
#[derive(Debug)]
pub enum PhysicsError {
    /// 无效的形状参数
    InvalidShapeParameters {
        shape_type: String,
        parameters: String,
    },
    /// 物理世界未初始化
    WorldNotInitialized,
    /// 刚体未找到
    BodyNotFound {
        body_id: usize,
    },
    /// 约束创建失败
    ConstraintCreationFailed {
        constraint_type: String,
        reason: String,
    },
}

impl fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PhysicsError::InvalidShapeParameters {
                shape_type,
                parameters,
            } => {
                write!(
                    f,
                    "Invalid parameters for {} shape: {}",
                    shape_type, parameters
                )
            }
            PhysicsError::WorldNotInitialized => {
                write!(f, "Physics world is not initialized")
            }
            PhysicsError::BodyNotFound { body_id } => {
                write!(f, "Physics body {} not found", body_id)
            }
            PhysicsError::ConstraintCreationFailed {
                constraint_type,
                reason,
            } => {
                write!(
                    f,
                    "Failed to create {} constraint: {}",
                    constraint_type, reason
                )
            }
        }
    }
}

impl StdError for PhysicsError {}

/// 音频错误
#[derive(Debug)]
pub enum AudioError {
    /// 设备初始化失败
    DeviceInitFailed {
        reason: String,
    },
    /// 不支持的音频格式
    UnsupportedFormat {
        file_path: PathBuf,
        format: String,
    },
    /// 播放失败
    PlaybackFailed {
        audio_id: usize,
        reason: String,
    },
    /// 音频资源未找到
    AudioNotFound {
        audio_id: usize,
    },
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AudioError::DeviceInitFailed { reason } => {
                write!(f, "Audio device initialization failed: {}", reason)
            }
            AudioError::UnsupportedFormat { file_path, format } => {
                write!(
                    f,
                    "Unsupported audio format '{}' for file {:?}",
                    format, file_path
                )
            }
            AudioError::PlaybackFailed { audio_id, reason } => {
                write!(f, "Audio playback failed (id {}): {}", audio_id, reason)
            }
            AudioError::AudioNotFound { audio_id } => {
                write!(f, "Audio resource {} not found", audio_id)
            }
        }
    }
}

impl StdError for AudioError {}

/// 网络错误
#[derive(Debug)]
pub enum NetworkError {
    /// 连接失败
    ConnectionFailed {
        url: String,
        reason: String,
    },
    /// 超时
    Timeout {
        url: String,
        timeout_ms: u64,
    },
    /// HTTP 错误
    HttpError {
        url: String,
        status_code: u16,
        message: String,
    },
    /// 序列化/反序列化错误
    SerializationError {
        data_type: String,
        reason: String,
    },
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::ConnectionFailed { url, reason } => {
                write!(f, "Connection to '{}' failed: {}", url, reason)
            }
            NetworkError::Timeout { url, timeout_ms } => {
                write!(f, "Request to '{}' timed out after {}ms", url, timeout_ms)
            }
            NetworkError::HttpError {
                url,
                status_code,
                message,
            } => {
                write!(
                    f,
                    "HTTP {} error for '{}': {}",
                    status_code, url, message
                )
            }
            NetworkError::SerializationError { data_type, reason } => {
                write!(f, "Serialization error for '{}': {}", data_type, reason)
            }
        }
    }
}

impl StdError for NetworkError {}

/// 通用游戏引擎错误
#[derive(Debug)]
pub enum EngineError {
    /// 资源错误
    Resource(ResourceError),
    /// 渲染错误
    Render(RenderError),
    /// 场景错误
    Scene(SceneError),
    /// 物理错误
    Physics(PhysicsError),
    /// 音频错误
    Audio(AudioError),
    /// 网络错误
    Network(NetworkError),
    /// 其他错误
    Other(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineError::Resource(e) => write!(f, "Resource error: {}", e),
            EngineError::Render(e) => write!(f, "Render error: {}", e),
            EngineError::Scene(e) => write!(f, "Scene error: {}", e),
            EngineError::Physics(e) => write!(f, "Physics error: {}", e),
            EngineError::Audio(e) => write!(f, "Audio error: {}", e),
            EngineError::Network(e) => write!(f, "Network error: {}", e),
            EngineError::Other(s) => write!(f, "Error: {}", s),
        }
    }
}

impl StdError for EngineError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            EngineError::Resource(e) => Some(e),
            EngineError::Render(e) => Some(e),
            EngineError::Scene(e) => Some(e),
            EngineError::Physics(e) => Some(e),
            EngineError::Audio(e) => Some(e),
            EngineError::Network(e) => Some(e),
            EngineError::Other(_) => None,
        }
    }
}

impl From<ResourceError> for EngineError {
    fn from(error: ResourceError) -> Self {
        EngineError::Resource(error)
    }
}

impl From<RenderError> for EngineError {
    fn from(error: RenderError) -> Self {
        EngineError::Render(error)
    }
}

impl From<SceneError> for EngineError {
    fn from(error: SceneError) -> Self {
        EngineError::Scene(error)
    }
}

impl From<PhysicsError> for EngineError {
    fn from(error: PhysicsError) -> Self {
        EngineError::Physics(error)
    }
}

impl From<AudioError> for EngineError {
    fn from(error: AudioError) -> Self {
        EngineError::Audio(error)
    }
}

impl From<NetworkError> for EngineError {
    fn from(error: NetworkError) -> Self {
        EngineError::Network(error)
    }
}

impl From<io::Error> for EngineError {
    fn from(error: io::Error) -> Self {
        EngineError::Resource(ResourceError::IoError(error))
    }
}

/// Result 类型别名，用于引擎操作
pub type EngineResult<T> = Result<T, EngineError>;

/// Result 类型别名，用于资源操作
pub type ResourceResult<T> = Result<T, ResourceError>;

/// Result 类型别名，用于渲染操作
pub type RenderResult<T> = Result<T, RenderError>;

/// Result 类型别名，用于场景操作
pub type SceneResult<T> = Result<T, SceneError>;

/// Result 类型别名，用于物理操作
pub type PhysicsResult<T> = Result<T, PhysicsError>;

/// Result 类型别名，用于音频操作
pub type AudioResult<T> = Result<T, AudioError>;

/// Result 类型别名，用于网络操作
pub type NetworkResult<T> = Result<T, NetworkError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_error_file_not_found() {
        let error = ResourceError::FileNotFound {
            path: PathBuf::from("texture.png"),
            reason: "Path does not exist".to_string(),
        };

        assert!(error.to_string().contains("texture.png"));
        assert!(error.to_string().contains("Path does not exist"));
    }

    #[test]
    fn test_resource_error_invalid_format() {
        let error = ResourceError::InvalidFormat {
            path: PathBuf::from("image.bmp"),
            expected: "PNG".to_string(),
            actual: "BMP".to_string(),
        };

        assert!(error.to_string().contains("image.bmp"));
        assert!(error.to_string().contains("PNG"));
        assert!(error.to_string().contains("BMP"));
    }

    #[test]
    fn test_render_error_shader_compile() {
        let error = RenderError::ShaderCompileError {
            shader_type: "vertex".to_string(),
            source: "void main() {}".to_string(),
            error_log: "syntax error".to_string(),
        };

        assert!(error.to_string().contains("vertex"));
        assert!(error.to_string().contains("syntax error"));
    }

    #[test]
    fn test_scene_error_node_not_found() {
        let error = SceneError::NodeNotFound {
            node_id: 42,
            parent_id: Some(10),
        };

        assert!(error.to_string().contains("42"));
        assert!(error.to_string().contains("10"));
    }

    #[test]
    fn test_scene_error_circular_reference() {
        let error = SceneError::CircularReference {
            node_id: 1,
            parent_id: 1,
        };

        assert!(error.to_string().contains("Circular reference"));
    }

    #[test]
    fn test_physics_error_world_not_initialized() {
        let error = PhysicsError::WorldNotInitialized;
        assert!(error.to_string().contains("not initialized"));
    }

    #[test]
    fn test_audio_error_device_init_failed() {
        let error = AudioError::DeviceInitFailed {
            reason: "No audio output device found".to_string(),
        };

        assert!(error.to_string().contains("initialization failed"));
        assert!(error.to_string().contains("No audio output device"));
    }

    #[test]
    fn test_network_error_timeout() {
        let error = NetworkError::Timeout {
            url: "http://example.com".to_string(),
            timeout_ms: 5000,
        };

        assert!(error.to_string().contains("example.com"));
        assert!(error.to_string().contains("5000"));
    }

    #[test]
    fn test_engine_error_from_resource() {
        let resource_error = ResourceError::FileNotFound {
            path: PathBuf::from("test.txt"),
            reason: "Not found".to_string(),
        };

        let engine_error: EngineError = resource_error.into();
        assert!(matches!(engine_error, EngineError::Resource(_)));
    }

    #[test]
    fn test_engine_error_from_render() {
        let render_error = RenderError::ShaderCompileError {
            shader_type: "fragment".to_string(),
            source: "".to_string(),
            error_log: "Error".to_string(),
        };

        let engine_error: EngineError = render_error.into();
        assert!(matches!(engine_error, EngineError::Render(_)));
    }

    #[test]
    fn test_engine_error_source() {
        let resource_error = ResourceError::FileNotFound {
            path: PathBuf::from("test.txt"),
            reason: "Not found".to_string(),
        };

        let engine_error = EngineError::Resource(resource_error);
        assert!(engine_error.source().is_some());
    }

    #[test]
    fn test_resource_error_from_io() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let resource_error: ResourceError = io_error.into();

        assert!(matches!(resource_error, ResourceError::IoError(_)));
    }

    #[test]
    fn test_engine_error_from_io() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let engine_error: EngineError = io_error.into();

        assert!(matches!(engine_error, EngineError::Resource(_)));
    }

    #[test]
    fn test_error_display_formatting() {
        let error = ResourceError::OutOfMemory {
            requested: 1024 * 1024 * 100,
            available: 1024 * 1024 * 50,
        };

        let display = format!("{}", error);
        assert!(display.contains("Out of memory"));
        assert!(display.contains("104857600"));
        assert!(display.contains("52428800"));
    }

    #[test]
    fn test_parse_error() {
        let error = ResourceError::ParseError {
            path: PathBuf::from("config.json"),
            line: 42,
            message: "Unexpected token".to_string(),
        };

        assert!(error.to_string().contains("config.json"));
        assert!(error.to_string().contains("42"));
        assert!(error.to_string().contains("Unexpected token"));
    }

    #[test]
    fn test_opengl_error() {
        let error = RenderError::OpenGLError {
            error_code: 0x0500,
            function: "glDrawArrays".to_string(),
        };

        assert!(error.to_string().contains("0x500"));
        assert!(error.to_string().contains("glDrawArrays"));
    }

    #[test]
    fn test_http_error() {
        let error = NetworkError::HttpError {
            url: "http://api.example.com".to_string(),
            status_code: 404,
            message: "Not Found".to_string(),
        };

        assert!(error.to_string().contains("404"));
        assert!(error.to_string().contains("Not Found"));
    }

    #[test]
    fn test_result_type_alias() {
        let result: EngineResult<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);

        let result: ResourceResult<String> = Err(ResourceError::FileNotFound {
            path: PathBuf::from("test"),
            reason: "Test".to_string(),
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_error_chain() {
        use std::error::Error;

        let io_error = io::Error::new(io::ErrorKind::NotFound, "Original error");
        let resource_error = ResourceError::IoError(io_error);
        let engine_error = EngineError::Resource(resource_error);

        assert!(engine_error.source().is_some());
        if let Some(source) = engine_error.source() {
            assert!(source.source().is_some());
        }
    }
}
