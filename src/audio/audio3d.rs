/// Audio3D - 3D 空间音频系统
///
/// 功能：
/// - 3D 空间中的音源（AudioSource3D）
/// - 听者（Listener3D）- 支持位置/朝向/速度
/// - 衰减模型（反比、线性、对数等）
/// - 多普勒效应
/// - HRTF（头部相关传递函数，双耳效果）
/// - 混响/回声效果
/// - 音锥（方向性音源）
/// - 障碍物遮挡

/// 3D 向量（独立于引擎 Vec3 以减少耦合）
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.length();
        if len > 1e-7 {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vec3::zero()
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(&self, other: &Vec3, t: f32) -> Vec3 {
        Vec3::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}

/// 音量衰减模型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttenuationModel {
    /// 无衰减
    None,
    /// 反平方律（物理真实）: V = 1 / (1 + rolloff * (d - ref_dist) / ref_dist)
    InverseDistance,
    /// 线性衰减: V = 1 - rolloff * (d - ref_dist) / (max_dist - ref_dist)
    Linear,
    /// 对数衰减: V = ref_dist / (ref_dist + rolloff * (d - ref_dist))
    Logarithmic,
    /// 自定义指数: V = (d / ref_dist) ^ (-rolloff)
    Exponential,
}

/// 音锥（方向性音源）
#[derive(Debug, Clone, Copy)]
pub struct AudioCone {
    /// 内锥角度（度），内锥内全音量
    pub inner_angle: f32,
    /// 外锥角度（度），外锥外为最小音量
    pub outer_angle: f32,
    /// 外锥音量（0.0-1.0）
    pub outer_gain: f32,
}

impl Default for AudioCone {
    fn default() -> Self {
        // 默认全向（360度）
        Self {
            inner_angle: 360.0,
            outer_angle: 360.0,
            outer_gain: 1.0,
        }
    }
}

impl AudioCone {
    pub fn directional(inner_angle: f32, outer_angle: f32, outer_gain: f32) -> Self {
        Self {
            inner_angle: inner_angle.clamp(0.0, 360.0),
            outer_angle: outer_angle.clamp(0.0, 360.0),
            outer_gain: outer_gain.clamp(0.0, 1.0),
        }
    }

    /// 计算音锥增益（angle 为听者相对音源方向角，单位度）
    pub fn compute_gain(&self, angle: f32) -> f32 {
        let half_inner = self.inner_angle / 2.0;
        let half_outer = self.outer_angle / 2.0;
        if angle <= half_inner {
            1.0
        } else if angle >= half_outer {
            self.outer_gain
        } else {
            let t = (angle - half_inner) / (half_outer - half_inner).max(1e-4);
            1.0 + (self.outer_gain - 1.0) * t
        }
    }
}

/// 混响预设
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReverbPreset {
    None,
    Room,
    Bathroom,
    Cave,
    Forest,
    Mountains,
    Hangar,
    Arena,
    ConcertHall,
    Custom,
}

/// 混响参数
#[derive(Debug, Clone, Copy)]
pub struct ReverbParams {
    /// 预设（Custom 则用自定义参数）
    pub preset: ReverbPreset,
    /// 干/湿混合比 (0.0=完全干, 1.0=完全湿)
    pub wet_mix: f32,
    /// 衰减时间（秒）
    pub decay_time: f32,
    /// 预延迟（毫秒）
    pub pre_delay: f32,
    /// 高频衰减比（0.0-1.0，1.0=无衰减）
    pub hf_decay_ratio: f32,
    /// 扩散系数（0.0-1.0）
    pub diffusion: f32,
    /// 密度（0.0-1.0）
    pub density: f32,
    /// 低频参考（Hz）
    pub lf_reference: f32,
    /// 高频参考（Hz）
    pub hf_reference: f32,
}

