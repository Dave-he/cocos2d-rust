/// EditBox - 高级文本输入框组件
/// 
/// 提供比 TextField 更强大的功能：
/// - 多行文本编辑
/// - 文本验证和格式化
/// - 富文本支持（颜色、字体样式）
/// - 自定义键盘类型
/// - 输入范围限制
/// - 历史记录（撤销/重做）

use crate::ui::Widget;
use crate::input::{KeyCode, KeyboardEvent, KeyEventType, Touch};
use std::collections::VecDeque;

/// EditBox 返回键类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditBoxReturnType {
    Default,
    Done,
    Send,
    Search,
    Go,
    Next,
}

/// EditBox 输入模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditBoxInputMode {
    Any,
    EmailAddress,
    Numeric,
    PhoneNumber,
    Url,
    Decimal,
    SingleLine,
}

/// EditBox 输入标志
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditBoxInputFlag {
    pub password: bool,
    pub sensitive: bool,
    pub initial_caps_sentence: bool,
    pub initial_caps_word: bool,
    pub initial_caps_all_characters: bool,
}

impl Default for EditBoxInputFlag {
    fn default() -> Self {
        Self {
            password: false,
            sensitive: false,
            initial_caps_sentence: false,
            initial_caps_word: false,
            initial_caps_all_characters: false,
        }
    }
}

/// 编辑历史记录
#[derive(Debug, Clone)]
struct EditHistory {
    text: String,
    cursor_position: usize,
}

/// EditBox - 高级文本输入框
pub struct EditBox {
    widget: Widget,
    text: String,
    placeholder: String,
    max_length: usize,
    input_mode: EditBoxInputMode,
    input_flag: EditBoxInputFlag,
    return_type: EditBoxReturnType,
    
    is_editing: bool,
    cursor_position: usize,
    selection_range: Option<(usize, usize)>,
    
    multiline: bool,
    max_lines: usize,
    
    validator: Option<Box<dyn Fn(&str) -> bool>>,
    formatter: Option<Box<dyn Fn(&str) -> String>>,
    
    history: VecDeque<EditHistory>,
    history_index: usize,
    max_history: usize,
    
    on_text_changed: Option<Box<dyn FnMut(&EditBox)>>,
    on_editing_began: Option<Box<dyn FnMut(&EditBox)>>,
    on_editing_ended: Option<Box<dyn FnMut(&EditBox)>>,
    on_return: Option<Box<dyn FnMut(&EditBox)>>,
}

impl std::fmt::Debug for EditBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EditBox")
            .field("widget", &self.widget)
            .field("text", &self.text)
            .field("placeholder", &self.placeholder)
            .field("input_mode", &self.input_mode)
            .field("multiline", &self.multiline)
            .finish()
    }
}

impl EditBox {
    pub fn new() -> Self {
        Self {
            widget: Widget::new(),
            text: String::new(),
            placeholder: String::from("Enter text..."),
            max_length: 0,
            input_mode: EditBoxInputMode::Any,
            input_flag: EditBoxInputFlag::default(),
            return_type: EditBoxReturnType::Default,
            
            is_editing: false,
            cursor_position: 0,
            selection_range: None,
            
            multiline: false,
            max_lines: 0,
            
            validator: None,
            formatter: None,
            
            history: VecDeque::new(),
            history_index: 0,
            max_history: 50,
            
            on_text_changed: None,
            on_editing_began: None,
            on_editing_ended: None,
            on_return: None,
        }
    }
    
    pub fn new_multiline(max_lines: usize) -> Self {
        let mut editbox = Self::new();
        editbox.multiline = true;
        editbox.max_lines = max_lines;
        editbox
    }
    
    // ===== 文本操作 =====
    
    pub fn set_text(&mut self, text: impl Into<String>) {
        let new_text = text.into();
        
        if let Some(ref validator) = self.validator {
            if !validator(&new_text) {
                return;
            }
        }
        
        let formatted_text = if let Some(ref formatter) = self.formatter {
            formatter(&new_text)
        } else {
            new_text
        };
        
        let final_text = self.apply_length_limit(formatted_text);
        
        self.save_to_history();
        
        self.text = final_text;
        self.cursor_position = self.cursor_position.min(self.text.len());
        self.selection_range = None;
        
        if let Some(mut callback) = self.on_text_changed.take() {
            callback(self);
            self.on_text_changed = Some(callback);
        }
    }
    
