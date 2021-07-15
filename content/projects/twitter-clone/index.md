---
title: Twitter Clone in Go
subtitle: My first use of Go
date: 2020-01-19
image: twitter-clone.png
role: Creator
stack: Go
github: "uncomputation/twitter_clone"
---

I've been interested in the Go programming language for a bit, largely due to the huge ecosystem exploding around it lately. Especially in web services (Mattermost, Keybase, Uber, etc.) and deployment tools (Docker, Kubernetes), there have been so many amazing new tools built in the language that I've been meaning to see what the excitement was all about. And, as a web-loving programmer, I had particular interest vested in the merits of Go.

I hope you'll forgive me for a small sidebar, but after this project, I believe I finally have a grasp on what makes Go so popular despite the on-paper disadvantages to many other languages. Go is slower, less safe, and less innovative than Rust yet less "pleasant" than Python. So why have so many people flocked to it then? Two reasons, in my opinion:

1. Go has a _niche_: web services. This gives it a clear entry into many new projects, whereas something like Rust isn't so "obvious."
2. Simplicity.

Rust borrows from a lot of programming language theory and functional paradigms (pattern matching, compile-time concurrency error detection, the borrowing system) making it exceptional, on paper. Go, on the other hand, is the statistical average of every Algol language since the 1960s and that can be _nice_. It's easy to learn, since you've seen it already. Rust is a stick shift; Go, an automatic. Sometimes that extra control and "correctness" can be nice, even essential. But for most web-based projects, the bottleneck is not in your server code, but in the inherent network latency (or the modern megabytes of JavaScript waiting to be downloaded). This makes Go perfect for web projects and other applications which need speed, but not at the expense of development time. Go provides a stellar standard library, an amazing implementation of green threads, and a lot of development niceties (don't talk to me about its dependency management situation though).

So, Go! Now that we've established a bit of the motivations for using it, let's talk about my overall experience.

## Things I Liked

Surprisingly, I found the lack of semicolons refreshing. I hate whitespace delimiting, but actually found the unterminated newlines nice and not at all "squishy" like Python.

The compile times are so snappy it feels like a scripting language. My Twitter project used Postgres drivers, Google Cloud libraries, and a non-trivial amount of application code and never once did I worry about killing the server to reload a one-line change. It just _worked_. And, unlike Node.js, I felt good knowing that it not just worked, but that performance was excellent. Not that Node is "slow," but it's hard to beat a compiled language.

Coming from the JavaScript community, the lack of a strong library community in Go was off-putting at first. With JavaScript, there are clear "standards": Express, React, Webpack, Passport. These almost go without saying. This can be really comforting, having tons of tutorials and examples and GitHub issues walking through your exact problem(s). Go isn't like this. Within the Go ecosystem, it's much more modular and the libraries provided are intentionally very minimal. For example, the [Gorilla sessions library](https://github.com/gorilla/sessions) I used was five files. I could go into the code and understand exactly was happening. Compare this to JavaScript, where you find yourself hunting some invisible rabbit down a hole 42 dependencies deep screaming "NODE GYP ERROR."

Documentation. [GoDoc](https://godoc.org) is basically Go's version of Python's [ReadTheDocs](https://readthedocs.io/) but because Go is statically-typed, it shows all the library-defined types, function parameters and return values, and even examples in an organized, clear way. This was invaluable.

Go's lack of "elegance" or "complexity" can be mind-clearing in some senses. You work with a basic set of building blocks: for loops, structs, `if err != nil`, type casting, etc. And the rest is up to you! There's no `map`, `reduce`, or contagious `async/await`. Once I got used to it, the approach was familiar and comfortable.

## Things I Disliked

Go has a concept of "zero values," meaning the sensible "default" value for a particular type. For integers, it's zero; for strings, empty string. So when you instantiate a struct, any unspecified fields will be their respective zero value. I didn't like this because when I accidentally forgot a field, Go didn't warn me at all! The more types you define, the bigger a mess this becomes and it spreads around the codebase. I wish there was some way to define optional fields à la Rust or Swift.

The semantics of assignment versus definition can get mildly annoying due to Go's use of the `:=` (definition operator). Whenever I would define another `err` variable earlier in a function, Go would complain that I was no longer initializing the former-first error variable. As an example, this is okay:

```go
// ...
_, err := someFallibleOp()
// ...
```

Whereas this isn't:

```go
result, err := someOtherFallibleOp()
// ...
_, err := someFallibleOp()
```

Which also brings us to unused variables. I understand that in a large project, especially one at Google, these small conveniences add up and lead to confusing and unclear code (i.e. "why did they declare this variable if they didn't use it?"), but it can be annoying when it's just you iteratively developing. But I get why Go has this opinion and overall I'm not mad about it.

For something as common as printing out values, Go should also really have a global `print()` function instead of `fmt.Printf()` but now that they don't, it's too late and we'll never get that feature! It's a very strange decision, especially once you consider that `append()` is a built-in, but `print()` isn't. I would greatly prefer an interface implementation instead (`slice.append(val)`), but the lack of generics in the language makes this impossible to implement, so we have a strange mix of things that are built in and things that aren't.

Go's VSCode extension is incessant about the numerous plugins it offers to the point I almost hated re-opening the application.

## Overall

I am overall a fan of the language and the huge ecosystem it offers and plan to continue to use Go in future projects! Even the "bad" things either made sense or weren't truly bad (except null pointer exceptions! In 2020!!!) and the trade-off of less than Rust-level safety for Go-level productivity and developing time felt very worth it.
