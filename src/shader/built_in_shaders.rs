/// 内置着色器集合
/// 包含 Cocos2d 常用的标准着色器
pub struct BuiltInShaders;

impl BuiltInShaders {
    /// Position Color 着色器（顶点颜色）
    pub const POSITION_COLOR_VERT: &'static str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPosition;
        layout (location = 1) in vec4 aColor;
        
        uniform mat4 uMVPMatrix;
        
        out vec4 vColor;
        
        void main() {
            gl_Position = uMVPMatrix * vec4(aPosition, 1.0);
            vColor = aColor;
        }
    "#;

    pub const POSITION_COLOR_FRAG: &'static str = r#"
        #version 330 core
        in vec4 vColor;
        out vec4 FragColor;
        
        void main() {
            FragColor = vColor;
        }
    "#;

    /// Position Texture 着色器（纹理）
    pub const POSITION_TEXTURE_VERT: &'static str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPosition;
        layout (location = 1) in vec2 aTexCoord;
        
        uniform mat4 uMVPMatrix;
        
        out vec2 vTexCoord;
        
        void main() {
            gl_Position = uMVPMatrix * vec4(aPosition, 1.0);
            vTexCoord = aTexCoord;
        }
    "#;

    pub const POSITION_TEXTURE_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        
        void main() {
            FragColor = texture(uTexture, vTexCoord);
        }
    "#;

    /// Position Texture Color 着色器（纹理 + 颜色混合）
    pub const POSITION_TEXTURE_COLOR_VERT: &'static str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPosition;
        layout (location = 1) in vec2 aTexCoord;
        layout (location = 2) in vec4 aColor;
        
        uniform mat4 uMVPMatrix;
        
        out vec2 vTexCoord;
        out vec4 vColor;
        
        void main() {
            gl_Position = uMVPMatrix * vec4(aPosition, 1.0);
            vTexCoord = aTexCoord;
            vColor = aColor;
        }
    "#;

    pub const POSITION_TEXTURE_COLOR_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        in vec4 vColor;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        
        void main() {
            FragColor = texture(uTexture, vTexCoord) * vColor;
        }
    "#;

    /// Position Texture Color Alpha Test 着色器（带 Alpha 测试）
    pub const POSITION_TEXTURE_ALPHA_TEST_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        in vec4 vColor;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        uniform float uAlphaTest;
        
        void main() {
            vec4 color = texture(uTexture, vTexCoord) * vColor;
            if (color.a < uAlphaTest) {
                discard;
            }
            FragColor = color;
        }
    "#;

    /// Label 着色器（文本渲染）
    pub const LABEL_VERT: &'static str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPosition;
        layout (location = 1) in vec2 aTexCoord;
        layout (location = 2) in vec4 aColor;
        
        uniform mat4 uMVPMatrix;
        
        out vec2 vTexCoord;
        out vec4 vColor;
        
        void main() {
            gl_Position = uMVPMatrix * vec4(aPosition, 1.0);
            vTexCoord = aTexCoord;
            vColor = aColor;
        }
    "#;

    pub const LABEL_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        in vec4 vColor;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        
        void main() {
            float alpha = texture(uTexture, vTexCoord).r;
            FragColor = vec4(vColor.rgb, vColor.a * alpha);
        }
    "#;

    /// Gray Scale 着色器（灰度效果）
    pub const GRAY_SCALE_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        
        void main() {
            vec4 color = texture(uTexture, vTexCoord);
            float gray = dot(color.rgb, vec3(0.299, 0.587, 0.114));
            FragColor = vec4(gray, gray, gray, color.a);
        }
    "#;

    /// Sepia 着色器（褐色效果）
    pub const SEPIA_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        
        void main() {
            vec4 color = texture(uTexture, vTexCoord);
            vec3 sepia;
            sepia.r = dot(color.rgb, vec3(0.393, 0.769, 0.189));
            sepia.g = dot(color.rgb, vec3(0.349, 0.686, 0.168));
            sepia.b = dot(color.rgb, vec3(0.272, 0.534, 0.131));
            FragColor = vec4(sepia, color.a);
        }
    "#;

    /// Blur 着色器（模糊效果）
    pub const BLUR_FRAG: &'static str = r#"
        #version 330 core
        in vec2 vTexCoord;
        out vec4 FragColor;
        
        uniform sampler2D uTexture;
        uniform vec2 uResolution;
        uniform float uBlurRadius;
        
        void main() {
            vec2 pixelSize = 1.0 / uResolution;
            vec4 color = vec4(0.0);
            
            for (float x = -uBlurRadius; x <= uBlurRadius; x++) {
                for (float y = -uBlurRadius; y <= uBlurRadius; y++) {
                    vec2 offset = vec2(x, y) * pixelSize;
                    color += texture(uTexture, vTexCoord + offset);
                }
            }
            
            float samples = (uBlurRadius * 2.0 + 1.0) * (uBlurRadius * 2.0 + 1.0);
            FragColor = color / samples;
        }
    "#;

    /// 获取所有内置着色器名称
    pub fn shader_names() -> Vec<&'static str> {
        vec![
            "position_color",
            "position_texture",
            "position_texture_color",
            "position_texture_alpha_test",
            "label",
            "gray_scale",
            "sepia",
            "blur",
        ]
    }

    /// 获取着色器源码
    pub fn get_shader_source(name: &str) -> Option<(&'static str, &'static str)> {
        match name {
            "position_color" => Some((
                Self::POSITION_COLOR_VERT,
                Self::POSITION_COLOR_FRAG,
            )),
            "position_texture" => Some((
                Self::POSITION_TEXTURE_VERT,
                Self::POSITION_TEXTURE_FRAG,
            )),
            "position_texture_color" => Some((
                Self::POSITION_TEXTURE_COLOR_VERT,
                Self::POSITION_TEXTURE_COLOR_FRAG,
            )),
            "position_texture_alpha_test" => Some((
                Self::POSITION_TEXTURE_COLOR_VERT,
                Self::POSITION_TEXTURE_ALPHA_TEST_FRAG,
            )),
            "label" => Some((
                Self::LABEL_VERT,
                Self::LABEL_FRAG,
            )),
            "gray_scale" => Some((
                Self::POSITION_TEXTURE_VERT,
                Self::GRAY_SCALE_FRAG,
            )),
            "sepia" => Some((
                Self::POSITION_TEXTURE_VERT,
                Self::SEPIA_FRAG,
            )),
            "blur" => Some((
                Self::POSITION_TEXTURE_VERT,
                Self::BLUR_FRAG,
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_names() {
        let names = BuiltInShaders::shader_names();
        assert!(names.len() > 0);
        assert!(names.contains(&"position_color"));
        assert!(names.contains(&"position_texture"));
    }

    #[test]
    fn test_get_shader_source() {
        let source = BuiltInShaders::get_shader_source("position_color");
        assert!(source.is_some());
        
        let (vert, frag) = source.unwrap();
        assert!(!vert.is_empty());
        assert!(!frag.is_empty());
        assert!(vert.contains("aPosition"));
        assert!(frag.contains("vColor"));
    }

    #[test]
    fn test_get_nonexistent_shader() {
        let source = BuiltInShaders::get_shader_source("nonexistent");
        assert!(source.is_none());
    }

    #[test]
    fn test_all_shaders_exist() {
        for name in BuiltInShaders::shader_names() {
            let source = BuiltInShaders::get_shader_source(name);
            assert!(source.is_some(), "Shader {} not found", name);
        }
    }
}
