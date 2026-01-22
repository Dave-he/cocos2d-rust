/// 键盘按键代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // 字母键
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    
    // 数字键
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    
    // 功能键
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    
    // 方向键
    Left, Right, Up, Down,
    
    // 控制键
    Escape, Tab, CapsLock, Shift, Control, Alt, Super,
    Space, Enter, Backspace, Delete,
    
    // 编辑键
    Insert, Home, End, PageUp, PageDown,
    
    // 符号键
    Minus, Equals, LeftBracket, RightBracket,
    Backslash, Semicolon, Quote, Comma, Period, Slash,
    Grave,
    
    // 数字键盘
    KpDivide, KpMultiply, KpMinus, KpPlus, KpEnter,
    Kp0, Kp1, Kp2, Kp3, Kp4, Kp5, Kp6, Kp7, Kp8, Kp9,
    KpDecimal,
    
    // 其他
    Unknown,
}

/// 键盘事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    /// 按键按下
    Pressed,
    /// 按键释放
    Released,
    /// 按键重复（长按）
    Repeat,
}

/// 键盘事件
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    /// 键码
    pub key_code: KeyCode,
    /// 事件类型
    pub event_type: KeyEventType,
    /// Shift 是否按下
    pub shift: bool,
    /// Ctrl 是否按下
    pub ctrl: bool,
    /// Alt 是否按下
    pub alt: bool,
    /// Super (Windows/Command) 是否按下
    pub super_key: bool,
    /// 字符（如果有）
    pub character: Option<char>,
}

impl KeyboardEvent {
    /// 创建新的键盘事件
    pub fn new(key_code: KeyCode, event_type: KeyEventType) -> Self {
        Self {
            key_code,
            event_type,
            shift: false,
            ctrl: false,
            alt: false,
            super_key: false,
            character: None,
        }
    }

    /// 设置修饰键状态
    pub fn with_modifiers(mut self, shift: bool, ctrl: bool, alt: bool, super_key: bool) -> Self {
        self.shift = shift;
        self.ctrl = ctrl;
        self.alt = alt;
        self.super_key = super_key;
        self
    }

    /// 设置字符
    pub fn with_character(mut self, character: char) -> Self {
        self.character = Some(character);
        self
    }

    /// 是否有修饰键按下
    pub fn has_modifiers(&self) -> bool {
        self.shift || self.ctrl || self.alt || self.super_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_event() {
        let event = KeyboardEvent::new(KeyCode::A, KeyEventType::Pressed)
            .with_modifiers(true, false, false, false)
            .with_character('A');
        
        assert_eq!(event.key_code, KeyCode::A);
        assert_eq!(event.event_type, KeyEventType::Pressed);
        assert!(event.shift);
        assert!(!event.ctrl);
        assert_eq!(event.character, Some('A'));
        assert!(event.has_modifiers());
    }

    #[test]
    fn test_no_modifiers() {
        let event = KeyboardEvent::new(KeyCode::Space, KeyEventType::Released);
        assert!(!event.has_modifiers());
    }
}
