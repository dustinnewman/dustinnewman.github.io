---
title: "Add a DEVLOG"
date: 2022-08-01T21:19:24-07:00
draft: false
---

When working on a long-running project, I often find myself coming back weeks or even months later, starting at a codebase I have only the faintest memory of writing, with an even fainter idea of where I left off and - most importantly - what needed to be completed next. While documentation can help out with understanding code structure, modules, background knowledge, or why things are implemented the way they are, it is not written chronologically to give the reader an idea of where the writer left off. Of course, this is for the best as cataloguing a roadmap is not what documentation is for. Perhaps something like issues or boards fills that roll best. But not every project has such a clear roadmap. At least, not every project I have ever worked on. Sometimes, development is much more "one foot after another," implementing the basic functionality you see ahead of you and just sort of winging it. For this, writing a roadmap document - much less, doing the work of converting that into issues and then a board - is difficult or even a waste of time.

Instead, I have just found a solution I would like to recommend: `DEVLOG`. A `DEVLOG` (`.md` or `.txt`) a prepend-only document where you write what you will be working on that day *before you start working on it*. Then you go about and do your work and, regardless of the progress you made, you will already have the log of your efforts in your devlog ready to commit to Git. Depending, you can use the commit text combined with your most recent devlog update to figure out later what trajectory you were heading in when you pick it up next... whenever.