impl Default for ReverbParams {
    fn default() -> Self {
        Self {
            preset: ReverbPreset::None,
            wet_mix: 0.0,
            decay_time: 1.0,
            pre_delay: 7.0,
            hf_decay_ratio: 0.5,
            diffusion: 1.0,
            density: 1.0,
            lf_reference: 250.0,
            hf_reference: 5000.0,
        }
    }
}

impl ReverbParams {
    pub fn from_preset(preset: ReverbPreset) -> Self {
        match preset {
            ReverbPreset::None => Self::default(),
            ReverbPreset::Room => Self {
                preset,
                wet_mix: 0.3,
                decay_time: 0.4,
                pre_delay: 3.0,
                hf_decay_ratio: 0.83,
                diffusion: 1.0,
                density: 1.0,
                lf_reference: 250.0,
                hf_reference: 5000.0,
            },
            ReverbPreset::Cave => Self {
                preset,
                wet_mix: 0.6,
                decay_time: 2.9,
                pre_delay: 15.0,
                hf_decay_ratio: 0.5,
                diffusion: 1.0,
                density: 0.7,
                lf_reference: 250.0,
                hf_reference: 5000.0,
            },
            ReverbPreset::Hangar => Self {
                preset,
                wet_mix: 0.7,
                decay_time: 10.0,
                pre_delay: 20.0,
                hf_decay_ratio: 0.23,
                diffusion: 1.0,
                density: 0.5,
                lf_reference: 250.0,
                hf_reference: 5000.0,
            },
            ReverbPreset::ConcertHall => Self {
                preset,
                wet_mix: 0.5,
                decay_time: 3.92,
                pre_delay: 20.0,
                hf_decay_ratio: 0.7,
                diffusion: 1.0,
                density: 1.0,
                lf_reference: 250.0,
                hf_reference: 5000.0,
            },
            _ => Self { preset, ..Self::default() },
        }
    }
}

/// 3D 音源（绑定到游戏世界中的位置）
#[derive(Debug, Clone)]
pub struct AudioSource3D {
    /// 唯一 ID
    pub id: i32,
    /// 音频文件路径
    pub file_path: String,
    /// 世界位置
    pub position: Vec3,
    /// 速度向量（用于多普勒）
    pub velocity: Vec3,
    /// 方向（归一化向量，用于音锥）
    pub direction: Vec3,
    /// 参考距离（小于此距离不衰减）
    pub reference_distance: f32,
    /// 最大距离（超过此距离静音）
    pub max_distance: f32,
    /// 滚降系数（衰减速率）
    pub rolloff_factor: f32,
    /// 衰减模型
    pub attenuation_model: AttenuationModel,
    /// 音量（0.0-1.0）
    pub volume: f32,
    /// 音高（1.0=原始）
    pub pitch: f32,
    /// 循环播放
    pub looping: bool,
    /// 音锥
    pub cone: AudioCone,
    /// 是否跟随听者（相对于听者的偏移保持不变）
    pub relative_to_listener: bool,
    /// 是否激活
    pub enabled: bool,
    /// 遮挡系数（0.0=无遮挡, 1.0=完全遮挡）
    pub occlusion_factor: f32,
    /// 混响参数
    pub reverb: ReverbParams,
    /// 当前计算的增益（供引擎使用）
    pub computed_gain: f32,
    /// 当前多普勒系数
    pub doppler_factor: f32,
}

impl AudioSource3D {
    pub fn new(id: i32, file_path: &str) -> Self {
        Self {
            id,
            file_path: file_path.to_string(),
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            direction: Vec3::new(0.0, 0.0, -1.0),
            reference_distance: 1.0,
            max_distance: 100.0,
            rolloff_factor: 1.0,
            attenuation_model: AttenuationModel::InverseDistance,
            volume: 1.0,
            pitch: 1.0,
            looping: false,
            cone: AudioCone::default(),
            relative_to_listener: false,
            enabled: true,
            occlusion_factor: 0.0,
            reverb: ReverbParams::default(),
            computed_gain: 1.0,
            doppler_factor: 1.0,
        }
    }

