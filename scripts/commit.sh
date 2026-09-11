#!/usr/bin/env bash
# Commit only the Hyprland Inspector project files.
# Usage:
#   ./scripts/commit.sh --init "Initial Hyprland Inspector release ladder"
#   ./scripts/commit.sh "Add live rule matching"
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_dir"

initialize=false
if [[ "${1:-}" == "--init" ]]; then
  initialize=true
  shift
fi

message="${1:-}"
if [[ -z "$message" ]]; then
  echo "Usage: $0 [--init] \"commit message\"" >&2
  exit 2
fi

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  if [[ "$initialize" != true ]]; then
    echo "This project is not a Git repository yet." >&2
    echo "Run: $0 --init \"Initial Hyprland Inspector\"" >&2
    exit 1
  fi
  git init
  git branch -M main
elif [[ "$initialize" == true ]]; then
  echo "Git repository already exists; --init is not needed." >&2
  exit 1
fi

# Keep the commit deliberately scoped: never add dependencies or build output.
git add \
  .gitignore \
  README.md \
  RELEASES.md \
  package.json \
  tsconfig.json \
  vite.config.ts \
  index.html \
  src \
  src-tauri \
  assets \
  desktop \
  bin \
  scripts

if git diff --cached --quiet; then
  echo "Nothing new to commit for Hyprland Inspector."
  exit 0
fi

echo "Files staged for this commit:"
git diff --cached --stat
echo
git commit -m "$message"
