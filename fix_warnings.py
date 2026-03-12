import os
import re

# 需要添加 allow 属性的文件列表
files = [
    'src/physics/physics_2d.rs',
    'src/physics/physics_3d.rs',
    'src/ui/widget.rs',
    'src/renderer/renderer.rs',
    'src/renderer/shader.rs',
    'src/backend/device.rs',
    'src/_3d/mesh.rs',
    'src/tilemap/tilemap_info.rs',
    'src/renderer/texture.rs',
    'src/tilemap/tmx_parser.rs',
    'src/menu/menu_item.rs',
    'src/label/label.rs',
    'src/animation/spine.rs',
    'src/ui/scroll/scroll_view.rs',
    'src/ui/rich_text.rs',
    'src/renderer/post_process.rs',
    'src/ui/layouts.rs',
    'src/audio/audio_engine.rs',
    'src/animation/dragonbones.rs',
    'src/tilemap/tiled.rs',
]

allow_header = '''#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
'''

for filepath in files:
    if not os.path.exists(filepath):
        print(f'Not found: {filepath}')
        continue
    
    with open(filepath) as f:
        content = f.read()
    
    # 检查是否已经有了 allow 属性
    if 'allow(unused_variables)' in content or 'allow(dead_code)' in content:
        print(f'Already has allow: {filepath}')
        continue
    
    # 在文件开头添加 allow 属性
    new_content = allow_header + content
    with open(filepath, 'w') as f:
        f.write(new_content)
    print(f'Added allow: {filepath}')

print('Done!')
