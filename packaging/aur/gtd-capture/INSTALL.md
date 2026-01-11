# AUR Package Installation Guide

## For Package Maintainers

### Publishing to AUR

1. **Create AUR account** at https://aur.archlinux.org/register

2. **Set up SSH key** for AUR:
   ```bash
   ssh-keygen -t ed25519 -C "aur@archlinux.org"
   # Add public key to your AUR account settings
   ```

3. **Clone AUR repository**:
   ```bash
   git clone ssh://aur@aur.archlinux.org/gtd-capture.git aur-gtd-capture
   cd aur-gtd-capture
   ```

4. **Copy package files**:
   ```bash
   cp /path/to/aur-packages/gtd-capture/* .
   ```

5. **Generate checksums**:
   ```bash
   updpkgsums
   ```

6. **Test build locally**:
   ```bash
   makepkg -si
   ```

7. **Generate .SRCINFO**:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

8. **Commit and push to AUR**:
   ```bash
   git add PKGBUILD .SRCINFO gtd-capture.sh gtd-capture.install sxhkdrc.example bspwmrc.example README.md
   git commit -m "Initial commit: gtd-capture v1.0.0"
   git push
   ```

### Updating the Package

1. **Update version in PKGBUILD**:
   - Increment `pkgver` for new upstream releases
   - Increment `pkgrel` for package-only changes

2. **Regenerate checksums and .SRCINFO**:
   ```bash
   updpkgsums
   makepkg --printsrcinfo > .SRCINFO
   ```

3. **Test build**:
   ```bash
   makepkg -si
   ```

4. **Commit and push**:
   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "Update to v1.0.1"
   git push
   ```

## For Users

### Installing from AUR

```bash
# Using yay
yay -S gtd-capture

# Using paru
paru -S gtd-capture

# Manual installation
git clone https://aur.archlinux.org/gtd-capture.git
cd gtd-capture
makepkg -si
```

### Post-Installation Configuration

The package installs the binary but requires manual dotfile configuration:

1. **Add hotkey to sxhkd** (`~/.config/sxhkd/sxhkdrc`):
   ```bash
   super + i
       gtd-capture
   ```

2. **Add window rule to bspwm** (`~/.config/bspwm/bspwmrc`):
   ```bash
   bspc rule -a wofi state=floating center=true
   ```

3. **Reload configs**:
   ```bash
   killall -USR1 sxhkd
   bspc wm -r
   ```

See `/usr/share/doc/gtd-capture/README.md` for full documentation.

## Testing Before Publishing

### Local Build Test

```bash
cd aur-packages/gtd-capture
makepkg -si
```

This will:
- Build the package
- Install dependencies
- Install the package locally
- Show post-install messages

### Verify Installation

```bash
# Check binary is in PATH
which gtd-capture

# Check docs are installed
ls /usr/share/doc/gtd-capture/

# Test the command (if wofi is installed)
gtd-capture
```

### Clean Up Test Installation

```bash
sudo pacman -R gtd-capture
```

## Package Structure

```
aur-packages/gtd-capture/
├── PKGBUILD                 # Build script (AUR entry point)
├── gtd-capture.install      # Post-install hooks (user instructions)
├── gtd-capture.sh           # Main capture script
├── sxhkdrc.example          # Example sxhkd config snippet
├── bspwmrc.example          # Example bspwm config snippet
├── README.md                # User documentation
└── INSTALL.md               # This file (maintainer guide)
```

## AUR Best Practices

1. **Keep PKGBUILD simple** - Don't over-engineer
2. **Use `install` parameter** - Show post-install instructions
3. **Provide examples** - Help users configure their dotfiles
4. **Test on clean system** - Use a VM or container
5. **Update .SRCINFO** - Always regenerate after PKGBUILD changes
6. **Respond to comments** - Monitor AUR page for user issues
7. **Semantic versioning** - Use proper version numbering

## Troubleshooting

### PKGBUILD syntax errors
```bash
namcap PKGBUILD
```

### Missing dependencies
```bash
makepkg -s  # Auto-install makedepends
```

### Permission errors on AUR push
```bash
# Verify SSH key is added to AUR account
ssh -T aur@aur.archlinux.org
```

---

**Ready to publish to AUR when needed.**
