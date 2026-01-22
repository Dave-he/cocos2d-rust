use crate::base::Node;
use crate::base::types::Color3B;
use crate::math::Vec2;
use crate::label::Label;
use crate::sprite::Sprite;
use std::collections::HashMap;

/// 富文本元素类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RichElementType {
    TEXT,
    IMAGE,
    CUSTOM_NODE,
}

/// 富文本元素
#[derive(Debug)]
pub struct RichElement {
    element_type: RichElementType,
    tag: String,
    color: Color3B,
    opacity: u8,
    text: String,
    font_name: String,
    font_size: f32,
    image_file: String,
    width: f32,
    height: f32,
    url: Option<String>,
}

impl RichElement {
    /// 创建文本元素
    pub fn create_text(tag: &str, color: Color3B, opacity: u8, text: &str, font_name: &str, font_size: f32) -> Self {
        RichElement {
            element_type: RichElementType::TEXT,
            tag: tag.to_string(),
            color,
            opacity,
            text: text.to_string(),
            font_name: font_name.to_string(),
            font_size,
            image_file: String::new(),
            width: 0.0,
            height: 0.0,
            url: None,
        }
    }
    
    /// 创建图片元素
    pub fn create_image(tag: &str, color: Color3B, opacity: u8, image_file: &str, width: f32, height: f32) -> Self {
        RichElement {
            element_type: RichElementType::IMAGE,
            tag: tag.to_string(),
            color,
            opacity,
            text: String::new(),
            font_name: String::new(),
            font_size: 0.0,
            image_file: image_file.to_string(),
            width,
            height,
            url: None,
        }
    }
    
    /// 设置 URL 链接
    pub fn set_url(&mut self, url: &str) {
        self.url = Some(url.to_string());
    }
    
    /// 获取元素类型
    pub fn get_type(&self) -> RichElementType {
        self.element_type.clone()
    }
}

/// 富文本元素渲染节点
#[derive(Debug)]
struct RichElementNode {
    node: Node,
    element: RichElement,
}

/// RichText 富文本组件
/// 
/// 支持多种富文本特性：
/// - HTML 标签解析（<font>、<img>、<a>、<b>、<i>、<u>）
/// - 文本和图片混排
/// - 超链接点击
/// - 自定义字体、颜色、大小
/// - 自动换行和对齐
#[derive(Debug)]
pub struct RichText {
    node: Node,
    elements: Vec<RichElement>,
    element_nodes: Vec<RichElementNode>,
    
    // 布局配置
    horizontal_space: f32,
    vertical_space: f32,
    max_width: f32,
    font_name: String,
    font_size: f32,
    font_color: Color3B,
    
    // 链接配置
    anchor_text_bold: bool,
    anchor_text_italic: bool,
    anchor_text_underline: bool,
    anchor_text_color: Color3B,
    anchor_text_shadow: bool,
    anchor_text_outline: bool,
    
    // 回调
    url_click_callback: Option<Box<dyn FnMut(&str)>>,
}

impl RichText {
    /// 创建富文本组件
    pub fn new() -> Self {
        RichText {
            node: Node::new(),
            elements: Vec::new(),
            element_nodes: Vec::new(),
            
            horizontal_space: 0.0,
            vertical_space: 0.0,
            max_width: 0.0,
            font_name: String::from("Arial"),
            font_size: 12.0,
            font_color: Color3B::WHITE,
            
            anchor_text_bold: false,
            anchor_text_italic: false,
            anchor_text_underline: true,
            anchor_text_color: Color3B::new(0, 0, 255), // 蓝色
            anchor_text_shadow: false,
            anchor_text_outline: false,
            
            url_click_callback: None,
        }
    }
    
    /// 创建富文本组件
    pub fn create() -> Self {
        Self::new()
    }
    
    /// 插入元素
    pub fn push_back_element(&mut self, element: RichElement) {
        self.elements.push(element);
        self.format_text();
    }
    
