# Cocos2d-Rust 测试指南

## 测试概述

本项目包含完整的单元测试和集成测试套件,用于验证 Cocos2d-Rust 引擎的各个模块功能。

## 测试结构

```
cocos2d-rust/
├── tests/                          # 集成测试
│   ├── integration_tests.rs        # 测试入口
│   ├── test_helpers.rs             # 测试辅助工具
│   ├── math_tests.rs               # 数学库测试
│   ├── scene_tests.rs              # 场景系统测试
│   ├── renderer_tests.rs           # 渲染系统测试
│   ├── ui_tests.rs                 # UI 组件测试
│   ├── physics_tests.rs            # 物理引擎测试
│   ├── animation_tests.rs          # 动画系统测试
│   ├── audio_tests.rs              # 音频系统测试
│   └── integration_scenarios.rs    # 集成场景测试
├── benches/                        # 性能基准测试
│   └── performance_benchmarks.rs
└── src/                            # 源代码包含单元测试
    └── **/*.rs                     # 各模块内嵌单元测试

```

## 运行测试

### 运行所有测试
```bash
cargo test
```

### 运行特定测试模块
```bash
# 数学库测试
cargo test --test integration_tests math_tests

# 场景系统测试
cargo test --test integration_tests scene_tests

# 物理引擎测试
cargo test --test integration_tests physics_tests

# UI 组件测试
cargo test --test integration_tests ui_tests
```

### 运行单元测试 (模块内测试)
```bash
# Vec2 单元测试
cargo test --lib vec2

# Mat4 单元测试
cargo test --lib mat4

# 所有单元测试
cargo test --lib
```

### 运行集成测试
```bash
cargo test --test integration_tests
```

### 运行性能基准测试
```bash
cargo bench
```

### 显示详细输出
```bash
cargo test -- --nocapture
```

### 并行运行测试
```bash
cargo test -- --test-threads=4
```

## 测试覆盖范围

### 1. 数学库测试 (`math_tests.rs`)
- ✅ Vec2: 算术运算、长度、归一化、点积、叉积、旋转、距离、插值
- ✅ Vec3: 向量运算、归一化、叉积
- ✅ Vec4: 基本向量操作
- ✅ Mat4: 单位矩阵、平移、缩放、旋转、矩阵乘法、求逆、透视投影、正交投影、LookAt
- ✅ Quaternion: 四元数旋转

**测试数量**: 40+ 测试用例

### 2. 场景系统测试 (`scene_tests.rs`)
- ✅ Node: 创建、位置、缩放、旋转、可见性、透明度
- ✅ 内容大小、锚点、Z 顺序、标签、名称
- ✅ 父子关系管理
- ✅ 节点查找 (按标签、按名称)
- ✅ 世界坐标转换
- ✅ 运行状态、暂停/恢复

**测试数量**: 25+ 测试用例

### 3. 渲染系统测试 (`renderer_tests.rs`)
- ✅ Renderer: 创建、初始化、清除颜色、视口
- ✅ Texture: 创建、属性、像素格式、内容大小
- ✅ Material: 材质创建和属性
- ✅ RenderCommand: 渲染命令

**测试数量**: 10+ 测试用例

### 4. UI 组件测试 (`ui_tests.rs`)
- ✅ Button: 创建、标题、启用状态、点击回调
- ✅ Label: 创建、字符串、字体大小
- ✅ Slider: 创建、值、百分比
- ✅ TextField: 创建、占位符、最大长度、密码模式
- ✅ Widget: 位置、大小、触摸启用

**测试数量**: 20+ 测试用例

### 5. 物理引擎测试 (`physics_tests.rs`)
- ✅ PhysicsWorld: 创建、重力
- ✅ PhysicsBody: 创建、动态属性、质量、速度、位置、旋转
- ✅ PhysicsShape: 盒子形状、圆形状
- ✅ 碰撞检测
- ✅ 力和冲量