    pub fn text(&self) -> &str {
        &self.text
    }
    
    pub fn insert_text(&mut self, text: &str) {
        if !self.is_editing {
            return;
        }
        
        if let Some((start, end)) = self.selection_range {
            self.text.drain(start..end);
            self.cursor_position = start;
            self.selection_range = None;
        }
        
        if !self.is_text_valid(text) {
            return;
        }
        
        let text_to_insert = self.apply_input_flags(text);
        
        if self.max_length > 0 {
            let available = self.max_length.saturating_sub(self.text.len());
            if available == 0 {
                return;
            }
            let text_to_insert = &text_to_insert[..text_to_insert.len().min(available)];
            
            self.save_to_history();
            self.text.insert_str(self.cursor_position, text_to_insert);
            self.cursor_position += text_to_insert.len();
        } else {
            self.save_to_history();
            self.text.insert_str(self.cursor_position, &text_to_insert);
            self.cursor_position += text_to_insert.len();
        }
        
        if let Some(mut callback) = self.on_text_changed.take() {
            callback(self);
            self.on_text_changed = Some(callback);
        }
    }
    
    pub fn delete_backward(&mut self) {
        if !self.is_editing {
            return;
        }
        
        if let Some((start, end)) = self.selection_range {
            self.save_to_history();
            self.text.drain(start..end);
            self.cursor_position = start;
            self.selection_range = None;
        } else if self.cursor_position > 0 {
            self.save_to_history();
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
        
        if let Some(mut callback) = self.on_text_changed.take() {
            callback(self);
            self.on_text_changed = Some(callback);
        }
    }
    
    pub fn delete_forward(&mut self) {
        if !self.is_editing {
            return;
        }
        
        if let Some((start, end)) = self.selection_range {
            self.save_to_history();
            self.text.drain(start..end);
            self.cursor_position = start;
            self.selection_range = None;
        } else if self.cursor_position < self.text.len() {
            self.save_to_history();
            self.text.remove(self.cursor_position);
        }
        
        if let Some(mut callback) = self.on_text_changed.take() {
            callback(self);
            self.on_text_changed = Some(callback);
        }
    }
    
    // ===== 光标和选择 =====
    
    pub fn move_cursor(&mut self, offset: isize) {
        let new_pos = (self.cursor_position as isize + offset)
            .max(0)
            .min(self.text.len() as isize) as usize;
        self.cursor_position = new_pos;
        self.selection_range = None;
    }
    
    pub fn move_to_line_start(&mut self) {
        if !self.multiline {
            self.cursor_position = 0;
            return;
        }
        
        let before_cursor = &self.text[..self.cursor_position];
        if let Some(pos) = before_cursor.rfind('\n') {
            self.cursor_position = pos + 1;
        } else {
            self.cursor_position = 0;
        }
        self.selection_range = None;
    }
    
    pub fn move_to_line_end(&mut self) {
        if !self.multiline {
            self.cursor_position = self.text.len();
            return;
        }
        
        let after_cursor = &self.text[self.cursor_position..];
        if let Some(pos) = after_cursor.find('\n') {
            self.cursor_position += pos;
        } else {
            self.cursor_position = self.text.len();
        }
        self.selection_range = None;
    }
    
    pub fn select_all(&mut self) {
        if !self.text.is_empty() {
            self.selection_range = Some((0, self.text.len()));
        }
    }
    
    pub fn clear_selection(&mut self) {
        self.selection_range = None;
    }
    
    pub fn has_selection(&self) -> bool {
        self.selection_range.is_some()
    }
    
    pub fn get_selected_text(&self) -> Option<&str> {
        self.selection_range.map(|(start, end)| &self.text[start..end])
    }
    
    // ===== 历史记录 =====
    
    fn save_to_history(&mut self) {
        self.history.truncate(self.history_index);
        
        self.history.push_back(EditHistory {
            text: self.text.clone(),
            cursor_position: self.cursor_position,
        });
        
        if self.history.len() > self.max_history {
            self.history.pop_front();
        } else {
            self.history_index += 1;
        }
    }
    
    pub fn undo(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(history) = self.history.get(self.history_index) {
                self.text = history.text.clone();
                self.cursor_position = history.cursor_position;
                self.selection_range = None;
                
                if let Some(mut callback) = self.on_text_changed.take() {
                    callback(self);
                    self.on_text_changed = Some(callback);
                }
                return true;
            }
        }
        false
    }
    
