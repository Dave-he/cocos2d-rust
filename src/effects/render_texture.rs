/// RenderTexture - 离屏渲染目标
///
/// 对应 cocos2d-x 的 `RenderTexture`，可将场景渲染到纹理，
/// 再将该纹理用于后处理（模糊、描边、灰度等）或保存为图片。

use crate::math::{Vec2, geometry::Size};
use crate::base::types::{Color4F, Color4B};

// ─── 像素格式 ────────────────────────────────────────────────────

/// 离屏帧缓冲的像素格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTextureFormat {
    /// RGBA 8-bit，最通用
    RGBA8,
    /// RGB 8-bit（无透明通道）
    RGB8,
    /// 单通道（灰度）
    R8,
    /// 16-bit 浮点（HDR 准备）
    RGBA16F,
    /// 带深度缓冲（用于 3D 效果）
    RGBA8_Depth,
}

impl Default for RenderTextureFormat {
    fn default() -> Self {
        Self::RGBA8
    }
}

// ─── RenderTexture ──────────────────────────────────────────────

/// 离屏渲染目标
///
/// 用法：
/// 1. `begin()` — 切换渲染目标到该纹理
/// 2. 正常绘制节点
/// 3. `end()` — 恢复到屏幕渲染目标
/// 4. 将 `get_texture()` 返回的纹理用于 Sprite
#[derive(Debug, Clone)]
pub struct OffscreenTarget {
    width: u32,
    height: u32,
    format: RenderTextureFormat,
    /// 像素数据（模拟，在实际 GPU 渲染中为 FBO 句柄）
    pixels: Vec<u8>,
    /// 是否处于渲染状态
    in_render: bool,
    /// 清除颜色
    clear_color: Color4F,
    /// 累计渲染次数
    render_count: u32,
    /// 是否已初始化
    initialized: bool,
}

impl OffscreenTarget {
    /// 创建 RenderTexture
    pub fn create(width: u32, height: u32) -> Self {
        Self::create_with_format(width, height, RenderTextureFormat::RGBA8)
    }

    /// 创建指定格式的 RenderTexture
    pub fn create_with_format(width: u32, height: u32, format: RenderTextureFormat) -> Self {
        let bytes_per_pixel = match format {
            RenderTextureFormat::R8 => 1,
            RenderTextureFormat::RGB8 => 3,
            RenderTextureFormat::RGBA8 | RenderTextureFormat::RGBA8_Depth => 4,
            RenderTextureFormat::RGBA16F => 8,
        };
        let pixel_count = (width * height) as usize * bytes_per_pixel;

        Self {
            width,
            height,
            format,
            pixels: vec![0u8; pixel_count],
            in_render: false,
            clear_color: Color4F::new(0.0, 0.0, 0.0, 0.0),
            render_count: 0,
            initialized: true,
        }
    }

    // ── 生命周期 ──────────────────────────────────────────────

    /// 开始渲染到此纹理
    pub fn begin(&mut self) {
        debug_assert!(!self.in_render, "RenderTexture: begin() called while already rendering");
        self.in_render = true;
        self.render_count += 1;
    }

