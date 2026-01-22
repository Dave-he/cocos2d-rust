use crate::base::{Node, Ref, RefPtr};
use crate::base::types::Color3B;
use crate::math::Vec2;
use crate::sprite::Sprite;
use crate::label::Label;

/// Callback function type for menu items
pub type MenuCallback = Box<dyn Fn(&MenuItem)>;

/// MenuItem is the base class for all menu items
#[derive(Debug)]
pub struct MenuItem {
    node: Node,
    enabled: bool,
    selected: bool,
    callback: Option<MenuCallback>,
}

impl MenuItem {
    /// Creates a new menu item
    pub fn new() -> MenuItem {
        MenuItem {
            node: Node::new(),
            enabled: true,
            selected: false,
            callback: None,
        }
    }

    /// Creates a menu item with a callback
    pub fn create_with_callback(callback: MenuCallback) -> MenuItem {
        let mut item = MenuItem::new();
        item.set_callback(callback);
        item
    }

    /// Sets the callback
    pub fn set_callback(&mut self, callback: MenuCallback) {
        self.callback = Some(callback);
    }

    /// Activates the menu item
    pub fn activate(&self) {
        if self.enabled {
            if let Some(ref callback) = self.callback {
                callback(self);
            }
        }
    }

    /// Selects the menu item
    pub fn selected(&mut self) {
        self.selected = true;
    }

    /// Unselects the menu item
    pub fn unselected(&mut self) {
        self.selected = false;
    }

    /// Sets enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Checks if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Checks if selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Gets the node
    pub fn get_node(&self) -> &Node {
        &self.node
    }

    /// Gets the node mutably
    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::new()
    }
}

/// MenuItemLabel is a menu item with a label
#[derive(Debug)]
pub struct MenuItemLabel {
    base: MenuItem,
    label: RefPtr<Label>,
    original_scale: f32,
    disabled_color: Color3B,
}

impl MenuItemLabel {
    /// Creates a new menu item label
    pub fn new(label: RefPtr<Label>) -> MenuItemLabel {
        MenuItemLabel {
            base: MenuItem::new(),
            label,
            original_scale: 1.0,
            disabled_color: Color3B::new(126, 126, 126),
        }
    }

    /// Creates a menu item label with callback
    pub fn create(label: RefPtr<Label>, callback: MenuCallback) -> MenuItemLabel {
        let mut item = MenuItemLabel::new(label);
        item.base.set_callback(callback);
        item
    }

    /// Gets the label
    pub fn get_label(&self) -> &RefPtr<Label> {
        &self.label
    }

    /// Gets the label mutably
    pub fn get_label_mut(&mut self) -> &mut RefPtr<Label> {
        &mut self.label
    }

    /// Sets the string
    pub fn set_string(&mut self, text: &str) {
        self.label.set_string(text);
    }

    /// Sets disabled color
    pub fn set_disabled_color(&mut self, color: Color3B) {
        self.disabled_color = color;
    }

    /// Gets disabled color
    pub fn get_disabled_color(&self) -> Color3B {
        self.disabled_color
    }
}

/// MenuItemImage is a menu item with images
#[derive(Debug)]
pub struct MenuItemImage {
    base: MenuItem,
    normal_image: Option<RefPtr<Sprite>>,
    selected_image: Option<RefPtr<Sprite>>,
    disabled_image: Option<RefPtr<Sprite>>,
}

impl MenuItemImage {
    /// Creates a new menu item image
    pub fn new() -> MenuItemImage {
        MenuItemImage {
            base: MenuItem::new(),
            normal_image: None,
            selected_image: None,
            disabled_image: None,
        }
    }

    /// Creates a menu item image with image files
    pub fn create(
        normal_image: &str,
        selected_image: &str,
        disabled_image: Option<&str>,
        callback: MenuCallback,
    ) -> MenuItemImage {
        let mut item = MenuItemImage::new();
        item.init_with_images(normal_image, selected_image, disabled_image);
        item.base.set_callback(callback);
        item
    }

