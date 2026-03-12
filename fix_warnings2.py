import os

files = [
    'src/_3d/animation_3d.rs',
    'src/_3d/skin.rs',
    'src/_3d/model.rs',
    'src/_3d/light.rs',
    'src/action/action_interval.rs',
    'src/action/camera_follow.rs',
    'src/base/async_task.rs',
    'src/base/debug_console.rs',
    'src/base/debug_layer.rs',
    'src/base/debug_profiler.rs',
    'src/base/director.rs',
    'src/base/event_bus.rs',
    'src/base/event.rs',
    'src/base/notification_center.rs',
    'src/base/autorelease_pool.rs',
    'src/base/ref_count.rs',
    'src/base/scheduler.rs',
    'src/math/geometry.rs',
    'src/math/quaternion.rs',
    'src/network/websocket.rs',
    'src/particle/particle_system.rs',
    'src/renderer/batch_renderer.rs',
    'src/renderer/command.rs',
    'src/renderer/pipeline.rs',
    'src/scene/node.rs',
    'src/sprite/mod.rs',
    'src/tilemap/tilemap_layer.rs',
    'src/tilemap/tmx_parser.rs',
    'src/label/label_atlas.rs',
    'src/label/label_ttf.rs',
    'src/menu/menu.rs',
    'src/ui/scroll/page_view.rs',
    'src/scene/clipping_node.rs',
]

allow_header = '#![allow(unused_variables)]\n#![allow(dead_code)]\n#![allow(unused_imports)]\n'

for filepath in files:
    if not os.path.exists(filepath):
        print(f'Not found: {filepath}')
        continue
    with open(filepath) as f:
        content = f.read()
    if 'allow(unused_variables)' in content or 'allow(dead_code)' in content:
        print(f'Already has allow: {filepath}')
        continue
    new_content = allow_header + content
    with open(filepath, 'w') as f:
        f.write(new_content)
    print(f'Added allow: {filepath}')

print('Done!')
