use crate::base::Node;
use crate::math::Vec2;
use super::scroll_view::{ScrollView, ScrollDirection};

/// 翻页视图事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageViewEventType {
    TURNING,
    TURNED,
}

/// 翻页回调类型
pub type PageTurnCallback = Box<dyn FnMut(&PageView, usize, PageViewEventType)>;

/// PageView 翻页视图组件
/// 
/// 基于 ScrollView，提供翻页功能：
/// - 页面自动对齐
/// - 页面切换动画
/// - 页面指示器支持
/// - 触摸翻页
#[derive(Debug)]
pub struct PageView {
    scroll_view: ScrollView,
    pages: Vec<Node>,
    current_page_index: usize,
    auto_scroll_stop_epsilon: f32,
    indicator_enabled: bool,
    indicator_position: Vec2,
    indicator_spacing: f32,
    event_callback: Option<PageTurnCallback>,
}

impl PageView {
    /// 创建新的翻页视图
    pub fn new() -> Self {
        PageView {
            scroll_view: ScrollView::create(ScrollDirection::HORIZONTAL),
            pages: Vec::new(),
            current_page_index: 0,
            auto_scroll_stop_epsilon: 0.001,
            indicator_enabled: true,
            indicator_position: Vec2::new(0.0, -20.0),
            indicator_spacing: 10.0,
            event_callback: None,
        }
    }
    
    /// 创建带方向的翻页视图
    pub fn create(direction: ScrollDirection) -> Self {
        let mut page_view = PageView::new();
        page_view.scroll_view.set_direction(direction);
        page_view
    }
    
    /// 添加页面
    pub fn add_page(&mut self, page: Node) {
        self.pages.push(page);
        self.update_pages_layout();
    }
    
    /// 在指定位置插入页面
    pub fn insert_page(&mut self, page: Node, index: usize) {
        if index <= self.pages.len() {
            self.pages.insert(index, page);
            self.update_pages_layout();
        }
    }
    
    /// 移除指定位置的页面
    pub fn remove_page(&mut self, index: usize) {
        if index < self.pages.len() {
            self.pages.remove(index);
            if self.current_page_index >= self.pages.len() && self.current_page_index > 0 {
                self.current_page_index = self.pages.len() - 1;
            }
            self.update_pages_layout();
        }
    }
    
    /// 移除所有页面
    pub fn remove_all_pages(&mut self) {
        self.pages.clear();
        self.current_page_index = 0;
        self.update_pages_layout();
    }
    
    /// 获取指定位置的页面
    pub fn get_page(&self, index: usize) -> Option<&Node> {
        self.pages.get(index)
    }
    
