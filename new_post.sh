#!/bin/sh

if [[ $# -eq 0 ]] ; then
    echo "Usage: new_post.sh <post-slug>"
    exit 1
fi

hugo new posts/$1/index.md
cd content/posts/$1

