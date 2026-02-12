# Cocos2d-Rust 快速测试指南

## 🚀 快速开始

### 运行所有测试
```bash
cd cocos2d-rust
cargo test --lib
```

### 查看测试总结
```bash
cat TEST_SUMMARY.txt
```

### 查看详细报告
```bash
cat UNIT_TEST_REPORT.md
```

---

## 📊 当前状态 (2026-02-12)

```
✅ 通过:  1141 个测试  (97.9%)
❌ 失败:    25 个测试  (2.1%)
⏸️  忽略:     2 个测试
⏱️  执行:   0.51秒
```

---

## 🎯 按模块测试

### 测试特定模块

```bash
# 数学库 (100% 通过 ✅)
cargo test --lib math::

# 物理引擎 (100% 通过 ✅)
cargo test --lib physics::

# 渲染系统 (100% 通过 ✅)
cargo test --lib renderer::

# 动画系统 (100% 通过 ✅)
cargo test --lib animation::

# 粒子系统 (100% 通过 ✅)
cargo test --lib particle::

# 动作系统 (90% 通过 ⚠️)
cargo test --lib action::

# 场景系统 (95% 通过 ⚠️)
cargo test --lib scene::

# UI 系统 (92% 通过 ⚠️)
cargo test --lib ui::
```

---

## 🔍 调试测试

### 显示详细输出
```bash
cargo test --lib -- --nocapture
```

### 单线程运行 (方便调试)
```bash
cargo test --lib -- --test-threads=1
```

### 运行单个测试
```bash
# 示例: 运行 Vec2 加法测试
cargo test --lib math::vec2::tests::test_add
```

### 查看失败的测试
```bash
cargo test --lib 2>&1 | grep "FAILED"
```

---

## ⚠️ 已知失败的测试

### Action 模块 (11个失败)
```bash
# 查看失败的测试
cargo test --lib action::action_interval::tests:: 2>&1 | grep "FAILED"

# 失败的测试:
# - test_delay_time
# - test_blink
# - test_fade_in / test_fade_out / test_fade_to
# - test_move_by / test_move_to
# - test_rotate_by / test_rotate_to
# - test_scale_by / test_scale_to
```

### Scene 模块 (2个失败)
```bash
cargo test --lib scene::node::tests::test_node_convert_space
cargo test --lib scene::node::tests::test_node_parent_child
```

### Base 模块 (2个失败)
```bash
cargo test --lib base::async_task::tests::test_task_group_progress
cargo test --lib base::notification_center::tests::test_observer_priority
```

### UI 模块 (10个失败)
```bash
# WebView (9个失败) - 模拟实现不完整
cargo test --lib ui::web_view::tests::

# VideoPlayer (1个失败)
cargo test --lib ui::video_player::tests::test_videoplayer_multiple_sources
```

---

## 📈 测试覆盖率

### 按模块统计

| 模块 | 通过率 | 评级 |
|------|--------|------|
| Math | 100% | ⭐⭐⭐⭐⭐ |
| Physics | 100% | ⭐⭐⭐⭐⭐ |
| Renderer | 100% | ⭐⭐⭐⭐⭐ |
| Animation | 100% | ⭐⭐⭐⭐⭐ |
| Particle | 100% | ⭐⭐⭐⭐⭐ |
| Camera | 100% | ⭐⭐⭐⭐⭐ |
| Effects | 100% | ⭐⭐⭐⭐⭐ |
| Label | 100% | ⭐⭐⭐⭐⭐ |
| Debug | 100% | ⭐⭐⭐⭐⭐ |
| Audio | 100% | ⭐⭐⭐⭐⭐ |
| **Action** | **90%** | **⭐⭐⭐** |
| **Scene** | **95%** | **⭐⭐⭐⭐** |
| **Base** | **98%** | **⭐⭐⭐⭐** |
| **UI** | **92%** | **⭐⭐⭐** |

---

## 🛠️ 常用测试命令

