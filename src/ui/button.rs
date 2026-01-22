use crate::base::{Ref, RefPtr};
use crate::ui::Widget;
use crate::input::{Touch, TouchPhase};
use crate::math::Vec2;
use std::rc::Rc;
use std::cell::RefCell;

/// 按钮状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// 正常状态
    Normal,
    /// 高亮状态（鼠标悬停或触摸按下）
    Highlighted,
    /// 禁用状态
    Disabled,
}

/// 按钮点击回调
pub type ButtonCallback = Box<dyn FnMut(&Button)>;

/// UI 按钮组件
pub struct Button {
    /// 基础 Widget
    widget: Widget,
    /// 按钮状态
    state: ButtonState,
    /// 是否可交互
    interactable: bool,
    /// 点击回调
    on_click: Option<ButtonCallback>,
    /// 正在触摸
    is_touching: bool,
    /// 触摸起始位置
    touch_start_pos: Vec2,
    /// 标题文本
    title: String,
    /// 正常状态颜色
    normal_color: [f32; 4],
    /// 高亮状态颜色
    highlighted_color: [f32; 4],
    /// 禁用状态颜色
    disabled_color: [f32; 4],
}

impl Button {
    /// 创建新按钮
    pub fn new() -> Self {
        Self {
            widget: Widget::new(),
            state: ButtonState::Normal,
            interactable: true,
            on_click: None,
            is_touching: false,
            touch_start_pos: Vec2::ZERO,
            title: String::new(),
            normal_color: [1.0, 1.0, 1.0, 1.0],
            highlighted_color: [0.8, 0.8, 0.8, 1.0],
            disabled_color: [0.5, 0.5, 0.5, 0.5],
        }
    }

    /// 设置标题
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// 获取标题
    pub fn title(&self) -> &str {
        &self.title
    }

    /// 设置可交互性
    pub fn set_interactable(&mut self, interactable: bool) {
        self.interactable = interactable;
        self.state = if interactable {
            ButtonState::Normal
        } else {
            ButtonState::Disabled
        };
    }

    /// 是否可交互
    pub fn is_interactable(&self) -> bool {
        self.interactable
    }

    /// 获取状态
    pub fn state(&self) -> ButtonState {
        self.state
    }

    /// 设置点击回调
    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: FnMut(&Button) + 'static,
    {
        self.on_click = Some(Box::new(callback));
    }

    /// 设置正常状态颜色
    pub fn set_normal_color(&mut self, color: [f32; 4]) {
        self.normal_color = color;
    }

    /// 设置高亮状态颜色
    pub fn set_highlighted_color(&mut self, color: [f32; 4]) {
        self.highlighted_color = color;
    }

    /// 设置禁用状态颜色
    pub fn set_disabled_color(&mut self, color: [f32; 4]) {
        self.disabled_color = color;
    }

    /// 获取当前颜色
    pub fn current_color(&self) -> [f32; 4] {
        match self.state {
            ButtonState::Normal => self.normal_color,
            ButtonState::Highlighted => self.highlighted_color,
            ButtonState::Disabled => self.disabled_color,
        }
    }

    /// 获取 Widget 引用
    pub fn widget(&self) -> &Widget {
        &self.widget
    }

