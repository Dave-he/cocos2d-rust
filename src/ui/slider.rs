use crate::base::{Ref, RefPtr};
use crate::ui::Widget;
use crate::input::Touch;
use crate::math::Vec2;
use std::rc::Rc;
use std::cell::RefCell;

/// 滑动条方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderDirection {
    /// 水平方向
    Horizontal,
    /// 垂直方向
    Vertical,
}

/// 值改变回调
pub type ValueChangedCallback = Box<dyn FnMut(&Slider, f32)>;

/// 滑动条组件
pub struct Slider {
    /// 基础 Widget
    widget: Widget,
    /// 当前值
    value: f32,
    /// 最小值
    min_value: f32,
    /// 最大值
    max_value: f32,
    /// 方向
    direction: SliderDirection,
    /// 是否可交互
    interactable: bool,
    /// 是否整数模式
    whole_numbers: bool,
    /// 是否正在拖动
    is_dragging: bool,
    /// 滑块大小（0.0-1.0，相对于轨道）
    handle_size: f32,
    /// 轨道颜色
    track_color: [f32; 4],
    /// 填充颜色
    fill_color: [f32; 4],
    /// 滑块颜色
    handle_color: [f32; 4],
    /// 值改变回调
    on_value_changed: Option<ValueChangedCallback>,
}

impl Slider {
    /// 创建新滑动条
    pub fn new() -> Self {
        Self {
            widget: Widget::new(),
            value: 0.0,
            min_value: 0.0,
            max_value: 1.0,
            direction: SliderDirection::Horizontal,
            interactable: true,
            whole_numbers: false,
            is_dragging: false,
            handle_size: 0.2,
            track_color: [0.8, 0.8, 0.8, 1.0],
            fill_color: [0.2, 0.6, 1.0, 1.0],
            handle_color: [1.0, 1.0, 1.0, 1.0],
            on_value_changed: None,
        }
    }

    /// 设置值
    pub fn set_value(&mut self, mut value: f32) {
        // 限制在范围内
        value = value.clamp(self.min_value, self.max_value);
        
        // 整数模式
        if self.whole_numbers {
            value = value.round();
        }

        if (self.value - value).abs() > f32::EPSILON {
            self.value = value;
            
            // 触发回调
            if let Some(ref mut callback) = self.on_value_changed {
                callback(self, value);
            }
        }
    }

    /// 获取值
    pub fn value(&self) -> f32 {
        self.value
    }

    /// 设置值范围
    pub fn set_range(&mut self, min: f32, max: f32) {
        assert!(min < max, "最小值必须小于最大值");
        self.min_value = min;
        self.max_value = max;
        
        // 重新设置当前值以确保在新范围内
        let current = self.value;
        self.set_value(current);
    }

    /// 获取最小值
    pub fn min_value(&self) -> f32 {
        self.min_value
    }

    /// 获取最大值
    pub fn max_value(&self) -> f32 {
        self.max_value
    }

    /// 设置方向
    pub fn set_direction(&mut self, direction: SliderDirection) {
        self.direction = direction;
    }

    /// 获取方向
    pub fn direction(&self) -> SliderDirection {
        self.direction
    }

    /// 设置可交互性
    pub fn set_interactable(&mut self, interactable: bool) {
        self.interactable = interactable;
        if !interactable {
            self.is_dragging = false;
        }
    }

    /// 是否可交互
    pub fn is_interactable(&self) -> bool {
        self.interactable
    }

    /// 设置整数模式
    pub fn set_whole_numbers(&mut self, whole_numbers: bool) {
        self.whole_numbers = whole_numbers;
        if whole_numbers {
            self.set_value(self.value);
        }
    }

    /// 是否整数模式
    pub fn is_whole_numbers(&self) -> bool {
        self.whole_numbers
    }

    /// 设置滑块大小
    pub fn set_handle_size(&mut self, size: f32) {
        self.handle_size = size.clamp(0.01, 1.0);
    }

    /// 获取滑块大小
    pub fn handle_size(&self) -> f32 {
        self.handle_size
    }

    /// 设置轨道颜色
    pub fn set_track_color(&mut self, color: [f32; 4]) {
        self.track_color = color;
    }

    /// 设置填充颜色
    pub fn set_fill_color(&mut self, color: [f32; 4]) {
        self.fill_color = color;
    }

    /// 设置滑块颜色
    pub fn set_handle_color(&mut self, color: [f32; 4]) {
        self.handle_color = color;
    }

    /// 获取 Widget 引用
    pub fn widget(&self) -> &Widget {
        &self.widget
    }

