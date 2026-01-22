use crate::base::Node;
use crate::math::Vec2;
use crate::ui::Widget;
use super::scroll_view::{ScrollView, ScrollDirection};

/// 列表视图重力（对齐方式）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListViewGravity {
    LEFT,
    RIGHT,
    CENTER_HORIZONTAL,
    TOP,
    BOTTOM,
    CENTER_VERTICAL,
}

/// 列表视图事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListViewEventType {
    ON_SELECTED_ITEM_START,
    ON_SELECTED_ITEM_END,
}

/// 列表项回调类型
pub type ListItemCallback = Box<dyn FnMut(&ListView, usize, ListViewEventType)>;

/// ListView 列表视图组件
/// 
/// 基于 ScrollView，提供列表项管理功能：
/// - 垂直/水平列表布局
/// - 列表项自动排列
/// - 列表项选择回调
/// - 动态添加/删除列表项
#[derive(Debug)]
pub struct ListView {
    scroll_view: ScrollView,
    items: Vec<Node>,
    item_gravity: ListViewGravity,
    item_spacing: f32,
    selected_index: Option<usize>,
    event_callback: Option<ListItemCallback>,
}

impl ListView {
    /// 创建新的列表视图
    pub fn new() -> Self {
        ListView {
            scroll_view: ScrollView::create(ScrollDirection::VERTICAL),
            items: Vec::new(),
            item_gravity: ListViewGravity::CENTER_HORIZONTAL,
            item_spacing: 0.0,
            selected_index: None,
            event_callback: None,
        }
    }
    
    /// 创建带方向的列表视图
    pub fn create(direction: ScrollDirection) -> Self {
        let mut list_view = ListView::new();
        list_view.scroll_view.set_direction(direction);
        list_view
    }
    
    /// 设置列表方向
    pub fn set_direction(&mut self, direction: ScrollDirection) {
        self.scroll_view.set_direction(direction);
        self.refresh_view();
    }
    
    /// 获取列表方向
    pub fn get_direction(&self) -> ScrollDirection {
        self.scroll_view.get_direction()
    }
    
    /// 设置列表项对齐方式
    pub fn set_gravity(&mut self, gravity: ListViewGravity) {
        self.item_gravity = gravity;
        self.refresh_view();
    }
    
    /// 获取列表项对齐方式
    pub fn get_gravity(&self) -> ListViewGravity {
        self.item_gravity
    }
    
    /// 设置列表项间距
    pub fn set_item_spacing(&mut self, spacing: f32) {
        self.item_spacing = spacing;
        self.refresh_view();
    }
    
    /// 获取列表项间距
    pub fn get_item_spacing(&self) -> f32 {
        self.item_spacing
    }
    
    /// 添加列表项
    pub fn push_back_custom_item(&mut self, item: Node) {
        self.items.push(item);
        self.refresh_view();
    }
    
    /// 在指定位置插入列表项
    pub fn insert_custom_item(&mut self, item: Node, index: usize) {
        if index <= self.items.len() {
            self.items.insert(index, item);
            self.refresh_view();
        }
    }
    