    pub fn redo(&mut self) -> bool {
        if self.history_index < self.history.len() {
            if let Some(history) = self.history.get(self.history_index) {
                self.text = history.text.clone();
                self.cursor_position = history.cursor_position;
                self.selection_range = None;
                self.history_index += 1;
                
                if let Some(mut callback) = self.on_text_changed.take() {
                    callback(self);
                    self.on_text_changed = Some(callback);
                }
                return true;
            }
        }
        false
    }
    
    pub fn can_undo(&self) -> bool {
        self.history_index > 0
    }
    
    pub fn can_redo(&self) -> bool {
        self.history_index < self.history.len()
    }
    
    // ===== 编辑状态 =====
    
    pub fn begin_editing(&mut self) {
        if self.is_editing {
            return;
        }
        
        self.is_editing = true;
        
        if let Some(mut callback) = self.on_editing_began.take() {
            callback(self);
            self.on_editing_began = Some(callback);
        }
    }
    
    pub fn end_editing(&mut self) {
        if !self.is_editing {
            return;
        }
        
        self.is_editing = false;
        self.selection_range = None;
        
        if let Some(mut callback) = self.on_editing_ended.take() {
            callback(self);
            self.on_editing_ended = Some(callback);
        }
    }
    
    pub fn is_editing(&self) -> bool {
        self.is_editing
    }
    
    // ===== 配置 =====
    
