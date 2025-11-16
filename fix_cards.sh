#!/bin/bash
# Add narrative_fragments: None to all Card structs in data files

for file in src/data/player_deck.rs src/data/buyer_personas.rs src/data/presets.rs; do
    echo "Processing $file..."

    # Use awk to add narrative_fragments before the closing brace of Card structs
    awk '
    /card_type: CardType::/ {
        card_type_line = $0
        print $0
        next
    }
    /^[[:space:]]*},?[[:space:]]*$/ && card_type_line != "" {
        # Insert narrative_fragments before the closing brace
        gsub(/},/, ", narrative_fragments: None // SOW-012 Phase 1\n        },")
        card_type_line = ""
    }
    { print }
    ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
done

echo "Done!"
