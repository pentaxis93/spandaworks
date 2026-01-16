#!/bin/bash
# GTD Inbox Capture - Frictionless thought capture via wofi
# Usage: Bound to Super+i, appears as floating popup

# Get input from wofi in dmenu mode
CAPTURE=$(wofi --dmenu --prompt "Capture" --width 600 --height 100 --hide-scroll)

# If user provided input (didn't cancel with Escape)
if [ -n "$CAPTURE" ]; then
    # Add to TaskWarrior inbox
    task add +inbox "$CAPTURE" 2>&1 >/dev/null
    
    # Brief confirmation (optional - can remove for even faster workflow)
    notify-send -t 1000 "GTD" "Captured: $CAPTURE"
fi