    /// 开始渲染并清除背景
    pub fn begin_with_clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color = Color4F::new(r, g, b, a);
        self.begin();
        self.clear();
    }

    /// 结束渲染
    pub fn end(&mut self) {
        debug_assert!(self.in_render, "RenderTexture: end() called without begin()");
        self.in_render = false;
    }

    /// 清除纹理内容（填充 clear_color）
    pub fn clear(&mut self) {
        let bytes_per_pixel = self.bytes_per_pixel();
        let r = (self.clear_color.r * 255.0) as u8;
        let g = (self.clear_color.g * 255.0) as u8;
        let b = (self.clear_color.b * 255.0) as u8;
        let a = (self.clear_color.a * 255.0) as u8;

        for chunk in self.pixels.chunks_mut(bytes_per_pixel) {
            if bytes_per_pixel >= 4 {
                chunk[0] = r;
                chunk[1] = g;
                chunk[2] = b;
                chunk[3] = a;
            } else if bytes_per_pixel == 3 {
                chunk[0] = r;
                chunk[1] = g;
                chunk[2] = b;
            } else {
                chunk[0] = r;
            }
        }
    }

    // ── 属性 ──────────────────────────────────────────────────

    pub fn get_width(&self) -> u32 { self.width }
    pub fn get_height(&self) -> u32 { self.height }
    pub fn get_size(&self) -> Size { Size::new(self.width as f32, self.height as f32) }
    pub fn get_format(&self) -> RenderTextureFormat { self.format }
    pub fn is_in_render(&self) -> bool { self.in_render }
    pub fn get_render_count(&self) -> u32 { self.render_count }
    pub fn is_initialized(&self) -> bool { self.initialized }

    pub fn set_clear_color(&mut self, color: Color4F) { self.clear_color = color; }
    pub fn get_clear_color(&self) -> Color4F { self.clear_color }

    pub fn get_pixels(&self) -> &[u8] { &self.pixels }
    pub fn get_pixels_mut(&mut self) -> &mut Vec<u8> { &mut self.pixels }

    /// 读取指定像素（x, y）的颜色
    pub fn get_pixel_at(&self, x: u32, y: u32) -> Option<Color4B> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let bpp = self.bytes_per_pixel();
        let idx = ((y * self.width + x) as usize) * bpp;
        if bpp >= 4 {
            Some(Color4B {
                r: self.pixels[idx],
                g: self.pixels[idx + 1],
                b: self.pixels[idx + 2],
                a: self.pixels[idx + 3],
            })
        } else if bpp == 3 {
            Some(Color4B {
                r: self.pixels[idx],
                g: self.pixels[idx + 1],
                b: self.pixels[idx + 2],
                a: 255,
            })
        } else {
            Some(Color4B { r: self.pixels[idx], g: self.pixels[idx], b: self.pixels[idx], a: 255 })
        }
    }

    /// 写入指定像素的颜色
    pub fn set_pixel_at(&mut self, x: u32, y: u32, color: Color4B) {
        if x >= self.width || y >= self.height { return; }
        let bpp = self.bytes_per_pixel();
        let idx = ((y * self.width + x) as usize) * bpp;
        if bpp >= 4 {
            self.pixels[idx]     = color.r;
            self.pixels[idx + 1] = color.g;
            self.pixels[idx + 2] = color.b;
            self.pixels[idx + 3] = color.a;
        } else if bpp == 3 {
            self.pixels[idx]     = color.r;
            self.pixels[idx + 1] = color.g;
            self.pixels[idx + 2] = color.b;
        } else {
            self.pixels[idx] = color.r;
        }
    }

    fn bytes_per_pixel(&self) -> usize {
        match self.format {
            RenderTextureFormat::R8 => 1,
            RenderTextureFormat::RGB8 => 3,
            RenderTextureFormat::RGBA8 | RenderTextureFormat::RGBA8_Depth => 4,
            RenderTextureFormat::RGBA16F => 8,
        }
    }
}

// ─── 后处理特效 ──────────────────────────────────────────────────

/// 将 RenderTexture 转换为灰度图像
pub fn apply_grayscale(rt: &mut OffscreenTarget) {
    let bpp = match rt.get_format() {
        RenderTextureFormat::RGBA8 | RenderTextureFormat::RGBA8_Depth => 4,
        RenderTextureFormat::RGB8 => 3,
        _ => return,
    };
    let pixels = rt.get_pixels_mut();
    for chunk in pixels.chunks_mut(bpp) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        // BT.601 luminance
        let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
        chunk[0] = gray;
        chunk[1] = gray;
        chunk[2] = gray;
    }
}

/// 对 RenderTexture 应用亮度/对比度调整
pub fn apply_brightness_contrast(rt: &mut OffscreenTarget, brightness: f32, contrast: f32) {
    let bpp = match rt.get_format() {
        RenderTextureFormat::RGBA8 | RenderTextureFormat::RGBA8_Depth => 4,
        RenderTextureFormat::RGB8 => 3,
        _ => return,
    };
    let pixels = rt.get_pixels_mut();
    for chunk in pixels.chunks_mut(bpp) {
        for c in chunk.iter_mut().take(3) {
            let mut v = *c as f32 / 255.0;
            // 对比度
            v = (v - 0.5) * contrast + 0.5;
            // 亮度
            v += brightness;
            v = v.clamp(0.0, 1.0);
            *c = (v * 255.0) as u8;
        }
    }
}