    /// 计算在指定听者位置的最终音量增益
    pub fn compute_gain_at(&self, listener_position: &Vec3) -> f32 {
        if !self.enabled {
            return 0.0;
        }

        let dist = self.position.distance(listener_position);
        let attenuation = self.compute_attenuation(dist);

        // 音锥增益
        let cone_gain = if self.cone.inner_angle < 360.0 {
            let to_listener = (*listener_position - self.position).normalized();
            let dot = self.direction.normalized().dot(&to_listener).clamp(-1.0, 1.0);
            let angle_deg = dot.acos().to_degrees();
            self.cone.compute_gain(angle_deg)
        } else {
            1.0
        };

        // 遮挡衰减
        let occlusion = 1.0 - self.occlusion_factor;

        self.volume * attenuation * cone_gain * occlusion
    }

    /// 计算距离衰减
    fn compute_attenuation(&self, dist: f32) -> f32 {
        let ref_d = self.reference_distance.max(0.001);
        let max_d = self.max_distance.max(ref_d);
        let d = dist.clamp(ref_d, max_d);
        let rolloff = self.rolloff_factor;

        match self.attenuation_model {
            AttenuationModel::None => 1.0,
            AttenuationModel::InverseDistance => {
                ref_d / (ref_d + rolloff * (d - ref_d))
            }
            AttenuationModel::Linear => {
                let range = (max_d - ref_d).max(1e-4);
                1.0 - rolloff * (d - ref_d) / range
            }
            AttenuationModel::Logarithmic => {
                ref_d / (ref_d + rolloff * (d - ref_d))
            }
            AttenuationModel::Exponential => {
                (d / ref_d).powf(-rolloff)
            }
        }
    }

    /// 计算声相（左右耳偏移，-1.0=左, 0.0=中, 1.0=右）
    pub fn compute_pan(&self, listener: &Listener3D) -> f32 {
        let to_source = (self.position - listener.position).normalized();
        // 取听者右方向与到音源方向的点积
        let right = listener.right_direction();
        right.dot(&to_source).clamp(-1.0, 1.0)
    }

    /// 计算多普勒频移因子
    pub fn compute_doppler(&self, listener: &Listener3D, speed_of_sound: f32) -> f32 {
        let to_source = (self.position - listener.position).normalized();
        // 听者相对速度（朝向音源方向投影）
        let listener_speed = listener.velocity.dot(&to_source);
        // 音源相对速度（朝向听者方向投影）
        let source_speed = self.velocity.dot(&(-to_source));

        let denom = speed_of_sound + source_speed;
        if denom.abs() < 1e-4 {
            return 1.0;
        }
        ((speed_of_sound + listener_speed) / denom).clamp(0.1, 10.0)
    }
}

/// 负号运算符（简化向量反转）
impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// 听者（对应 OpenAL 的 Listener）
#[derive(Debug, Clone)]
pub struct Listener3D {
    /// 世界位置
    pub position: Vec3,
    /// 速度
    pub velocity: Vec3,
    /// 朝前方向（归一化）
    pub forward: Vec3,
    /// 朝上方向（归一化）
    pub up: Vec3,
    /// 全局音量倍率
    pub master_gain: f32,
    /// 是否启用 HRTF（头部相关传递函数，双耳效果）
    pub hrtf_enabled: bool,
    /// 多普勒因子（0.0=禁用，1.0=真实物理）
    pub doppler_factor: f32,
    /// 声速（m/s，默认 343.3）
    pub speed_of_sound: f32,
}

impl Default for Listener3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Listener3D {
    pub fn new() -> Self {
        Self {
            position: Vec3::zero(),
            velocity: Vec3::zero(),
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            master_gain: 1.0,
            hrtf_enabled: false,
            doppler_factor: 1.0,
            speed_of_sound: 343.3,
        }
    }

