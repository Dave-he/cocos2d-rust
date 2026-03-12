#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use super::widget::Widget;
use crate::base::RefPtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    ABSOLUTE,
    VERTICAL,
    HORIZONTAL,
    GRID,
    RELATIVE,
}

#[derive(Debug)]
pub struct Layout {
    widget: Widget,
    layout_type: LayoutType,
    clipping_enabled: bool,
    clip_margin: f32,
    children: Vec<RefPtr<Widget>>,
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            widget: Widget::new(),
            layout_type: LayoutType::ABSOLUTE,
            clipping_enabled: false,
            clip_margin: 0.0,
            children: Vec::new(),
        }
    }

    pub fn set_layout_type(&mut self, layout_type: LayoutType) {
        self.layout_type = layout_type;
    }

    pub fn get_layout_type(&self) -> LayoutType {
        self.layout_type
    }

    pub fn set_clipping_enabled(&mut self, enabled: bool) {
        self.clipping_enabled = enabled;
    }

    pub fn is_clipping_enabled(&self) -> bool {
        self.clipping_enabled
    }

    pub fn add_child(&mut self, child: RefPtr<Widget>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &RefPtr<Widget>) {
        self.children
            .retain(|c| c.borrow().get_tag() != child.borrow().get_tag());
    }

    pub fn get_children(&self) -> &Vec<RefPtr<Widget>> {
        &self.children
    }

    pub fn request_layout(&mut self) {}
}

#[derive(Debug)]
pub struct LinearLayout {
    layout: Layout,
    gravity: LinearGravity,
    space: f32,
    padding_left: f32,
    padding_right: f32,
    padding_top: f32,
    padding_bottom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinearGravity {
    NONE,
    LEFT,
    TOP,
    RIGHT,
    BOTTOM,
    CenterVertical,
    CenterHorizontal,
}

impl Default for LinearLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearLayout {
    pub fn new() -> LinearLayout {
        LinearLayout {
            layout: Layout::new(),
            gravity: LinearGravity::NONE,
            space: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,
            padding_top: 0.0,
            padding_bottom: 0.0,
        }
    }

    pub fn set_gravity(&mut self, gravity: LinearGravity) {
        self.gravity = gravity;
    }

    pub fn get_gravity(&self) -> LinearGravity {
        self.gravity
    }

    pub fn set_space(&mut self, space: f32) {
        self.space = space;
    }

    pub fn get_space(&self) -> f32 {
        self.space
    }

    pub fn set_padding(&mut self, padding: f32) {
        self.padding_left = padding;
        self.padding_right = padding;
        self.padding_top = padding;
        self.padding_bottom = padding;
    }
}

#[derive(Debug)]
pub struct RelativeLayout {
    layout: Layout,
    relative_align: RelativeAlign,
    padding_left: f32,
    padding_right: f32,
    padding_top: f32,
    padding_bottom: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeAlign {
    AlignNone,
    AlignParentTopLeft,
    AlignParentTopCenter,
    AlignParentTopRight,
    AlignParentLeftCenter,
    AlignParentCenter,
    AlignParentRightCenter,
    AlignParentBottomLeft,
    AlignParentBottomCenter,
    AlignParentBottomRight,
    LocationAboveLeft,
    LocationAboveCenter,
    LocationAboveRight,
    LocationLeftOfTopLeft,
    LocationLeftOfTopCenter,
    LocationLeftOfTopRight,
    LocationLeftOfCenter,
    LocationLeftOfBottomLeft,
    LocationLeftOfBottomCenter,
    LocationLeftOfBottomRight,
    LocationRightOfTopLeft,
    LocationRightOfTopCenter,
    LocationRightOfTopRight,
    LocationRightOfCenter,
    LocationRightOfBottomLeft,
    LocationRightOfBottomCenter,
    LocationRightOfBottomRight,
    LocationBelowTopLeft,
    LocationBelowTopCenter,
    LocationBelowTopRight,
    LocationCenterInParent,
    LocationCenterHorizontal,
    LocationCenterVertical,
}

impl Default for RelativeLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl RelativeLayout {
    pub fn new() -> RelativeLayout {
        RelativeLayout {
            layout: Layout::new(),
            relative_align: RelativeAlign::AlignNone,
            padding_left: 0.0,
            padding_right: 0.0,
            padding_top: 0.0,
            padding_bottom: 0.0,
        }
    }

    pub fn set_align(&mut self, align: RelativeAlign) {
        self.relative_align = align;
    }

    pub fn get_align(&self) -> RelativeAlign {
        self.relative_align
    }
}

#[derive(Debug)]
pub struct GridLayout {
    layout: Layout,
    column_count: i32,
    row_count: i32,
    cell_size: (f32, f32),
    start_axis: AxisDirection,
    padding_left: f32,
    padding_right: f32,
    padding_top: f32,
    padding_bottom: f32,
    column_gap: f32,
    row_gap: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisDirection {
    HORIZONTAL,
    VERTICAL,
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl GridLayout {
    pub fn new() -> GridLayout {
        GridLayout {
            layout: Layout::new(),
            column_count: 2,
            row_count: 0,
            cell_size: (100.0, 100.0),
            start_axis: AxisDirection::HORIZONTAL,
            padding_left: 0.0,
            padding_right: 0.0,
            padding_top: 0.0,
            padding_bottom: 0.0,
            column_gap: 0.0,
            row_gap: 0.0,
        }
    }

    pub fn set_column_count(&mut self, count: i32) {
        self.column_count = count;
    }

    pub fn get_column_count(&self) -> i32 {
        self.column_count
    }

    pub fn set_row_count(&mut self, count: i32) {
        self.row_count = count;
    }

    pub fn get_row_count(&self) -> i32 {
        self.row_count
    }

    pub fn set_cell_size(&mut self, width: f32, height: f32) {
        self.cell_size = (width, height);
    }

    pub fn get_cell_size(&self) -> (f32, f32) {
        self.cell_size
    }

    pub fn set_start_axis(&mut self, axis: AxisDirection) {
        self.start_axis = axis;
    }

    pub fn get_start_axis(&self) -> AxisDirection {
        self.start_axis
    }
}
