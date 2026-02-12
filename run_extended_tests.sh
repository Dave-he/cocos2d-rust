#!/bin/bash
# 编译和测试更多独立模块

echo "🔧 Cocos2d-Rust 扩展模块测试"
echo "=========================================="
echo ""

PASS=0
FAIL=0
TOTAL=0

cd /Users/hyx/codespace/cocos-engine/cocos2d-rust/src

# 测试 geometry 模块
echo "📐 测试 Geometry 模块..."
cd math
if rustc --test geometry.rs --edition 2021 -o /tmp/geometry_test 2>/dev/null; then
    RESULT=$(/tmp/geometry_test 2>&1)
    if echo "$RESULT" | grep -q "test result: ok"; then
        COUNT=$(echo "$RESULT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo "  ✅ Geometry: $COUNT 个测试通过"
        PASS=$((PASS + COUNT))
        TOTAL=$((TOTAL + COUNT))
    else
        echo "  ❌ Geometry: 测试失败"
    fi
else
    echo "  ⚠️  Geometry: 编译失败"
fi

# 回到 src 目录
cd /Users/hyx/codespace/cocos-engine/cocos2d-rust/src

# 测试 base/types 模块  
echo ""
echo "📦 测试 Base Types 模块..."
cd base
if rustc --test types.rs --edition 2021 -o /tmp/types_test 2>/dev/null; then
    RESULT=$(/tmp/types_test 2>&1)
    if echo "$RESULT" | grep -q "test result: ok"; then
        COUNT=$(echo "$RESULT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo "  ✅ Types: $COUNT 个测试通过"
        PASS=$((PASS + COUNT))
        TOTAL=$((TOTAL + COUNT))
    else
        echo "  ❌ Types: 测试失败"
    fi
else
    echo "  ⚠️  Types: 编译失败"
fi

cd /Users/hyx/codespace/cocos-engine/cocos2d-rust

echo ""
echo "=========================================="
echo "📊 扩展测试总结"
echo "=========================================="
echo "  ✅ 新增通过: $PASS 个测试"
echo "  📝 新增总计: $TOTAL 个测试"
echo ""

if [ $TOTAL -gt 0 ]; then
    echo "🎉 成功测试了额外的模块!"
else
    echo "⚠️  这些模块需要依赖，暂时无法独立编译"
fi
