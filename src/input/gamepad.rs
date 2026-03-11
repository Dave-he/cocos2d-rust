/// Gamepad - 游戏手柄输入模块
///
/// 特性：
/// - 多手柄支持（最多 4 个）
/// - 按钮状态追踪（按下/释放/持续按住）
/// - 摇杆轴值读取（带死区处理）
/// - 振动/震动支持
/// - 手柄连接/断开事件
/// - 预设按钮映射

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 手柄索引（支持最多 4 个手柄）
pub type GamepadIndex = u8;

/// 手柄按钮枚举（参考 Xbox/PS 布局）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    // 面部按钮
    /// Xbox: A / PS: 叉
    Cross,
    /// Xbox: B / PS: 圆圈
    Circle,
    /// Xbox: X / PS: 方块
    Square,
    /// Xbox: Y / PS: 三角
    Triangle,

    // 肩键
    LeftShoulder,
    RightShoulder,
    LeftTrigger,
    RightTrigger,

    // 方向键
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    // 特殊键
    Start,
    Select,
    Home,

    // 摇杆按下
    LeftThumbstick,
    RightThumbstick,
}

impl GamepadButton {
    /// 获取所有按钮列表
    pub fn all() -> &'static [GamepadButton] {
        &[
            GamepadButton::Cross,
            GamepadButton::Circle,
            GamepadButton::Square,
            GamepadButton::Triangle,
            GamepadButton::LeftShoulder,
            GamepadButton::RightShoulder,
            GamepadButton::LeftTrigger,
            GamepadButton::RightTrigger,
            GamepadButton::DPadUp,
            GamepadButton::DPadDown,
            GamepadButton::DPadLeft,
            GamepadButton::DPadRight,
            GamepadButton::Start,
            GamepadButton::Select,
            GamepadButton::Home,
            GamepadButton::LeftThumbstick,
            GamepadButton::RightThumbstick,
        ]
    }

    /// 获取按钮的用户友好名称
    pub fn name(&self) -> &'static str {
        match self {
            GamepadButton::Cross => "Cross/A",
            GamepadButton::Circle => "Circle/B",
            GamepadButton::Square => "Square/X",
            GamepadButton::Triangle => "Triangle/Y",
            GamepadButton::LeftShoulder => "L1/LB",
            GamepadButton::RightShoulder => "R1/RB",
            GamepadButton::LeftTrigger => "L2/LT",
            GamepadButton::RightTrigger => "R2/RT",
            GamepadButton::DPadUp => "D-Pad Up",
            GamepadButton::DPadDown => "D-Pad Down",
            GamepadButton::DPadLeft => "D-Pad Left",
            GamepadButton::DPadRight => "D-Pad Right",
            GamepadButton::Start => "Start/Options",
            GamepadButton::Select => "Select/Share",
            GamepadButton::Home => "Home/PS",
            GamepadButton::LeftThumbstick => "L3/LS",
            GamepadButton::RightThumbstick => "R3/RS",
        }
    }
}

impl std::fmt::Display for GamepadButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// 摇杆轴
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    /// 左扳机（模拟轴，0.0 ~ 1.0）
    LeftTrigger,
    /// 右扳机（模拟轴，0.0 ~ 1.0）
    RightTrigger,
}

impl GamepadAxis {
    pub fn all() -> &'static [GamepadAxis] {
        &[
            GamepadAxis::LeftX,
            GamepadAxis::LeftY,
            GamepadAxis::RightX,
            GamepadAxis::RightY,
            GamepadAxis::LeftTrigger,
            GamepadAxis::RightTrigger,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            GamepadAxis::LeftX => "Left X",
            GamepadAxis::LeftY => "Left Y",
            GamepadAxis::RightX => "Right X",
            GamepadAxis::RightY => "Right Y",
            GamepadAxis::LeftTrigger => "Left Trigger",
            GamepadAxis::RightTrigger => "Right Trigger",
        }
    }
}

/// 手柄按钮状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// 未按下
    Released,
    /// 刚按下（本帧）
    JustPressed,
    /// 持续按住
    Held,
    /// 刚释放（本帧）
    JustReleased,
}

