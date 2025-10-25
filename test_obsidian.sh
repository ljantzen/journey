#!/bin/bash

# Test script for Obsidian functionality
echo "Testing Obsidian functionality..."

# Create a temporary Obsidian vault structure
TEMP_DIR=$(mktemp -d)
echo "Created temp directory: $TEMP_DIR"

# Create .obsidian directory
mkdir -p "$TEMP_DIR/.obsidian"

# Create a basic app.json with Daily Notes enabled
cat > "$TEMP_DIR/.obsidian/app.json" << 'EOF'
{
  "dailyNotes": {
    "enabled": true,
    "format": "YYYY-MM-DD",
    "folder": "/Daily Notes",
    "template": "Templates/Daily Note Template"
  }
}
EOF

# Create plugins directory and Periodic Notes plugin
mkdir -p "$TEMP_DIR/.obsidian/plugins/periodic-notes"
cat > "$TEMP_DIR/.obsidian/plugins/periodic-notes/data.json" << 'EOF'
{
  "enabled": true,
  "weeklyFormat": "YYYY-[W]ww",
  "monthlyFormat": "YYYY-MM",
  "quarterlyFormat": "YYYY-[Q]Q",
  "yearlyFormat": "YYYY"
}
EOF

# Create Journals plugin
mkdir -p "$TEMP_DIR/.obsidian/plugins/journals"
cat > "$TEMP_DIR/.obsidian/plugins/journals/data.json" << 'EOF'
{
  "enabled": true,
  "journalFolder": "/Journals",
  "dateFormat": "YYYY-MM-DD"
}
EOF

echo "Created Obsidian vault structure at: $TEMP_DIR"
echo "Testing journeyctl init with --obsidian flag..."

# Test the obsidian functionality
cd /home/leif/dev/p/privat/rust/journey
cargo run --bin journeyctl -- init --path "$TEMP_DIR" --obsidian

echo "Test completed. Temp directory: $TEMP_DIR"
echo "You can inspect the generated journey.yaml file to see the configuration."
