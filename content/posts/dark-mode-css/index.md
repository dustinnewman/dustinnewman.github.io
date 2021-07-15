---
title: "How to use CSS with native dark mode"
subtitle: "Let there be... dark"
description: "Tutorial on how to use CSS to support iOS dark mode."
date: 2019-12-16T12:00:00-07:00
draft: false
---

With iOS 13, I have been enjoying not just the beautiful new dark mode, but the automatic transition after sunset. It's easier on my eyes and keeps me conscious of using technology too much before bed. That said, the native dark mode is only for native apps. Or is it?

I thought it would be nice if there were CSS selectors capable of targeting users' native preferences on the matter and luckily Apple did not disappoint. In [this tutorial post](https://webkit.org/blog/8840/dark-mode-support-in-webkit/) from the WebKit blog, they introduce the `color-scheme` CSS property, which supports both `light` and `dark` values. I was drawn to this solution over the DIY option (which usually involves some sort of moon icon) because it offers:

1. Less UI complexity
2. More seamless integration with the user's system
3. Less JavaScript logic required

All wins in my book, so let's get started! (For this blog, I use SCSS, but I only use the SCSS variables for media queries and the rest of this tutorial uses regular CSS variables.)

Add this snippet to your top-most styling file (for me, it was my `main.scss`):

```css
:root {
  color-scheme: light dark;
}
```

This tells the browser that your site supports both `light` and `dark` themes. By itself, however, it's not super useful, so let's define some variables that we can switch depending on the theme.

```css
:root {
  color-scheme: light dark;
  --bg-color: #fcfcff;
  --text-color: #000000;
}
```

Here, we use an off-white as the background color and pure black as the text color. I strongly recommend using functional names rather than descriptive (i.e. `bg-color` instead of `off-white`) because it allows us to use the same variable declarations with only one media query, rather than using a media query each time we want to use either `off-white` or the dark-theme counterpart.

Now the magic!

```css
@media (prefers-color-scheme: dark) {
  :root {
    --bg-color: #121212;
    --text-color: #fcfcfc;
  }
}
```

With one media query, we re-define our variables so that the _text_ is now off-white and the background is off-black. To apply this, let's use our main `body` element as an example.

```css
body {
  background-color: var(--bg-color);
  color: var(--text-color);
}
```

Now our `body` will use the `bg-color` variable which, if the user prefers dark mode, will be an off-black! Nice! I hope this mini-tutorial was helpful and that you see the advantages of this way over rolling your own moon icon; although I'll admit that might be a bit more fun.