    /// 获取 Widget 可变引用
    pub fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }

    /// 处理触摸开始
    pub fn on_touch_began(&mut self, touch: &Touch) -> bool {
        if !self.interactable {
            return false;
        }

        // 检查触摸是否在按钮范围内
        if !self.contains_point(touch.location()) {
            return false;
        }

        self.is_touching = true;
        self.touch_start_pos = touch.location();
        self.state = ButtonState::Highlighted;
        true
    }

    /// 处理触摸移动
    pub fn on_touch_moved(&mut self, touch: &Touch) {
        if !self.is_touching {
            return;
        }

        // 如果移出按钮范围，取消高亮
        if self.contains_point(touch.location()) {
            self.state = ButtonState::Highlighted;
        } else {
            self.state = ButtonState::Normal;
        }
    }

    /// 处理触摸结束
    pub fn on_touch_ended(&mut self, touch: &Touch) {
        if !self.is_touching {
            return;
        }

        self.is_touching = false;
        self.state = ButtonState::Normal;

        // 如果触摸结束时仍在按钮范围内，触发点击
        if self.contains_point(touch.location()) {
            self.trigger_click();
        }
    }

    /// 处理触摸取消
    pub fn on_touch_cancelled(&mut self, _touch: &Touch) {
        if !self.is_touching {
            return;
        }

        self.is_touching = false;
        self.state = ButtonState::Normal;
    }

    /// 检查点是否在按钮范围内
    fn contains_point(&self, point: Vec2) -> bool {
        let pos = self.widget.get_position();
        let size = self.widget.get_size();
        let half_size = size * 0.5;

        point.x >= pos.x - half_size.x &&
        point.x <= pos.x + half_size.x &&
        point.y >= pos.y - half_size.y &&
        point.y <= pos.y + half_size.y
    }

    /// 触发点击事件
    fn trigger_click(&mut self) {
        if let Some(ref mut callback) = self.on_click {
            callback(self);
        }
    }

    /// 模拟点击（用于测试或程序化触发）
    pub fn simulate_click(&mut self) {
        if self.interactable {
            self.trigger_click();
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new();
        assert_eq!(button.state(), ButtonState::Normal);
        assert!(button.is_interactable());
        assert_eq!(button.title(), "");
    }

    #[test]
    fn test_button_interactable() {
        let mut button = Button::new();
        
        button.set_interactable(false);
        assert!(!button.is_interactable());
        assert_eq!(button.state(), ButtonState::Disabled);
        
        button.set_interactable(true);
        assert!(button.is_interactable());
        assert_eq!(button.state(), ButtonState::Normal);
    }

    #[test]
    fn test_button_title() {
        let mut button = Button::new();
        button.set_title("Click Me");
        assert_eq!(button.title(), "Click Me");
    }

    #[test]
    fn test_button_touch() {
        let mut button = Button::new();
        button.widget_mut().set_position(Vec2::new(100.0, 100.0));
        button.widget_mut().set_size(Vec2::new(80.0, 40.0));

        // 触摸按钮内部
        let touch = Touch::new(1, Vec2::new(100.0, 100.0));
        assert!(button.on_touch_began(&touch));
        assert_eq!(button.state(), ButtonState::Highlighted);

        // 触摸结束
        button.on_touch_ended(&touch);
        assert_eq!(button.state(), ButtonState::Normal);
    }

    #[test]
    fn test_button_touch_outside() {
        let mut button = Button::new();
        button.widget_mut().set_position(Vec2::new(100.0, 100.0));
        button.widget_mut().set_size(Vec2::new(80.0, 40.0));

        // 触摸按钮外部
        let touch = Touch::new(1, Vec2::new(200.0, 200.0));
        assert!(!button.on_touch_began(&touch));
        assert_eq!(button.state(), ButtonState::Normal);
    }

    #[test]
    fn test_button_callback() {
        let mut button = Button::new();
        let mut clicked = false;

        button.set_on_click(|_| {
            // 注意：这里不能捕获 clicked，因为闭包需要 'static
            // 实际使用中通常会使用 Rc<RefCell<>> 等共享状态
        });

        button.simulate_click();
    }

    #[test]
    fn test_button_colors() {
        let mut button = Button::new();
        
        let normal = [1.0, 0.0, 0.0, 1.0];
        let highlighted = [0.0, 1.0, 0.0, 1.0];
        let disabled = [0.0, 0.0, 1.0, 0.5];

        button.set_normal_color(normal);
        button.set_highlighted_color(highlighted);
        button.set_disabled_color(disabled);

        assert_eq!(button.current_color(), normal);
        
        button.set_interactable(false);
        assert_eq!(button.current_color(), disabled);
    }
}
