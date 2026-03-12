import os
import re

def to_pascal_case(name):
    """Convert UPPER_CASE or UPPER_CASE to PascalCase"""
    return ''.join(word.capitalize() for word in name.split('_'))

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    changes = {}

    # Find enum blocks and collect SCREAMING_SNAKE_CASE variants
    enum_pattern = re.compile(r'pub\s+enum\s+\w+\s*\{([^}]+)\}', re.DOTALL)
    for enum_match in enum_pattern.finditer(content):
        enum_body = enum_match.group(1)
        # Match variants: lines starting with whitespace then UPPER_CASE identifier
        variant_pattern = re.compile(r'^\s+([A-Z][A-Z0-9]*(?:_[A-Z0-9]+)+)\s*[,\{(]', re.MULTILINE)
        for var_match in variant_pattern.finditer(enum_body):
            variant = var_match.group(1)
            pascal = to_pascal_case(variant)
            if pascal != variant:
                changes[variant] = pascal

    if changes:
        print(f"\n{filepath}:")
        for old, new in sorted(changes.items()):
            print(f"  {old} -> {new}")
        # Apply changes: replace all occurrences of each variant (as whole word)
        for old, new in changes.items():
            # Replace as whole word to avoid partial matches
            content = re.sub(r'\b' + re.escape(old) + r'\b', new, content)
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"  [SAVED]")
    
    return changes


files = [
    'src/effects/render_texture.rs',
    'src/physics/physics_3d.rs',
    'src/platform/image.rs',
    'src/renderer/pipeline.rs',
    'src/renderer/texture.rs',
    'src/ui/layouts.rs',
    'src/ui/rich_text.rs',
    'src/ui/scroll/list_view.rs',
    'src/ui/scroll/scroll_view.rs',
    'src/audio/audio_player.rs',
    'src/backend/pipeline_state.rs',
    'src/renderer/texture.rs',
]

all_changes = {}
for f in files:
    if os.path.exists(f):
        changes = process_file(f)
        for old, new in changes.items():
            all_changes[old] = new

print(f"\n\nTotal unique replacements: {len(all_changes)}")
