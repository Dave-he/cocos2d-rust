// integration_tests.rs
// 
// 集成测试入口文件
// 注意：cocos2d-rust 的各模块测试在各自的 tests/*.rs 文件中
// 本文件提供统一的辅助工具
//
// 运行方式：
//   cargo test --test integration_tests
//   cargo test (运行所有测试)

/// 确保库能够正确编译和链接
#[test]
fn test_library_compiles() {
    // 这个测试仅验证库可以被正常使用
    let _ = cocos2d_rust::scene::Node::new();
    let _ = cocos2d_rust::scene::Scene::new();
}
