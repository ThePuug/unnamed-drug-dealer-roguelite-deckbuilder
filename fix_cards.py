#!/usr/bin/env python3
import re
import sys

def fix_card_structs(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    # Pattern: Find "card_type: CardType::..., }" and replace with "card_type: CardType::..., narrative_fragments: None },"
    pattern = r'(card_type: CardType::[^}]+}),\n(\s+)}'
    replacement = r'\1, narrative_fragments: None\n\2}'

    content = re.sub(pattern, replacement, content)

    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

    print(f"Fixed {filepath}")

if __name__ == "__main__":
    files = [
        'src/data/player_deck.rs',
        'src/data/narc_deck.rs',
        'src/data/buyer_personas.rs',
        'src/data/presets.rs',
    ]

    for filepath in files:
        fix_card_structs(filepath)
