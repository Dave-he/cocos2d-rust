use super::widget::{Widget, LayoutParameter, WidgetSizeType};

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
    children: Vec<Ref<Widget>>,
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

    pub fn add_child(&mut self, child: Ref<Widget>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: &Ref<Widget>) {
        self.children.retain(|c| c.get_tag() != child.get_tag());
    }

    pub fn get_children(&self) -> &Vec<Ref<Widget>> {
        &self.children
    }

    pub fn request_layout(&mut self) {
    }
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
    CENTER_VERTICAL,
    CENTER_HORIZONTAL,
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
    ALIGN_NONE,
    ALIGN_PARENT_TOP_LEFT,
    ALIGN_PARENT_TOP_CENTER,
    ALIGN_PARENT_TOP_RIGHT,
    ALIGN_PARENT_LEFT_CENTER,
    ALIGN_PARENT_CENTER,
    ALIGN_PARENT_RIGHT_CENTER,
    ALIGN_PARENT_BOTTOM_LEFT,
    ALIGN_PARENT_BOTTOM_CENTER,
    ALIGN_PARENT_BOTTOM_RIGHT,
    LOCATION_ABOVE_LEFT,
    LOCATION_ABOVE_CENTER,
    LOCATION_ABOVE_RIGHT,
    LOCATION_LEFT_OF_TOP_LEFT,
    LOCATION_LEFT_OF_TOP_CENTER,
    LOCATION_LEFT_OF_TOP_RIGHT,
    LOCATION_LEFT_OF_CENTER,
    LOCATION_LEFT_OF_BOTTOM_LEFT,
    LOCATION_LEFT_OF_BOTTOM_CENTER,
    LOCATION_LEFT_OF_BOTTOM_RIGHT,
    LOCATION_RIGHT_OF_TOP_LEFT,
    LOCATION_RIGHT_OF_TOP_CENTER,
    LOCATION_RIGHT_OF_TOP_RIGHT,
    LOCATION_RIGHT_OF_CENTER,
    LOCATION_RIGHT_OF_BOTTOM_LEFT,
    LOCATION_RIGHT_OF_BOTTOM_CENTER,
    LOCATION_RIGHT_OF_BOTTOM_RIGHT,
    LOCATION_BELOW_TOP_LEFT,
    LOCATION_BELOW_TOP_CENTER,
    LOCATION_BELOW_TOP_RIGHT,
    LOCATION_CENTER_IN_PARENT,
    LOCATION_CENTER_HORIZONTAL,
    LOCATION_CENTER_VERTICAL,
}

impl RelativeLayout {
    pub fn new() -> RelativeLayout {
        RelativeLayout {
            layout: Layout::new(),
            relative_align: RelativeAlign::ALIGN_NONE,
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