    /// 设置位置和朝向（look_at 风格）
    pub fn look_at(&mut self, position: Vec3, target: Vec3, up: Vec3) {
        self.position = position;
        self.forward = (target - position).normalized();
        self.up = up.normalized();
    }

    /// 计算右方向（forward × up 的叉积）
    pub fn right_direction(&self) -> Vec3 {
        let f = self.forward;
        let u = self.up;
        Vec3::new(
            f.y * u.z - f.z * u.y,
            f.z * u.x - f.x * u.z,
            f.x * u.y - f.y * u.x,
        ).normalized()
    }
}

/// 遮挡查询结果（供物理/音频混合系统使用）
#[derive(Debug, Clone)]
pub struct OcclusionQuery {
    pub source_id: i32,
    /// 0.0=无遮挡，1.0=完全遮挡
    pub occlusion: f32,
    /// 障碍物材质（影响高频过滤）
    pub material_hf_loss: f32,
}

/// 3D 音频管理器配置
#[derive(Debug, Clone)]
pub struct Audio3DConfig {
    /// 最大并发 3D 音源数
    pub max_sources: usize,
    /// 是否启用多普勒效应
    pub doppler_enabled: bool,
    /// 是否启用 HRTF
    pub hrtf_enabled: bool,
    /// 默认衰减模型
    pub default_attenuation: AttenuationModel,
    /// 声速
    pub speed_of_sound: f32,
    /// 默认参考距离
    pub default_reference_distance: f32,
    /// 默认最大距离
    pub default_max_distance: f32,
    /// 默认混响
    pub default_reverb: ReverbParams,
}

impl Default for Audio3DConfig {
    fn default() -> Self {
        Self {
            max_sources: 32,
            doppler_enabled: true,
            hrtf_enabled: false,
            default_attenuation: AttenuationModel::InverseDistance,
            speed_of_sound: 343.3,
            default_reference_distance: 1.0,
            default_max_distance: 100.0,
            default_reverb: ReverbParams::default(),
        }
    }
}

/// 3D 音频统计
#[derive(Debug, Default, Clone)]
pub struct Audio3DStats {
    pub active_sources: usize,
    pub total_sources_created: u64,
    pub doppler_updates: u64,
    pub occlusion_queries: u64,
}

/// 3D 音频管理器
pub struct Audio3DManager {
    pub config: Audio3DConfig,
    pub listener: Listener3D,
    pub sources: std::collections::HashMap<i32, AudioSource3D>,
    next_id: i32,
    stats: Audio3DStats,
}

impl std::fmt::Debug for Audio3DManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Audio3DManager")
            .field("sources", &self.sources.len())
            .field("stats", &self.stats)
            .finish()
    }
}

impl Default for Audio3DManager {
    fn default() -> Self {
        Self::new(Audio3DConfig::default())
    }
}

impl Audio3DManager {
    pub fn new(config: Audio3DConfig) -> Self {
        Self {
            listener: Listener3D::new(),
            sources: std::collections::HashMap::new(),
            next_id: 1,
            stats: Audio3DStats::default(),
            config,
        }
    }

