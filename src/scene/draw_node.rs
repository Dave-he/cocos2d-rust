use crate::base::{Color4F, Rect};
use crate::math::Vec2;
use crate::renderer::Renderer;
use crate::scene::Node;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: Vec2,
    pub color: Color4F,
}

impl Vertex {
    pub fn new(position: Vec2, color: Color4F) -> Self {
        Self { position, color }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Dot {
        pos: Vec2,
        radius: f32,
        color: Color4F,
    },
    Line {
        from: Vec2,
        to: Vec2,
        color: Color4F,
    },
    Rect {
        rect: Rect,
        color: Color4F,
        filled: bool,
    },
    Circle {
        center: Vec2,
        radius: f32,
        angle: f32,
        segments: u32,
        draw_line_to_center: bool,
        color: Color4F,
        filled: bool,
    },
    Polygon {
        vertices: Vec<Vec2>,
        color: Color4F,
        filled: bool,
    },
    Triangle {
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        color: Color4F,
        filled: bool,
    },
    QuadBezier {
        origin: Vec2,
        control: Vec2,
        destination: Vec2,
        segments: u32,
        color: Color4F,
    },
    CubicBezier {
        origin: Vec2,
        control1: Vec2,
        control2: Vec2,
        destination: Vec2,
        segments: u32,
        color: Color4F,
    },
}

#[derive(Debug, Clone)]
pub struct DrawNode {
    node: Node,
    commands: Vec<DrawCommand>,
    line_width: f32,
    default_color: Color4F,
    vertices: Vec<Vertex>,
    dirty: bool,
}

impl DrawNode {
    pub fn new() -> Self {
        Self {
            node: Node::new(),
            commands: Vec::new(),
            line_width: 1.0,
            default_color: Color4F::WHITE,
            vertices: Vec::new(),
            dirty: false,
        }
    }

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.vertices.clear();
        self.dirty = true;
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.line_width = width.max(0.0);
    }

    pub fn get_line_width(&self) -> f32 {
        self.line_width
    }

    pub fn set_default_color(&mut self, color: Color4F) {
        self.default_color = color;
    }

    pub fn get_default_color(&self) -> Color4F {
        self.default_color
    }

    pub fn draw_dot(&mut self, pos: Vec2, radius: f32, color: Color4F) {
        self.commands.push(DrawCommand::Dot { pos, radius, color });
        self.dirty = true;
    }

    pub fn draw_line(&mut self, from: Vec2, to: Vec2, color: Color4F) {
        self.commands.push(DrawCommand::Line { from, to, color });
        self.dirty = true;
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color4F) {
        self.commands.push(DrawCommand::Rect {
            rect,
            color,
            filled: false,
        });
        self.dirty = true;
    }

    pub fn draw_solid_rect(&mut self, rect: Rect, color: Color4F) {
        self.commands.push(DrawCommand::Rect {
            rect,
            color,
            filled: true,
        });
        self.dirty = true;
    }

    pub fn draw_circle(&mut self, center: Vec2, radius: f32, angle: f32, segments: u32, draw_line_to_center: bool, color: Color4F) {
        self.commands.push(DrawCommand::Circle {
            center,
            radius,
            angle,
            segments,
            draw_line_to_center,
            color,
            filled: false,
        });
        self.dirty = true;
    }

    pub fn draw_solid_circle(&mut self, center: Vec2, radius: f32, angle: f32, segments: u32, color: Color4F) {
        self.commands.push(DrawCommand::Circle {
            center,
            radius,
            angle,
            segments,
            draw_line_to_center: false,
            color,
            filled: true,
        });
        self.dirty = true;
    }

    pub fn draw_polygon(&mut self, vertices: &[Vec2], color: Color4F) {
        self.commands.push(DrawCommand::Polygon {
            vertices: vertices.to_vec(),
            color,
            filled: false,
        });
        self.dirty = true;
    }

    pub fn draw_solid_polygon(&mut self, vertices: &[Vec2], color: Color4F) {
        self.commands.push(DrawCommand::Polygon {
            vertices: vertices.to_vec(),
            color,
            filled: true,
        });
        self.dirty = true;
    }

