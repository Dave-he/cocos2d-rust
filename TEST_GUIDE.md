# Label 模块测试用例文档

## 概述

本文档介绍了 `label` 模块的测试用例设计，展示了使用明确的 Rust 功能编写高质量测试的最佳实践。

## Rust 特性使用

### 1. **Builder 模式** (测试夹具)

使用 Builder 模式创建测试夹具，提高测试可读性和可维护性：

```rust
#[derive(Default)]
struct LabelTestBuilder {
    text: Option<String>,
    font_name: Option<String>,
    font_size: Option<f32>,
    // ...
}

impl LabelTestBuilder {
    fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    // ... 更多 builder 方法
}
```

**优势**：
- 使用 `Default` trait 提供零参数构造
- 使用 `impl Into<String>` 支持多种字符串类型
- 链式调用提高可读性

### 2. **类型安全的枚举测试**

充分利用 Rust 的枚举和模式匹配：

```rust
#[test]
fn test_all_alignment_combinations() {
    let h_alignments = [
        TextHAlignment::LEFT,
        TextHAlignment::CENTER,
        TextHAlignment::RIGHT,
    ];
    
    for &h_align in &h_alignments {
        for &v_align in &v_alignments {
            let mut label = Label::new();
            label.set_alignment(h_align, v_align);
            
            assert_eq!(label.get_horizontal_alignment(), h_align);
            assert_eq!(label.get_vertical_alignment(), v_align);
        }
    }
}
```

**优势**：
- 编译时保证所有枚举值都被测试
- 使用迭代器进行穷举测试

### 3. **Trait 约束验证**

测试派生的 trait 是否正常工作：

```rust
#[test]
fn test_text_halignment_traits() {
    let align1 = TextHAlignment::CENTER;
    let align2 = align1;  // Copy trait
    let align3 = align1.clone();  // Clone trait
    
    assert_eq!(align1, align2);  // PartialEq trait
    
    // Debug trait
    let debug_str = format!("{:?}", align1);
    assert!(debug_str.contains("CENTER"));
}
```

**优势**：
- 验证 `Copy`, `Clone`, `PartialEq`, `Debug` 等 trait 正常工作
- 确保类型行为符合预期

### 4. **所有权和借用测试**

测试数据所有权转移和借用：

```rust
#[test]
fn test_property_independence() {
    let mut label = Label::new();
    
    let original_font = label.get_font_name().to_string();  // 克隆数据
    let original_size = label.get_font_size();  // Copy 类型
    
    label.set_string("New Text");
    
    // 验证修改一个属性不影响其他属性
    assert_eq!(label.get_font_name(), original_font);
    assert_eq!(label.get_font_size(), original_size);
}
```

**优势**：
- 明确展示数据所有权
- 测试属性独立性

### 5. **边界值和特殊情况测试**

使用 Rust 的类型系统测试边界条件：

```rust
#[test]
fn test_font_size_boundary_values() {
    let mut label = Label::new();
    
    label.set_font_size(0.1);
    assert_eq!(label.get_font_size(), 0.1);
    
    label.set_font_size(1000.0);
    assert_eq!(label.get_font_size(), 1000.0);
}

#[test]
fn test_unicode_string() {
    let mut label = Label::new();
    let unicode_text = "Hello 世界 🌍";
    
    label.set_string(unicode_text);
    assert_eq!(label.get_string(), unicode_text);
}
```

**优势**：
- Rust 的 String 天然支持 UTF-8
- 浮点数边界值测试

### 6. **测试组织和模块化**

使用 `#[cfg(test)]` 条件编译：

```rust
// 在 mod.rs 中
#[cfg(test)]
mod tests;
```

**优势**：
- 测试代码只在测试时编译
- 减小最终二进制文件大小

### 7. **性能测试标记**

使用 `#[ignore]` 属性标记性能测试：

```rust
#[test]
#[ignore]
fn stress_test_rapid_updates() {
    // 压力测试代码
}
```

**优势**：
- 默认测试运行时跳过耗时测试
- 使用 `cargo test -- --ignored` 单独运行

## 测试覆盖范围

