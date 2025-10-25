#!/bin/bash

# Test script for real Journals plugin data structure
echo "Testing real Journals plugin data structure..."

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

# Create Journals plugin with REAL data structure
mkdir -p "$TEMP_DIR/.obsidian/plugins/journals"
cat > "$TEMP_DIR/.obsidian/plugins/journals/data.json" << 'EOF'
{
  "version": 3,
  "ui": {
    "calendarShelf": null
  },
  "pendingMigrations": [],
  "dismissedNotifications": [
    "v2-commands-change"
  ],
  "useShelves": false,
  "showReloadHint": false,
  "openOnStartup": "",
  "journals": {
    "Dagbok": {
      "name": "Dagbok",
      "shelves": [],
      "write": {
        "type": "day"
      },
      "confirmCreation": false,
      "nameTemplate": "{{date}}",
      "dateFormat": "YYYY-MM-DD",
      "folder": "10-Journal/{{date:y}}/{{date:MM}}",
      "templates": [
        "templates/md/0-Dagbok.md"
      ],
      "start": "2000-01-01",
      "end": {
        "type": "never"
      },
      "index": {
        "enabled": false,
        "anchorDate": "2000-01-01",
        "anchorIndex": 1,
        "allowBefore": false,
        "type": "increment",
        "resetAfter": 2
      },
      "autoCreate": false,
      "commands": [
        {
          "icon": "calendar-day",
          "name": "Today",
          "type": "same",
          "context": "today",
          "showInRibbon": true,
          "openMode": "active"
        }
      ],
      "decorations": [
        {
          "mode": "and",
          "conditions": [
            {
              "type": "has-note"
            }
          ],
          "styles": [
            {
              "type": "shape",
              "size": 0.4,
              "shape": "circle",
              "color": {
                "type": "theme",
                "name": "interactive-accent"
              },
              "placement_x": "center",
              "placement_y": "bottom"
            }
          ]
        }
      ],
      "navBlock": {
        "type": "create",
        "decorateWholeBlock": false,
        "rows": []
      },
      "calendarViewBlock": {
        "rows": [],
        "decorateWholeBlock": false
      },
      "frontmatter": {
        "dateField": "",
        "addStartDate": false,
        "startDateField": "",
        "addEndDate": false,
        "endDateField": "",
        "indexField": ""
      }
    }
  },
  "shelves": {},
  "commands": [
    {
      "name": "Open today's note",
      "writeType": "day",
      "type": "same",
      "openMode": "tab",
      "showInRibbon": false,
      "icon": ""
    }
  ],
  "calendar": {
    "dow": 1,
    "doy": 4,
    "global": false
  },
  "calendarView": {
    "display": "month",
    "leaf": "right",
    "weeks": "left",
    "todayMode": "navigate",
    "pickMode": "create",
    "todayStyle": {
      "color": {
        "type": "theme",
        "name": "text-accent"
      },
      "background": {
        "type": "transparent"
      }
    },
    "activeStyle": {
      "color": {
        "type": "theme",
        "name": "text-on-accent"
      },
      "background": {
        "type": "theme",
        "name": "interactive-accent"
      }
    }
  }
}
EOF

echo "Created Obsidian vault structure with real Journals plugin data at: $TEMP_DIR"
echo "Testing journeyctl init with --obsidian flag..."

# Test the obsidian functionality
cd /home/leif/dev/p/privat/rust/journey
cargo run --bin journeyctl -- init --path "$TEMP_DIR" --obsidian

echo "Test completed. Temp directory: $TEMP_DIR"
echo "You can inspect the generated journey.yaml file to see the configuration."