    pub fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color4F) {
        self.commands.push(DrawCommand::Triangle {
            p1,
            p2,
            p3,
            color,
            filled: false,
        });
        self.dirty = true;
    }

    pub fn draw_solid_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color4F) {
        self.commands.push(DrawCommand::Triangle {
            p1,
            p2,
            p3,
            color,
            filled: true,
        });
        self.dirty = true;
    }

    pub fn draw_quad_bezier(&mut self, origin: Vec2, control: Vec2, destination: Vec2, segments: u32, color: Color4F) {
        self.commands.push(DrawCommand::QuadBezier {
            origin,
            control,
            destination,
            segments,
            color,
        });
        self.dirty = true;
    }

    pub fn draw_cubic_bezier(&mut self, origin: Vec2, control1: Vec2, control2: Vec2, destination: Vec2, segments: u32, color: Color4F) {
        self.commands.push(DrawCommand::CubicBezier {
            origin,
            control1,
            control2,
            destination,
            segments,
            color,
        });
        self.dirty = true;
    }

    /// 绘制点（别名方法，兼容 cocos2d-x API）
    pub fn draw_point(&mut self, pos: Vec2, size: f32, color: Color4F) {
        self.draw_dot(pos, size, color);
    }

    /// 绘制矩形（用两个角点指定，兼容 cocos2d-x API）
    pub fn draw_rect_corners(&mut self, origin: Vec2, destination: Vec2, color: Color4F) {
        let rect = Rect::new(origin.x, origin.y, destination.x - origin.x, destination.y - origin.y);
        self.draw_rect(rect, color);
    }

    /// 绘制圆（简化版，不需要 draw_line_to_center 参数）
    pub fn draw_circle_simple(&mut self, center: Vec2, radius: f32, angle: f32, segments: u32, color: Color4F) {
        self.draw_circle(center, radius, angle, segments, false, color);
    }

    pub fn get_commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    pub fn get_command_count(&self) -> usize {
        self.commands.len()
    }

    pub fn visit(&mut self, _renderer: &mut Renderer, _parent_transform: &crate::math::Mat4) {
        if !self.node.is_visible() {
            return;
        }

        self.update_vertices();
    }

    fn update_vertices(&mut self) {
        if !self.dirty {
            return;
        }

        self.vertices.clear();

        let commands = self.commands.clone();
        for command in &commands {
            match command {
                DrawCommand::Dot { pos, radius, color } => {
                    self.generate_circle_vertices(*pos, *radius, 0.0, 16, false, *color, true);
                }
                DrawCommand::Line { from, to, color } => {
                    self.vertices.push(Vertex::new(*from, *color));
                    self.vertices.push(Vertex::new(*to, *color));
                }
                DrawCommand::Rect { rect, color, filled } => {
                    self.generate_rect_vertices(rect, *color, *filled);
                }
                DrawCommand::Circle { center, radius, angle, segments, draw_line_to_center, color, filled } => {
                    self.generate_circle_vertices(*center, *radius, *angle, *segments, *draw_line_to_center, *color, *filled);
                }
                DrawCommand::Polygon { vertices, color, filled } => {
                    self.generate_polygon_vertices(vertices, *color, *filled);
                }
                DrawCommand::Triangle { p1, p2, p3, color, filled } => {
                    self.generate_triangle_vertices(*p1, *p2, *p3, *color, *filled);
                }
                DrawCommand::QuadBezier { origin, control, destination, segments, color } => {
                    self.generate_quad_bezier_vertices(*origin, *control, *destination, *segments, *color);
                }
                DrawCommand::CubicBezier { origin, control1, control2, destination, segments, color } => {
                    self.generate_cubic_bezier_vertices(*origin, *control1, *control2, *destination, *segments, *color);
                }
            }
        }

        self.dirty = false;
    }

    fn generate_rect_vertices(&mut self, rect: &Rect, color: Color4F, filled: bool) {
        let p1 = Vec2::new(rect.origin.x, rect.origin.y);
        let p2 = Vec2::new(rect.origin.x + rect.size.width, rect.origin.y);
        let p3 = Vec2::new(rect.origin.x + rect.size.width, rect.origin.y + rect.size.height);
        let p4 = Vec2::new(rect.origin.x, rect.origin.y + rect.size.height);

        if filled {
            self.vertices.push(Vertex::new(p1, color));
            self.vertices.push(Vertex::new(p2, color));
            self.vertices.push(Vertex::new(p3, color));
            
            self.vertices.push(Vertex::new(p1, color));
            self.vertices.push(Vertex::new(p3, color));
            self.vertices.push(Vertex::new(p4, color));
        } else {
            self.vertices.push(Vertex::new(p1, color));
            self.vertices.push(Vertex::new(p2, color));
            
            self.vertices.push(Vertex::new(p2, color));
            self.vertices.push(Vertex::new(p3, color));
            
            self.vertices.push(Vertex::new(p3, color));
            self.vertices.push(Vertex::new(p4, color));
            
            self.vertices.push(Vertex::new(p4, color));
            self.vertices.push(Vertex::new(p1, color));
        }
    }

    fn generate_circle_vertices(&mut self, center: Vec2, radius: f32, angle: f32, segments: u32, draw_line_to_center: bool, color: Color4F, filled: bool) {
        let coef = 2.0 * std::f32::consts::PI / segments as f32;

        if filled {
            for i in 0..segments {
                let rads1 = i as f32 * coef;
                let rads2 = (i + 1) as f32 * coef;

                let p1 = Vec2::new(
                    center.x + radius * rads1.cos(),
                    center.y + radius * rads1.sin(),
                );
                let p2 = Vec2::new(
                    center.x + radius * rads2.cos(),
                    center.y + radius * rads2.sin(),
                );

                self.vertices.push(Vertex::new(center, color));
                self.vertices.push(Vertex::new(p1, color));
                self.vertices.push(Vertex::new(p2, color));
            }
        } else {
            for i in 0..segments {
                let rads = i as f32 * coef;
                let p = Vec2::new(
                    center.x + radius * rads.cos(),
                    center.y + radius * rads.sin(),
                );
                let next_rads = ((i + 1) % segments) as f32 * coef;
                let next_p = Vec2::new(
                    center.x + radius * next_rads.cos(),
                    center.y + radius * next_rads.sin(),
                );

                self.vertices.push(Vertex::new(p, color));
                self.vertices.push(Vertex::new(next_p, color));
            }

            if draw_line_to_center {
                let p = Vec2::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                );
                self.vertices.push(Vertex::new(center, color));
                self.vertices.push(Vertex::new(p, color));
            }
        }
    }

    fn generate_polygon_vertices(&mut self, vertices: &[Vec2], color: Color4F, filled: bool) {
        if vertices.len() < 3 {
            return;
        }

        if filled {
            for i in 1..vertices.len() - 1 {
                self.vertices.push(Vertex::new(vertices[0], color));
                self.vertices.push(Vertex::new(vertices[i], color));
                self.vertices.push(Vertex::new(vertices[i + 1], color));
            }
        } else {
            for i in 0..vertices.len() {
                let next_i = (i + 1) % vertices.len();
                self.vertices.push(Vertex::new(vertices[i], color));
                self.vertices.push(Vertex::new(vertices[next_i], color));
            }
        }
    }

    fn generate_triangle_vertices(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color4F, filled: bool) {
        if filled {
            self.vertices.push(Vertex::new(p1, color));
            self.vertices.push(Vertex::new(p2, color));
            self.vertices.push(Vertex::new(p3, color));
        } else {
            self.vertices.push(Vertex::new(p1, color));
            self.vertices.push(Vertex::new(p2, color));
            
            self.vertices.push(Vertex::new(p2, color));
            self.vertices.push(Vertex::new(p3, color));
            
            self.vertices.push(Vertex::new(p3, color));
            self.vertices.push(Vertex::new(p1, color));
        }
    }

    fn generate_quad_bezier_vertices(&mut self, origin: Vec2, control: Vec2, destination: Vec2, segments: u32, color: Color4F) {
        let mut prev = origin;
        
        for i in 1..=segments {
            let t = i as f32 / segments as f32;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let t2 = t * t;

            let point = Vec2::new(
                mt2 * origin.x + 2.0 * mt * t * control.x + t2 * destination.x,
                mt2 * origin.y + 2.0 * mt * t * control.y + t2 * destination.y,
            );

            self.vertices.push(Vertex::new(prev, color));
            self.vertices.push(Vertex::new(point, color));
            
            prev = point;
        }
    }

    fn generate_cubic_bezier_vertices(&mut self, origin: Vec2, control1: Vec2, control2: Vec2, destination: Vec2, segments: u32, color: Color4F) {
        let mut prev = origin;
        
        for i in 1..=segments {
            let t = i as f32 / segments as f32;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;
            let t2 = t * t;
            let t3 = t2 * t;

            let point = Vec2::new(
                mt3 * origin.x + 3.0 * mt2 * t * control1.x + 3.0 * mt * t2 * control2.x + t3 * destination.x,
                mt3 * origin.y + 3.0 * mt2 * t * control1.y + 3.0 * mt * t2 * control2.y + t3 * destination.y,
            );

            self.vertices.push(Vertex::new(prev, color));
            self.vertices.push(Vertex::new(point, color));
            
            prev = point;
        }
    }
}

