use crate::renderer::command::RenderCommand;
use crate::renderer::renderer::Renderer;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    None,
    BackToFront,
    FrontToBack,
    StateBatching,
}

pub struct CommandQueue {
    commands: Vec<Box<dyn RenderCommand>>,
    sort_mode: SortMode,
    stats: QueueStats,
}

#[derive(Debug, Default, Clone)]
pub struct QueueStats {
    pub total_commands: usize,
    pub sorted_count: usize,
    pub state_changes: usize,
    pub optimized_count: usize,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self::with_sort_mode(SortMode::BackToFront)
    }

    pub fn with_sort_mode(sort_mode: SortMode) -> Self {
        Self {
            commands: Vec::new(),
            sort_mode,
            stats: QueueStats::default(),
        }
    }

    pub fn with_capacity(capacity: usize, sort_mode: SortMode) -> Self {
        Self {
            commands: Vec::with_capacity(capacity),
            sort_mode,
            stats: QueueStats::default(),
        }
    }

    pub fn push(&mut self, command: Box<dyn RenderCommand>) {
        self.commands.push(command);
    }

    pub fn sort_commands(&mut self) {
        match self.sort_mode {
            SortMode::None => {},
            SortMode::BackToFront => {
                self.commands.sort_by(|a, b| {
                    b.get_global_order()
                        .partial_cmp(&a.get_global_order())
                        .unwrap_or(Ordering::Equal)
                });
                self.stats.sorted_count = self.commands.len();
            }
            SortMode::FrontToBack => {
                self.commands.sort_by(|a, b| {
                    a.get_global_order()
                        .partial_cmp(&b.get_global_order())
                        .unwrap_or(Ordering::Equal)
                });
                self.stats.sorted_count = self.commands.len();
            }
            SortMode::StateBatching => {
                self.commands.sort_by(|a, b| {
                    let type_cmp = (a.get_command_type() as i32).cmp(&(b.get_command_type() as i32));
                    if type_cmp != Ordering::Equal {
                        return type_cmp;
                    }
                    a.get_global_order()
                        .partial_cmp(&b.get_global_order())
                        .unwrap_or(Ordering::Equal)
                });
                self.stats.sorted_count = self.commands.len();
            }
        }
    }

    pub fn optimize(&mut self) {
        let mut optimized = 0;
        
        let mut i = 0;
        while i < self.commands.len() {
            if i + 1 < self.commands.len() {
                let same_type = self.commands[i].get_command_type() == self.commands[i + 1].get_command_type();
                if same_type {
                    optimized += 1;
                }
            }
            i += 1;
        }
        
        self.stats.optimized_count = optimized;
    }

    pub fn execute(&mut self, renderer: &mut Renderer) {
        self.sort_commands();
        self.optimize();
        
        self.stats.total_commands = self.commands.len();
        self.stats.state_changes = self.count_state_changes();
        
        for command in &self.commands {
            command.execute(renderer);
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.stats = QueueStats::default();
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn set_sort_mode(&mut self, mode: SortMode) {
        self.sort_mode = mode;
    }

    pub fn get_sort_mode(&self) -> SortMode {
        self.sort_mode
    }

    pub fn get_stats(&self) -> &QueueStats {
        &self.stats
    }

    fn count_state_changes(&self) -> usize {
        if self.commands.is_empty() {
            return 0;
        }
        
        let mut changes = 0;
        let mut prev_type = self.commands[0].get_command_type();
        
        for command in &self.commands[1..] {
            let current_type = command.get_command_type();
            if current_type != prev_type {
                changes += 1;
                prev_type = current_type;
            }
        }
        
        changes
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StateCache {
    current_shader: Option<u32>,
    current_texture: Option<u32>,
    current_blend_mode: Option<(u32, u32)>,
    state_changes: usize,
}

impl StateCache {
    pub fn new() -> Self {
        Self {
            current_shader: None,
            current_texture: None,
            current_blend_mode: None,
            state_changes: 0,
        }
    }

    pub fn set_shader(&mut self, shader_id: u32) -> bool {
        if self.current_shader != Some(shader_id) {
            self.current_shader = Some(shader_id);
            self.state_changes += 1;
            true
        } else {
            false
        }
    }

    pub fn set_texture(&mut self, texture_id: u32) -> bool {
        if self.current_texture != Some(texture_id) {
            self.current_texture = Some(texture_id);
            self.state_changes += 1;
            true
        } else {
            false
        }
    }

    pub fn set_blend_mode(&mut self, src: u32, dst: u32) -> bool {
        let blend_mode = (src, dst);
        if self.current_blend_mode != Some(blend_mode) {
            self.current_blend_mode = Some(blend_mode);
            self.state_changes += 1;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.current_shader = None;
        self.current_texture = None;
        self.current_blend_mode = None;
        self.state_changes = 0;
    }

    pub fn get_state_changes(&self) -> usize {
        self.state_changes
    }

    pub fn get_current_shader(&self) -> Option<u32> {
        self.current_shader
    }

    pub fn get_current_texture(&self) -> Option<u32> {
        self.current_texture
    }

    pub fn get_current_blend_mode(&self) -> Option<(u32, u32)> {
        self.current_blend_mode
    }
}

impl Default for StateCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::command::CommandType;

    struct MockCommand {
        cmd_type: CommandType,
        global_order: f32,
    }

    impl RenderCommand for MockCommand {
        fn get_command_type(&self) -> CommandType {
            self.cmd_type
        }

        fn get_global_order(&self) -> f32 {
            self.global_order
        }

        fn execute(&self, _renderer: &mut Renderer) {}
    }

    #[test]
    fn test_command_queue_creation() {
        let queue = CommandQueue::new();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.get_sort_mode(), SortMode::BackToFront);
    }

    #[test]
    fn test_command_queue_with_sort_mode() {
        let queue = CommandQueue::with_sort_mode(SortMode::FrontToBack);
        assert_eq!(queue.get_sort_mode(), SortMode::FrontToBack);
    }

    #[test]
    fn test_command_queue_push() {
        let mut queue = CommandQueue::new();
        let cmd = Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        });
        
        queue.push(cmd);
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_command_queue_clear() {
        let mut queue = CommandQueue::new();
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        }));
        
        queue.clear();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_sort_mode_none() {
        let mut queue = CommandQueue::with_sort_mode(SortMode::None);
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 3.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        }));
        
        queue.sort_commands();
        assert_eq!(queue.get_stats().sorted_count, 0);
    }

    #[test]
    fn test_sort_back_to_front() {
        let mut queue = CommandQueue::with_sort_mode(SortMode::BackToFront);
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 3.0,
        }));
        
        queue.sort_commands();
        assert_eq!(queue.get_stats().sorted_count, 2);
        assert_eq!(queue.commands[0].get_global_order(), 3.0);
        assert_eq!(queue.commands[1].get_global_order(), 1.0);
    }

    #[test]
    fn test_sort_front_to_back() {
        let mut queue = CommandQueue::with_sort_mode(SortMode::FrontToBack);
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 3.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        }));
        
        queue.sort_commands();
        assert_eq!(queue.get_stats().sorted_count, 2);
        assert_eq!(queue.commands[0].get_global_order(), 1.0);
        assert_eq!(queue.commands[1].get_global_order(), 3.0);
    }

    #[test]
    fn test_sort_state_batching() {
        let mut queue = CommandQueue::with_sort_mode(SortMode::StateBatching);
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Triangles,
            global_order: 1.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 2.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Triangles,
            global_order: 3.0,
        }));
        
        queue.sort_commands();
        assert_eq!(queue.get_stats().sorted_count, 3);
    }

    #[test]
    fn test_state_cache_creation() {
        let cache = StateCache::new();
        assert_eq!(cache.get_state_changes(), 0);
        assert_eq!(cache.get_current_shader(), None);
        assert_eq!(cache.get_current_texture(), None);
    }

    #[test]
    fn test_state_cache_shader() {
        let mut cache = StateCache::new();
        
        assert!(cache.set_shader(1));
        assert_eq!(cache.get_current_shader(), Some(1));
        assert_eq!(cache.get_state_changes(), 1);
        
        assert!(!cache.set_shader(1));
        assert_eq!(cache.get_state_changes(), 1);
        
        assert!(cache.set_shader(2));
        assert_eq!(cache.get_state_changes(), 2);
    }

    #[test]
    fn test_state_cache_texture() {
        let mut cache = StateCache::new();
        
        assert!(cache.set_texture(10));
        assert_eq!(cache.get_current_texture(), Some(10));
        assert_eq!(cache.get_state_changes(), 1);
        
        assert!(!cache.set_texture(10));
        assert_eq!(cache.get_state_changes(), 1);
    }

    #[test]
    fn test_state_cache_blend_mode() {
        let mut cache = StateCache::new();
        
        assert!(cache.set_blend_mode(770, 771));
        assert_eq!(cache.get_current_blend_mode(), Some((770, 771)));
        assert_eq!(cache.get_state_changes(), 1);
        
        assert!(!cache.set_blend_mode(770, 771));
        assert_eq!(cache.get_state_changes(), 1);
    }

    #[test]
    fn test_state_cache_reset() {
        let mut cache = StateCache::new();
        
        cache.set_shader(1);
        cache.set_texture(10);
        cache.set_blend_mode(770, 771);
        
        cache.reset();
        assert_eq!(cache.get_state_changes(), 0);
        assert_eq!(cache.get_current_shader(), None);
    }

    #[test]
    fn test_queue_stats_default() {
        let stats = QueueStats::default();
        assert_eq!(stats.total_commands, 0);
        assert_eq!(stats.sorted_count, 0);
        assert_eq!(stats.state_changes, 0);
    }

    #[test]
    fn test_command_queue_with_capacity() {
        let queue = CommandQueue::with_capacity(100, SortMode::None);
        assert_eq!(queue.commands.capacity(), 100);
    }

    #[test]
    fn test_set_sort_mode() {
        let mut queue = CommandQueue::new();
        assert_eq!(queue.get_sort_mode(), SortMode::BackToFront);
        
        queue.set_sort_mode(SortMode::FrontToBack);
        assert_eq!(queue.get_sort_mode(), SortMode::FrontToBack);
    }

    #[test]
    fn test_count_state_changes() {
        let mut queue = CommandQueue::new();
        
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 2.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Triangles,
            global_order: 3.0,
        }));
        
        let changes = queue.count_state_changes();
        assert_eq!(changes, 1);
    }

    #[test]
    fn test_queue_optimize() {
        let mut queue = CommandQueue::new();
        
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 1.0,
        }));
        queue.push(Box::new(MockCommand {
            cmd_type: CommandType::Quad,
            global_order: 2.0,
        }));
        
        queue.optimize();
        assert!(queue.get_stats().optimized_count > 0);
    }

    #[test]
    fn test_state_cache_default() {
        let cache = StateCache::default();
        assert_eq!(cache.get_state_changes(), 0);
    }

    #[test]
    fn test_command_queue_default() {
        let queue = CommandQueue::default();
        assert_eq!(queue.get_sort_mode(), SortMode::BackToFront);
    }

    #[test]
    fn test_multiple_state_changes() {
        let mut cache = StateCache::new();
        
        cache.set_shader(1);
        cache.set_texture(10);
        cache.set_blend_mode(770, 771);
        
        assert_eq!(cache.get_state_changes(), 3);
    }

    #[test]
    fn test_empty_queue_state_changes() {
        let queue = CommandQueue::new();
        assert_eq!(queue.count_state_changes(), 0);
    }
}
