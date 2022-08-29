---
title: Cascading Style S-Expressions
date: 2022-04-03T19:44:12-07:00
draft: false
tags: tech, project, frontend
---

Since I first learned it, Lisp has fascinated me. Not enough to seriously use it beyond projects for college, mind you, but enough where - years later - I’m working on something completely random and the thought crosses my mind “It would be fun to implement this in Lisp,” especially using + as a function, same as any other, in full prefix notation, something even the noble and esteemed Haskell couldn’t seem to commit to. Beyond the simple grammar and elegance, there is something about Lisp which feels fundamental to computation itself, likely the similarity to lambda calculus, which actually *is* fundamental to computation itself. Sadly, Lisp is rarely used much of anywhere nowadays except as a novelty or pet project. This is strange, considering that Brendan Eich was inspired by Scheme while designing JavaScript. You would think there would be a more direct influence.

Fortunately for us, it doesn’t have to be that way. I have long thought of writing CSS is a Lisp-style syntax. I’m not sure why exactly. Something about it just *feels* right. Simple. For maximum confusion, I have decided to name this **Cascading Style S-Expressions**: CSS for short.

Now, I’m not gonna claim that we can bring Lisp to the web. Lisp’s acolytes are a devout and proud people, who would scoff at the claim that CSS properties and markup even come close to represent the power and beauty of Lisp. And this is correct on some level. CSS is a language for configuration, not computation. There are no functions, no inputs or outputs, no arithmetic, none of that. Just a bunch of key-value pairs. 

That said, I’m gonna do it anyways because, like I said, it’s fun. There are some parts of Lisp which fit nicely with CSS. The nesting of S-expressions is a cleaner way to combine selectors; the extremely simple syntax (the only special characters are parentheses) eschews semicolons, colons, and braces; and honestly something about CSS has just always felt Lisp-y to me. But mostly this is just for fun and an itch being scratched. The quintessential example, what every CSS tutorial starts with, is setting the background-color of the body element.

```
(body color red)
```

Here the first atom is the selector: the `body` element. The next two atoms are actually a pair: the property (color) and its value (red). Together these make up a CSS rule which is just setting one property to some value. This is equivalent to the CSS configuration:

```
body {
    color: red;
}
```

Can you feel the aesthetic purity already? I sure can. Multiple rules can be chained together by just repeating these pairs.

```
(body color red font-size 14px)
```

I’m gonna assume you already know what this would be equivalent too, so I’m not gonna write it out anymore. The real star of the show though, is nested selectors. Compared to CSS where you have no way to nest without writing an entirely new rule set, CSS (Lisp-style) allows you to specify rules for the current selector, and then nest any children in that same rule set. For example, a common use case when styling a navigation bar that uses an unordered list behind the scenes is to get rid of the styling for both the list and the links within the nav bar. In regular CSS, this is two separate rule sets (one for `ul` and one for `ul li a`), but in our CSS, it’s just one straight line.

```
(ul list-style none (li (a text-decoration none)))
```

Allowing the children to be specified as sub-S-expressions is immensely satisfying to me, as I find it better clarifies the relationship between the selectors. Of course, we can also have CSS properties that are more than one word, or contain spaces, the most common being the margin shorthand taking up to four parameters of the border shorthand taking three. For this, I overload the parentheses to create lists. In proper Lisp notation, these would start with a single quote.

```
(body margin (0px 8px 0px 8px) border (1px solid black))
```

This is actually not ambiguous to the parser because all rules come in pairs of two: property and value. The nested S-expressions will then be those that don’t follow a property. And what about if you have two S-expressions right next to each other? How do we tell them apart from a rule? Simple: properties cannot contain spaces and thus cannot be listed in parentheses. Therefore, in this case we would have two S-expressions and not a rule. 

If you would like to try this out for yourself, the code is [open-source](https://github.com/dustinnewman/cascading-style-s-expressions) but who wants to go through all the work of pulling and compiling my parser? For that reason, there is also an [online demo](https://dustinnewman.net/cascading-style-s-expressions/) I put together with my first time using WASM. The page is pre-populated with the stylings of that page and you can dynamically edit the CSS to see your Lisp applied. I might end up using this on my own site, the very one you’re reading right now, but time will tell if I have the patience to integrate all of this into Hugo or not.