impl Default for DrawNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_node_creation() {
        let draw_node = DrawNode::new();
        
        assert_eq!(draw_node.get_line_width(), 1.0);
        assert_eq!(draw_node.get_default_color(), Color4F::WHITE);
        assert_eq!(draw_node.get_command_count(), 0);
    }

    #[test]
    fn test_draw_line() {
        let mut draw_node = DrawNode::new();
        let from = Vec2::new(0.0, 0.0);
        let to = Vec2::new(100.0, 100.0);
        
        draw_node.draw_line(from, to, Color4F::RED);
        
        assert_eq!(draw_node.get_command_count(), 1);
    }

    #[test]
    fn test_draw_rect() {
        let mut draw_node = DrawNode::new();
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        
        draw_node.draw_rect(rect, Color4F::BLUE);
        assert_eq!(draw_node.get_command_count(), 1);
        
        draw_node.draw_solid_rect(rect, Color4F::GREEN);
        assert_eq!(draw_node.get_command_count(), 2);
    }

    #[test]
    fn test_draw_circle() {
        let mut draw_node = DrawNode::new();
        let center = Vec2::new(50.0, 50.0);
        
        draw_node.draw_circle(center, 30.0, 0.0, 32, false, Color4F::YELLOW);
        assert_eq!(draw_node.get_command_count(), 1);
        
        draw_node.draw_solid_circle(center, 20.0, 0.0, 24, Color4F::MAGENTA);
        assert_eq!(draw_node.get_command_count(), 2);
    }

    #[test]
    fn test_draw_polygon() {
        let mut draw_node = DrawNode::new();
        let vertices = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(0.0, 100.0),
        ];
        
        draw_node.draw_polygon(&vertices, Color4F::WHITE);
        assert_eq!(draw_node.get_command_count(), 1);
    }

    #[test]
    fn test_draw_triangle() {
        let mut draw_node = DrawNode::new();
        
        draw_node.draw_triangle(
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(50.0, 100.0),
            Color4F::RED,
        );
        
        assert_eq!(draw_node.get_command_count(), 1);
    }

    #[test]
    fn test_draw_dot() {
        let mut draw_node = DrawNode::new();
        
        draw_node.draw_dot(Vec2::new(50.0, 50.0), 5.0, Color4F::GREEN);
        assert_eq!(draw_node.get_command_count(), 1);
    }

    #[test]
    fn test_draw_bezier() {
        let mut draw_node = DrawNode::new();
        
        draw_node.draw_quad_bezier(
            Vec2::new(0.0, 0.0),
            Vec2::new(50.0, 100.0),
            Vec2::new(100.0, 0.0),
            20,
            Color4F::BLUE,
        );
        assert_eq!(draw_node.get_command_count(), 1);
        
        draw_node.draw_cubic_bezier(
            Vec2::new(0.0, 0.0),
            Vec2::new(30.0, 100.0),
            Vec2::new(70.0, 100.0),
            Vec2::new(100.0, 0.0),
            30,
            Color4F::YELLOW,
        );
        assert_eq!(draw_node.get_command_count(), 2);
    }

    #[test]
    fn test_clear() {
        let mut draw_node = DrawNode::new();
        
        draw_node.draw_line(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color4F::RED);
        draw_node.draw_circle(Vec2::new(50.0, 50.0), 30.0, 0.0, 32, false, Color4F::BLUE);
        
        assert_eq!(draw_node.get_command_count(), 2);
        
        draw_node.clear();
        assert_eq!(draw_node.get_command_count(), 0);
    }

    #[test]
    fn test_line_width() {
        let mut draw_node = DrawNode::new();
        
        draw_node.set_line_width(2.5);
        assert_eq!(draw_node.get_line_width(), 2.5);
        
        draw_node.set_line_width(-1.0);
        assert_eq!(draw_node.get_line_width(), 0.0);
    }

    #[test]
    fn test_default_color() {
        let mut draw_node = DrawNode::new();
        
        draw_node.set_default_color(Color4F::RED);
        assert_eq!(draw_node.get_default_color(), Color4F::RED);
    }

    #[test]
    fn test_vertex_generation() {
        let mut draw_node = DrawNode::new();
        
        draw_node.draw_line(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), Color4F::RED);
        draw_node.update_vertices();
    }
}

