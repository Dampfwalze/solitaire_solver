#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT_DIR" ]]; then
  echo "Error: run this script inside a git repository."
  exit 1
fi

SOURCE_DIR="$ROOT_DIR/solutions"
if [[ ! -d "$SOURCE_DIR" ]]; then
  echo "Error: solutions directory not found at $SOURCE_DIR"
  exit 1
fi

origin_url="$(git -C "$ROOT_DIR" remote get-url origin 2>/dev/null || true)"
if [[ -z "$origin_url" ]]; then
  echo "Error: could not determine origin remote URL."
  exit 1
fi

derive_wiki_url() {
  local remote_url="$1"

  if [[ "$remote_url" =~ ^git@([^:]+):(.+)$ ]]; then
    local host="${BASH_REMATCH[1]}"
    local path="${BASH_REMATCH[2]}"
    path="${path%.git}"
    echo "git@${host}:${path}.wiki.git"
    return 0
  fi

  if [[ "$remote_url" =~ ^ssh://git@([^/]+)/(.+)$ ]]; then
    local host="${BASH_REMATCH[1]}"
    local path="${BASH_REMATCH[2]}"
    path="${path%.git}"
    echo "ssh://git@${host}/${path}.wiki.git"
    return 0
  fi

  if [[ "$remote_url" =~ ^https?://([^/]+)/(.+)$ ]]; then
    local proto host path
    proto="${remote_url%%://*}"
    host="${BASH_REMATCH[1]}"
    path="${BASH_REMATCH[2]}"
    path="${path%.git}"
    echo "${proto}://${host}/${path}.wiki.git"
    return 0
  fi

  return 1
}

wiki_url="$(derive_wiki_url "$origin_url" || true)"
if [[ -z "$wiki_url" ]]; then
  echo "Error: unsupported origin URL format: $origin_url"
  exit 1
fi

tmp_dir="$(mktemp -d)"
wiki_dir="$tmp_dir/wiki"

cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

echo "Cloning wiki repository: $wiki_url"
git clone "$wiki_url" "$wiki_dir"

# Remove all existing wiki content while keeping the .git directory.
find "$wiki_dir" -mindepth 1 -maxdepth 1 ! -name ".git" -exec rm -rf {} +

echo "Copying local solutions into wiki repository"
cp -a "$SOURCE_DIR"/. "$wiki_dir"/
cp "$wiki_dir"/Home.md "$wiki_dir"/_Sidebar.md

git -C "$wiki_dir" add -A

if git -C "$wiki_dir" diff --cached --quiet; then
  echo "No wiki changes detected. Nothing to push."
  exit 0
fi

timestamp="$(date -u +"%Y-%m-%d %H:%M:%S UTC")"
commit_message="Sync wiki from solutions (${timestamp})"

git -C "$wiki_dir" commit -m "$commit_message"
git -C "$wiki_dir" push

echo "Wiki sync completed successfully."