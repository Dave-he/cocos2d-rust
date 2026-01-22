use crate::math::Vec2;

/// 鼠标按键
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// 左键
    Left,
    /// 右键
    Right,
    /// 中键
    Middle,
    /// 其他按键
    Other(u8),
}

/// 鼠标事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    /// 鼠标按下
    Down,
    /// 鼠标释放
    Up,
    /// 鼠标移动
    Move,
    /// 鼠标滚轮
    Scroll,
    /// 鼠标进入
    Enter,
    /// 鼠标离开
    Leave,
}

/// 鼠标事件
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// 事件类型
    pub event_type: MouseEventType,
    /// 鼠标按键（对于按下/释放事件）
    pub button: Option<MouseButton>,
    /// 鼠标位置
    pub position: Vec2,
    /// 前一个位置
    pub previous_position: Vec2,
    /// 滚轮增量（对于滚轮事件）
    pub scroll_delta: Vec2,
    /// Shift 是否按下
    pub shift: bool,
    /// Ctrl 是否按下
    pub ctrl: bool,
    /// Alt 是否按下
    pub alt: bool,
    /// 点击次数（单击、双击等）
    pub click_count: u32,
}

impl MouseEvent {
    /// 创建鼠标按下事件
    pub fn down(button: MouseButton, position: Vec2) -> Self {
        Self {
            event_type: MouseEventType::Down,
            button: Some(button),
            position,
            previous_position: position,
            scroll_delta: Vec2::ZERO,
            shift: false,
            ctrl: false,
            alt: false,
            click_count: 1,
        }
    }

    /// 创建鼠标释放事件
    pub fn up(button: MouseButton, position: Vec2) -> Self {
        Self {
            event_type: MouseEventType::Up,
            button: Some(button),
            position,
            previous_position: position,
            scroll_delta: Vec2::ZERO,
            shift: false,
            ctrl: false,
            alt: false,
            click_count: 1,
        }
    }

    /// 创建鼠标移动事件
    pub fn move_event(position: Vec2, previous_position: Vec2) -> Self {
        Self {
            event_type: MouseEventType::Move,
            button: None,
            position,
            previous_position,
            scroll_delta: Vec2::ZERO,
            shift: false,
            ctrl: false,
            alt: false,
            click_count: 0,
        }
    }

    /// 创建鼠标滚轮事件
    pub fn scroll(position: Vec2, delta: Vec2) -> Self {
        Self {
            event_type: MouseEventType::Scroll,
            button: None,
            position,
            previous_position: position,
            scroll_delta: delta,
            shift: false,
            ctrl: false,
            alt: false,
            click_count: 0,
        }
    }

    /// 创建鼠标进入事件
    pub fn enter(position: Vec2) -> Self {
        Self {
            event_type: MouseEventType::Enter,
            button: None,
            position,
            previous_position: position,
            scroll_delta: Vec2::ZERO,
            shift: false,
            ctrl: false,
            alt: false,
            click_count: 0,
        }
    }

    /// 创建鼠标离开事件
    pub fn leave(position: Vec2) -> Self {
        Self {
            event_type: MouseEventType::Leave,
            button: None,
            position,
            previous_position: position,
            scroll_delta: Vec2::ZERO,
            shift: false,
            ctrl: false,
            alt: false,
            click_count: 0,
        }
    }

    /// 设置修饰键状态
    pub fn with_modifiers(mut self, shift: bool, ctrl: bool, alt: bool) -> Self {
        self.shift = shift;
        self.ctrl = ctrl;
        self.alt = alt;
        self
    }

    /// 设置点击次数
    pub fn with_click_count(mut self, count: u32) -> Self {
        self.click_count = count;
        self
    }

    /// 获取移动增量
    pub fn delta(&self) -> Vec2 {
        self.position - self.previous_position
    }

    /// 是否有修饰键按下
    pub fn has_modifiers(&self) -> bool {
        self.shift || self.ctrl || self.alt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_down_event() {
        let event = MouseEvent::down(MouseButton::Left, Vec2::new(100.0, 200.0));
        assert_eq!(event.event_type, MouseEventType::Down);
        assert_eq!(event.button, Some(MouseButton::Left));
        assert_eq!(event.position, Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_mouse_move_event() {
        let event = MouseEvent::move_event(
            Vec2::new(150.0, 250.0),
            Vec2::new(100.0, 200.0),
        );
        assert_eq!(event.event_type, MouseEventType::Move);
        assert_eq!(event.delta(), Vec2::new(50.0, 50.0));
    }

    #[test]
    fn test_mouse_scroll_event() {
        let event = MouseEvent::scroll(
            Vec2::new(100.0, 200.0),
            Vec2::new(0.0, 10.0),
        );
        assert_eq!(event.event_type, MouseEventType::Scroll);
        assert_eq!(event.scroll_delta, Vec2::new(0.0, 10.0));
    }

    #[test]
    fn test_modifiers() {
        let event = MouseEvent::down(MouseButton::Left, Vec2::new(100.0, 200.0))
            .with_modifiers(true, true, false);
        assert!(event.shift);
        assert!(event.ctrl);
        assert!(!event.alt);
        assert!(event.has_modifiers());
    }
}
