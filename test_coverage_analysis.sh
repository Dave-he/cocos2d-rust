#!/bin/bash
# 测试覆盖率分析脚本

echo "=== Cocos2d-Rust 测试覆盖率分析 ==="
echo ""

total_files=0
tested_files=0

for module in _3d action animation audio backend base input label math menu network particle physics platform renderer scene shader sprite tilemap transition ui; do
    if [ -d "src/$module" ]; then
        module_total=$(find "src/$module" -name "*.rs" -type f 2>/dev/null | wc -l | tr -d ' ')
        module_tested=$(find "src/$module" -name "*.rs" -type f -exec grep -l "#\[cfg(test)\]" {} \; 2>/dev/null | wc -l | tr -d ' ')
        
        total_files=$((total_files + module_total))
        tested_files=$((tested_files + module_tested))
        
        if [ "$module_total" -gt 0 ]; then
            percentage=$((module_tested * 100 / module_total))
            printf "%-12s: %2d/%2d 文件有测试 (%3d%%)\n" "$module" "$module_tested" "$module_total" "$percentage"
        fi
    fi
done

echo ""
echo "----------------------------------------"
overall_percentage=$((tested_files * 100 / total_files))
printf "总计: %d/%d 文件有测试 (%d%%)\n" "$tested_files" "$total_files" "$overall_percentage"
echo ""
echo "当前测试数量: 206 个"
echo "目标: 100% 代码覆盖率"