impl ButtonState {
    pub fn is_pressed(&self) -> bool {
        matches!(self, ButtonState::JustPressed | ButtonState::Held)
    }

    pub fn is_released(&self) -> bool {
        matches!(self, ButtonState::Released | ButtonState::JustReleased)
    }
}

/// 手柄振动强度（0.0 ~ 1.0）
#[derive(Debug, Clone, Copy)]
pub struct GamepadVibration {
    pub left_motor: f32,
    pub right_motor: f32,
    pub duration_ms: u32,
}

impl GamepadVibration {
    pub fn new(left: f32, right: f32, duration_ms: u32) -> Self {
        Self {
            left_motor: left.clamp(0.0, 1.0),
            right_motor: right.clamp(0.0, 1.0),
            duration_ms,
        }
    }

    pub fn strong_pulse(duration_ms: u32) -> Self {
        Self::new(1.0, 1.0, duration_ms)
    }

    pub fn light_buzz(duration_ms: u32) -> Self {
        Self::new(0.3, 0.3, duration_ms)
    }

    pub fn asymmetric(duration_ms: u32) -> Self {
        Self::new(0.8, 0.3, duration_ms)
    }
}

/// 手柄事件
#[derive(Debug, Clone)]
pub enum GamepadEvent {
    /// 手柄连接
    Connected(GamepadIndex),
    /// 手柄断开
    Disconnected(GamepadIndex),
    /// 按钮按下
    ButtonPressed { index: GamepadIndex, button: GamepadButton },
    /// 按钮释放
    ButtonReleased { index: GamepadIndex, button: GamepadButton },
    /// 轴值变化
    AxisMoved { index: GamepadIndex, axis: GamepadAxis, value: f32 },
}

/// 单个手柄状态
#[derive(Debug, Clone)]
pub struct GamepadState {
    /// 手柄索引
    pub index: GamepadIndex,
    /// 是否连接
    pub connected: bool,
    /// 手柄名称
    pub name: String,
    /// 按钮状态
    button_states: HashMap<GamepadButton, ButtonState>,
    /// 轴值
    axis_values: HashMap<GamepadAxis, f32>,
    /// 死区阈值
    deadzone: f32,
}

impl GamepadState {
    pub fn new(index: GamepadIndex) -> Self {
        let mut button_states = HashMap::new();
        for btn in GamepadButton::all() {
            button_states.insert(*btn, ButtonState::Released);
        }

        let mut axis_values = HashMap::new();
        for axis in GamepadAxis::all() {
            axis_values.insert(*axis, 0.0f32);
        }

        Self {
            index,
            connected: false,
            name: format!("Gamepad {}", index),
            button_states,
            axis_values,
            deadzone: 0.15,
        }
    }

    /// 获取按钮状态
    pub fn get_button_state(&self, button: GamepadButton) -> ButtonState {
        self.button_states.get(&button).cloned().unwrap_or(ButtonState::Released)
    }

