use crate::base::{Node, Ref, RefPtr};
use crate::math::Vec2;

/// Action is the base class for all actions
#[derive(Debug)]
pub struct Action {
    target: Option<RefPtr<Node>>,
    original_target: Option<RefPtr<Node>>,
    tag: i32,
    flags: u32,
}

impl Action {
    /// Creates a new action
    pub fn new() -> Action {
        Action {
            target: None,
            original_target: None,
            tag: 0,
            flags: 0,
        }
    }

    /// Clones the action
    pub fn clone(&self) -> Action {
        Action {
            target: None,
            original_target: None,
            tag: self.tag,
            flags: self.flags,
        }
    }

    /// Starts the action with a target
    pub fn start_with_target(&mut self, target: RefPtr<Node>) {
        self.original_target = Some(target.clone());
        self.target = Some(target);
    }

    /// Stops the action
    pub fn stop(&mut self) {
        self.target = None;
    }

    /// Steps the action
    pub fn step(&mut self, _dt: f32) {
        // Override in subclasses
    }

    /// Updates the action
    pub fn update(&mut self, _time: f32) {
        // Override in subclasses
    }

    /// Gets the target
    pub fn get_target(&self) -> Option<&RefPtr<Node>> {
        self.target.as_ref()
    }

    /// Gets the original target
    pub fn get_original_target(&self) -> Option<&RefPtr<Node>> {
        self.original_target.as_ref()
    }

    /// Sets the tag
    pub fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    /// Gets the tag
    pub fn get_tag(&self) -> i32 {
        self.tag
    }

    /// Checks if the action is done
    pub fn is_done(&self) -> bool {
        true
    }
}

/// Finite Time Action is an action that takes a finite amount of time
#[derive(Debug)]
pub struct FiniteTimeAction {
    base: Action,
    duration: f32,
}

impl FiniteTimeAction {
    /// Creates a new finite time action
    pub fn new(duration: f32) -> FiniteTimeAction {
        FiniteTimeAction {
            base: Action::new(),
            duration,
        }
    }

    /// Gets the duration
    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    /// Sets the duration
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }
}

/// Speed controls the speed of an action
#[derive(Debug)]
pub struct Speed {
    base: Action,
    inner_action: Box<Action>,
    speed: f32,
}

impl Speed {
    /// Creates a new speed action
    pub fn new(action: Box<Action>, speed: f32) -> Speed {
        Speed {
            base: Action::new(),
            inner_action: action,
            speed,
        }
    }

    /// Gets the speed
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    /// Sets the speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    /// Gets the inner action
    pub fn get_inner_action(&self) -> &Box<Action> {
        &self.inner_action
    }

    /// Gets mutable inner action
    pub fn get_inner_action_mut(&mut self) -> &mut Box<Action> {
        &mut self.inner_action
    }
}

/// Follow is an action that follows a node
#[derive(Debug)]
pub struct Follow {
    base: Action,
    target_node: RefPtr<Node>,
    boundary_set: bool,
    left_boundary: f32,
    right_boundary: f32,
    top_boundary: f32,
    bottom_boundary: f32,
    world_rect: (f32, f32, f32, f32),
}

impl Follow {
    /// Creates a new follow action
    pub fn new(target: RefPtr<Node>) -> Follow {
        Follow {
            base: Action::new(),
            target_node: target,
            boundary_set: false,
            left_boundary: 0.0,
            right_boundary: 0.0,
            top_boundary: 0.0,
            bottom_boundary: 0.0,
            world_rect: (0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Creates a follow action with a boundary
    pub fn new_with_boundary(target: RefPtr<Node>, left: f32, bottom: f32, right: f32, top: f32) -> Follow {
        let mut follow = Follow::new(target);
        follow.boundary_set = true;
        follow.left_boundary = left;
        follow.bottom_boundary = bottom;
        follow.right_boundary = right;
        follow.top_boundary = top;
        follow
    }

    /// Creates a follow action with a world rect
    pub fn new_with_world_rect(target: RefPtr<Node>, rect: (f32, f32, f32, f32)) -> Follow {
        let mut follow = Follow::new(target);
        follow.boundary_set = true;
        follow.world_rect = rect;
        follow
    }

    /// Checks if the boundary is set
    pub fn is_boundary_set(&self) -> bool {
        self.boundary_set
    }

    /// Gets the boundary
    pub fn get_boundary(&self) -> (f32, f32, f32, f32) {
        if self.boundary_set {
            (self.left_boundary, self.bottom_boundary, self.right_boundary, self.top_boundary)
        } else {
            self.world_rect
        }
    }
}

/// ActionManager manages all actions
#[derive(Debug)]
pub struct ActionManager {
    action_hash: std::collections::HashMap<i32, RefPtr<Action>>,
    current_action: Option<RefPtr<Action>>,
    current_action_removed: bool,
    target_map: std::collections::HashMap<usize, Vec<RefPtr<Action>>>,
}

impl ActionManager {
    /// Creates a new action manager
    pub fn new() -> ActionManager {
        ActionManager {
            action_hash: std::collections::HashMap::new(),
            current_action: None,
            current_action_removed: false,
            target_map: std::collections::HashMap::new(),
        }
    }

    /// Gets the singleton instance
    pub fn get_instance() -> &'static mut ActionManager {
        static mut ACTION_MANAGER: Option<ActionManager> = None;
        unsafe {
            if ACTION_MANAGER.is_none() {
                ACTION_MANAGER = Some(ActionManager::new());
            }
            ACTION_MANAGER.as_mut().unwrap()
        }
    }

    /// Adds an action
    pub fn add_action(&mut self, action: RefPtr<Action>, target: RefPtr<Node>, paused: bool) {
        let target_id = &target as *const _ as usize;

        if let Some(actions) = self.target_map.get_mut(&target_id) {
            actions.push(action);
        } else {
            self.target_map.insert(target_id, vec![action]);
        }
    }

    /// Removes an action by tag
    pub fn remove_action_by_tag(&mut self, tag: i32, target: &RefPtr<Node>) {
        let target_id = target as *const _ as usize;
        if let Some(actions) = self.target_map.get_mut(&target_id) {
            actions.retain(|action| action.get_tag() != tag);
        }
    }

    /// Removes all actions
    pub fn remove_all_actions(&mut self) {
        self.target_map.clear();
    }

    /// Removes all actions from a target
    pub fn remove_all_actions_from_target(&mut self, target: &RefPtr<Node>) {
        let target_id = target as *const _ as usize;
        self.target_map.remove(&target_id);
    }

    /// Gets an action by tag
    pub fn get_action_by_tag(&self, tag: i32, target: &RefPtr<Node>) -> Option<&RefPtr<Action>> {
        let target_id = target as *const _ as usize;
        if let Some(actions) = self.target_map.get(&target_id) {
            for action in actions {
                if action.get_tag() == tag {
                    return Some(action);
                }
            }
        }
        None
    }

    /// Updates the action manager
    pub fn update(&mut self, dt: f32) {
        for (target_id, actions) in &mut self.target_map {
            for action in actions {
                action.borrow_mut().step(dt);
            }
        }
    }
}