**测试数量**: 15+ 测试用例

### 6. 动画系统测试 (`animation_tests.rs`)
- ✅ Animation: 创建、添加帧、延迟、循环
- ✅ Animate: 动画动作
- ✅ SpriteFrame: 精灵帧、矩形、偏移
- ✅ 动画时长、恢复原始帧

**测试数量**: 10+ 测试用例

### 7. 音频系统测试 (`audio_tests.rs`)
- ✅ AudioEngine: 创建、初始化
- ✅ 音效: 预加载、播放、暂停、恢复、停止
- ✅ 音量控制
- ✅ 背景音乐: 播放、停止、暂停、音量

**测试数量**: 12+ 测试用例

### 8. 集成场景测试 (`integration_scenarios.rs`)
- ✅ 完整场景设置
- ✅ 多精灵交互
- ✅ 场景转换
- ✅ 游戏循环模拟
- ✅ 用户数据保存和加载

**测试数量**: 5+ 综合测试场景

## 性能基准测试

性能基准测试使用 `criterion` 框架,包括:

- **Vec2 操作**: 加法、归一化、距离、点积
- **Vec3 操作**: 叉积、归一化
- **Mat4 操作**: 矩阵乘法、求逆、点变换
- **节点层级**: 添加子节点、查找子节点

运行基准测试:
```bash
cargo bench
```

## 测试辅助工具

### `test_helpers.rs`
提供通用测试辅助函数:
- `assert_float_eq`: 浮点数相等断言 (ε = 0.0001)
- `assert_float_near`: 自定义精度的浮点数断言
- `EPSILON`: 默认浮点数比较精度

## 编写新测试

### 单元测试示例
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_my_function() {
        let result = my_function(5);
        assert_eq!(result, 10);
    }
}
```

### 集成测试示例
在 `tests/` 目录下创建新文件:
```rust
use cocos2d_rust::module::MyStruct;

#[test]
fn test_integration_scenario() {
    let obj = MyStruct::new();
    assert!(obj.is_valid());
}
```

## 持续集成

建议在 CI/CD 流程中运行:
```bash
# 运行所有测试
cargo test --all

# 运行基准测试
cargo bench --no-run

# 检查测试覆盖率 (需要 tarpaulin)
cargo tarpaulin --out Html
```

## 测试最佳实践

1. **命名规范**: 使用 `test_` 前缀,描述性命名
2. **独立性**: 每个测试应该独立运行
3. **可重复性**: 测试结果应该可重复
4. **清晰断言**: 使用清晰的断言消息
5. **边界条件**: 测试边界情况和错误路径
6. **性能**: 避免在测试中使用 sleep,使用模拟时间

## 测试覆盖率目标

- **单元测试**: 目标 80%+ 代码覆盖率
- **集成测试**: 覆盖所有主要功能路径
- **性能测试**: 关键路径性能基准

## 已知限制

由于某些模块需要 OpenGL 上下文或系统资源,部分测试可能需要:
- 模拟对象 (Mock)
- 测试夹具 (Fixtures)
- 条件编译 (#[cfg(feature = "integration")])

## 故障排查

### 测试失败
```bash
# 运行单个测试查看详细输出
cargo test test_name -- --nocapture

# 显示所有测试输出
RUST_TEST_THREADS=1 cargo test -- --nocapture
```

### 性能测试
```bash
# 生成详细的性能报告
cargo bench -- --verbose
```

## 贡献指南

添加新功能时,请确保:
1. 编写对应的单元测试
2. 更新集成测试(如果适用)
3. 运行 `cargo test` 确保所有测试通过
4. 更新本文档

## 总结

本测试套件提供:
- **130+ 测试用例** 覆盖核心功能
- **性能基准测试** 确保性能
- **集成测试** 验证模块协作
- **清晰的文档** 便于维护

运行 `cargo test` 即可验证整个引擎的正确性!