/// 水平方向简单箱式模糊（半径 radius 像素）
pub fn apply_box_blur_h(rt: &mut OffscreenTarget, radius: u32) {
    if rt.get_format() != RenderTextureFormat::RGBA8 { return; }
    let w = rt.get_width() as usize;
    let h = rt.get_height() as usize;
    let r = radius as usize;
    let src = rt.get_pixels().to_vec();
    let dst = rt.get_pixels_mut();

    for y in 0..h {
        for x in 0..w {
            let mut sum = [0u32; 4];
            let mut cnt = 0u32;
            let x_start = x.saturating_sub(r);
            let x_end = (x + r + 1).min(w);
            for sx in x_start..x_end {
                let idx = (y * w + sx) * 4;
                sum[0] += src[idx] as u32;
                sum[1] += src[idx + 1] as u32;
                sum[2] += src[idx + 2] as u32;
                sum[3] += src[idx + 3] as u32;
                cnt += 1;
            }
            let idx = (y * w + x) * 4;
            dst[idx]     = (sum[0] / cnt) as u8;
            dst[idx + 1] = (sum[1] / cnt) as u8;
            dst[idx + 2] = (sum[2] / cnt) as u8;
            dst[idx + 3] = (sum[3] / cnt) as u8;
        }
    }
}

/// 垂直方向简单箱式模糊
pub fn apply_box_blur_v(rt: &mut OffscreenTarget, radius: u32) {
    if rt.get_format() != RenderTextureFormat::RGBA8 { return; }
    let w = rt.get_width() as usize;
    let h = rt.get_height() as usize;
    let r = radius as usize;
    let src = rt.get_pixels().to_vec();
    let dst = rt.get_pixels_mut();

    for y in 0..h {
        for x in 0..w {
            let mut sum = [0u32; 4];
            let mut cnt = 0u32;
            let y_start = y.saturating_sub(r);
            let y_end = (y + r + 1).min(h);
            for sy in y_start..y_end {
                let idx = (sy * w + x) * 4;
                sum[0] += src[idx] as u32;
                sum[1] += src[idx + 1] as u32;
                sum[2] += src[idx + 2] as u32;
                sum[3] += src[idx + 3] as u32;
                cnt += 1;
            }
            let idx = (y * w + x) * 4;
            dst[idx]     = (sum[0] / cnt) as u8;
            dst[idx + 1] = (sum[1] / cnt) as u8;
            dst[idx + 2] = (sum[2] / cnt) as u8;
            dst[idx + 3] = (sum[3] / cnt) as u8;
        }
    }
}

/// 全屏模糊（先水平再垂直）
pub fn apply_blur(rt: &mut OffscreenTarget, radius: u32) {
    apply_box_blur_h(rt, radius);
    apply_box_blur_v(rt, radius);
}

/// 简单 Bloom 高光增强（提取亮区，添加光晕）
pub fn apply_bloom(rt: &mut OffscreenTarget, threshold: f32, intensity: f32) {
    if rt.get_format() != RenderTextureFormat::RGBA8 { return; }
    let w = rt.get_width() as usize;
    let h = rt.get_height() as usize;
    let src = rt.get_pixels().to_vec();

    // 先提取亮区
    let mut bright = vec![0u8; w * h * 4];
    for (i, chunk) in src.chunks(4).enumerate() {
        let r = chunk[0] as f32 / 255.0;
        let g = chunk[1] as f32 / 255.0;
        let b = chunk[2] as f32 / 255.0;
        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        if luminance > threshold {
            let idx = i * 4;
            bright[idx]     = chunk[0];
            bright[idx + 1] = chunk[1];
            bright[idx + 2] = chunk[2];
            bright[idx + 3] = chunk[3];
        }
    }

    // 对亮区模糊
    let mut bright_rt = OffscreenTarget::create(w as u32, h as u32);
    bright_rt.get_pixels_mut().copy_from_slice(&bright);
    apply_blur(&mut bright_rt, 3);

    // 叠加到原图
    let bloom_pixels = bright_rt.get_pixels().to_vec();
    let dst = rt.get_pixels_mut();
    for (i, chunk) in dst.chunks_mut(4).enumerate() {
        let bi = i * 4;
        let br = bloom_pixels[bi] as f32 * intensity;
        let bg = bloom_pixels[bi + 1] as f32 * intensity;
        let bb = bloom_pixels[bi + 2] as f32 * intensity;

        chunk[0] = (chunk[0] as f32 + br).min(255.0) as u8;
        chunk[1] = (chunk[1] as f32 + bg).min(255.0) as u8;
        chunk[2] = (chunk[2] as f32 + bb).min(255.0) as u8;
    }
}