### 构造函数测试
- ✅ `new()` 默认值
- ✅ `Default` trait
- ✅ `create_with_ttf()`
- ✅ `create_with_system_font()`
- ✅ `create_with_bmfont()`
- ✅ `create_with_char_map()`

### 文本内容测试
- ✅ 设置和获取字符串
- ✅ 空字符串
- ✅ Unicode 字符串
- ✅ 多行字符串
- ✅ 字符串长度计算
- ✅ 行数统计

### 字体属性测试
- ✅ 字体名称
- ✅ 字体大小
- ✅ 边界值测试

### 对齐测试
- ✅ 水平对齐
- ✅ 垂直对齐
- ✅ 所有对齐组合

### 颜色测试
- ✅ 预定义颜色
- ✅ 自定义颜色

### 布局测试
- ✅ 尺寸设置
- ✅ 行高
- ✅ 行间距
- ✅ 最大行宽

### 换行测试
- ✅ 启用/禁用
- ✅ 切换测试

### 溢出测试
- ✅ 所有溢出类型

### 特效测试
- ✅ 阴影效果
- ✅ 描边效果

### Builder 模式测试
- ✅ 基础用法
- ✅ 完整配置
- ✅ 最小配置

### 集成测试
- ✅ 复杂配置组合

### Trait 测试
- ✅ Copy/Clone
- ✅ PartialEq/Eq
- ✅ Debug

### 边界测试
- ✅ 超长字符串
- ✅ 特殊字符
- ✅ 空白字符
- ✅ 换行符

### 压力测试
- ✅ 快速更新测试

## Rust 最佳实践

### 1. 使用 `assert_eq!` 而非 `assert!`
```rust
// ✅ 好
assert_eq!(label.get_string(), "Expected");

// ❌ 差
assert!(label.get_string() == "Expected");
```

### 2. 使用常量和数组进行穷举测试
```rust
const COLORS: [Color3B; 9] = [
    Color3B::WHITE,
    Color3B::BLACK,
    // ...
];

for &color in &COLORS {
    // 测试代码
}
```

### 3. 利用类型系统
```rust
// 使用 impl Into<T> 接受多种类型
fn with_text(mut self, text: impl Into<String>) -> Self {
    self.text = Some(text.into());
    self
}
```

### 4. 明确的生命周期
```rust
// 如果需要，明确指定生命周期
pub fn get_string(&self) -> &str {
    &self.text
}
```

### 5. 使用 `#[test]` 属性而非命名约定
```rust
// ✅ 使用属性
#[test]
fn test_functionality() { }

// ❌ 仅依赖命名
fn test_functionality() { }
```

## 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test label::tests

# 运行特定测试
cargo test test_label_new_has_default_values

# 显示测试输出
cargo test -- --nocapture

# 运行被忽略的测试
cargo test -- --ignored

# 运行测试并显示详细信息
cargo test -- --show-output
```

## 测试组织结构

```
cocos2d-rust/
├── src/
│   ├── label/
│   │   ├── mod.rs           # 声明 tests 模块
│   │   ├── label.rs         # 主要实现
│   │   └── tests.rs         # 测试代码
```

## 后续改进

1. **属性测试 (Property-based Testing)**
   - 考虑使用 `proptest` 或 `quickcheck` 库
   - 自动生成测试用例

2. **基准测试 (Benchmarking)**
   - 使用 `criterion` 库进行性能测试
   - 监控性能回归

3. **覆盖率报告**
   - 使用 `tarpaulin` 生成覆盖率报告
   - 目标：>80% 覆盖率

4. **集成测试**
   - 在 `tests/` 目录添加集成测试
   - 测试跨模块交互

5. **文档测试**
   - 在文档注释中添加示例代码
   - 使用 `cargo test --doc`

## 示例：添加新测试

```rust
#[test]
fn test_new_feature() {
    // 1. 准备 (Arrange)
    let mut label = LabelTestBuilder::new()
        .with_text("Test")
        .build();
    
    // 2. 执行 (Act)
    label.set_some_property(value);
    
    // 3. 断言 (Assert)
    assert_eq!(label.get_some_property(), expected);
}
```

## 参考资源

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