    /// 在指定位置插入元素
    pub fn insert_element(&mut self, element: RichElement, index: usize) {
        if index <= self.elements.len() {
            self.elements.insert(index, element);
            self.format_text();
        }
    }
    
    /// 移除指定位置的元素
    pub fn remove_element(&mut self, index: usize) {
        if index < self.elements.len() {
            self.elements.remove(index);
            self.format_text();
        }
    }
    
    /// 移除所有元素
    pub fn remove_all_elements(&mut self) {
        self.elements.clear();
        self.element_nodes.clear();
        self.format_text();
    }
    
    /// 设置默认字体名称
    pub fn set_font_name(&mut self, font_name: &str) {
        self.font_name = font_name.to_string();
    }
    
    /// 获取字体名称
    pub fn get_font_name(&self) -> &str {
        &self.font_name
    }
    
    /// 设置默认字体大小
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }
    
    /// 获取字体大小
    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }
    
    /// 设置默认字体颜色
    pub fn set_font_color(&mut self, color: Color3B) {
        self.font_color = color;
    }
    
    /// 获取字体颜色
    pub fn get_font_color(&self) -> Color3B {
        self.font_color
    }
    
    /// 设置水平间距
    pub fn set_horizontal_space(&mut self, space: f32) {
        self.horizontal_space = space;
        self.format_text();
    }
    
    /// 获取水平间距
    pub fn get_horizontal_space(&self) -> f32 {
        self.horizontal_space
    }
    
    /// 设置垂直间距
    pub fn set_vertical_space(&mut self, space: f32) {
        self.vertical_space = space;
        self.format_text();
    }
    
    /// 获取垂直间距
    pub fn get_vertical_space(&self) -> f32 {
        self.vertical_space
    }
    
    /// 设置最大宽度（用于自动换行）
    pub fn set_max_width(&mut self, width: f32) {
        self.max_width = width;
        self.format_text();
    }
    
    /// 获取最大宽度
    pub fn get_max_width(&self) -> f32 {
        self.max_width
    }
    
    /// 设置锚点文本是否加粗
    pub fn set_anchor_text_bold(&mut self, bold: bool) {
        self.anchor_text_bold = bold;
    }
    
    /// 设置锚点文本是否斜体
    pub fn set_anchor_text_italic(&mut self, italic: bool) {
        self.anchor_text_italic = italic;
    }
    
    /// 设置锚点文本是否下划线
    pub fn set_anchor_text_underline(&mut self, underline: bool) {
        self.anchor_text_underline = underline;
    }
    
    /// 设置锚点文本颜色
    pub fn set_anchor_text_color(&mut self, color: Color3B) {
        self.anchor_text_color = color;
    }
    
    /// 设置 URL 点击回调
    pub fn set_url_click_callback(&mut self, callback: Box<dyn FnMut(&str)>) {
        self.url_click_callback = Some(callback);
    }
    
    /// 解析并设置富文本字符串
    /// 支持的标签：
    /// - <font color="#RRGGBB" size="12" face="Arial">文本</font>
    /// - <img src="image.png" width="32" height="32"/>
    /// - <a href="http://example.com">链接</a>
    /// - <b>粗体</b>
    /// - <i>斜体</i>
    /// - <u>下划线</u>
    pub fn set_string(&mut self, text: &str) {
        self.elements.clear();
        self.parse_html(text);
        self.format_text();
    }
    
    /// 解析 HTML 标签
    fn parse_html(&mut self, text: &str) {
        // 简化的 HTML 解析器
        // 实际应用中应使用专业的 HTML 解析库
        
        let mut current_text = String::new();
        let mut current_font = self.font_name.clone();
        let mut current_size = self.font_size;
        let mut current_color = self.font_color;
        
        // 这里是简化版本，实际需要完整的标签解析
        if !text.contains('<') {
            // 纯文本
            let element = RichElement::create_text(
                "text",
                current_color,
                255,
                text,
                &current_font,
                current_size,
            );
            self.elements.push(element);
        } else {
            // 包含标签，需要解析
            // TODO: 实现完整的 HTML 标签解析
            let element = RichElement::create_text(
                "text",
                current_color,
                255,
                text,
                &current_font,
                current_size,
            );
            self.elements.push(element);
        }
    }
    
    /// 格式化文本布局
    fn format_text(&mut self) {
        // 清除旧的渲染节点
        self.element_nodes.clear();
        
        if self.elements.is_empty() {
            return;
        }
        
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut line_height = 0.0;
        
        for element in &self.elements {
            match element.element_type {
                RichElementType::TEXT => {
                    // 创建文本标签
                    let mut label = Label::create_with_ttf(
                        &element.text,
                        &element.font_name,
                        element.font_size,
                    );
                    label.set_text_color(element.color);
                    
                    let size = label.get_content_size();
                    
                    // 检查是否需要换行
                    if self.max_width > 0.0 && current_x + size.x > self.max_width {
                        current_x = 0.0;
                        current_y -= line_height + self.vertical_space;
                        line_height = 0.0;
                    }
                    
                    // 设置位置
                    label.get_node_mut().set_position(Vec2::new(current_x, current_y));
                    
                    current_x += size.x + self.horizontal_space;
                    line_height = line_height.max(size.y);
                }
                
                RichElementType::IMAGE => {
                    // 创建图片精灵
                    // let sprite = Sprite::create(&element.image_file);
                    // let size = Vec2::new(element.width, element.height);
                    
                    // 检查是否需要换行
                    if self.max_width > 0.0 && current_x + element.width > self.max_width {
                        current_x = 0.0;
                        current_y -= line_height + self.vertical_space;
                        line_height = 0.0;
                    }
                    
                    current_x += element.width + self.horizontal_space;
                    line_height = line_height.max(element.height);
                }
                
                RichElementType::CUSTOM_NODE => {
                    // 自定义节点处理
                }
            }
        }
    }
    
    /// 处理 URL 点击
    fn on_url_clicked(&mut self, url: &str) {
        if let Some(ref mut callback) = self.url_click_callback {
            callback(url);
        }
    }
    
    /// 获取节点
    pub fn get_node(&self) -> &Node {
        &self.node
    }
    
    /// 获取可变节点
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rich_text_creation() {
        let rich_text = RichText::new();
        assert_eq!(rich_text.get_font_name(), "Arial");
        assert_eq!(rich_text.get_font_size(), 12.0);
    }
    
    #[test]
    fn test_add_text_element() {
        let mut rich_text = RichText::new();
        let element = RichElement::create_text(
            "tag1",
            Color3B::WHITE,
            255,
            "Hello",
            "Arial",
            12.0,
        );
        rich_text.push_back_element(element);
        assert_eq!(rich_text.elements.len(), 1);
    }
    
    #[test]
    fn test_add_image_element() {
        let mut rich_text = RichText::new();
        let element = RichElement::create_image(
            "img1",
            Color3B::WHITE,
            255,
            "image.png",
            32.0,
            32.0,
        );
        rich_text.push_back_element(element);
        assert_eq!(rich_text.elements.len(), 1);
    }
    
    #[test]
    fn test_set_font() {
        let mut rich_text = RichText::new();
        rich_text.set_font_name("Helvetica");
        rich_text.set_font_size(16.0);
        assert_eq!(rich_text.get_font_name(), "Helvetica");
        assert_eq!(rich_text.get_font_size(), 16.0);
    }
    
    #[test]
    fn test_set_spacing() {
        let mut rich_text = RichText::new();
        rich_text.set_horizontal_space(5.0);
        rich_text.set_vertical_space(10.0);
        assert_eq!(rich_text.get_horizontal_space(), 5.0);
        assert_eq!(rich_text.get_vertical_space(), 10.0);
    }
    
    #[test]
    fn test_max_width() {
        let mut rich_text = RichText::new();
        rich_text.set_max_width(400.0);
        assert_eq!(rich_text.get_max_width(), 400.0);
    }
}
