use crate::base::{Ref, RefPtr};
use crate::ui::Widget;
use crate::input::{Touch, KeyCode, KeyboardEvent, KeyEventType};
use crate::math::Vec2;
use std::rc::Rc;
use std::cell::RefCell;

/// 文本对齐方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// 文本输入类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextInputType {
    /// 普通文本
    Text,
    /// 密码（显示为 * 或 •）
    Password,
    /// 数字
    Number,
    /// 邮箱
    Email,
}

/// 文本改变回调
pub type TextChangedCallback = Box<dyn FnMut(&TextField, &str)>;

/// 文本输入框组件
pub struct TextField {
    /// 基础 Widget
    widget: Widget,
    /// 文本内容
    text: String,
    /// 占位符文本
    placeholder: String,
    /// 最大长度（0 表示无限制）
    max_length: usize,
    /// 是否可编辑
    editable: bool,
    /// 是否正在编辑
    is_editing: bool,
    /// 光标位置
    cursor_position: usize,
    /// 选择起始位置
    selection_start: Option<usize>,
    /// 选择结束位置
    selection_end: Option<usize>,
    /// 文本对齐
    text_align: TextAlign,
    /// 输入类型
    input_type: TextInputType,
    /// 文本颜色
    text_color: [f32; 4],
    /// 占位符颜色
    placeholder_color: [f32; 4],
    /// 光标颜色
    cursor_color: [f32; 4],
    /// 选择背景颜色
    selection_color: [f32; 4],
    /// 文本改变回调
    on_text_changed: Option<TextChangedCallback>,
    /// 编辑开始回调
    on_editing_began: Option<Box<dyn FnMut(&TextField)>>,
    /// 编辑结束回调
    on_editing_ended: Option<Box<dyn FnMut(&TextField)>>,
}

impl TextField {
    /// 创建新文本输入框
    pub fn new() -> Self {
        Self {
            widget: Widget::new(),
            text: String::new(),
            placeholder: String::from("Enter text..."),
            max_length: 0,
            editable: true,
            is_editing: false,
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
            text_align: TextAlign::Left,
            input_type: TextInputType::Text,
            text_color: [0.0, 0.0, 0.0, 1.0],
            placeholder_color: [0.5, 0.5, 0.5, 1.0],
            cursor_color: [0.0, 0.0, 0.0, 1.0],
            selection_color: [0.5, 0.7, 1.0, 0.3],
            on_text_changed: None,
            on_editing_began: None,
            on_editing_ended: None,
        }
    }

    /// 设置文本
    pub fn set_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        
        // 检查最大长度
        let text_to_set = if self.max_length > 0 && new_text.len() > self.max_length {
            new_text[..self.max_length].to_string()
        } else {
            new_text
        };

        self.text = text_to_set.clone();
        self.cursor_position = self.text.len();
        self.clear_selection();