    /// 获取可变页面
    pub fn get_page_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.pages.get_mut(index)
    }
    
    /// 获取页面数量
    pub fn get_pages_count(&self) -> usize {
        self.pages.len()
    }
    
    /// 获取所有页面
    pub fn get_pages(&self) -> &Vec<Node> {
        &self.pages
    }
    
    /// 获取当前页面索引
    pub fn get_current_page_index(&self) -> usize {
        self.current_page_index
    }
    
    /// 滚动到指定页面
    pub fn scroll_to_page(&mut self, index: usize) {
        if index >= self.pages.len() {
            return;
        }
        
        self.current_page_index = index;
        
        let direction = self.scroll_view.get_direction();
        match direction {
            ScrollDirection::HORIZONTAL => {
                let page_width = self.scroll_view.get_widget().get_size().x;
                let dest_x = -(index as f32 * page_width);
                self.scroll_view.set_inner_container_position(Vec2::new(dest_x, 0.0));
            }
            ScrollDirection::VERTICAL => {
                let page_height = self.scroll_view.get_widget().get_size().y;
                let dest_y = -(index as f32 * page_height);
                self.scroll_view.set_inner_container_position(Vec2::new(0.0, dest_y));
            }
            _ => {}
        }
        
        self.trigger_event(index, PageViewEventType::TURNING);
        self.trigger_event(index, PageViewEventType::TURNED);
    }
    
    /// 滚动到指定页面（带动画）
    pub fn scroll_to_page_with_time(&mut self, index: usize, time: f32) {
        if index >= self.pages.len() {
            return;
        }
        
        let old_index = self.current_page_index;
        self.current_page_index = index;
        
        let direction = self.scroll_view.get_direction();
        match direction {
            ScrollDirection::HORIZONTAL => {
                let percent = (index as f32 / self.pages.len() as f32) * 100.0;
                self.scroll_view.scroll_to_percent_horizontal(percent, time, true);
            }
            ScrollDirection::VERTICAL => {
                let percent = (index as f32 / self.pages.len() as f32) * 100.0;
                self.scroll_view.scroll_to_percent_vertical(percent, time, true);
            }
            _ => {}
        }
        
        if old_index != index {
            self.trigger_event(index, PageViewEventType::TURNING);
        }
    }
    
    /// 滚动到下一页
    pub fn scroll_to_next_page(&mut self) {
        if self.current_page_index < self.pages.len() - 1 {
            self.scroll_to_page_with_time(self.current_page_index + 1, 0.3);
        }
    }
    
    /// 滚动到上一页
    pub fn scroll_to_previous_page(&mut self) {
        if self.current_page_index > 0 {
            self.scroll_to_page_with_time(self.current_page_index - 1, 0.3);
        }
    }
    
    /// 启用/禁用指示器
    pub fn set_indicator_enabled(&mut self, enabled: bool) {
        self.indicator_enabled = enabled;
    }
    
    /// 检查指示器是否启用
    pub fn is_indicator_enabled(&self) -> bool {
        self.indicator_enabled
    }
    
    /// 设置指示器位置
    pub fn set_indicator_position(&mut self, position: Vec2) {
        self.indicator_position = position;
    }
    
    /// 获取指示器位置
    pub fn get_indicator_position(&self) -> Vec2 {
        self.indicator_position
    }
    
    /// 设置指示器间距
    pub fn set_indicator_spacing(&mut self, spacing: f32) {
        self.indicator_spacing = spacing;
    }
    
    /// 获取指示器间距
    pub fn get_indicator_spacing(&self) -> f32 {
        self.indicator_spacing
    }
    
    /// 设置事件回调
    pub fn set_event_callback(&mut self, callback: PageTurnCallback) {
        self.event_callback = Some(callback);
    }
    
    /// 更新页面布局
    fn update_pages_layout(&mut self) {
        if self.pages.is_empty() {
            return;
        }
        
        let direction = self.scroll_view.get_direction();
        let container_size = self.scroll_view.get_widget().get_size();
        
        match direction {
            ScrollDirection::HORIZONTAL => {
                let page_width = container_size.x;
                let total_width = page_width * self.pages.len() as f32;
                
                for (i, page) in self.pages.iter_mut().enumerate() {
                    let x = page_width / 2.0 + i as f32 * page_width;
                    let y = container_size.y / 2.0;
                    page.set_position(Vec2::new(x, y));
                }
                
                self.scroll_view.set_inner_container_size(Vec2::new(total_width, container_size.y));
            }
            
            ScrollDirection::VERTICAL => {
                let page_height = container_size.y;
                let total_height = page_height * self.pages.len() as f32;
                
                for (i, page) in self.pages.iter_mut().enumerate() {
                    let x = container_size.x / 2.0;
                    let y = page_height / 2.0 + i as f32 * page_height;
                    page.set_position(Vec2::new(x, y));
                }
                
                self.scroll_view.set_inner_container_size(Vec2::new(container_size.x, total_height));
            }
            
            _ => {}
        }
    }
    
    /// 触发翻页事件
    fn trigger_event(&mut self, index: usize, event_type: PageViewEventType) {
        if let Some(ref mut callback) = self.event_callback {
            callback(self, index, event_type);
        }
    }
    
    /// 更新翻页视图
    pub fn update(&mut self, dt: f32) {
        self.scroll_view.update(dt);
        
        // 检查是否需要对齐到最近的页面
        self.check_page_alignment();
    }
    
    /// 检查页面对齐
    fn check_page_alignment(&mut self) {
        // 当滚动停止时，对齐到最近的页面
        let direction = self.scroll_view.get_direction();
        let position = self.scroll_view.get_inner_container_position();
        
        let current_index = match direction {
            ScrollDirection::HORIZONTAL => {
                let page_width = self.scroll_view.get_widget().get_size().x;
                ((-position.x / page_width).round() as usize).min(self.pages.len().saturating_sub(1))
            }
            ScrollDirection::VERTICAL => {
                let page_height = self.scroll_view.get_widget().get_size().y;
                ((-position.y / page_height).round() as usize).min(self.pages.len().saturating_sub(1))
            }
            _ => 0,
        };
        
        if current_index != self.current_page_index {
            let old_index = self.current_page_index;
            self.current_page_index = current_index;
            
            if old_index != current_index {
                self.trigger_event(current_index, PageViewEventType::TURNED);
            }
        }
    }
    
    /// 获取底层 ScrollView
    pub fn get_scroll_view(&self) -> &ScrollView {
        &self.scroll_view
    }
    
    /// 获取可变底层 ScrollView
    pub fn get_scroll_view_mut(&mut self) -> &mut ScrollView {
        &mut self.scroll_view
    }
}

impl Default for PageView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_page_view_creation() {
        let page_view = PageView::new();
        assert_eq!(page_view.get_pages_count(), 0);
        assert_eq!(page_view.get_current_page_index(), 0);
    }
    
    #[test]
    fn test_add_pages() {
        let mut page_view = PageView::new();
        page_view.add_page(Node::new());
        page_view.add_page(Node::new());
        assert_eq!(page_view.get_pages_count(), 2);
    }
    
    #[test]
    fn test_remove_pages() {
        let mut page_view = PageView::new();
        page_view.add_page(Node::new());
        page_view.add_page(Node::new());
        page_view.remove_page(0);
        assert_eq!(page_view.get_pages_count(), 1);
    }
    
    #[test]
    fn test_scroll_to_page() {
        let mut page_view = PageView::new();
        page_view.add_page(Node::new());
        page_view.add_page(Node::new());
        page_view.add_page(Node::new());
        
        page_view.scroll_to_page(1);
        assert_eq!(page_view.get_current_page_index(), 1);
    }
    
    #[test]
    fn test_indicator() {
        let mut page_view = PageView::new();
        page_view.set_indicator_enabled(false);
        assert!(!page_view.is_indicator_enabled());
        
        page_view.set_indicator_spacing(15.0);
        assert_eq!(page_view.get_indicator_spacing(), 15.0);
    }
}