    /// 移除指定位置的列表项
    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
            self.refresh_view();
        }
    }
    
    /// 移除最后一个列表项
    pub fn remove_last_item(&mut self) {
        if !self.items.is_empty() {
            self.items.pop();
            self.refresh_view();
        }
    }
    
    /// 移除所有列表项
    pub fn remove_all_items(&mut self) {
        self.items.clear();
        self.selected_index = None;
        self.refresh_view();
    }
    
    /// 获取指定位置的列表项
    pub fn get_item(&self, index: usize) -> Option<&Node> {
        self.items.get(index)
    }
    
    /// 获取可变列表项
    pub fn get_item_mut(&mut self, index: usize) -> Option<&mut Node> {
        self.items.get_mut(index)
    }
    
    /// 获取列表项数量
    pub fn get_items_count(&self) -> usize {
        self.items.len()
    }
    
    /// 获取所有列表项
    pub fn get_items(&self) -> &Vec<Node> {
        &self.items
    }
    
    /// 获取选中的索引
    pub fn get_current_selected_index(&self) -> Option<usize> {
        self.selected_index
    }
    
    /// 设置选中的索引
    pub fn set_current_selected_index(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = Some(index);
            self.trigger_event(index, ListViewEventType::ON_SELECTED_ITEM_START);
        }
    }
    
    /// 设置事件回调
    pub fn set_event_callback(&mut self, callback: ListItemCallback) {
        self.event_callback = Some(callback);
    }
    
    /// 滚动到指定项
    pub fn scroll_to_item(&mut self, index: usize, time_in_sec: f32, attenuated: bool) {
        if index >= self.items.len() {
            return;
        }
        
        let direction = self.scroll_view.get_direction();
        match direction {
            ScrollDirection::VERTICAL => {
                let percent = (index as f32 / self.items.len() as f32) * 100.0;
                self.scroll_view.scroll_to_percent_vertical(percent, time_in_sec, attenuated);
            }
            ScrollDirection::HORIZONTAL => {
                let percent = (index as f32 / self.items.len() as f32) * 100.0;
                self.scroll_view.scroll_to_percent_horizontal(percent, time_in_sec, attenuated);
            }
            _ => {}
        }
    }
    
    /// 跳转到指定项（无动画）
    pub fn jump_to_item(&mut self, index: usize) {
        self.scroll_to_item(index, 0.0, false);
    }
    
    /// 刷新列表视图布局
    fn refresh_view(&mut self) {
        let direction = self.scroll_view.get_direction();
        let mut total_size = 0.0;
        
        match direction {
            ScrollDirection::VERTICAL => {
                let mut current_y = 0.0;
                for (i, item) in self.items.iter_mut().enumerate() {
                    let item_size = item.get_content_size();
                    
                    // 设置垂直位置
                    current_y -= item_size.y / 2.0;
                    
                    // 设置水平位置（根据对齐方式）
                    let x = match self.item_gravity {
                        ListViewGravity::LEFT => item_size.x / 2.0,
                        ListViewGravity::RIGHT => {
                            let container_width = self.scroll_view.get_widget().get_size().x;
                            container_width - item_size.x / 2.0
                        }
                        ListViewGravity::CENTER_HORIZONTAL => {
                            let container_width = self.scroll_view.get_widget().get_size().x;
                            container_width / 2.0
                        }
                        _ => item_size.x / 2.0,
                    };
                    
                    item.set_position(Vec2::new(x, current_y));
                    
                    current_y -= item_size.y / 2.0;
                    if i < self.items.len() - 1 {
                        current_y -= self.item_spacing;
                    }
                    
                    total_size += item_size.y + self.item_spacing;
                }
                
                // 更新内部容器大小
                let container_width = self.scroll_view.get_widget().get_size().x;
                self.scroll_view.set_inner_container_size(Vec2::new(container_width, total_size));
            }
            
            ScrollDirection::HORIZONTAL => {
                let mut current_x = 0.0;
                for (i, item) in self.items.iter_mut().enumerate() {
                    let item_size = item.get_content_size();
                    
                    // 设置水平位置
                    current_x += item_size.x / 2.0;
                    
                    // 设置垂直位置（根据对齐方式）
                    let y = match self.item_gravity {
                        ListViewGravity::TOP => {
                            let container_height = self.scroll_view.get_widget().get_size().y;
                            container_height - item_size.y / 2.0
                        }
                        ListViewGravity::BOTTOM => item_size.y / 2.0,
                        ListViewGravity::CENTER_VERTICAL => {
                            let container_height = self.scroll_view.get_widget().get_size().y;
                            container_height / 2.0
                        }
                        _ => item_size.y / 2.0,
                    };
                    
                    item.set_position(Vec2::new(current_x, y));
                    
                    current_x += item_size.x / 2.0;
                    if i < self.items.len() - 1 {
                        current_x += self.item_spacing;
                    }
                    
                    total_size += item_size.x + self.item_spacing;
                }
                
                // 更新内部容器大小
                let container_height = self.scroll_view.get_widget().get_size().y;
                self.scroll_view.set_inner_container_size(Vec2::new(total_size, container_height));
            }
            
            _ => {}
        }
    }
    
    /// 触发列表项事件
    fn trigger_event(&mut self, index: usize, event_type: ListViewEventType) {
        if let Some(ref mut callback) = self.event_callback {
            callback(self, index, event_type);
        }
    }
    
    /// 更新列表视图
    pub fn update(&mut self, dt: f32) {
        self.scroll_view.update(dt);
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

impl Default for ListView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_list_view_creation() {
        let list_view = ListView::new();
        assert_eq!(list_view.get_items_count(), 0);
        assert_eq!(list_view.get_direction(), ScrollDirection::VERTICAL);
    }
    
    #[test]
    fn test_add_items() {
        let mut list_view = ListView::new();
        list_view.push_back_custom_item(Node::new());
        list_view.push_back_custom_item(Node::new());
        assert_eq!(list_view.get_items_count(), 2);
    }
    
    #[test]
    fn test_remove_items() {
        let mut list_view = ListView::new();
        list_view.push_back_custom_item(Node::new());
        list_view.push_back_custom_item(Node::new());
        list_view.remove_item(0);
        assert_eq!(list_view.get_items_count(), 1);
    }
    
    #[test]
    fn test_item_spacing() {
        let mut list_view = ListView::new();
        list_view.set_item_spacing(10.0);
        assert_eq!(list_view.get_item_spacing(), 10.0);
    }
    
    #[test]
    fn test_gravity() {
        let mut list_view = ListView::new();
        list_view.set_gravity(ListViewGravity::CENTER_HORIZONTAL);
        assert_eq!(list_view.get_gravity(), ListViewGravity::CENTER_HORIZONTAL);
    }
}