    /// 是否按钮按下（包括刚按下和持续按住）
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        self.get_button_state(button).is_pressed()
    }

    /// 是否刚按下（本帧）
    pub fn is_button_just_pressed(&self, button: GamepadButton) -> bool {
        matches!(self.get_button_state(button), ButtonState::JustPressed)
    }

    /// 是否刚释放（本帧）
    pub fn is_button_just_released(&self, button: GamepadButton) -> bool {
        matches!(self.get_button_state(button), ButtonState::JustReleased)
    }

    /// 获取轴值（应用死区后）
    pub fn get_axis(&self, axis: GamepadAxis) -> f32 {
        let raw = self.axis_values.get(&axis).cloned().unwrap_or(0.0);
        self.apply_deadzone(raw)
    }

    /// 获取原始轴值（不应用死区）
    pub fn get_axis_raw(&self, axis: GamepadAxis) -> f32 {
        self.axis_values.get(&axis).cloned().unwrap_or(0.0)
    }

    /// 获取左摇杆向量
    pub fn get_left_stick(&self) -> (f32, f32) {
        (self.get_axis(GamepadAxis::LeftX), self.get_axis(GamepadAxis::LeftY))
    }

    /// 获取右摇杆向量
    pub fn get_right_stick(&self) -> (f32, f32) {
        (self.get_axis(GamepadAxis::RightX), self.get_axis(GamepadAxis::RightY))
    }

    /// 设置死区（0.0 ~ 1.0）
    pub fn set_deadzone(&mut self, deadzone: f32) {
        self.deadzone = deadzone.clamp(0.0, 0.9);
    }

    /// 获取死区
    pub fn get_deadzone(&self) -> f32 {
        self.deadzone
    }

    fn apply_deadzone(&self, value: f32) -> f32 {
        if value.abs() < self.deadzone {
            0.0
        } else {
            // 重新映射到 0..1 区间（排除死区）
            let sign = value.signum();
            let magnitude = (value.abs() - self.deadzone) / (1.0 - self.deadzone);
            sign * magnitude.clamp(0.0, 1.0)
        }
    }

    /// 内部：更新按钮（由 GamepadManager 调用）
    pub(crate) fn update_button(&mut self, button: GamepadButton, pressed: bool) {
        let current = self.get_button_state(button);
        let new_state = match (current, pressed) {
            (ButtonState::Released, true) | (ButtonState::JustReleased, true) => ButtonState::JustPressed,
            (ButtonState::JustPressed, true) | (ButtonState::Held, true) => ButtonState::Held,
            (ButtonState::JustPressed, false) | (ButtonState::Held, false) => ButtonState::JustReleased,
            _ => ButtonState::Released,
        };
        self.button_states.insert(button, new_state);
    }

    /// 内部：每帧结束后重置瞬时状态
    pub(crate) fn end_frame(&mut self) {
        for state in self.button_states.values_mut() {
            *state = match *state {
                ButtonState::JustPressed => ButtonState::Held,
                ButtonState::JustReleased => ButtonState::Released,
                s => s,
            };
        }
    }

    /// 内部：更新轴值
    pub(crate) fn update_axis(&mut self, axis: GamepadAxis, value: f32) {
        self.axis_values.insert(axis, value.clamp(-1.0, 1.0));
    }
}

/// 手柄管理器 —— 管理所有连接的手柄
pub struct GamepadManager {
    gamepads: Arc<Mutex<[GamepadState; 4]>>,
    event_queue: Arc<Mutex<Vec<GamepadEvent>>>,
    vibration_queue: Arc<Mutex<Vec<(GamepadIndex, GamepadVibration)>>>,
}

