use crate::base::Node;
use crate::math::Vec2;
use crate::ui::Widget;
use std::time::Duration;

/// 滚动方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    NONE,
    VERTICAL,
    HORIZONTAL,
    BOTH,
}

/// 滚动视图事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollViewEventType {
    SCROLL_TO_TOP,
    SCROLL_TO_BOTTOM,
    SCROLL_TO_LEFT,
    SCROLL_TO_RIGHT,
    SCROLLING,
    SCROLL_ENDED,
    BOUNCE_TOP,
    BOUNCE_BOTTOM,
    BOUNCE_LEFT,
    BOUNCE_RIGHT,
}

/// 滚动视图回调类型
pub type ScrollEventCallback = Box<dyn FnMut(&ScrollView, ScrollViewEventType)>;

/// ScrollView 滚动视图组件
/// 
/// 提供滚动容器功能，支持：
/// - 垂直/水平/双向滚动
/// - 惯性滚动
/// - 边界反弹效果
/// - 滚动条显示
/// - 滚动事件回调
#[derive(Debug)]
pub struct ScrollView {
    widget: Widget,
    inner_container: Node,
    direction: ScrollDirection,
    content_size: Vec2,
    inner_size: Vec2,
    inner_position: Vec2,
    
    // 滚动配置
    bounce_enabled: bool,
    inertia_scroll_enabled: bool,
    scroll_bar_enabled: bool,
    scroll_bar_auto_hide: bool,
    scroll_bar_opacity: f32,
    
    // 滚动状态
    is_scrolling: bool,
    is_auto_scrolling: bool,
    auto_scroll_duration: Duration,
    auto_scroll_elapsed: Duration,
    
    // 触摸和拖拽
    touch_began_position: Vec2,
    touch_moved_position: Vec2,
    touch_ended_position: Vec2,
    touch_move_distance: Vec2,
    
    // 惯性滚动
    inertia_scroll_velocity: Vec2,
    inertia_scroll_friction: f32,
    
    // 边界反弹
    bounce_duration: Duration,
    bounce_back_distance: f32,
    
    // 事件回调
    event_callback: Option<ScrollEventCallback>,
}

impl ScrollView {
    /// 创建新的滚动视图
    pub fn new() -> Self {
        ScrollView {
            widget: Widget::new(),
            inner_container: Node::new(),
            direction: ScrollDirection::VERTICAL,
            content_size: Vec2::ZERO,
            inner_size: Vec2::ZERO,
            inner_position: Vec2::ZERO,
            
            bounce_enabled: true,
            inertia_scroll_enabled: true,
            scroll_bar_enabled: true,
            scroll_bar_auto_hide: true,
            scroll_bar_opacity: 0.4,
            
            is_scrolling: false,
            is_auto_scrolling: false,
            auto_scroll_duration: Duration::from_millis(300),
            auto_scroll_elapsed: Duration::ZERO,
            
            touch_began_position: Vec2::ZERO,
            touch_moved_position: Vec2::ZERO,
            touch_ended_position: Vec2::ZERO,
            touch_move_distance: Vec2::ZERO,
            
            inertia_scroll_velocity: Vec2::ZERO,
            inertia_scroll_friction: 0.95,
            
            bounce_duration: Duration::from_millis(200),
            bounce_back_distance: 100.0,
            
            event_callback: None,
        }
    }
    
    /// 创建带方向的滚动视图
    pub fn create(direction: ScrollDirection) -> Self {
        let mut scroll_view = ScrollView::new();
        scroll_view.direction = direction;
        scroll_view
    }
    
    /// 设置滚动方向
    pub fn set_direction(&mut self, direction: ScrollDirection) {
        self.direction = direction;
    }
    
    /// 获取滚动方向
    pub fn get_direction(&self) -> ScrollDirection {
        self.direction
    }
    
    /// 设置内容大小
    pub fn set_inner_container_size(&mut self, size: Vec2) {
        self.inner_size = size;
        self.update_inner_container();
    }
    
    /// 获取内容大小
    pub fn get_inner_container_size(&self) -> Vec2 {
        self.inner_size
    }
    
    /// 设置内容位置
    pub fn set_inner_container_position(&mut self, position: Vec2) {
        self.inner_position = position;
        self.limit_inner_position();
        self.update_inner_container();
    }
    
    /// 获取内容位置
    pub fn get_inner_container_position(&self) -> Vec2 {
        self.inner_position
    }
    
    /// 启用/禁用边界反弹
    pub fn set_bounce_enabled(&mut self, enabled: bool) {
        self.bounce_enabled = enabled;
    }
    
    /// 检查边界反弹是否启用
    pub fn is_bounce_enabled(&self) -> bool {
        self.bounce_enabled
    }
    