    /// Initializes with image files
    pub fn init_with_images(
        &mut self,
        normal_image: &str,
        selected_image: &str,
        disabled_image: Option<&str>,
    ) -> bool {
        // Load sprites
        // self.normal_image = Some(Sprite::create(normal_image));
        // self.selected_image = Some(Sprite::create(selected_image));
        // if let Some(disabled) = disabled_image {
        //     self.disabled_image = Some(Sprite::create(disabled));
        // }
        true
    }

    /// Sets the normal image
    pub fn set_normal_image(&mut self, sprite: RefPtr<Sprite>) {
        self.normal_image = Some(sprite);
    }

    /// Sets the selected image
    pub fn set_selected_image(&mut self, sprite: RefPtr<Sprite>) {
        self.selected_image = Some(sprite);
    }

    /// Sets the disabled image
    pub fn set_disabled_image(&mut self, sprite: RefPtr<Sprite>) {
        self.disabled_image = Some(sprite);
    }
}

impl Default for MenuItemImage {
    fn default() -> Self {
        Self::new()
    }
}

/// MenuItemSprite is a menu item with sprites
#[derive(Debug)]
pub struct MenuItemSprite {
    base: MenuItem,
    normal_sprite: Option<RefPtr<Sprite>>,
    selected_sprite: Option<RefPtr<Sprite>>,
    disabled_sprite: Option<RefPtr<Sprite>>,
}

impl MenuItemSprite {
    /// Creates a new menu item sprite
    pub fn new() -> MenuItemSprite {
        MenuItemSprite {
            base: MenuItem::new(),
            normal_sprite: None,
            selected_sprite: None,
            disabled_sprite: None,
        }
    }

    /// Creates a menu item sprite with sprites
    pub fn create(
        normal_sprite: RefPtr<Sprite>,
        selected_sprite: RefPtr<Sprite>,
        disabled_sprite: Option<RefPtr<Sprite>>,
        callback: MenuCallback,
    ) -> MenuItemSprite {
        let mut item = MenuItemSprite::new();
        item.normal_sprite = Some(normal_sprite);
        item.selected_sprite = Some(selected_sprite);
        item.disabled_sprite = disabled_sprite;
        item.base.set_callback(callback);
        item
    }

    /// Sets the normal sprite
    pub fn set_normal_sprite(&mut self, sprite: RefPtr<Sprite>) {
        self.normal_sprite = Some(sprite);
    }

    /// Sets the selected sprite
    pub fn set_selected_sprite(&mut self, sprite: RefPtr<Sprite>) {
        self.selected_sprite = Some(sprite);
    }

    /// Sets the disabled sprite
    pub fn set_disabled_sprite(&mut self, sprite: RefPtr<Sprite>) {
        self.disabled_sprite = Some(sprite);
    }
}

impl Default for MenuItemSprite {
    fn default() -> Self {
        Self::new()
    }
}

/// MenuItemToggle is a menu item that can toggle between sub items
#[derive(Debug)]
pub struct MenuItemToggle {
    base: MenuItem,
    sub_items: Vec<RefPtr<MenuItem>>,
    selected_index: usize,
}

impl MenuItemToggle {
    /// Creates a new menu item toggle
    pub fn new() -> MenuItemToggle {
        MenuItemToggle {
            base: MenuItem::new(),
            sub_items: Vec::new(),
            selected_index: 0,
        }
    }

    /// Creates a menu item toggle with items
    pub fn create(items: Vec<RefPtr<MenuItem>>, callback: MenuCallback) -> MenuItemToggle {
        let mut item = MenuItemToggle::new();
        item.sub_items = items;
        item.base.set_callback(callback);
        item
    }

    /// Adds a sub item
    pub fn add_sub_item(&mut self, item: RefPtr<MenuItem>) {
        self.sub_items.push(item);
    }

    /// Gets the selected index
    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    /// Sets the selected index
    pub fn set_selected_index(&mut self, index: usize) {
        if index < self.sub_items.len() {
            self.selected_index = index;
        }
    }

    /// Gets the selected item
    pub fn get_selected_item(&self) -> Option<&RefPtr<MenuItem>> {
        self.sub_items.get(self.selected_index)
    }

    /// Gets sub items
    pub fn get_sub_items(&self) -> &Vec<RefPtr<MenuItem>> {
        &self.sub_items
    }
}

impl Default for MenuItemToggle {
    fn default() -> Self {
        Self::new()
    }
}