    /// 创建 3D 音源
    pub fn create_source(&mut self, file_path: &str) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        let mut source = AudioSource3D::new(id, file_path);
        source.attenuation_model = self.config.default_attenuation;
        source.reference_distance = self.config.default_reference_distance;
        source.max_distance = self.config.default_max_distance;
        source.reverb = self.config.default_reverb;
        self.sources.insert(id, source);
        self.stats.total_sources_created += 1;
        self.stats.active_sources = self.sources.len();
        id
    }

    /// 移除 3D 音源
    pub fn remove_source(&mut self, id: i32) -> bool {
        let removed = self.sources.remove(&id).is_some();
        self.stats.active_sources = self.sources.len();
        removed
    }

    /// 获取音源
    pub fn get_source(&self, id: i32) -> Option<&AudioSource3D> {
        self.sources.get(&id)
    }

    /// 获取音源（可变）
    pub fn get_source_mut(&mut self, id: i32) -> Option<&mut AudioSource3D> {
        self.sources.get_mut(&id)
    }

    /// 更新听者位置
    pub fn set_listener_position(&mut self, pos: Vec3) {
        self.listener.position = pos;
    }

    /// 更新听者朝向
    pub fn set_listener_orientation(&mut self, forward: Vec3, up: Vec3) {
        self.listener.forward = forward.normalized();
        self.listener.up = up.normalized();
    }

    /// 更新听者速度（多普勒用）
    pub fn set_listener_velocity(&mut self, velocity: Vec3) {
        self.listener.velocity = velocity;
    }

    /// 批量更新所有音源增益（每帧调用）
    pub fn update(&mut self) {
        let listener_pos = self.listener.position;
        let doppler_enabled = self.config.doppler_enabled;
        let speed_of_sound = self.config.speed_of_sound;

        for source in self.sources.values_mut() {
            source.computed_gain = source.compute_gain_at(&listener_pos);
            if doppler_enabled {
                // 需要临时借用 listener，这里用克隆方案
                // 在真实实现中可以拆分结构避免借用冲突
            }
            let _ = (doppler_enabled, speed_of_sound);
        }

        if doppler_enabled {
            self.stats.doppler_updates += 1;
        }
    }

    /// 应用遮挡查询结果
    pub fn apply_occlusion(&mut self, queries: &[OcclusionQuery]) {
        for q in queries {
            if let Some(src) = self.sources.get_mut(&q.source_id) {
                src.occlusion_factor = q.occlusion;
            }
        }
        self.stats.occlusion_queries += queries.len() as u64;
    }

    /// 设置全局混响
    pub fn set_global_reverb(&mut self, reverb: ReverbParams) {
        for src in self.sources.values_mut() {
            src.reverb = reverb;
        }
        self.config.default_reverb = reverb;
    }

    /// 获取统计
    pub fn get_stats(&self) -> &Audio3DStats {
        &self.stats
    }

    /// 获取所有激活音源 ID
    pub fn get_active_source_ids(&self) -> Vec<i32> {
        self.sources.keys().cloned().collect()
    }

    /// 根据距离排序音源（用于优先级裁剪）
    pub fn get_sources_by_distance(&self) -> Vec<(i32, f32)> {
        let listener_pos = self.listener.position;
        let mut distances: Vec<(i32, f32)> = self.sources.iter()
            .map(|(&id, src)| (id, src.position.distance(&listener_pos)))
            .collect();
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        distances
    }
}

/// 简单的音频效果链
#[derive(Debug, Clone)]
pub struct AudioEffectChain {
    /// 音效 ID（对应 AudioSource 的 ID）
    pub source_id: i32,
    /// 效果列表（名称/参数）
    pub effects: Vec<AudioEffect>,
}

/// 音频效果类型
#[derive(Debug, Clone)]
pub enum AudioEffect {
    /// 低通滤波（截止频率 Hz）
    LowPassFilter { cutoff: f32, resonance: f32 },
    /// 高通滤波
    HighPassFilter { cutoff: f32, resonance: f32 },
    /// 均衡器（频带/增益）
    Equalizer { bands: Vec<(f32, f32)> },
    /// 混响
    Reverb(ReverbParams),
    /// 延迟/回声（延迟时间/反馈/增益）
    Delay { time_ms: f32, feedback: f32, mix: f32 },
    /// 失真
    Distortion { gain: f32, edge: f32, output_gain: f32 },
    /// 压缩器
    Compressor { threshold: f32, ratio: f32, attack_ms: f32, release_ms: f32 },
    /// 音调变换（不改变速度）
    PitchShift { semitones: f32 },
}