### 基础测试
```bash
# 运行所有测试
cargo test --lib

# 安静模式 (只显示失败)
cargo test --lib --quiet

# 运行特定测试文件
cargo test --lib --test integration_tests
```

### 性能测试
```bash
# 运行性能基准测试
cargo bench

# 只运行特定基准
cargo bench vec2_operations
```

### 代码覆盖率 (需要 tarpaulin)
```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 查看报告
open tarpaulin-report.html
```

---

## 📝 测试相关文件

```
cocos2d-rust/
├── UNIT_TEST_REPORT.md       # 详细测试报告
├── TEST_SUMMARY.txt           # 测试结果总结
├── QUICK_TEST_GUIDE.md        # 本文档
├── TESTING_GUIDE.md           # 测试指南
├── TESTS_README.md            # 测试说明
├── TEST_EXECUTION_REPORT.md   # 执行报告
├── tests/                     # 测试目录
│   ├── integration_tests.rs
│   ├── simple_tests.rs
│   ├── math_tests.rs
│   ├── scene_tests.rs
│   ├── renderer_tests.rs
│   ├── ui_tests.rs
│   ├── physics_tests.rs
│   ├── animation_tests.rs
│   └── audio_tests.rs
└── benches/                   # 性能测试
    └── performance_benchmarks.rs
```

---

## 🎓 测试最佳实践

### 1. 编写新测试
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // 准备
        let mut system = MySystem::new();
        
        // 执行
        system.do_something();
        
        // 验证
        assert_eq!(system.get_result(), expected_value);
    }
}
```

### 2. 使用测试辅助函数
```rust
// tests/test_helpers.rs 中定义的辅助函数
use crate::test_helpers::*;

#[test]
fn test_with_helper() {
    let scene = create_test_scene();
    assert!(scene.is_valid());
}
```

### 3. 忽略慢速测试
```rust
#[test]
#[ignore]
fn test_slow_operation() {
    // 长时间运行的测试
}

// 运行忽略的测试
// cargo test --lib -- --ignored
```

---

## 🔧 故障排除

### 测试失败时如何调试?

1. **查看详细输出**
   ```bash
   cargo test --lib failing_test_name -- --nocapture
   ```

2. **单线程运行**
   ```bash
   cargo test --lib -- --test-threads=1
   ```

3. **运行特定失败的测试**
   ```bash
   cargo test --lib action::action_interval::tests::test_delay_time
   ```

4. **添加调试输出**
   ```rust
   #[test]
   fn test_debug() {
       let value = compute();
       println!("Debug: value = {:?}", value);  // 会显示在 --nocapture 模式
       assert_eq!(value, expected);
   }
   ```

### 编译警告过多?

```bash
# 只显示错误,忽略警告
cargo test --lib 2>&1 | grep -v "warning:"

# 或者修复警告 (推荐)
cargo clippy --lib
```

---

## 📊 持续集成 (未来)

### GitHub Actions 示例配置

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --lib
      - run: cargo bench
```

---

## 💡 提示

### 快速检查测试状态
```bash
# 创建别名 (添加到 ~/.zshrc 或 ~/.bashrc)
alias ctest='cd cocos2d-rust && cargo test --lib'
alias ctest-fast='cd cocos2d-rust && cargo test --lib --quiet'
alias ctest-failed='cd cocos2d-rust && cargo test --lib 2>&1 | grep FAILED'
```

### 测试前清理
```bash
# 清理构建缓存
cargo clean

# 重新构建和测试
cargo test --lib
```

### 并行测试
```bash
# 使用多个线程 (默认)
cargo test --lib

# 指定线程数
cargo test --lib -- --test-threads=4
```

---

## 📚 更多资源

- **Rust 测试书**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Criterion 性能测试**: https://github.com/bheisler/criterion.rs
- **Tarpaulin 覆盖率**: https://github.com/xd009642/tarpaulin

---

**更新日期**: 2026年2月12日  
**版本**: v0.1.0-alpha
