# CSS - Cascading Style S-Expressions

[Online Demo via WASM](https://dustinnewman.net/cascading-style-s-expressions/)

Cascading Style S-Expressions allow you to write Cascading Style Sheets rules using S-expressions. For example,

```css
body {
    color: red;
}
```

becomes

```
(body color red)
```

And

```css
body {
    color: red;
}

body a {
    text-decoration: underline;
}
```

becomes

```
(body color red (a text-decoration underline))
```

This is a format for personal use, but of course, you can take a look if it interests you. I may or may not end up using this on my own [personal site](https://dustinnewman.net).
