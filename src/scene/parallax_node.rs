use crate::math::Vec2;
use crate::scene::Node;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct PointObject {
    ratio: Vec2,
    offset: Vec2,
    child: Option<Rc<RefCell<Node>>>,
}

impl PointObject {
    fn new(ratio: Vec2, offset: Vec2) -> Self {
        Self {
            ratio,
            offset,
            child: None,
        }
    }

    fn set_child(&mut self, child: Rc<RefCell<Node>>) {
        self.child = Some(child);
    }
}

pub struct ParallaxNode {
    node: Node,
    parallax_array: Vec<PointObject>,
    last_position: Vec2,
}

impl ParallaxNode {
    pub fn new() -> Self {
        Self {
            node: Node::new(),
            parallax_array: Vec::new(),
            last_position: Vec2::new(-100.0, -100.0),
        }
    }

    pub fn add_child_with_parallax(
        &mut self,
        child: Rc<RefCell<Node>>,
        z_order: i32,
        ratio: Vec2,
        offset: Vec2,
    ) {
        let mut obj = PointObject::new(ratio, offset);
        obj.set_child(child.clone());

        let pos = self.absolute_position();
        let new_x = -pos.x + pos.x * ratio.x + offset.x;
        let new_y = -pos.y + pos.y * ratio.y + offset.y;
        child.borrow_mut().set_position(Vec2::new(new_x, new_y));

        self.node.add_child(child, z_order, None);
        self.parallax_array.push(obj);
    }

    pub fn remove_child(&mut self, child: &Rc<RefCell<Node>>) {
        let child_ptr = child.as_ptr();
        self.parallax_array.retain(|obj| {
            if let Some(ref c) = obj.child {
                c.as_ptr() != child_ptr
            } else {
                true
            }
        });

        self.node.remove_child(child, true);
    }

    pub fn remove_all_children(&mut self, cleanup: bool) {
        self.parallax_array.clear();
        self.node.remove_all_children(cleanup);
    }

    pub fn absolute_position(&self) -> Vec2 {
        let mut ret = self.node.position();
        let mut current_opt = self.node.get_parent();

        while let Some(current) = current_opt {
            let pos = current.borrow().position();
            ret = ret + pos;
            current_opt = current.borrow().get_parent();
        }

        ret
    }

    pub fn update_positions(&mut self) {
        let pos = self.absolute_position();

        if pos != self.last_position {
            for obj in &self.parallax_array {
                if let Some(ref child) = obj.child {
                    let x = -pos.x + pos.x * obj.ratio.x + obj.offset.x;
                    let y = -pos.y + pos.y * obj.ratio.y + obj.offset.y;
                    child.borrow_mut().set_position(Vec2::new(x, y));
                }
            }
            self.last_position = pos;
        }
    }

    pub fn get_node(&self) -> &Node {
        &self.node
    }

    pub fn get_node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    pub fn get_parallax_count(&self) -> usize {
        self.parallax_array.len()
    }
}

