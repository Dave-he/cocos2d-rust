use super::menu_item::MenuItem;
use crate::base::{Node, Ref, RefPtr};
use crate::math::Vec2;

/// Menu state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    WAITING,
    TRACKING_TOUCH,
}

/// Menu is a container for menu items
#[derive(Debug)]
pub struct Menu {
    node: RefPtr<Node>,
    items: Vec<RefPtr<MenuItem>>,
    selected_item: Option<RefPtr<MenuItem>>,
    state: MenuState,
    enabled: bool,
}

impl Menu {
    /// Creates a new menu
    pub fn new() -> Menu {
        Menu {
            node: RefPtr::new(Node::new()),
            items: Vec::new(),
            selected_item: None,
            state: MenuState::WAITING,
            enabled: true,
        }
    }

    /// Creates a menu with items
    pub fn create_with_items(items: Vec<RefPtr<MenuItem>>) -> Menu {
        let mut menu = Menu::new();
        for item in items {
            menu.add_item(item);
        }
        menu
    }

    /// Adds a menu item
    pub fn add_item(&mut self, item: RefPtr<MenuItem>) {
        self.items.push(item);
        self.update_item_positions();
    }

    /// Removes a menu item
    pub fn remove_item(&mut self, item: &RefPtr<MenuItem>) {
        self.items.retain(|i| !RefPtr::ptr_eq(i, item));
        self.update_item_positions();
    }

    /// Removes all items
    pub fn remove_all_items(&mut self) {
        self.items.clear();
        self.selected_item = None;
    }

    /// Gets menu items
    pub fn get_items(&self) -> &Vec<RefPtr<MenuItem>> {
        &self.items
    }

    /// Gets menu items mutably
    pub fn get_items_mut(&mut self) -> &mut Vec<RefPtr<MenuItem>> {
        &mut self.items
    }

    /// Sets the menu enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Checks if menu is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Aligns items vertically
    pub fn align_items_vertically(&mut self) {
        self.align_items_vertically_with_padding(0.0);
    }

    /// Aligns items vertically with padding
    pub fn align_items_vertically_with_padding(&mut self, padding: f32) {
        let mut height = -(self.items.len() as f32 - 1.0) * padding / 2.0;

        for item in &self.items {
            let item_height = item.borrow().get_node().borrow().get_content_size().y;
            height -= item_height / 2.0;
        }

        let mut pos_y = height;
        for item in &mut self.items {
            let item_height = item.borrow().get_node().borrow().get_content_size().y;
            pos_y += item_height / 2.0;
            item.borrow_mut()
                .get_node_mut()
                .borrow_mut()
                .set_position(Vec2::new(0.0, pos_y));
            pos_y += item_height / 2.0 + padding;
        }
    }

    /// Aligns items horizontally
    pub fn align_items_horizontally(&mut self) {
        self.align_items_horizontally_with_padding(0.0);
    }

    /// Aligns items horizontally with padding
    pub fn align_items_horizontally_with_padding(&mut self, padding: f32) {
        let mut width = -(self.items.len() as f32 - 1.0) * padding / 2.0;

        for item in &self.items {
            let item_width = item.borrow().get_node().borrow().get_content_size().x;
            width -= item_width / 2.0;
        }

        let mut pos_x = width;
        for item in &mut self.items {
            let item_width = item.borrow().get_node().borrow().get_content_size().x;
            pos_x += item_width / 2.0;
            item.borrow_mut()
                .get_node_mut()
                .borrow_mut()
                .set_position(Vec2::new(pos_x, 0.0));
            pos_x += item_width / 2.0 + padding;
        }
    }