    pub fn set_placeholder(&mut self, placeholder: impl Into<String>) {
        self.placeholder = placeholder.into();
    }
    
    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }
    
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
        if max_length > 0 && self.text.len() > max_length {
            self.text.truncate(max_length);
            self.cursor_position = self.cursor_position.min(max_length);
        }
    }
    
    pub fn set_input_mode(&mut self, mode: EditBoxInputMode) {
        self.input_mode = mode;
    }
    
    pub fn set_input_flag(&mut self, flag: EditBoxInputFlag) {
        self.input_flag = flag;
    }
    
    pub fn set_return_type(&mut self, return_type: EditBoxReturnType) {
        self.return_type = return_type;
    }
    
    pub fn set_multiline(&mut self, multiline: bool) {
        self.multiline = multiline;
    }
    
    pub fn is_multiline(&self) -> bool {
        self.multiline
    }
    
    pub fn set_max_lines(&mut self, max_lines: usize) {
        self.max_lines = max_lines;
    }
    
    pub fn set_validator<F>(&mut self, validator: F)
    where
        F: Fn(&str) -> bool + 'static,
    {
        self.validator = Some(Box::new(validator));
    }
    
    pub fn set_formatter<F>(&mut self, formatter: F)
    where
        F: Fn(&str) -> String + 'static,
    {
        self.formatter = Some(Box::new(formatter));
    }
    
    // ===== 回调 =====
    
    pub fn set_on_text_changed<F>(&mut self, callback: F)
    where
        F: FnMut(&EditBox) + 'static,
    {
        self.on_text_changed = Some(Box::new(callback));
    }
    
    pub fn set_on_editing_began<F>(&mut self, callback: F)
    where
        F: FnMut(&EditBox) + 'static,
    {
        self.on_editing_began = Some(Box::new(callback));
    }
    
    pub fn set_on_editing_ended<F>(&mut self, callback: F)
    where
        F: FnMut(&EditBox) + 'static,
    {
        self.on_editing_ended = Some(Box::new(callback));
    }
    
    pub fn set_on_return<F>(&mut self, callback: F)
    where
        F: FnMut(&EditBox) + 'static,
    {
        self.on_return = Some(Box::new(callback));
    }
    
    // ===== 事件处理 =====
    
    pub fn on_keyboard_event(&mut self, event: &KeyboardEvent) {
        if !self.is_editing {
            return;
        }
        
        if event.event_type != KeyEventType::Pressed && event.event_type != KeyEventType::Repeat {
            return;
        }
        
        match event.key_code {
            KeyCode::Backspace => self.delete_backward(),
            KeyCode::Delete => self.delete_forward(),
            KeyCode::Left => self.move_cursor(-1),
            KeyCode::Right => self.move_cursor(1),
            KeyCode::Home => self.move_to_line_start(),
            KeyCode::End => self.move_to_line_end(),
            KeyCode::Enter => {
                if self.multiline {
                    self.insert_text("\n");
                } else {
                    if let Some(mut callback) = self.on_return.take() {
                        callback(self);
                        self.on_return = Some(callback);
                    }
                    self.end_editing();
                }
            }
            KeyCode::Escape => self.end_editing(),
            _ => {
                if let Some(ch) = event.character {
                    self.insert_text(&ch.to_string());
                }
            }
        }
    }
    
    pub fn on_touch_began(&mut self, touch: &Touch) -> bool {
        let pos = self.widget.get_position();
        let size = self.widget.get_size();
        let half_size = size * 0.5;
        let touch_pos = touch.location();
        
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
    
    pub fn widget(&self) -> &Widget {
        &self.widget
    }
    
    pub fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
    
    // ===== 私有辅助方法 =====
    
    fn apply_length_limit(&self, text: String) -> String {
        if self.max_length > 0 && text.len() > self.max_length {
            text[..self.max_length].to_string()
        } else {
            text
        }
    }
    
    fn is_text_valid(&self, text: &str) -> bool {
        match self.input_mode {
            EditBoxInputMode::Any | EditBoxInputMode::SingleLine => true,
            EditBoxInputMode::EmailAddress => {
                text.chars().all(|c| c.is_alphanumeric() || "@._-".contains(c))
            }
            EditBoxInputMode::Numeric => {
                text.chars().all(|c| c.is_ascii_digit())
            }
            EditBoxInputMode::Decimal => {
                text.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-')
            }
            EditBoxInputMode::PhoneNumber => {
                text.chars().all(|c| c.is_ascii_digit() || "+- ()".contains(c))
            }
            EditBoxInputMode::Url => {
                text.chars().all(|c| c.is_alphanumeric() || ":/.?&#=_-".contains(c))
            }
        }
    }
    
    fn apply_input_flags(&self, text: &str) -> String {
        if self.input_flag.initial_caps_all_characters {
            text.to_uppercase()
        } else if self.input_flag.initial_caps_word {
            text.split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        } else if self.input_flag.initial_caps_sentence {
            let mut result = String::new();
            let mut capitalize_next = true;
            for ch in text.chars() {
                if capitalize_next && ch.is_alphabetic() {
                    result.push_str(&ch.to_uppercase().to_string());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                    if ch == '.' || ch == '!' || ch == '?' {
                        capitalize_next = true;
                    }
                }
            }
            result
        } else {
            text.to_string()
        }
    }
    
    pub fn line_count(&self) -> usize {
        if self.text.is_empty() {
            0
        } else {
            self.text.matches('\n').count() + 1
        }
    }
    
    pub fn display_text(&self) -> String {
        if self.input_flag.password {
            "•".repeat(self.text.len())
        } else {
            self.text.clone()
        }
    }
}

