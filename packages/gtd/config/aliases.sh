#!/usr/bin/env bash
# ============================================
# spanda-gtd Shell Aliases
# Source this file from ~/.bashrc or ~/.zshrc:
#   source ~/src/spanda-gtd/config/aliases.sh
# ============================================

# === Capture ===

# Quick inbox capture
alias in='task add +in'

# Tickler file (future inbox)
tickle() {
    deadline=$1
    shift
    in +tickle wait:$deadline "$@"
}
alias tick=tickle

# Think it over (tomorrow's inbox)
alias think='tickle +1d'

# Research task
alias rnd='task add +rnd +next +@computer +@online'

# === Quick Views ===

# Inbox
alias ti='task in'

# Next actions
alias tn='task next'

# Waiting for
alias tw='task waiting'

# Someday/maybe
alias ts='task sdm'

# Projects without next action
alias tnna='task nna'

# === Context Switching ===

alias tcw='task context work'
alias tch='task context home'
alias tcf='task context focus'
alias tca='task context anywhere'
alias tcn='task context none'

# === Review Helpers ===

# Count inbox items (for prompt)
alias tic='task +in +PENDING count'

# Start tasksh review
alias review='tasksh -c review'

# === Processing ===

# Process inbox item (removes +in, adds +next)
process() {
    task "$1" modify -in +next "${@:2}"
}

# Defer to someday/maybe
defer() {
    task "$1" modify -in +sdm
}

# Delegate (waiting for)
delegate() {
    task "$1" modify -in +waiting due:+1w wait:+3d
}