impl Default for GamepadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GamepadManager {
    /// 创建手柄管理器
    pub fn new() -> Self {
        let gamepads = [
            GamepadState::new(0),
            GamepadState::new(1),
            GamepadState::new(2),
            GamepadState::new(3),
        ];

        Self {
            gamepads: Arc::new(Mutex::new(gamepads)),
            event_queue: Arc::new(Mutex::new(Vec::new())),
            vibration_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// 获取指定手柄状态（克隆）
    pub fn get_gamepad(&self, index: GamepadIndex) -> Option<GamepadState> {
        if index >= 4 {
            return None;
        }
        Some(self.gamepads.lock().unwrap()[index as usize].clone())
    }

    /// 获取所有已连接的手柄索引
    pub fn get_connected_indices(&self) -> Vec<GamepadIndex> {
        self.gamepads.lock().unwrap()
            .iter()
            .filter(|g| g.connected)
            .map(|g| g.index)
            .collect()
    }

    /// 手柄数量（已连接）
    pub fn connected_count(&self) -> usize {
        self.gamepads.lock().unwrap()
            .iter()
            .filter(|g| g.connected)
            .count()
    }

    /// 是否有任何手柄连接
    pub fn any_connected(&self) -> bool {
        self.connected_count() > 0
    }

    /// 模拟手柄连接（测试/开发用）
    pub fn simulate_connect(&self, index: GamepadIndex, name: &str) {
        if index >= 4 { return; }
        let mut pads = self.gamepads.lock().unwrap();
        pads[index as usize].connected = true;
        pads[index as usize].name = name.to_string();
        drop(pads);
        self.event_queue.lock().unwrap().push(GamepadEvent::Connected(index));
    }

    /// 模拟手柄断开（测试/开发用）
    pub fn simulate_disconnect(&self, index: GamepadIndex) {
        if index >= 4 { return; }
        let mut pads = self.gamepads.lock().unwrap();
        pads[index as usize].connected = false;
        drop(pads);
        self.event_queue.lock().unwrap().push(GamepadEvent::Disconnected(index));
    }

    /// 模拟按钮按下（测试/开发用）
    pub fn simulate_button_press(&self, index: GamepadIndex, button: GamepadButton) {
        if index >= 4 { return; }
        let mut pads = self.gamepads.lock().unwrap();
        pads[index as usize].update_button(button, true);
        drop(pads);
        self.event_queue.lock().unwrap().push(GamepadEvent::ButtonPressed { index, button });
    }

    /// 模拟按钮释放（测试/开发用）
    pub fn simulate_button_release(&self, index: GamepadIndex, button: GamepadButton) {
        if index >= 4 { return; }
        let mut pads = self.gamepads.lock().unwrap();
        pads[index as usize].update_button(button, false);
        drop(pads);
        self.event_queue.lock().unwrap().push(GamepadEvent::ButtonReleased { index, button });
    }

    /// 模拟轴移动（测试/开发用）
    pub fn simulate_axis_move(&self, index: GamepadIndex, axis: GamepadAxis, value: f32) {
        if index >= 4 { return; }
        let mut pads = self.gamepads.lock().unwrap();
        pads[index as usize].update_axis(axis, value);
        drop(pads);
        self.event_queue.lock().unwrap().push(GamepadEvent::AxisMoved { index, axis, value });
    }

    /// 每帧更新（重置瞬时状态）
    pub fn update(&self) {
        let mut pads = self.gamepads.lock().unwrap();
        for pad in pads.iter_mut() {
            pad.end_frame();
        }
    }

    /// 获取并清空事件队列
    pub fn poll_events(&self) -> Vec<GamepadEvent> {
        self.event_queue.lock().unwrap().drain(..).collect()
    }

    /// 发送振动命令
    pub fn vibrate(&self, index: GamepadIndex, vibration: GamepadVibration) {
        if index >= 4 { return; }
        self.vibration_queue.lock().unwrap().push((index, vibration));
        // 实际实现中这里会发送到硬件
        log::debug!("Gamepad {} vibrate: {:?}", index, vibration);
    }

    /// 停止指定手柄振动
    pub fn stop_vibration(&self, index: GamepadIndex) {
        self.vibrate(index, GamepadVibration::new(0.0, 0.0, 0));
    }

    /// 检查按钮是否按下（便捷方法）
    pub fn is_button_pressed(&self, index: GamepadIndex, button: GamepadButton) -> bool {
        if index >= 4 { return false; }
        self.gamepads.lock().unwrap()[index as usize].is_button_pressed(button)
    }

    /// 获取轴值（便捷方法）
    pub fn get_axis(&self, index: GamepadIndex, axis: GamepadAxis) -> f32 {
        if index >= 4 { return 0.0; }
        self.gamepads.lock().unwrap()[index as usize].get_axis(axis)
    }

    /// 生成调试报告
    pub fn generate_report(&self) -> String {
        let pads = self.gamepads.lock().unwrap();
        let connected: Vec<String> = pads.iter()
            .filter(|p| p.connected)
            .map(|p| format!("  [{}] {} (connected)", p.index, p.name))
            .collect();

        format!(
            "=== Gamepad Manager Report ===\n\
             Connected: {}/{}\n{}",
            connected.len(),
            4,
            if connected.is_empty() { "  (no gamepads connected)".to_string() } else { connected.join("\n") }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamepad_state_creation() {
        let state = GamepadState::new(0);
        assert_eq!(state.index, 0);
        assert!(!state.connected);
        assert!(!state.is_button_pressed(GamepadButton::Cross));
    }

    #[test]
    fn test_gamepad_button_state_transitions() {
        let mut state = GamepadState::new(0);

        // Released -> JustPressed
        state.update_button(GamepadButton::Cross, true);
        assert_eq!(state.get_button_state(GamepadButton::Cross), ButtonState::JustPressed);
        assert!(state.is_button_just_pressed(GamepadButton::Cross));

        // JustPressed -> Held (after end_frame)
        state.end_frame();
        assert_eq!(state.get_button_state(GamepadButton::Cross), ButtonState::Held);
        assert!(state.is_button_pressed(GamepadButton::Cross));
        assert!(!state.is_button_just_pressed(GamepadButton::Cross));

        // Held -> JustReleased
        state.update_button(GamepadButton::Cross, false);
        assert_eq!(state.get_button_state(GamepadButton::Cross), ButtonState::JustReleased);
        assert!(state.is_button_just_released(GamepadButton::Cross));

        // JustReleased -> Released
        state.end_frame();
        assert_eq!(state.get_button_state(GamepadButton::Cross), ButtonState::Released);
    }

    #[test]
    fn test_gamepad_axis_deadzone() {
        let mut state = GamepadState::new(0);
        state.set_deadzone(0.15);

        // 死区内应返回 0
        state.update_axis(GamepadAxis::LeftX, 0.1);
        assert_eq!(state.get_axis(GamepadAxis::LeftX), 0.0);

        // 死区外应返回非零值
        state.update_axis(GamepadAxis::LeftX, 0.5);
        assert!(state.get_axis(GamepadAxis::LeftX) > 0.0);

        // 负值死区
        state.update_axis(GamepadAxis::LeftX, -0.5);
        assert!(state.get_axis(GamepadAxis::LeftX) < 0.0);
    }

    #[test]
    fn test_gamepad_axis_clamp() {
        let mut state = GamepadState::new(0);
        state.update_axis(GamepadAxis::LeftX, 2.0); // 超出范围
        assert!(state.get_axis_raw(GamepadAxis::LeftX) <= 1.0);

        state.update_axis(GamepadAxis::LeftY, -2.0);
        assert!(state.get_axis_raw(GamepadAxis::LeftY) >= -1.0);
    }

    #[test]
    fn test_gamepad_left_right_stick() {
        let mut state = GamepadState::new(0);
        state.set_deadzone(0.0); // 禁用死区以便测试
        state.update_axis(GamepadAxis::LeftX, 0.8);
        state.update_axis(GamepadAxis::LeftY, -0.6);

        let (lx, ly) = state.get_left_stick();
        assert!((lx - 0.8).abs() < 0.01);
        assert!((ly + 0.6).abs() < 0.01);
    }

    #[test]
    fn test_gamepad_manager_creation() {
        let manager = GamepadManager::new();
        assert_eq!(manager.connected_count(), 0);
        assert!(!manager.any_connected());
    }

    #[test]
    fn test_gamepad_simulate_connect() {
        let manager = GamepadManager::new();
        manager.simulate_connect(0, "Xbox Controller");

        assert_eq!(manager.connected_count(), 1);
        assert!(manager.any_connected());

        let state = manager.get_gamepad(0).unwrap();
        assert!(state.connected);
        assert_eq!(state.name, "Xbox Controller");

        let events = manager.poll_events();
        assert_eq!(events.len(), 1);
        assert!(matches!(events[0], GamepadEvent::Connected(0)));
    }

    #[test]
    fn test_gamepad_simulate_disconnect() {
        let manager = GamepadManager::new();
        manager.simulate_connect(1, "PS5 DualSense");
        manager.simulate_disconnect(1);

        assert_eq!(manager.connected_count(), 0);

        let events = manager.poll_events();
        assert_eq!(events.len(), 2); // Connected + Disconnected
    }

    #[test]
    fn test_gamepad_simulate_button() {
        let manager = GamepadManager::new();
        manager.simulate_connect(0, "Test Gamepad");

        manager.simulate_button_press(0, GamepadButton::Triangle);
        assert!(manager.is_button_pressed(0, GamepadButton::Triangle));
        assert!(!manager.is_button_pressed(0, GamepadButton::Circle));

        manager.simulate_button_release(0, GamepadButton::Triangle);
        manager.update(); // end_frame
        assert!(!manager.is_button_pressed(0, GamepadButton::Triangle));
    }

    #[test]
    fn test_gamepad_simulate_axis() {
        let manager = GamepadManager::new();
        manager.simulate_connect(0, "Test Gamepad");
        manager.simulate_axis_move(0, GamepadAxis::RightX, 0.9);

        let value = manager.get_axis(0, GamepadAxis::RightX);
        // 应用死区后应该 > 0
        assert!(value > 0.0);
    }

    #[test]
    fn test_gamepad_poll_events() {
        let manager = GamepadManager::new();
        manager.simulate_connect(0, "Pad1");
        manager.simulate_button_press(0, GamepadButton::Start);
        manager.simulate_axis_move(0, GamepadAxis::LeftX, 0.5);

        let events = manager.poll_events();
        assert_eq!(events.len(), 3);

        // 再次 poll 应该为空
        let events2 = manager.poll_events();
        assert!(events2.is_empty());
    }

    #[test]
    fn test_gamepad_vibration() {
        let vib = GamepadVibration::new(0.8, 0.5, 500);
        assert_eq!(vib.left_motor, 0.8);
        assert_eq!(vib.right_motor, 0.5);
        assert_eq!(vib.duration_ms, 500);

        let vib_strong = GamepadVibration::strong_pulse(200);
        assert_eq!(vib_strong.left_motor, 1.0);
        assert_eq!(vib_strong.right_motor, 1.0);

        let vib_clamp = GamepadVibration::new(2.0, -1.0, 100);
        assert_eq!(vib_clamp.left_motor, 1.0);
        assert_eq!(vib_clamp.right_motor, 0.0);
    }

    #[test]
    fn test_gamepad_out_of_range() {
        let manager = GamepadManager::new();
        // 索引超出范围应该安全
        assert!(manager.get_gamepad(4).is_none());
        assert!(!manager.is_button_pressed(5, GamepadButton::Cross));
        assert_eq!(manager.get_axis(10, GamepadAxis::LeftX), 0.0);
    }

    #[test]
    fn test_gamepad_button_names() {
        assert_eq!(GamepadButton::Cross.name(), "Cross/A");
        assert_eq!(GamepadButton::Triangle.name(), "Triangle/Y");
        assert_eq!(GamepadButton::Start.name(), "Start/Options");
        assert_eq!(GamepadButton::LeftShoulder.name(), "L1/LB");
    }

    #[test]
    fn test_gamepad_manager_report() {
        let manager = GamepadManager::new();
        manager.simulate_connect(0, "Player1 Pad");
        manager.simulate_connect(2, "Player3 Pad");

        let report = manager.generate_report();
        assert!(report.contains("Gamepad Manager Report"));
        assert!(report.contains("2/4"));
        assert!(report.contains("Player1 Pad"));
    }

    #[test]
    fn test_gamepad_connected_indices() {
        let manager = GamepadManager::new();
        manager.simulate_connect(0, "P1");
        manager.simulate_connect(3, "P4");

        let indices = manager.get_connected_indices();
        assert_eq!(indices.len(), 2);
        assert!(indices.contains(&0));
        assert!(indices.contains(&3));
    }

    #[test]
    fn test_gamepad_all_buttons_initial_state() {
        let state = GamepadState::new(0);
        for btn in GamepadButton::all() {
            assert_eq!(state.get_button_state(*btn), ButtonState::Released,
                "Button {:?} should be released initially", btn);
        }
    }

    #[test]
    fn test_gamepad_all_axes_initial_value() {
        let state = GamepadState::new(0);
        for axis in GamepadAxis::all() {
            assert_eq!(state.get_axis_raw(*axis), 0.0,
                "Axis {:?} should be 0.0 initially", axis);
        }
    }
}
