---
title: "Tags as Set Theory"
subtitle: "Let's play #"
description: "Applying set theory to web tagging systems."
keywords: "tags, set theory, tagging"
date: 2019-12-29T12:00:00-07:00
draft: false
---

As a former Tumblr-er, there was nothing more frustrating about that site than the tagging system. Aside from the near-universal problem of "this tag" vs. "thistag" vs. "This tag" vs. ad infinitum, I have yet to see a tagging system that allows set theory operations. What if I want to see posts analyzing the parallels between Taylor Swift’s "reputation" and "Lover"? Sure, I can search "reputation," but then I have to sift through mostly irrelevant details to find the hidden gems. What I really want is a way to say "reputation ∩ Lover."

Going back to the "this tag" problem, imagine I'm searching for posts about Star Wars. I want a way to search for "star wars ∪ starwars ∪ Star Wars" so that I can capture all posts no matter which tag they used. Or I want to see posts only about the 49th state, Alaska, and none about the drag queen of the same name. I need "alaska \ rpdr."

Now that the motivation has been laid down, we need a more concrete syntax.

## Syntax

```abnf
query = 1*expression

expression = tag / expression, operator, expression / "(" expression ")"

tag = 1*VCHAR ; except union, intersection, and difference

operator = union / intersection / difference

union = "||"

intersection = "&&"

difference = "\\"
```

Some example queries:

```
reputation && lover

starwars || star wars || Star Wars

alaska \\ rpdr
```

With the exception of pissing off some programmers, I don’t think this grammar is too restrictive. Tags can include any characters except those representing union, intersection, and difference operations, which don’t seem to be fairly common. Of course, the specifics can always be changed in a real world scenario.

Speaking of the real world usage, the above example queries in URL encoding would be:

```
reputation%26%26lover

starwars%7C%7Cstar%20wars%7C%7CStar%20Wars

alaska%5C%5Crpdr
```

Particularly observant readers will note that my ABNF grammar permits use of parentheses. Although both union and intersection operations are associative, difference is not and I used parentheses to allow precedence in queries using difference. What if I want to view all posts with JUST cats? I don’t want to see any puppy dog eyes on my screen whatsoever!

```
cats \\ dogs ; just cats
dogs \\ cats ; just dogs
dogs \\ (cats \\ cute) ; all dogs of any cuteness and only cats that aren't ugly
(dogs \\ cats) \\ cute ; all dogs that aren’t cute
```

As you can see, there is a lot more nuance here than is typically captured by tagging systems. It also offers a lot more complex queries without requiring exponentially complex tagging. In practice, assuming a normalized database, these queries don’t seem to be adding too much complexity beyond the standard “get all posts of a certain tag” query. This is because relational database management systems themselves draw from set theory to ground most of their operation.
