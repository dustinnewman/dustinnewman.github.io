#!/bin/sh
# Usage: ./new_post.sh "Post Title"  -> site/posts/post-title.mark
set -e
title="$1"
slug=$(printf '%s' "$title" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-|-$//g')
file="site/posts/$slug.mark"
[ -e "$file" ] && { echo "$file exists"; exit 1; }
printf 'prop title = "%s"\nprop date = "%s"\nprop draft = true\n\n' "$title" "$(date -u +%Y-%m-%d)" > "$file"
echo "$file"