    /// 启用/禁用惯性滚动
    pub fn set_inertia_scroll_enabled(&mut self, enabled: bool) {
        self.inertia_scroll_enabled = enabled;
    }
    
    /// 检查惯性滚动是否启用
    pub fn is_inertia_scroll_enabled(&self) -> bool {
        self.inertia_scroll_enabled
    }
    
    /// 启用/禁用滚动条
    pub fn set_scroll_bar_enabled(&mut self, enabled: bool) {
        self.scroll_bar_enabled = enabled;
    }
    
    /// 检查滚动条是否启用
    pub fn is_scroll_bar_enabled(&self) -> bool {
        self.scroll_bar_enabled
    }
    
    /// 设置滚动条自动隐藏
    pub fn set_scroll_bar_auto_hide_enabled(&mut self, enabled: bool) {
        self.scroll_bar_auto_hide = enabled;
    }
    
    /// 设置滚动条透明度
    pub fn set_scroll_bar_opacity(&mut self, opacity: f32) {
        self.scroll_bar_opacity = opacity.clamp(0.0, 1.0);
    }
    
    /// 滚动到顶部
    pub fn scroll_to_top(&mut self, time_in_sec: f32, attenuated: bool) {
        self.start_auto_scroll(Vec2::new(self.inner_position.x, 0.0), time_in_sec);
        self.trigger_event(ScrollViewEventType::SCROLL_TO_TOP);
    }
    
    /// 滚动到底部
    pub fn scroll_to_bottom(&mut self, time_in_sec: f32, attenuated: bool) {
        let min_y = self.content_size.y - self.inner_size.y;
        self.start_auto_scroll(Vec2::new(self.inner_position.x, min_y), time_in_sec);
        self.trigger_event(ScrollViewEventType::SCROLL_TO_BOTTOM);
    }
    
    /// 滚动到左侧
    pub fn scroll_to_left(&mut self, time_in_sec: f32, attenuated: bool) {
        self.start_auto_scroll(Vec2::new(0.0, self.inner_position.y), time_in_sec);
        self.trigger_event(ScrollViewEventType::SCROLL_TO_LEFT);
    }
    
    /// 滚动到右侧
    pub fn scroll_to_right(&mut self, time_in_sec: f32, attenuated: bool) {
        let min_x = self.content_size.x - self.inner_size.x;
        self.start_auto_scroll(Vec2::new(min_x, self.inner_position.y), time_in_sec);
        self.trigger_event(ScrollViewEventType::SCROLL_TO_RIGHT);
    }
    
    /// 滚动到指定百分比位置
    pub fn scroll_to_percent_vertical(&mut self, percent: f32, time_in_sec: f32, attenuated: bool) {
        let percent = percent.clamp(0.0, 100.0);
        let h = self.inner_size.y - self.content_size.y;
        let dest = Vec2::new(self.inner_position.x, h * percent / 100.0);
        self.start_auto_scroll(dest, time_in_sec);
    }
    
    /// 滚动到指定百分比位置（水平）
    pub fn scroll_to_percent_horizontal(&mut self, percent: f32, time_in_sec: f32, attenuated: bool) {
        let percent = percent.clamp(0.0, 100.0);
        let w = self.inner_size.x - self.content_size.x;
        let dest = Vec2::new(w * percent / 100.0, self.inner_position.y);
        self.start_auto_scroll(dest, time_in_sec);
    }
    
    /// 滚动到指定百分比位置（双向）
    pub fn scroll_to_percent_both_direction(&mut self, percent_h: f32, percent_v: f32, time_in_sec: f32, attenuated: bool) {
        let percent_h = percent_h.clamp(0.0, 100.0);
        let percent_v = percent_v.clamp(0.0, 100.0);
        let w = self.inner_size.x - self.content_size.x;
        let h = self.inner_size.y - self.content_size.y;
        let dest = Vec2::new(w * percent_h / 100.0, h * percent_v / 100.0);
        self.start_auto_scroll(dest, time_in_sec);
    }
    
    /// 跳转到顶部（无动画）
    pub fn jump_to_top(&mut self) {
        self.inner_position.y = 0.0;
        self.update_inner_container();
    }
    
    /// 跳转到底部（无动画）
    pub fn jump_to_bottom(&mut self) {
        self.inner_position.y = self.content_size.y - self.inner_size.y;
        self.update_inner_container();
    }
    
    /// 跳转到左侧（无动画）
    pub fn jump_to_left(&mut self) {
        self.inner_position.x = 0.0;
        self.update_inner_container();
    }
    