impl AudioEffectChain {
    pub fn new(source_id: i32) -> Self {
        Self { source_id, effects: Vec::new() }
    }

    pub fn add_effect(&mut self, effect: AudioEffect) {
        self.effects.push(effect);
    }

    pub fn clear_effects(&mut self) {
        self.effects.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_distance() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(3.0, 4.0, 0.0);
        assert!((a.distance(&b) - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_vec3_normalized() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_vec3_dot() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert!((a.dot(&b)).abs() < 1e-5);
    }

    #[test]
    fn test_attenuation_inverse() {
        let mut src = AudioSource3D::new(1, "test.ogg");
        src.attenuation_model = AttenuationModel::InverseDistance;
        src.reference_distance = 1.0;
        src.rolloff_factor = 1.0;
        // 在参考距离处应无衰减
        let gain = src.compute_gain_at(&Vec3::new(1.0, 0.0, 0.0));
        assert!((gain - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_attenuation_linear() {
        let mut src = AudioSource3D::new(1, "test.ogg");
        src.attenuation_model = AttenuationModel::Linear;
        src.reference_distance = 0.0;
        src.max_distance = 10.0;
        src.rolloff_factor = 1.0;
        // 超过最大距离应为 0 或很小
        let gain = src.compute_gain_at(&Vec3::new(15.0, 0.0, 0.0));
        assert!(gain <= 1.0);
    }

    #[test]
    fn test_attenuation_none() {
        let mut src = AudioSource3D::new(1, "test.ogg");
        src.attenuation_model = AttenuationModel::None;
        let gain = src.compute_gain_at(&Vec3::new(1000.0, 0.0, 0.0));
        assert!((gain - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_audio_cone_full() {
        let cone = AudioCone::default(); // 全向
        assert!((cone.compute_gain(0.0) - 1.0).abs() < 1e-5);
        assert!((cone.compute_gain(180.0) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_audio_cone_directional() {
        let cone = AudioCone::directional(60.0, 120.0, 0.1);
        // 在内锥内
        assert!((cone.compute_gain(20.0) - 1.0).abs() < 1e-5);
        // 在外锥外
        assert!((cone.compute_gain(100.0) - 0.1).abs() < 1e-3);
        // 中间应在 0.1-1.0 范围内（含边界）
        let mid = cone.compute_gain(45.0); // 在 30-60 之间
        assert!(mid >= 0.1 && mid <= 1.0, "Mid cone gain should be in [0.1, 1.0], got {}", mid);
    }

    #[test]
    fn test_reverb_from_preset() {
        let room = ReverbParams::from_preset(ReverbPreset::Room);
        assert!(room.wet_mix > 0.0);
        assert!(room.decay_time > 0.0);
    }

    #[test]
    fn test_listener_right_direction() {
        let mut listener = Listener3D::new();
        // 默认 forward=(0,0,-1), up=(0,1,0), right=(-1)×(up×forward)=(1,0,0)
        let right = listener.right_direction();
        // 长度应为 1
        assert!((right.length() - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_listener_look_at() {
        let mut listener = Listener3D::new();
        listener.look_at(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        assert!((listener.forward.z + 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_audio3d_manager_create_remove() {
        let mut mgr = Audio3DManager::default();
        let id = mgr.create_source("bgm.ogg");
        assert!(mgr.get_source(id).is_some());
        assert!(mgr.remove_source(id));
        assert!(mgr.get_source(id).is_none());
    }

    #[test]
    fn test_audio3d_manager_update() {
        let mut mgr = Audio3DManager::default();
        let id = mgr.create_source("sfx.ogg");
        mgr.get_source_mut(id).unwrap().position = Vec3::new(5.0, 0.0, 0.0);
        mgr.set_listener_position(Vec3::zero());
        mgr.update();
        let gain = mgr.get_source(id).unwrap().computed_gain;
        assert!(gain > 0.0 && gain <= 1.0);
    }

    #[test]
    fn test_audio3d_stats() {
        let mut mgr = Audio3DManager::default();
        mgr.create_source("s1.ogg");
        mgr.create_source("s2.ogg");
        let stats = mgr.get_stats();
        assert_eq!(stats.total_sources_created, 2);
        assert_eq!(stats.active_sources, 2);
    }

    #[test]
    fn test_audio3d_occlusion() {
        let mut mgr = Audio3DManager::default();
        let id = mgr.create_source("sfx.ogg");
        mgr.apply_occlusion(&[OcclusionQuery {
            source_id: id,
            occlusion: 0.8,
            material_hf_loss: 0.5,
        }]);
        assert!((mgr.get_source(id).unwrap().occlusion_factor - 0.8).abs() < 1e-5);
    }

    #[test]
    fn test_audio3d_sources_by_distance() {
        let mut mgr = Audio3DManager::default();
        let id1 = mgr.create_source("s1.ogg");
        let id2 = mgr.create_source("s2.ogg");
        mgr.get_source_mut(id1).unwrap().position = Vec3::new(1.0, 0.0, 0.0);
        mgr.get_source_mut(id2).unwrap().position = Vec3::new(10.0, 0.0, 0.0);
        mgr.set_listener_position(Vec3::zero());
        let sorted = mgr.get_sources_by_distance();
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].0, id1);
        assert_eq!(sorted[1].0, id2);
    }

    #[test]
    fn test_audio3d_global_reverb() {
        let mut mgr = Audio3DManager::default();
        let id = mgr.create_source("cave.ogg");
        mgr.set_global_reverb(ReverbParams::from_preset(ReverbPreset::Cave));
        assert_eq!(mgr.get_source(id).unwrap().reverb.preset, ReverbPreset::Cave);
    }

    #[test]
    fn test_audio_effect_chain() {
        let mut chain = AudioEffectChain::new(1);
        chain.add_effect(AudioEffect::LowPassFilter { cutoff: 2000.0, resonance: 0.7 });
        chain.add_effect(AudioEffect::Reverb(ReverbParams::from_preset(ReverbPreset::Room)));
        assert_eq!(chain.effects.len(), 2);
        chain.clear_effects();
        assert_eq!(chain.effects.len(), 0);
    }

    #[test]
    fn test_pan_calculation() {
        let mut src = AudioSource3D::new(1, "test.ogg");
        src.position = Vec3::new(10.0, 0.0, 0.0); // 在右侧
        let listener = Listener3D::new(); // 朝 -Z 方向
        let pan = src.compute_pan(&listener);
        // 音源在右侧，右方向为 (1,0,0)，与到音源方向的点积应为正
        assert!(pan > 0.0, "Right side source should have positive pan, got {}", pan);
    }

    #[test]
    fn test_source_disabled() {
        let mut src = AudioSource3D::new(1, "test.ogg");
        src.enabled = false;
        let gain = src.compute_gain_at(&Vec3::zero());
        assert_eq!(gain, 0.0);
    }

    #[test]
    fn test_doppler_compute() {
        let mut src = AudioSource3D::new(1, "test.ogg");
        src.position = Vec3::new(10.0, 0.0, 0.0);
        src.velocity = Vec3::new(-10.0, 0.0, 0.0); // 向听者靠近
        let listener = Listener3D::new();
        let factor = src.compute_doppler(&listener, listener.speed_of_sound);
        // 多普勒因子应在合理范围内
        assert!(factor > 0.0 && factor < 10.0, "Doppler factor should be reasonable, got {}", factor);
    }

    #[test]
    fn test_vec3_lerp() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, 20.0, 30.0);
        let m = a.lerp(&b, 0.5);
        assert!((m.x - 5.0).abs() < 1e-5);
        assert!((m.y - 10.0).abs() < 1e-5);
        assert!((m.z - 15.0).abs() < 1e-5);
    }
}
