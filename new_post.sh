#!/bin/sh

if [ $# -eq 0 ]; then
    echo "Usage: new_post.sh <post-slug>"
    exit 1
fi

dir="src/content/posts/$1"
if [ -e "$dir" ]; then
    echo "$dir already exists"
    exit 1
fi

title=$(echo "$1" | tr '-' ' ' | awk '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)}1')
mkdir -p "$dir"
cat > "$dir/index.md" <<EOF
---
title: "$title"
date: $(date -Iseconds)
draft: true
---

EOF
echo "Created $dir/index.md"