    /// 跳转到右侧（无动画）
    pub fn jump_to_right(&mut self) {
        self.inner_position.x = self.content_size.x - self.inner_size.x;
        self.update_inner_container();
    }
    
    /// 设置事件回调
    pub fn set_event_callback(&mut self, callback: ScrollEventCallback) {
        self.event_callback = Some(callback);
    }
    
    /// 更新内容容器
    fn update_inner_container(&mut self) {
        // 更新内部容器的位置
        self.inner_container.set_position(self.inner_position);
    }
    
    /// 限制内容位置在合法范围内
    fn limit_inner_position(&mut self) {
        let min_x = self.content_size.x - self.inner_size.x;
        let min_y = self.content_size.y - self.inner_size.y;
        
        match self.direction {
            ScrollDirection::VERTICAL => {
                self.inner_position.y = self.inner_position.y.clamp(min_y.min(0.0), 0.0);
            }
            ScrollDirection::HORIZONTAL => {
                self.inner_position.x = self.inner_position.x.clamp(min_x.min(0.0), 0.0);
            }
            ScrollDirection::BOTH => {
                self.inner_position.x = self.inner_position.x.clamp(min_x.min(0.0), 0.0);
                self.inner_position.y = self.inner_position.y.clamp(min_y.min(0.0), 0.0);
            }
            ScrollDirection::NONE => {}
        }
    }
    
    /// 开始自动滚动
    fn start_auto_scroll(&mut self, dest: Vec2, time_in_sec: f32) {
        self.is_auto_scrolling = true;
        self.auto_scroll_duration = Duration::from_secs_f32(time_in_sec);
        self.auto_scroll_elapsed = Duration::ZERO;
    }
    
    /// 触发滚动事件
    fn trigger_event(&mut self, event_type: ScrollViewEventType) {
        if let Some(ref mut callback) = self.event_callback {
            callback(self, event_type);
        }
    }
    
    /// 更新滚动状态
    pub fn update(&mut self, dt: f32) {
        // 更新自动滚动
        if self.is_auto_scrolling {
            self.auto_scroll_elapsed += Duration::from_secs_f32(dt);
            if self.auto_scroll_elapsed >= self.auto_scroll_duration {
                self.is_auto_scrolling = false;
                self.trigger_event(ScrollViewEventType::SCROLL_ENDED);
            }
        }
        
        // 更新惯性滚动
        if self.inertia_scroll_enabled && !self.is_auto_scrolling {
            if self.inertia_scroll_velocity.length() > 0.1 {
                self.inner_position += self.inertia_scroll_velocity * dt;
                self.limit_inner_position();
                self.update_inner_container();
                
                self.inertia_scroll_velocity *= self.inertia_scroll_friction;
                self.trigger_event(ScrollViewEventType::SCROLLING);
            } else {
                self.inertia_scroll_velocity = Vec2::ZERO;
            }
        }
    }
    
    /// 获取 Widget
    pub fn get_widget(&self) -> &Widget {
        &self.widget
    }
    
    /// 获取可变 Widget
    pub fn get_widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
    
    /// 获取内部容器
    pub fn get_inner_container(&self) -> &Node {
        &self.inner_container
    }
    
    /// 获取可变内部容器
    pub fn get_inner_container_mut(&mut self) -> &mut Node {
        &mut self.inner_container
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scroll_view_creation() {
        let scroll_view = ScrollView::new();
        assert_eq!(scroll_view.get_direction(), ScrollDirection::VERTICAL);
        assert!(scroll_view.is_bounce_enabled());
        assert!(scroll_view.is_inertia_scroll_enabled());
    }
    
    #[test]
    fn test_scroll_direction() {
        let mut scroll_view = ScrollView::new();
        scroll_view.set_direction(ScrollDirection::HORIZONTAL);
        assert_eq!(scroll_view.get_direction(), ScrollDirection::HORIZONTAL);
    }
    
    #[test]
    fn test_inner_container_size() {
        let mut scroll_view = ScrollView::new();
        let size = Vec2::new(800.0, 1200.0);
        scroll_view.set_inner_container_size(size);
        assert_eq!(scroll_view.get_inner_container_size(), size);
    }
    
    #[test]
    fn test_bounce_enabled() {
        let mut scroll_view = ScrollView::new();
        scroll_view.set_bounce_enabled(false);
        assert!(!scroll_view.is_bounce_enabled());
    }
    
    #[test]
    fn test_scroll_bar() {
        let mut scroll_view = ScrollView::new();
        scroll_view.set_scroll_bar_enabled(false);
        assert!(!scroll_view.is_scroll_bar_enabled());
        
        scroll_view.set_scroll_bar_opacity(0.5);
        assert_eq!(scroll_view.scroll_bar_opacity, 0.5);
    }
}