        // 触发回调
        if let Some(callback) = &mut self.on_text_changed {
            callback(self, &text_to_set);
        }
    }

    /// 获取文本
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 设置占位符
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder = placeholder.into();
    }

    /// 获取占位符
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    /// 设置最大长度
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
        
        // 截断现有文本
        if max_length > 0 && self.text.len() > max_length {
            self.text.truncate(max_length);
            self.cursor_position = self.cursor_position.min(max_length);
        }
    }

    /// 获取最大长度
    pub fn max_length(&self) -> usize {
        self.max_length
    }

    /// 设置可编辑性
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
        if !editable {
            self.end_editing();
        }
    }

    /// 是否可编辑
    pub fn is_editable(&self) -> bool {
        self.editable
    }

    /// 是否正在编辑
    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    /// 设置输入类型
    pub fn set_input_type(&mut self, input_type: TextInputType) {
        self.input_type = input_type;
    }

    /// 获取输入类型
    pub fn input_type(&self) -> TextInputType {
        self.input_type
    }

    /// 设置文本对齐
    pub fn set_text_align(&mut self, align: TextAlign) {
        self.text_align = align;
    }

    /// 获取文本对齐
    pub fn text_align(&self) -> TextAlign {
        self.text_align
    }

    /// 获取 Widget 引用
    pub fn widget(&self) -> &Widget {
        &self.widget
    }

    /// 获取 Widget 可变引用
    pub fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }

    /// 开始编辑
    pub fn begin_editing(&mut self) {
        if !self.editable || self.is_editing {
            return;
        }

        self.is_editing = true;
        
        if let Some(callback) = &mut self.on_editing_began {
            callback(self);
        }
    }

    /// 结束编辑
    pub fn end_editing(&mut self) {
        if !self.is_editing {
            return;
        }

        self.is_editing = false;
        self.clear_selection();
        
        if let Some(callback) = &mut self.on_editing_ended {
            callback(self);
        }
    }

    /// 插入文本
    pub fn insert_text(&mut self, text: &str) {
        if !self.is_editing || text.is_empty() {
            return;
        }

        // 如果有选择，先删除选择的文本
        self.delete_selection();

        // 检查长度限制
        let available_space = if self.max_length > 0 {
            self.max_length.saturating_sub(self.text.len())
        } else {
            usize::MAX
        };

        let text_to_insert = if text.len() > available_space {
            &text[..available_space]
        } else {
            text
        };

        // 插入文本
        self.text.insert_str(self.cursor_position, text_to_insert);
        self.cursor_position += text_to_insert.len();

        // 触发回调
        if let Some(callback) = &mut self.on_text_changed {
            callback(self, &self.text.clone());
        }
    }

    /// 删除字符（Backspace）
    pub fn delete_backward(&mut self) {
        if !self.is_editing {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        } else if self.cursor_position > 0 {
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;

            if let Some(callback) = &mut self.on_text_changed {
                callback(self, &self.text.clone());
            }
        }
    }

    /// 删除字符（Delete）
    pub fn delete_forward(&mut self) {
        if !self.is_editing {
            return;
        }

        if self.has_selection() {
            self.delete_selection();
        } else if self.cursor_position < self.text.len() {
            self.text.remove(self.cursor_position);

            if let Some(callback) = &mut self.on_text_changed {
                callback(self, &self.text.clone());
            }
        }
    }

    /// 移动光标
    pub fn move_cursor(&mut self, offset: isize) {
        let new_pos = (self.cursor_position as isize + offset).max(0) as usize;
        self.cursor_position = new_pos.min(self.text.len());
        self.clear_selection();
    }

    /// 移动光标到开始
    pub fn move_cursor_to_start(&mut self) {
        self.cursor_position = 0;
        self.clear_selection();
    }

    /// 移动光标到结束
    pub fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.text.len();
        self.clear_selection();
    }

    /// 选择所有文本
    pub fn select_all(&mut self) {
        if !self.text.is_empty() {
            self.selection_start = Some(0);
            self.selection_end = Some(self.text.len());
        }
    }

    /// 清除选择
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }

    /// 是否有选择
    pub fn has_selection(&self) -> bool {
        self.selection_start.is_some() && self.selection_end.is_some()
    }

    /// 删除选择的文本
    fn delete_selection(&mut self) {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (start, end) = if start < end {
                (start, end)
            } else {
                (end, start)
            };

            self.text.drain(start..end);
            self.cursor_position = start;
            self.clear_selection();

            if let Some(callback) = &mut self.on_text_changed {
                callback(self, &self.text.clone());
            }
        }
    }

    /// 处理键盘事件
    pub fn on_keyboard_event(&mut self, event: &KeyboardEvent) {
        if !self.is_editing {
            return;
        }

        match event.event_type {
            KeyEventType::Pressed | KeyEventType::Repeat => {
                match event.key_code {
                    KeyCode::Backspace => self.delete_backward(),
                    KeyCode::Delete => self.delete_forward(),
                    KeyCode::Left => self.move_cursor(-1),
                    KeyCode::Right => self.move_cursor(1),
                    KeyCode::Home => self.move_cursor_to_start(),
                    KeyCode::End => self.move_cursor_to_end(),
                    KeyCode::Enter => self.end_editing(),
                    KeyCode::Escape => self.end_editing(),
                    _ => {
                        // 处理字符输入
                        if let Some(ch) = event.character {
                            if self.is_valid_character(ch) {
                                self.insert_text(&ch.to_string());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// 检查字符是否有效
    fn is_valid_character(&self, ch: char) -> bool {
        match self.input_type {
            TextInputType::Text | TextInputType::Password => true,
            TextInputType::Number => ch.is_ascii_digit() || ch == '.' || ch == '-',
            TextInputType::Email => ch.is_alphanumeric() || ch == '@' || ch == '.' || ch == '_' || ch == '-',
        }
    }

    /// 处理触摸事件
    pub fn on_touch_began(&mut self, touch: &Touch) -> bool {
        let pos = self.widget.get_position();
        let size = self.widget.get_size();
        let half_size = size * 0.5;
        let touch_pos = touch.location();

        // 检查是否在输入框内
        let inside = touch_pos.x >= pos.x - half_size.x &&
                    touch_pos.x <= pos.x + half_size.x &&
                    touch_pos.y >= pos.y - half_size.y &&
                    touch_pos.y <= pos.y + half_size.y;

        if inside {
            self.begin_editing();
            true
        } else {
            self.end_editing();
            false
        }
    }

    /// 设置文本改变回调
    pub fn set_on_text_changed<F>(&mut self, callback: F)
    where
        F: FnMut(&TextField, &str) + 'static,
    {
        self.on_text_changed = Some(Box::new(callback));
    }

    /// 设置编辑开始回调
    pub fn set_on_editing_began<F>(&mut self, callback: F)
    where
        F: FnMut(&TextField) + 'static,
    {
        self.on_editing_began = Some(Box::new(callback));
    }

    /// 设置编辑结束回调
    pub fn set_on_editing_ended<F>(&mut self, callback: F)
    where
        F: FnMut(&TextField) + 'static,
    {
        self.on_editing_ended = Some(Box::new(callback));
    }

    /// 获取显示文本（处理密码类型）
    pub fn display_text(&self) -> String {
        if self.text.is_empty() {
            return String::new();
        }

        match self.input_type {
            TextInputType::Password => "•".repeat(self.text.len()),
            _ => self.text.clone(),
        }
    }
}

impl Default for TextField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textfield_creation() {
        let field = TextField::new();
        assert_eq!(field.text(), "");
        assert!(field.is_editable());
        assert!(!field.is_editing());
    }

    #[test]
    fn test_textfield_text() {
        let mut field = TextField::new();
        field.set_text("Hello");
        assert_eq!(field.text(), "Hello");
    }

    #[test]
    fn test_textfield_max_length() {
        let mut field = TextField::new();
        field.set_max_length(5);
        field.set_text("Hello World");
        assert_eq!(field.text(), "Hello");
    }

    #[test]
    fn test_textfield_insert() {
        let mut field = TextField::new();
        field.begin_editing();
        field.insert_text("Hello");
        assert_eq!(field.text(), "Hello");
        assert_eq!(field.cursor_position, 5);
    }

    #[test]
    fn test_textfield_delete() {
        let mut field = TextField::new();
        field.set_text("Hello");
        field.begin_editing();
        field.move_cursor_to_end();
        
        field.delete_backward();
        assert_eq!(field.text(), "Hell");
        
        field.delete_backward();
        assert_eq!(field.text(), "Hel");
    }

    #[test]
    fn test_textfield_cursor() {
        let mut field = TextField::new();
        field.set_text("Hello");
        field.begin_editing();
        
        field.move_cursor_to_end();
        assert_eq!(field.cursor_position, 5);
        
        field.move_cursor(-2);
        assert_eq!(field.cursor_position, 3);
        
        field.move_cursor_to_start();
        assert_eq!(field.cursor_position, 0);
    }

    #[test]
    fn test_textfield_selection() {
        let mut field = TextField::new();
        field.set_text("Hello");
        field.begin_editing();
        
        field.select_all();
        assert!(field.has_selection());
        
        field.clear_selection();
        assert!(!field.has_selection());
    }

    #[test]
    fn test_textfield_password() {
        let mut field = TextField::new();
        field.set_input_type(TextInputType::Password);
        field.set_text("secret");
        
        assert_eq!(field.text(), "secret");
        assert_eq!(field.display_text(), "••••••");
    }
}