/// 色调旋转（Hue Shift）
pub fn apply_hue_shift(rt: &mut OffscreenTarget, degrees: f32) {
    let bpp = match rt.get_format() {
        RenderTextureFormat::RGBA8 | RenderTextureFormat::RGBA8_Depth => 4,
        RenderTextureFormat::RGB8 => 3,
        _ => return,
    };
    let cos_h = degrees.to_radians().cos();
    let sin_h = degrees.to_radians().sin();

    // 色调旋转矩阵（简化版）
    let matrix = [
        [0.787 + cos_h * 0.213 - sin_h * 0.213, 0.787 - cos_h * 0.787 + sin_h * 0.143, 0.787 - cos_h * 0.787 - sin_h * 0.928],
        [0.715 - cos_h * 0.715 - sin_h * 0.715, 0.715 + cos_h * 0.285 + sin_h * 0.140, 0.715 - cos_h * 0.715 + sin_h * 0.140],
        [0.072 - cos_h * 0.072 + sin_h * 0.928, 0.072 - cos_h * 0.072 - sin_h * 0.283, 0.072 + cos_h * 0.928 + sin_h * 0.072],
    ];

    let pixels = rt.get_pixels_mut();
    for chunk in pixels.chunks_mut(bpp) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        chunk[0] = (matrix[0][0] * r + matrix[0][1] * g + matrix[0][2] * b).clamp(0.0, 255.0) as u8;
        chunk[1] = (matrix[1][0] * r + matrix[1][1] * g + matrix[1][2] * b).clamp(0.0, 255.0) as u8;
        chunk[2] = (matrix[2][0] * r + matrix[2][1] * g + matrix[2][2] * b).clamp(0.0, 255.0) as u8;
    }
}

/// 色彩反转（Invert）
pub fn apply_invert(rt: &mut OffscreenTarget) {
    let bpp = match rt.get_format() {
        RenderTextureFormat::RGBA8 | RenderTextureFormat::RGBA8_Depth => 4,
        RenderTextureFormat::RGB8 => 3,
        _ => return,
    };
    let pixels = rt.get_pixels_mut();
    for chunk in pixels.chunks_mut(bpp) {
        chunk[0] = 255 - chunk[0];
        chunk[1] = 255 - chunk[1];
        chunk[2] = 255 - chunk[2];
    }
}

/// 色调映射（Vignette 暗角效果）
pub fn apply_vignette(rt: &mut OffscreenTarget, strength: f32) {
    if rt.get_format() != RenderTextureFormat::RGBA8 { return; }
    let w = rt.get_width() as f32;
    let h = rt.get_height() as f32;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let max_dist = (cx * cx + cy * cy).sqrt();
    let width = rt.get_width();
    let height = rt.get_height(); // 提前读取避免借用冲突

    let pixels = rt.get_pixels_mut();
    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt() / max_dist;
            let factor = (1.0 - dist * strength).max(0.0);
            let idx = ((y * width + x) * 4) as usize;
            pixels[idx]     = (pixels[idx] as f32 * factor) as u8;
            pixels[idx + 1] = (pixels[idx + 1] as f32 * factor) as u8;
            pixels[idx + 2] = (pixels[idx + 2] as f32 * factor) as u8;
        }
    }
}

// ─── 后处理流水线 ─────────────────────────────────────────────

/// 后处理效果类型
#[derive(Debug, Clone)]
pub enum PostProcessEffect {
    Grayscale,
    Invert,
    Blur { radius: u32 },
    Bloom { threshold: f32, intensity: f32 },
    BrightnessContrast { brightness: f32, contrast: f32 },
    HueShift { degrees: f32 },
    Vignette { strength: f32 },
}

/// 后处理流水线 — 按顺序应用多个效果
#[derive(Debug, Default)]
pub struct PostProcessPipeline {
    effects: Vec<PostProcessEffect>,
    enabled: bool,
}

impl PostProcessPipeline {
    pub fn new() -> Self {
        Self { effects: Vec::new(), enabled: true }
    }

