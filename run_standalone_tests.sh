#!/bin/bash
# Cocos2d-Rust 独立模块测试运行器

echo "🧪 Cocos2d-Rust 独立模块测试"
echo "=========================================="
echo ""

PASS=0
FAIL=0
TOTAL=0

# 测试 Vec2 模块
echo "📐 测试 Vec2 模块..."
cd /Users/hyx/codespace/cocos-engine/cocos2d-rust/src/math
if rustc --test vec2.rs --edition 2021 -o /tmp/vec2_test 2>/dev/null; then
    RESULT=$(/tmp/vec2_test 2>&1)
    if echo "$RESULT" | grep -q "test result: ok"; then
        COUNT=$(echo "$RESULT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo "  ✅ Vec2: $COUNT 个测试通过"
        PASS=$((PASS + COUNT))
        TOTAL=$((TOTAL + COUNT))
    else
        echo "  ❌ Vec2: 测试失败"
        FAIL=$((FAIL + 1))
    fi
else
    echo "  ⚠️  Vec2: 编译失败"
fi

# 测试 Vec3 模块
echo ""
echo "📐 测试 Vec3 模块..."
if rustc --test vec3.rs --edition 2021 -o /tmp/vec3_test 2>/dev/null; then
    RESULT=$(/tmp/vec3_test 2>&1)
    if echo "$RESULT" | grep -q "test result: ok"; then
        COUNT=$(echo "$RESULT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo "  ✅ Vec3: $COUNT 个测试通过"
        PASS=$((PASS + COUNT))
        TOTAL=$((TOTAL + COUNT))
    else
        echo "  ❌ Vec3: 测试失败"
        FAIL=$((FAIL + 1))
    fi
else
    echo "  ⚠️  Vec3: 编译失败"
fi

# 测试 Vec4 模块
echo ""
echo "📐 测试 Vec4 模块..."
if rustc --test vec4.rs --edition 2021 -o /tmp/vec4_test 2>/dev/null; then
    RESULT=$(/tmp/vec4_test 2>&1)
    if echo "$RESULT" | grep -q "test result: ok"; then
        COUNT=$(echo "$RESULT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo "  ✅ Vec4: $COUNT 个测试通过"
        PASS=$((PASS + COUNT))
        TOTAL=$((TOTAL + COUNT))
    else
        echo "  ❌ Vec4: 测试失败"
        FAIL=$((FAIL + 1))
    fi
else
    echo "  ⚠️  Vec4: 编译失败"
fi

# 测试 Quaternion 模块
echo ""
echo "📐 测试 Quaternion 模块..."
if rustc --test quaternion.rs --edition 2021 -o /tmp/quat_test 2>/dev/null; then
    RESULT=$(/tmp/quat_test 2>&1)
    if echo "$RESULT" | grep -q "test result: ok"; then
        COUNT=$(echo "$RESULT" | grep "test result:" | grep -oE "[0-9]+ passed" | grep -oE "[0-9]+")
        echo "  ✅ Quaternion: $COUNT 个测试通过"
        PASS=$((PASS + COUNT))
        TOTAL=$((TOTAL + COUNT))
    else
        echo "  ❌ Quaternion: 测试失败"
        FAIL=$((FAIL + 1))
    fi
else
    echo "  ⚠️  Quaternion: 编译失败"
fi

echo ""
echo "=========================================="
echo "📊 测试总结"
echo "=========================================="
echo "  ✅ 通过: $PASS 个测试"
echo "  ❌ 失败: $FAIL 个测试"
echo "  📝 总计: $TOTAL 个测试"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "🎉 所有测试通过!"
    exit 0
else
    echo "⚠️  部分测试失败"
    exit 1
fi