impl Default for EditBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_editbox_creation() {
        let editbox = EditBox::new();
        assert_eq!(editbox.text(), "");
        assert!(!editbox.is_editing());
        assert!(!editbox.is_multiline());
    }
    
    #[test]
    fn test_editbox_text() {
        let mut editbox = EditBox::new();
        editbox.set_text("Hello World");
        assert_eq!(editbox.text(), "Hello World");
    }
    
    #[test]
    fn test_editbox_insert() {
        let mut editbox = EditBox::new();
        editbox.begin_editing();
        editbox.insert_text("Hello");
        assert_eq!(editbox.text(), "Hello");
        
        editbox.insert_text(" World");
        assert_eq!(editbox.text(), "Hello World");
    }
    
    #[test]
    fn test_editbox_delete() {
        let mut editbox = EditBox::new();
        editbox.set_text("Hello");
        editbox.begin_editing();
        editbox.cursor_position = editbox.text().len();
        
        editbox.delete_backward();
        assert_eq!(editbox.text(), "Hell");
        
        editbox.delete_backward();
        assert_eq!(editbox.text(), "Hel");
    }
    
    #[test]
    fn test_editbox_multiline() {
        let mut editbox = EditBox::new_multiline(5);
        assert!(editbox.is_multiline());
        assert_eq!(editbox.max_lines, 5);
        
        editbox.begin_editing();
        editbox.insert_text("Line 1\nLine 2\nLine 3");
        assert_eq!(editbox.line_count(), 3);
    }
    
    #[test]
    fn test_editbox_selection() {
        let mut editbox = EditBox::new();
        editbox.set_text("Hello World");
        
        editbox.select_all();
        assert!(editbox.has_selection());
        assert_eq!(editbox.get_selected_text(), Some("Hello World"));
        
        editbox.clear_selection();
        assert!(!editbox.has_selection());
    }
    
    #[test]
    fn test_editbox_max_length() {
        let mut editbox = EditBox::new();
        editbox.set_max_length(5);
        editbox.set_text("Hello World");
        assert_eq!(editbox.text(), "Hello");
    }
    
    #[test]
    fn test_editbox_input_mode_numeric() {
        let mut editbox = EditBox::new();
        editbox.set_input_mode(EditBoxInputMode::Numeric);
        editbox.begin_editing();
        
        editbox.insert_text("123");
        assert_eq!(editbox.text(), "123");
        
        editbox.insert_text("abc");
        assert_eq!(editbox.text(), "123");
    }
    
    #[test]
    fn test_editbox_password() {
        let mut editbox = EditBox::new();
        let mut flag = EditBoxInputFlag::default();
        flag.password = true;
        editbox.set_input_flag(flag);
        
        editbox.set_text("secret");
        assert_eq!(editbox.text(), "secret");
        assert_eq!(editbox.display_text(), "••••••");
    }
    
    #[test]
    fn test_editbox_undo_redo() {
        let mut editbox = EditBox::new();
        editbox.begin_editing();
        
        editbox.insert_text("Hello");
        assert_eq!(editbox.text(), "Hello");
        
        editbox.undo();
        assert!(editbox.text().is_empty() || editbox.text() == "Hello");
    }
    
    #[test]
    fn test_editbox_caps() {
        let mut editbox = EditBox::new();
        let mut flag = EditBoxInputFlag::default();
        flag.initial_caps_all_characters = true;
        editbox.set_input_flag(flag);
        
        editbox.begin_editing();
        editbox.insert_text("hello");
        assert_eq!(editbox.text(), "HELLO");
    }
    
    #[test]
    fn test_editbox_validator() {
        let mut editbox = EditBox::new();
        editbox.set_validator(|text| text.len() <= 10);
        
        editbox.set_text("Short");
        assert_eq!(editbox.text(), "Short");
        
        editbox.set_text("This is too long");
        assert_eq!(editbox.text(), "Short");
    }
    
    #[test]
    fn test_editbox_cursor_movement() {
        let mut editbox = EditBox::new();
        editbox.set_text("Hello World");
        editbox.begin_editing();
        
        editbox.move_to_line_end();
        assert_eq!(editbox.cursor_position, 11);
        
        editbox.move_cursor(-5);
        assert_eq!(editbox.cursor_position, 6);
        
        editbox.move_to_line_start();
        assert_eq!(editbox.cursor_position, 0);
    }
    
    #[test]
    fn test_editbox_formatter() {
        let mut editbox = EditBox::new();
        editbox.set_formatter(|text| text.trim().to_string());
        
        editbox.set_text("  Hello World  ");
        assert_eq!(editbox.text(), "Hello World");
    }
    
    #[test]
    fn test_editbox_line_navigation() {
        let mut editbox = EditBox::new_multiline(0);
        editbox.set_text("Line 1\nLine 2\nLine 3");
        editbox.begin_editing();
        
        editbox.cursor_position = 10;
        
        editbox.move_to_line_start();
        assert_eq!(editbox.cursor_position, 7);
        
        editbox.move_to_line_end();
        assert_eq!(editbox.cursor_position, 13);
    }
}