    /// 添加一个效果到流水线末尾
    pub fn add_effect(&mut self, effect: PostProcessEffect) -> &mut Self {
        self.effects.push(effect);
        self
    }

    /// 移除所有效果
    pub fn clear_effects(&mut self) {
        self.effects.clear();
    }

    pub fn get_effect_count(&self) -> usize { self.effects.len() }
    pub fn is_enabled(&self) -> bool { self.enabled }
    pub fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }

    /// 对 RenderTexture 应用流水线中的所有效果
    pub fn apply(&self, rt: &mut OffscreenTarget) {
        if !self.enabled { return; }
        for effect in &self.effects {
            match effect {
                PostProcessEffect::Grayscale => apply_grayscale(rt),
                PostProcessEffect::Invert => apply_invert(rt),
                PostProcessEffect::Blur { radius } => apply_blur(rt, *radius),
                PostProcessEffect::Bloom { threshold, intensity } => {
                    apply_bloom(rt, *threshold, *intensity);
                }
                PostProcessEffect::BrightnessContrast { brightness, contrast } => {
                    apply_brightness_contrast(rt, *brightness, *contrast);
                }
                PostProcessEffect::HueShift { degrees } => apply_hue_shift(rt, *degrees),
                PostProcessEffect::Vignette { strength } => apply_vignette(rt, *strength),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_texture_create() {
        let rt = OffscreenTarget::create(800, 600);
        assert_eq!(rt.get_width(), 800);
        assert_eq!(rt.get_height(), 600);
        assert!(!rt.is_in_render());
        assert!(rt.is_initialized());
        assert_eq!(rt.get_format(), RenderTextureFormat::RGBA8);
    }

    #[test]
    fn test_render_texture_begin_end() {
        let mut rt = OffscreenTarget::create(100, 100);
        assert!(!rt.is_in_render());

        rt.begin();
        assert!(rt.is_in_render());
        assert_eq!(rt.get_render_count(), 1);

        rt.end();
        assert!(!rt.is_in_render());
    }

    #[test]
    fn test_render_texture_begin_with_clear() {
        let mut rt = OffscreenTarget::create(10, 10);
        rt.begin_with_clear(1.0, 0.0, 0.0, 1.0);
        rt.end();

        // 应该填充为红色
        let pixel = rt.get_pixel_at(5, 5).unwrap();
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
        assert_eq!(pixel.a, 255);
    }

    #[test]
    fn test_render_texture_pixel_rw() {
        let mut rt = OffscreenTarget::create(4, 4);
        let color = Color4B { r: 128, g: 64, b: 255, a: 200 };
        rt.set_pixel_at(2, 3, color);
        let read_back = rt.get_pixel_at(2, 3).unwrap();
        assert_eq!(read_back.r, 128);
        assert_eq!(read_back.g, 64);
        assert_eq!(read_back.b, 255);
        assert_eq!(read_back.a, 200);
    }

    #[test]
    fn test_render_texture_pixel_out_of_bounds() {
        let rt = OffscreenTarget::create(10, 10);
        assert!(rt.get_pixel_at(10, 10).is_none());
        assert!(rt.get_pixel_at(9, 9).is_some());
    }

    #[test]
    fn test_render_texture_formats() {
        let r8 = OffscreenTarget::create_with_format(4, 4, RenderTextureFormat::R8);
        assert_eq!(r8.get_pixels().len(), 16); // 4*4*1

        let rgb = OffscreenTarget::create_with_format(4, 4, RenderTextureFormat::RGB8);
        assert_eq!(rgb.get_pixels().len(), 48); // 4*4*3

        let rgba = OffscreenTarget::create_with_format(4, 4, RenderTextureFormat::RGBA8);
        assert_eq!(rgba.get_pixels().len(), 64); // 4*4*4
    }

    #[test]
    fn test_apply_grayscale() {
        let mut rt = OffscreenTarget::create(2, 1);
        // 设置红色像素
        rt.set_pixel_at(0, 0, Color4B { r: 255, g: 0, b: 0, a: 255 });
        rt.set_pixel_at(1, 0, Color4B { r: 0, g: 255, b: 0, a: 255 });

        apply_grayscale(&mut rt);

        let p0 = rt.get_pixel_at(0, 0).unwrap();
        let p1 = rt.get_pixel_at(1, 0).unwrap();

        // 灰度后 R=G=B
        assert_eq!(p0.r, p0.g);
        assert_eq!(p0.g, p0.b);
        assert_eq!(p1.r, p1.g);
        assert_eq!(p1.g, p1.b);
    }

    #[test]
    fn test_apply_invert() {
        let mut rt = OffscreenTarget::create(1, 1);
        rt.set_pixel_at(0, 0, Color4B { r: 100, g: 150, b: 200, a: 255 });
        apply_invert(&mut rt);
        let p = rt.get_pixel_at(0, 0).unwrap();
        assert_eq!(p.r, 155);
        assert_eq!(p.g, 105);
        assert_eq!(p.b, 55);
    }

    #[test]
    fn test_apply_brightness_contrast() {
        let mut rt = OffscreenTarget::create(1, 1);
        // 中灰像素
        rt.set_pixel_at(0, 0, Color4B { r: 128, g: 128, b: 128, a: 255 });

        // 增加亮度
        apply_brightness_contrast(&mut rt, 0.2, 1.0);
        let p = rt.get_pixel_at(0, 0).unwrap();
        // 变亮了
        assert!(p.r > 128);
    }

    #[test]
    fn test_apply_blur() {
        let mut rt = OffscreenTarget::create(5, 5);
        // 中心像素设为白色
        rt.set_pixel_at(2, 2, Color4B { r: 255, g: 255, b: 255, a: 255 });
        apply_blur(&mut rt, 1);
        // 模糊后周围像素应该有颜色
        let center = rt.get_pixel_at(2, 2).unwrap();
        let neighbor = rt.get_pixel_at(2, 3).unwrap();
        // 中心颜色降低，周围颜色提升
        assert!(center.r < 255 || neighbor.r > 0);
    }

    #[test]
    fn test_post_process_pipeline_grayscale() {
        let mut pipeline = PostProcessPipeline::new();
        pipeline.add_effect(PostProcessEffect::Grayscale);
        assert_eq!(pipeline.get_effect_count(), 1);

        let mut rt = OffscreenTarget::create(2, 2);
        rt.set_pixel_at(0, 0, Color4B { r: 255, g: 0, b: 0, a: 255 });
        pipeline.apply(&mut rt);

        let p = rt.get_pixel_at(0, 0).unwrap();
        assert_eq!(p.r, p.g);
    }

    #[test]
    fn test_post_process_pipeline_multiple() {
        let mut pipeline = PostProcessPipeline::new();
        pipeline
            .add_effect(PostProcessEffect::Grayscale)
            .add_effect(PostProcessEffect::Invert);
        assert_eq!(pipeline.get_effect_count(), 2);

        let mut rt = OffscreenTarget::create(1, 1);
        rt.set_pixel_at(0, 0, Color4B { r: 200, g: 100, b: 50, a: 255 });
        pipeline.apply(&mut rt);

        // 先灰度后反转：最终颜色可预期
        let p = rt.get_pixel_at(0, 0).unwrap();
        assert_eq!(p.r, p.g);
        assert_eq!(p.g, p.b);
    }

    #[test]
    fn test_post_process_pipeline_disabled() {
        let mut pipeline = PostProcessPipeline::new();
        pipeline.add_effect(PostProcessEffect::Grayscale);
        pipeline.set_enabled(false);

        let mut rt = OffscreenTarget::create(1, 1);
        rt.set_pixel_at(0, 0, Color4B { r: 255, g: 0, b: 0, a: 255 });
        pipeline.apply(&mut rt);

        // 禁用时不应用效果
        let p = rt.get_pixel_at(0, 0).unwrap();
        assert_eq!(p.r, 255);
        assert_eq!(p.g, 0);
    }

    #[test]
    fn test_post_process_pipeline_clear() {
        let mut pipeline = PostProcessPipeline::new();
        pipeline.add_effect(PostProcessEffect::Grayscale);
        pipeline.add_effect(PostProcessEffect::Invert);
        assert_eq!(pipeline.get_effect_count(), 2);

        pipeline.clear_effects();
        assert_eq!(pipeline.get_effect_count(), 0);
    }

    #[test]
    fn test_render_texture_multiple_renders() {
        let mut rt = OffscreenTarget::create(10, 10);
        for _ in 0..5 {
            rt.begin();
            rt.end();
        }
        assert_eq!(rt.get_render_count(), 5);
    }
}