impl Default for ParallaxNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallax_node_creation() {
        let node = ParallaxNode::new();
        assert_eq!(node.get_parallax_count(), 0);
        assert_eq!(node.last_position, Vec2::new(-100.0, -100.0));
    }

    #[test]
    fn test_add_child_with_parallax() {
        let mut parallax = ParallaxNode::new();
        let child = Rc::new(RefCell::new(Node::new()));

        parallax.add_child_with_parallax(
            child.clone(),
            0,
            Vec2::new(0.5, 0.5),
            Vec2::new(10.0, 10.0),
        );

        assert_eq!(parallax.get_parallax_count(), 1);
    }

    #[test]
    fn test_parallax_ratio_calculation() {
        let mut parallax = ParallaxNode::new();
        let child = Rc::new(RefCell::new(Node::new()));

        parallax.get_node_mut().set_position(Vec2::new(100.0, 100.0));

        parallax.add_child_with_parallax(
            child.clone(),
            0,
            Vec2::new(0.5, 0.5),
            Vec2::new(0.0, 0.0),
        );

        let pos = child.borrow().position();
        assert_eq!(pos.x, -50.0);
        assert_eq!(pos.y, -50.0);
    }

    #[test]
    fn test_parallax_with_offset() {
        let mut parallax = ParallaxNode::new();
        let child = Rc::new(RefCell::new(Node::new()));

        parallax.get_node_mut().set_position(Vec2::new(100.0, 100.0));

        parallax.add_child_with_parallax(
            child.clone(),
            0,
            Vec2::new(1.0, 1.0),
            Vec2::new(20.0, 30.0),
        );

        let pos = child.borrow().position();
        assert_eq!(pos.x, 20.0);
        assert_eq!(pos.y, 30.0);
    }

    #[test]
    fn test_update_positions() {
        let mut parallax = ParallaxNode::new();
        let child = Rc::new(RefCell::new(Node::new()));

        parallax.add_child_with_parallax(
            child.clone(),
            0,
            Vec2::new(0.5, 0.5),
            Vec2::new(0.0, 0.0),
        );

        parallax.get_node_mut().set_position(Vec2::new(200.0, 200.0));
        parallax.update_positions();

        let pos = child.borrow().position();
        assert_eq!(pos.x, -100.0);
        assert_eq!(pos.y, -100.0);
    }

    #[test]
    fn test_multiple_children() {
        let mut parallax = ParallaxNode::new();
        let child1 = Rc::new(RefCell::new(Node::new()));
        let child2 = Rc::new(RefCell::new(Node::new()));
        let child3 = Rc::new(RefCell::new(Node::new()));

        parallax.add_child_with_parallax(child1, 0, Vec2::new(0.2, 0.2), Vec2::ZERO);
        parallax.add_child_with_parallax(child2, 1, Vec2::new(0.5, 0.5), Vec2::ZERO);
        parallax.add_child_with_parallax(child3, 2, Vec2::new(1.0, 1.0), Vec2::ZERO);

        assert_eq!(parallax.get_parallax_count(), 3);
    }

    #[test]
    fn test_remove_child() {
        let mut parallax = ParallaxNode::new();
        let child1 = Rc::new(RefCell::new(Node::new()));
        let child2 = Rc::new(RefCell::new(Node::new()));

        parallax.add_child_with_parallax(child1.clone(), 0, Vec2::new(0.5, 0.5), Vec2::ZERO);
        parallax.add_child_with_parallax(child2, 1, Vec2::new(1.0, 1.0), Vec2::ZERO);

        assert_eq!(parallax.get_parallax_count(), 2);

        parallax.remove_child(&child1);
        assert_eq!(parallax.get_parallax_count(), 1);
    }

    #[test]
    fn test_remove_all_children() {
        let mut parallax = ParallaxNode::new();

        parallax.add_child_with_parallax(
            Rc::new(RefCell::new(Node::new())),
            0,
            Vec2::new(0.5, 0.5),
            Vec2::ZERO,
        );
        parallax.add_child_with_parallax(
            Rc::new(RefCell::new(Node::new())),
            1,
            Vec2::new(1.0, 1.0),
            Vec2::ZERO,
        );

        parallax.remove_all_children(true);
        assert_eq!(parallax.get_parallax_count(), 0);
    }

    #[test]
    fn test_absolute_position() {
        let parallax = ParallaxNode::new();
        let pos = parallax.absolute_position();
        assert_eq!(pos, Vec2::ZERO);
    }

    #[test]
    fn test_no_update_when_position_unchanged() {
        let mut parallax = ParallaxNode::new();
        let child = Rc::new(RefCell::new(Node::new()));

        parallax.add_child_with_parallax(child.clone(), 0, Vec2::new(0.5, 0.5), Vec2::ZERO);

        let initial_pos = child.borrow().position();
        parallax.update_positions();
        let pos_after_update = child.borrow().position();

        assert_eq!(initial_pos, pos_after_update);
    }

    #[test]
    fn test_different_z_orders() {
        let mut parallax = ParallaxNode::new();

        parallax.add_child_with_parallax(
            Rc::new(RefCell::new(Node::new())),
            -1,
            Vec2::new(0.3, 0.3),
            Vec2::ZERO,
        );
        parallax.add_child_with_parallax(
            Rc::new(RefCell::new(Node::new())),
            0,
            Vec2::new(0.5, 0.5),
            Vec2::ZERO,
        );
        parallax.add_child_with_parallax(
            Rc::new(RefCell::new(Node::new())),
            1,
            Vec2::new(0.8, 0.8),
            Vec2::ZERO,
        );

        assert_eq!(parallax.get_parallax_count(), 3);
    }

    #[test]
    fn test_negative_ratio() {
        let mut parallax = ParallaxNode::new();
        let child = Rc::new(RefCell::new(Node::new()));

        parallax.get_node_mut().set_position(Vec2::new(100.0, 100.0));

        parallax.add_child_with_parallax(
            child.clone(),
            0,
            Vec2::new(-0.5, -0.5),
            Vec2::ZERO,
        );

        let pos = child.borrow().position();
        assert_eq!(pos.x, -150.0);
        assert_eq!(pos.y, -150.0);
    }
}
