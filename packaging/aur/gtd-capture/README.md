# gtd-capture

Frictionless GTD inbox capture via keyboard hotkey for bspwm + wofi + TaskWarrior.

## What This Does

Press `Super+i` from anywhere → floating input popup appears → type capture text → press Enter → item added to TaskWarrior inbox (`+inbox` tag) → popup disappears.

**Total capture time: < 3 seconds**

## Installation

### Via AUR (CachyOS/Arch)

```bash
yay -S gtd-capture
```

or

```bash
paru -S gtd-capture
```

### Manual Configuration (Required)

After installing the package, you must add configuration snippets to your dotfiles:

#### 1. Add hotkey binding to sxhkd

Edit `~/.config/sxhkd/sxhkdrc` and add:

```bash
# GTD Inbox Capture
# Super + i = instant capture popup
super + i
    gtd-capture
```

Then reload sxhkd:
```bash
killall -USR1 sxhkd
```

#### 2. Add bspwm window rule

Edit `~/.config/bspwm/bspwmrc` and add:

```bash
# Make wofi windows float and center
bspc rule -a wofi state=floating center=true
```

Then reload bspwm:
```bash
bspc wm -r
```

**Tip:** Example config snippets are installed at:
- `/usr/share/doc/gtd-capture/sxhkdrc.example`
- `/usr/share/doc/gtd-capture/bspwmrc.example`

## Usage

1. Press `Super+i` from any context (or your configured hotkey)
2. Floating input popup appears
3. Type capture text (single line)
4. Press `Enter` to capture
5. Press `Escape` to cancel

Captured items appear in TaskWarrior inbox:
```bash
task in
```

## Dependencies

- **wofi** - Wayland-oriented launcher (popup interface)
- **task** - TaskWarrior 3.x (GTD task management)
- **libnotify** - Desktop notifications (capture confirmation)
- **bspwm** - Tiling window manager
- **sxhkd** - Simple X hotkey daemon

All dependencies are automatically installed by the package manager.

## Customization

### Remove notification

Edit `/usr/bin/gtd-capture` (or copy to `~/.local/bin/gtd-capture` and modify) and remove:
```bash
notify-send -t 1000 "GTD" "Captured: $CAPTURE"
```

### Change hotkey

Edit your hotkey in `~/.config/sxhkd/sxhkdrc` (e.g., change `super + i` to `super + shift + c`).

### Adjust popup size

Modify wofi parameters in `/usr/bin/gtd-capture`:
- `--width 600`: popup width in pixels
- `--height 100`: popup height in pixels

### wofi styling

Create `~/.config/wofi/style.css` for custom styling. See [wofi documentation](https://hg.sr.ht/~scoopta/wofi).

## Troubleshooting

### Hotkey doesn't work
- Verify sxhkd is running: `pgrep sxhkd`
- Reload sxhkd: `killall -USR1 sxhkd`
- Check sxhkd config syntax: `sxhkd -p`

### Popup doesn't appear
- Test command manually: `gtd-capture`
- Verify wofi works: `echo "test" | wofi --dmenu`
- Check bspwm rules: `bspc rule -l`

### Items not added to TaskWarrior
- Verify TaskWarrior works: `task add +inbox "test"`
- Check inbox: `task in`
- Ensure TaskWarrior is configured (run `task` first time to create config)

## Design Notes

**Why wofi?** Wayland-native launcher designed for modern compositors. Faster and lighter than rofi for Wayland-based workflows.

**Why single-line?** Multi-line capture is rare in GTD practice. For longer items, capture trigger phrase and elaborate during inbox processing.

**Why notify-send?** Brief visual confirmation provides closure. Optional—can be removed for purist speed.

## License

MIT

## References

- [GTD Methodology](https://gettingthingsdone.com/) - David Allen's "Getting Things Done"
- [TaskWarrior](https://taskwarrior.org/docs/) - Command-line task management
- [wofi](https://hg.sr.ht/~scoopta/wofi) - Wayland launcher
- [sxhkd](https://github.com/baskerville/sxhkd) - Simple X Hotkey Daemon
- [bspwm](https://github.com/baskerville/bspwm) - Binary Space Partitioning Window Manager

---

**Quick win that solves daily friction in GTD capture.**
