import os
import re

# 需要修复的文件和对应的函数模式
files_patterns = {
    'src/renderer/texture_cache.rs': 'get_instance',
    'src/platform/file_utils.rs': 'get_instance',
    'src/network/network.rs': 'get_instance',
    'src/animation/animation_cache.rs': 'get_instance',
    'src/animation/sprite_frame_cache.rs': 'get_instance',
    'src/shader/shader_cache.rs': 'get_instance',
    'src/base/director.rs': 'get_instance',
    'src/base/autorelease_pool.rs': 'get_instance',
}

# 添加 allow 属性到包含 static mut 的 unsafe 块
allow_attr = '#[allow(static_mut_refs)]'

for filepath, func_name in files_patterns.items():
    if not os.path.exists(filepath):
        print(f'Not found: {filepath}')
        continue
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # 找到 "pub fn get_instance" 前面添加 allow 属性
    # 先检查是否已经有了
    if allow_attr in content:
        print(f'Already fixed: {filepath}')
        continue
    
    # 在 "pub fn get_instance" 前添加 allow 属性
    # 需要保持相同的缩进
    pattern = re.compile(r'^(\s*)(pub fn ' + func_name + r'\b)', re.MULTILINE)
    
    def add_allow(m):
        indent = m.group(1)
        func = m.group(2)
        return f'{indent}{allow_attr}\n{indent}{func}'
    
    new_content = pattern.sub(add_allow, content)
    
    if new_content != content:
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f'Fixed: {filepath}')
    else:
        print(f'No match found: {filepath}')

print('Done!')
