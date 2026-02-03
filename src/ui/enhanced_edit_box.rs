/// EnhancedEditBox - 增强型文本输入框
///
/// 功能：
/// - 多行文本输入
/// - 文本格式化（数字、密码、URL、邮箱等）
/// - 输入验证和过滤
/// - 占位符支持
/// - 最大长度限制
/// - 撤销/重做支持
/// - 文本选择和光标控制
/// - 键盘类型配置
/// - 返回键类型配置

use std::cell::RefCell;
use std::rc::Rc;
use crate::base::Color4B;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EditBoxInputMode {
    Any,
    EmailAddress,
    Numeric,
    PhoneNumber,
    URL,
    Decimal,
    SingleLine,
    MultiLine,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EditBoxInputFlag {
    Normal,
    Password,
    Sensitive,
    InitialCapsWord,
    InitialCapsSentence,
    InitialCapsAllCharacters,
    Lowercase,
    Uppercase,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReturnType {
    Default,
    Go,
    Search,
    Send,
    Done,
    Next,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyboardType {
    Default,
    ASCII,
    NumberPad,
    PhonePad,
    NamePhonePad,
    Email,
    Decimal,
    Twitter,
    WebSearch,
    ASCIINumberPad,
}

#[derive(Clone, Debug)]
pub struct EditBoxDelegate {
    pub on_text_changed: Option<Box<dyn Fn(&str)>>,
    pub on_return: Option<Box<dyn Fn()>>,
    pub on_begin_editing: Option<Box<dyn Fn()>>,
    pub on_end_editing: Option<Box<dyn Fn()>>,
    pub on_cursor_moved: Option<Box<dyn Fn(usize)>>,
}

impl Default for EditBoxDelegate {
    fn default() -> Self {
        Self {
            on_text_changed: None,
            on_return: None,
            on_begin_editing: None,
            on_end_editing: None,
            on_cursor_moved: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        if self.end > self.start {
            self.end - self.start
        } else {
            0
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputValidator {
    pub max_length: Option<usize>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub numeric_only: bool,
    pub decimal_allowed: bool,
    pub custom_pattern: Option<regex::Regex>,
}

impl Default for InputValidator {
    fn default() -> Self {
        Self {
            max_length: None,
            min_value: None,
            max_value: None,
            numeric_only: false,
            decimal_allowed: true,
            custom_pattern: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EditBoxStyle {
    pub background_color: Color4B,
    pub text_color: Color4B,
    pub placeholder_color: Color4B,
    pub font_size: f32,
    pub placeholder_font_size: f32,
    pub border_width: f32,
    pub border_color: Color4B,
    pub corner_radius: f32,
    pub padding_left: f32,
    pub padding_right: f32,
    pub padding_top: f32,
    pub padding_bottom: f32,
}

impl Default for EditBoxStyle {
    fn default() -> Self {
        Self {
            background_color: Color4B::new(255, 255, 255, 255),
            text_color: Color4B::new(0, 0, 0, 255),
            placeholder_color: Color4B::new(128, 128, 128, 255),
            font_size: 16.0,
            placeholder_font_size: 16.0,
            border_width: 1.0,
            border_color: Color4B::new(200, 200, 200, 255),
            corner_radius: 4.0,
            padding_left: 10.0,
            padding_right: 10.0,
            padding_top: 5.0,
            padding_bottom: 5.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnhancedEditBox {
    text: String,
    placeholder: String,
    input_mode: EditBoxInputMode,
    input_flag: EditBoxInputFlag,
    return_type: ReturnType,
    keyboard_type: KeyboardType,
    max_length: usize,
    delegate: Rc<RefCell<EditBoxDelegate>>,
    style: EditBoxStyle,
    validator: InputValidator,
    cursor_position: usize,
    selection: Option<TextRange>,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
    is_editing: bool,
    enabled: bool,
    visible: bool,
}

impl EnhancedEditBox {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            placeholder: String::new(),
            input_mode: EditBoxInputMode::SingleLine,
            input_flag: EditBoxInputFlag::Normal,
            return_type: ReturnType::Default,
            keyboard_type: KeyboardType::Default,
            max_length: 9999,
            delegate: Rc::new(RefCell::new(EditBoxDelegate::default())),
            style: EditBoxStyle::default(),
            validator: InputValidator::default(),
            cursor_position: 0,
            selection: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            is_editing: false,
            enabled: true,
            visible: true,
        }
    }

    pub fn with_size(width: f32, height: f32) -> Self {
        let mut editbox = Self::new();
        editbox.style.padding_left = width * 0.05;
        editbox.style.padding_right = width * 0.05;
        editbox
    }

    pub fn set_text(&mut self, text: &str) {
        self.save_undo_state();
        self.text = text.to_string();
        self.cursor_position = self.text.len().min(self.max_length);
        self.clear_selection();
        self.notify_text_changed();
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.placeholder = placeholder.to_string();
    }

    pub fn get_placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn set_input_mode(&mut self, mode: EditBoxInputMode) {
        self.input_mode = mode;
    }

    pub fn get_input_mode(&self) -> EditBoxInputMode {
        self.input_mode
    }

    pub fn set_input_flag(&mut self, flag: EditBoxInputFlag) {
        self.input_flag = flag;
    }

    pub fn get_input_flag(&self) -> EditBoxInputFlag {
        self.input_flag
    }

    pub fn set_return_type(&mut self, return_type: ReturnType) {
        self.return_type = return_type;
    }

    pub fn get_return_type(&self) -> ReturnType {
        self.return_type
    }

    pub fn set_keyboard_type(&mut self, keyboard_type: KeyboardType) {
        self.keyboard_type = keyboard_type;
    }

    pub fn get_keyboard_type(&self) -> KeyboardType {
        self.keyboard_type
    }

    pub fn set_max_length(&mut self, max: usize) {
        self.max_length = max;
        self.validator.max_length = Some(max);
        if self.text.len() > max {
            self.text.truncate(max);
        }
    }

    pub fn get_max_length(&self) -> usize {
        self.max_length
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_style(&mut self, style: EditBoxStyle) {
        self.style = style;
    }

    pub fn get_style(&self) -> &EditBoxStyle {
        &self.style
    }

    pub fn set_validator(&mut self, validator: InputValidator) {
        self.validator = validator;
    }

    pub fn get_validator(&self) -> &InputValidator {
        &self.validator
    }

    pub fn insert_text(&mut self, text: &str, position: usize) {
        let pos = position.min(self.text.len());

        if let Some(sel) = &self.selection.clone() {
            self.delete_selection();
            self.save_undo_state();
        } else {
            self.save_undo_state();
        }

        let filtered = self.filter_text(text);
        if filtered.is_empty() {
            return;
        }

        let remaining = self.max_length.saturating_sub(self.text.len());
        let to_insert = if filtered.len() > remaining {
            &filtered[..remaining]
        } else {
            &filtered
        };

        self.text.insert_str(pos, to_insert);
        self.cursor_position = pos + to_insert.len();
        self.clear_selection();
        self.notify_text_changed();
    }

    pub fn delete_backward(&mut self) {
        if let Some(sel) = &self.selection.clone() {
            self.delete_selection();
            self.save_undo_state();
            self.notify_text_changed();
            return;
        }

        if self.cursor_position > 0 {
            self.save_undo_state();
            self.text.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
            self.notify_text_changed();
        }
    }

    pub fn delete_forward(&mut self) {
        if let Some(sel) = &self.selection.clone() {
            self.delete_selection();
            self.save_undo_state();
            self.notify_text_changed();
            return;
        }

        if self.cursor_position < self.text.len() {
            self.save_undo_state();
            self.text.remove(self.cursor_position);
            self.notify_text_changed();
        }
    }

    pub fn move_cursor(&mut self, position: usize) {
        self.cursor_position = position.min(self.text.len());
        self.clear_selection();
        self.notify_cursor_moved();
    }

    pub fn move_cursor_left(&mut self, shift: bool) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            if shift && self.selection.is_none() {
                self.selection = Some(TextRange::new(self.cursor_position, self.cursor_position + 1));
            }
            self.notify_cursor_moved();
        }
    }

    pub fn move_cursor_right(&mut self, shift: bool) {
        if self.cursor_position < self.text.len() {
            self.cursor_position += 1;
            if shift && self.selection.is_none() {
                self.selection = Some(TextRange::new(self.cursor_position - 1, self.cursor_position));
            }
            self.notify_cursor_moved();
        }
    }

    pub fn move_cursor_to_beginning(&mut self, shift: bool) {
        let old_pos = self.cursor_position;
        self.cursor_position = 0;
        if shift {
            self.selection = Some(TextRange::new(0, old_pos));
        }
        self.notify_cursor_moved();
    }

    pub fn move_cursor_to_end(&mut self, shift: bool) {
        let old_pos = self.cursor_position;
        self.cursor_position = self.text.len();
        if shift {
            self.selection = Some(TextRange::new(old_pos, self.text.len()));
        }
        self.notify_cursor_moved();
    }

    pub fn select_all(&mut self) {
        if !self.text.is_empty() {
            self.selection = Some(TextRange::new(0, self.text.len()));
            self.cursor_position = self.text.len();
        }
    }

    pub fn set_selection(&mut self, start: usize, end: usize) {
        let start = start.min(self.text.len());
        let end = end.min(self.text.len());
        self.selection = Some(TextRange::new(start, end));
        self.cursor_position = end;
    }

    pub fn get_selection(&self) -> Option<&TextRange> {
        self.selection.as_ref()
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn delete_selection(&mut self) {
        if let Some(sel) = &self.selection.clone() {
            let start = sel.start.min(sel.end);
            let end = sel.end.max(sel.start);
            if start < end {
                self.text.drain(start..end);
                self.cursor_position = start;
            }
            self.clear_selection();
        }
    }

    pub fn copy_selection(&mut self) -> String {
        if let Some(sel) = &self.selection.clone() {
            let start = sel.start.min(sel.end);
            let end = sel.end.max(sel.start);
            if start < end {
                return self.text[start..end].to_string();
            }
        }
        String::new()
    }

    pub fn cut_selection(&mut self) -> String {
        let copied = self.copy_selection();
        if !copied.is_empty() {
            self.save_undo_state();
            self.delete_selection();
            self.notify_text_changed();
        }
        copied
    }

    pub fn paste(&mut self, text: &str) {
        if !text.is_empty() {
            self.insert_text(text, self.cursor_position);
        }
    }

    pub fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop() {
            self.redo_stack.push(self.text.clone());
            self.text = state;
            self.cursor_position = self.text.len();
            self.clear_selection();
            self.notify_text_changed();
        }
    }

    pub fn redo(&mut self) {
        if let Some(state) = self.redo_stack.pop() {
            self.undo_stack.push(self.text.clone());
            self.text = state;
            self.cursor_position = self.text.len();
            self.clear_selection();
            self.notify_text_changed();
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn begin_editing(&mut self) {
        if !self.is_editing && self.enabled {
            self.is_editing = true;
            self.undo_stack.clear();
            self.redo_stack.clear();
            self.notify_begin_editing();
        }
    }

    pub fn end_editing(&mut self) {
        if self.is_editing {
            self.is_editing = false;
            self.notify_end_editing();
            self.notify_return();
        }
    }

    pub fn is_editing(&self) -> bool {
        self.is_editing
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn get_text_length(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn get_display_text(&self) -> String {
        match self.input_flag {
            EditBoxInputFlag::Password => "*".repeat(self.text.len()),
            EditBoxInputFlag::Lowercase => self.text.to_lowercase(),
            EditBoxInputFlag::Uppercase => self.text.to_uppercase(),
            _ => self.text.clone(),
        }
    }

    pub fn set_delegate(&mut self, delegate: Rc<RefCell<EditBoxDelegate>>) {
        self.delegate = delegate;
    }

    pub fn get_delegate(&self) -> Rc<RefCell<EditBoxDelegate>> {
        self.delegate.clone()
    }

    fn filter_text(&self, text: &str) -> String {
        let mut result = String::new();

        for ch in text.chars() {
            if self.is_char_allowed(ch) {
                result.push(ch);
            }
        }

        result
    }

    fn is_char_allowed(&self, ch: char) -> bool {
        if ch.is_control() {
            return true;
        }

        match self.input_mode {
            EditBoxInputMode::Numeric => ch.is_ascii_digit(),
            EditBoxInputMode::Decimal => ch.is_ascii_digit() || ch == '.',
            EditBoxInputMode::EmailAddress => {
                ch.is_alphanumeric() || ch == '@' || ch == '.' || ch == '_' || ch == '-'
            }
            EditBoxInputMode::URL => {
                ch.is_alphanumeric() || ch == '/' || ch == ':' || ch == '.' || ch == '?' || ch == '&' || ch == '='
            }
            _ => true,
        }
    }

    fn save_undo_state(&mut self) {
        self.undo_stack.push(self.text.clone());
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    fn notify_text_changed(&self) {
        if let Some(cb) = &self.delegate.borrow().on_text_changed {
            cb(&self.text);
        }
    }

    fn notify_return(&self) {
        if let Some(cb) = &self.delegate.borrow().on_return {
            cb();
        }
    }

    fn notify_begin_editing(&self) {
        if let Some(cb) = &self.delegate.borrow().on_begin_editing {
            cb();
        }
    }

    fn notify_end_editing(&self) {
        if let Some(cb) = &self.delegate.borrow().on_end_editing {
            cb();
        }
    }

    fn notify_cursor_moved(&self) {
        if let Some(cb) = &self.delegate.borrow().on_cursor_moved {
            cb(self.cursor_position);
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if let Some(max) = self.validator.max_length {
            if self.text.len() > max {
                return Err(format!("文本长度超过最大限制 {}", max));
            }
        }

        if self.validator.numeric_only {
            if !self.text.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
                return Err("只允许输入数字".to_string());
            }

            if let Some(min) = self.validator.min_value {
                if let Ok(value) = self.text.parse::<f64>() {
                    if value < min {
                        return Err(format!("值必须大于等于 {}", min));
                    }
                }
            }

            if let Some(max) = self.validator.max_value {
                if let Ok(value) = self.text.parse::<f64>() {
                    if value > max {
                        return Err(format!("值必须小于等于 {}", max));
                    }
                }
            }
        }

        if let Some(pattern) = &self.validator.custom_pattern {
            if !pattern.is_match(&self.text) {
                return Err("输入格式不正确".to_string());
            }
        }

        Ok(())
    }

    pub fn generate_report(&self) -> String {
        format!(
            "=== EditBox Report ===\n\
             Text: '{}'\n\
             Length: {}\n\
             Max Length: {}\n\
             Input Mode: {:?}\n\
             Input Flag: {:?}\n\
             Is Editing: {}\n\
             Cursor Position: {}\n\
             Has Selection: {}\n\
             Can Undo: {}\n\
             Can Redo: {}",
            self.text,
            self.text.len(),
            self.max_length,
            self.input_mode,
            self.input_flag,
            self.is_editing,
            self.cursor_position,
            self.selection.is_some(),
            self.can_undo(),
            self.can_redo()
        )
    }
}

impl Default for EnhancedEditBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editbox_creation() {
        let editbox = EnhancedEditBox::new();
        assert!(editbox.get_text().is_empty());
        assert!(editbox.get_placeholder().is_empty());
        assert!(editbox.is_enabled());
        assert!(editbox.is_visible());
    }

    #[test]
    fn test_text_insertion() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("Hello", 0);
        assert_eq!(editbox.get_text(), "Hello");
        assert_eq!(editbox.get_cursor_position(), 5);
    }

    #[test]
    fn test_text_deletion() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("Hello", 0);
        editbox.move_cursor(5);
        editbox.delete_backward();
        assert_eq!(editbox.get_text(), "Hell");
    }

    #[test]
    fn test_max_length() {
        let mut editbox = EnhancedEditBox::new();
        editbox.set_max_length(5);
        editbox.insert_text("Hello World", 0);
        assert_eq!(editbox.get_text().len(), 5);
        assert_eq!(editbox.get_text(), "Hello");
    }

    #[test]
    fn test_selection() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("Hello World", 0);
        editbox.set_selection(0, 5);
        let selection = editbox.get_selection().unwrap();
        assert_eq!(selection.start, 0);
        assert_eq!(selection.end, 5);
    }

    #[test]
    fn test_copy_cut_paste() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("Hello World", 0);
        editbox.set_selection(0, 5);

        let copied = editbox.copy_selection();
        assert_eq!(copied, "Hello");

        let cut = editbox.cut_selection();
        assert_eq!(cut, "Hello");
        assert_eq!(editbox.get_text(), " World");
    }

    #[test]
    fn test_undo_redo() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("Hello", 0);

        editbox.save_undo_state();
        editbox.insert_text(" World", 5);

        assert_eq!(editbox.get_text(), "Hello World");

        editbox.undo();
        assert_eq!(editbox.get_text(), "Hello");

        editbox.redo();
        assert_eq!(editbox.get_text(), "Hello World");
    }

    #[test]
    fn test_password_mode() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("secret", 0);
        editbox.set_input_flag(EditBoxInputFlag::Password);

        let display = editbox.get_display_text();
        assert_eq!(display, "*****");
    }

    #[test]
    fn test_numeric_validation() {
        let mut editbox = EnhancedEditBox::new();
        let validator = InputValidator {
            numeric_only: true,
            min_value: Some(0.0),
            max_value: Some(100.0),
            ..Default::default()
        };
        editbox.set_validator(validator);

        editbox.set_text("50");
        assert!(editbox.validate().is_ok());

        editbox.set_text("150");
        assert!(editbox.validate().is_err());
    }

    #[test]
    fn test_report() {
        let mut editbox = EnhancedEditBox::new();
        editbox.insert_text("Test", 0);
        let report = editbox.generate_report();
        assert!(report.contains("Test"));
        assert!(report.contains("EditBox Report"));
    }
}