    /// 获取 Widget 可变引用
    pub fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }

    /// 获取归一化值（0.0-1.0）
    pub fn normalized_value(&self) -> f32 {
        if (self.max_value - self.min_value).abs() < f32::EPSILON {
            return 0.0;
        }
        (self.value - self.min_value) / (self.max_value - self.min_value)
    }

    /// 从位置计算值
    fn value_from_position(&self, position: Vec2) -> f32 {
        let widget_pos = self.widget.get_position();
        let widget_size = self.widget.get_size();
        
        let normalized = match self.direction {
            SliderDirection::Horizontal => {
                let min_x = widget_pos.x - widget_size.x * 0.5;
                let max_x = widget_pos.x + widget_size.x * 0.5;
                ((position.x - min_x) / (max_x - min_x)).clamp(0.0, 1.0)
            }
            SliderDirection::Vertical => {
                let min_y = widget_pos.y - widget_size.y * 0.5;
                let max_y = widget_pos.y + widget_size.y * 0.5;
                ((position.y - min_y) / (max_y - min_y)).clamp(0.0, 1.0)
            }
        };

        self.min_value + normalized * (self.max_value - self.min_value)
    }

    /// 检查点是否在滑动条范围内
    fn contains_point(&self, point: Vec2) -> bool {
        let pos = self.widget.get_position();
        let size = self.widget.get_size();
        let half_size = size * 0.5;

        point.x >= pos.x - half_size.x &&
        point.x <= pos.x + half_size.x &&
        point.y >= pos.y - half_size.y &&
        point.y <= pos.y + half_size.y
    }

    /// 处理触摸开始
    pub fn on_touch_began(&mut self, touch: &Touch) -> bool {
        if !self.interactable {
            return false;
        }

        if !self.contains_point(touch.location()) {
            return false;
        }

        self.is_dragging = true;
        let new_value = self.value_from_position(touch.location());
        self.set_value(new_value);
        true
    }

    /// 处理触摸移动
    pub fn on_touch_moved(&mut self, touch: &Touch) {
        if !self.is_dragging {
            return;
        }

        let new_value = self.value_from_position(touch.location());
        self.set_value(new_value);
    }

    /// 处理触摸结束
    pub fn on_touch_ended(&mut self, _touch: &Touch) {
        self.is_dragging = false;
    }

    /// 处理触摸取消
    pub fn on_touch_cancelled(&mut self, _touch: &Touch) {
        self.is_dragging = false;
    }

    /// 设置值改变回调
    pub fn set_on_value_changed<F>(&mut self, callback: F)
    where
        F: FnMut(&Slider, f32) + 'static,
    {
        self.on_value_changed = Some(Box::new(callback));
    }

    /// 获取滑块位置（归一化坐标，相对于轨道）
    pub fn handle_position(&self) -> Vec2 {
        let normalized = self.normalized_value();
        let pos = self.widget.get_position();
        let size = self.widget.get_size();

        match self.direction {
            SliderDirection::Horizontal => {
                let x = pos.x - size.x * 0.5 + normalized * size.x;
                Vec2::new(x, pos.y)
            }
            SliderDirection::Vertical => {
                let y = pos.y - size.y * 0.5 + normalized * size.y;
                Vec2::new(pos.x, y)
            }
        }
    }

    /// 增加值
    pub fn increment(&mut self, amount: f32) {
        self.set_value(self.value + amount);
    }

    /// 减少值
    pub fn decrement(&mut self, amount: f32) {
        self.set_value(self.value - amount);
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_creation() {
        let slider = Slider::new();
        assert_eq!(slider.value(), 0.0);
        assert_eq!(slider.min_value(), 0.0);
        assert_eq!(slider.max_value(), 1.0);
        assert!(slider.is_interactable());
    }

    #[test]
    fn test_slider_value() {
        let mut slider = Slider::new();
        
        slider.set_value(0.5);
        assert_eq!(slider.value(), 0.5);
        
        // 测试范围限制
        slider.set_value(1.5);
        assert_eq!(slider.value(), 1.0);
        
        slider.set_value(-0.5);
        assert_eq!(slider.value(), 0.0);
    }

    #[test]
    fn test_slider_range() {
        let mut slider = Slider::new();
        slider.set_range(0.0, 100.0);
        
        slider.set_value(50.0);
        assert_eq!(slider.value(), 50.0);
        assert_eq!(slider.normalized_value(), 0.5);
    }

    #[test]
    fn test_slider_whole_numbers() {
        let mut slider = Slider::new();
        slider.set_range(0.0, 10.0);
        slider.set_whole_numbers(true);
        
        slider.set_value(5.7);
        assert_eq!(slider.value(), 6.0);
        
        slider.set_value(3.2);
        assert_eq!(slider.value(), 3.0);
    }

    #[test]
    fn test_slider_increment_decrement() {
        let mut slider = Slider::new();
        slider.set_range(0.0, 10.0);
        slider.set_value(5.0);
        
        slider.increment(2.0);
        assert_eq!(slider.value(), 7.0);
        
        slider.decrement(3.0);
        assert_eq!(slider.value(), 4.0);
    }

    #[test]
    fn test_slider_direction() {
        let mut slider = Slider::new();
        assert_eq!(slider.direction(), SliderDirection::Horizontal);
        
        slider.set_direction(SliderDirection::Vertical);
        assert_eq!(slider.direction(), SliderDirection::Vertical);
    }

    #[test]
    fn test_slider_interactable() {
        let mut slider = Slider::new();
        
        slider.set_interactable(false);
        assert!(!slider.is_interactable());
        
        let touch = Touch::new(1, Vec2::new(100.0, 100.0));
        assert!(!slider.on_touch_began(&touch));
    }

    #[test]
    fn test_slider_normalized_value() {
        let mut slider = Slider::new();
        slider.set_range(0.0, 100.0);
        
        slider.set_value(0.0);
        assert_eq!(slider.normalized_value(), 0.0);
        
        slider.set_value(50.0);
        assert!((slider.normalized_value() - 0.5).abs() < f32::EPSILON);
        
        slider.set_value(100.0);
        assert_eq!(slider.normalized_value(), 1.0);
    }

    #[test]
    fn test_slider_handle_size() {
        let mut slider = Slider::new();
        
        slider.set_handle_size(0.3);
        assert_eq!(slider.handle_size(), 0.3);
        
        // 测试范围限制
        slider.set_handle_size(1.5);
        assert_eq!(slider.handle_size(), 1.0);
        
        slider.set_handle_size(-0.1);
        assert_eq!(slider.handle_size(), 0.01);
    }
}