    /// Aligns items in columns
    pub fn align_items_in_columns(&mut self, columns: &[usize]) {
        let mut height = 0.0;
        let mut row = 0;
        let mut row_height: f32 = 0.0;
        let mut row_columns = 0;
        let mut tmp = 0;

        for &column_count in columns {
            if column_count == 0 {
                break;
            }

            row_columns = column_count;

            for _ in 0..column_count {
                if tmp >= self.items.len() {
                    break;
                }

                let item_height = self.items[tmp]
                    .borrow()
                    .get_node()
                    .borrow()
                    .get_content_size()
                    .y;
                row_height = row_height.max(item_height);
                tmp += 1;
            }

            height += row_height;
            row += 1;
        }

        // Position items
        let mut pos_y = height / 2.0;
        row = 0;
        tmp = 0;

        for &column_count in columns {
            if column_count == 0 {
                break;
            }

            row_height = 0.0;

            for _ in 0..column_count {
                if tmp >= self.items.len() {
                    break;
                }

                let item_height = self.items[tmp]
                    .borrow()
                    .get_node()
                    .borrow()
                    .get_content_size()
                    .y;
                row_height = row_height.max(item_height);
                tmp += 1;
            }

            pos_y -= row_height / 2.0;

            for col in 0..column_count {
                let index = row * row_columns + col;
                if index >= self.items.len() {
                    break;
                }

                let pos_x = 0.0; // Center horizontally for now
                self.items[index]
                    .borrow_mut()
                    .get_node_mut()
                    .borrow_mut()
                    .set_position(Vec2::new(pos_x, pos_y));
            }

            pos_y -= row_height / 2.0;
            row += 1;
        }
    }

    /// Aligns items in rows
    pub fn align_items_in_rows(&mut self, rows: &[usize]) {
        let mut width = 0.0;
        let mut column = 0;
        let mut column_width: f32 = 0.0;
        let mut column_rows = 0;
        let mut tmp = 0;

        for &row_count in rows {
            if row_count == 0 {
                break;
            }

            column_rows = row_count;

            for _ in 0..row_count {
                if tmp >= self.items.len() {
                    break;
                }

                let item_width = self.items[tmp]
                    .borrow()
                    .get_node()
                    .borrow()
                    .get_content_size()
                    .x;
                column_width = column_width.max(item_width);
                tmp += 1;
            }

            width += column_width;
            column += 1;
        }

        // Position items
        let mut pos_x = -width / 2.0;
        column = 0;
        tmp = 0;

        for &row_count in rows {
            if row_count == 0 {
                break;
            }

            column_width = 0.0;

            for _ in 0..row_count {
                if tmp >= self.items.len() {
                    break;
                }

                let item_width = self.items[tmp]
                    .borrow()
                    .get_node()
                    .borrow()
                    .get_content_size()
                    .x;
                column_width = column_width.max(item_width);
                tmp += 1;
            }

            pos_x += column_width / 2.0;

            for row in 0..row_count {
                let index = column * column_rows + row;
                if index >= self.items.len() {
                    break;
                }

                let pos_y = 0.0; // Center vertically for now
                self.items[index]
                    .borrow_mut()
                    .get_node_mut()
                    .borrow_mut()
                    .set_position(Vec2::new(pos_x, pos_y));
            }

            pos_x += column_width / 2.0;
            column += 1;
        }
    }

    /// Updates item positions
    fn update_item_positions(&mut self) {
        // Default vertical alignment
        self.align_items_vertically();
    }

    /// Gets the node
    pub fn get_node(&self) -> &RefPtr<Node> {
        &self.node
    }

    /// Gets the node mutably
    pub fn get_node_mut(&mut self) -> &mut RefPtr<Node> {
        &mut self.node
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_new() {
        let menu = Menu::new();
        assert!(menu.is_enabled());
        assert!(menu.get_items().is_empty());
    }

    #[test]
    fn test_menu_add_item() {
        let mut menu = Menu::new();
        let item = RefPtr::new(MenuItem::new());
        menu.add_item(item.clone());
        assert_eq!(menu.get_items().len(), 1);
    }

    #[test]
    fn test_menu_remove_item() {
        let mut menu = Menu::new();
        let item = RefPtr::new(MenuItem::new());
        menu.add_item(item.clone());
        menu.remove_item(&item);
        assert!(menu.get_items().is_empty());
    }

    #[test]
    fn test_menu_remove_all_items() {
        let mut menu = Menu::new();
        menu.add_item(RefPtr::new(MenuItem::new()));
        menu.add_item(RefPtr::new(MenuItem::new()));
        menu.remove_all_items();
        assert!(menu.get_items().is_empty());
    }

    #[test]
    fn test_menu_set_enabled() {
        let mut menu = Menu::new();
        assert!(menu.is_enabled());
        menu.set_enabled(false);
        assert!(!menu.is_enabled());
    }

    #[test]
    fn test_menu_align_vertically() {
        let mut menu = Menu::new();
        menu.add_item(RefPtr::new(MenuItem::new()));
        menu.add_item(RefPtr::new(MenuItem::new()));
        menu.align_items_vertically();
    }
}
