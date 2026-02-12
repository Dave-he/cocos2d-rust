#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::Node;
    use crate::action::action_interval::MoveBy;
    use crate::action::{Action, FiniteTimeAction, Sequence, Spawn};
    use crate::math::Vec2;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_sequence_creation() {
        let move1 = Rc::new(RefCell::new(MoveBy::new(1.0, Vec2::new(10.0, 0.0))));
        let move2 = Rc::new(RefCell::new(MoveBy::new(1.0, Vec2::new(0.0, 10.0))));
        
        let seq = Sequence::create_with_two_actions(
            move1 as Rc<RefCell<dyn FiniteTimeAction>>,
            move2 as Rc<RefCell<dyn FiniteTimeAction>>
        );
        
        assert!(seq.is_ok());
        
        if let Ok(sequence) = seq {
            let duration = sequence.borrow().get_duration();
            assert_eq!(duration, 2.0);
        }
    }

    #[test]
    fn test_spawn_creation() {
        let move1 = Rc::new(RefCell::new(MoveBy::new(1.0, Vec2::new(10.0, 0.0))));
        let move2 = Rc::new(RefCell::new(MoveBy::new(2.0, Vec2::new(0.0, 10.0))));
        
        let spawn = Spawn::create_with_two_actions(
            move1 as Rc<RefCell<dyn FiniteTimeAction>>,
            move2 as Rc<RefCell<dyn FiniteTimeAction>>
        );
        
        assert!(spawn.is_ok());
        
        if let Ok(sp) = spawn {
            let duration = sp.borrow().get_duration();
            assert_eq!(duration, 2.0);
        }
    }

    #[test]
    fn test_sequence_with_node() {
        let node = Rc::new(RefCell::new(Node::new()));
        let move1 = Rc::new(RefCell::new(MoveBy::new(1.0, Vec2::new(10.0, 0.0))));
        let move2 = Rc::new(RefCell::new(MoveBy::new(1.0, Vec2::new(0.0, 10.0))));
        
        let seq = Sequence::create_with_two_actions(
            move1 as Rc<RefCell<dyn FiniteTimeAction>>,
            move2 as Rc<RefCell<dyn FiniteTimeAction>>
        );
        
        if let Ok(sequence) = seq {
            {
                let mut seq_mut = sequence.borrow_mut();
                seq_mut.start_with_target(&node);
            }
            {
                let seq_ref = sequence.borrow();
                assert_eq!(seq_ref.get_target().is_some(), true);
            }
        }
    }
}
