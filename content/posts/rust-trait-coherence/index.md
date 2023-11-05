---
title: "Rust Trait Coherence"
date: 2023-11-04T16:36:26-07:00
draft: false
tags: tech, rust
---

## The Background

I defined a struct in a Rust project.

```rust
struct Content
```

I then wanted an easy way to convert from different, other types into `Content`, ideally staying within the Rust type system. My motivating example will be that you want to go from a file path to `Content`. Since I am a good, active Rust user, I know that I don't want to unnecessarily restrict my conversion function to only the concrete type `Path` because the same logic applies to all sorts of `Path`-like types e.g. `PathBuf`, `Components`, `OsStr`, and even `String`. So I implemented this trait generically.

```rust
impl<P> TryFrom<P> for Content where P: AsRef<Path>
```

The compiler threw an error message. This in itself is not too surprising: compiler errors are the bread and butter of Rust programming. But this particular compiler error threw me off guard.

```rust
conflicting implementation in crate `core`:
- impl<T, U> TryFrom<U> for T
  where U: Into<T>;
```

This came as a bit of a shock to me because, well, I had just defined this struct `Content`. How could Rust `core` already have an implementation for it? To understand, we have to unpack what the generics in this type signature are really conveying. This type signature says that we have an implementation of `TryFrom` *from* any type `<U>` *for* any type `<T>` if and only if `<U>` already implements `Into<T>`. Ok, when you put it like that, this actually seems quite reasonable. If we already have the logic to go from `<U>` to `<T>` infallibly, then surely we can represent this as a "fallible" operation, which simply returns `Ok` all the time. And, in fact, when you look at the [Rust core source code](https://github.com/rust-lang/rust/blob/f5ca57e153afaed818f8be88abf5ce46715c0f9a/library/core/src/convert/mod.rs#L800), this is exactly what it does.

```rust
// Infallible conversions are semantically equivalent to fallible conversions
// with an uninhabited error type.
#[stable(feature = "try_from", since = "1.34.0")]
impl<T, U> TryFrom<U> for T
where
    U: Into<T>,
{
    type Error = Infallible;

    #[inline]
    fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::into(value))
    }
}
```

Put this way, this seems not just reasonable but obvious. Of course, this makes sense. The issue I still didn't understand though was that, sure, for any type `<U>` if `<U>` implements `Into<T>` you can use this logical, "blanket" implementation to kind of vacuously implement `TryFrom<U> for T`, *but* my issue was that *my* `<U>` (`P: AsRef<Path>`) **doesn't** implement `Into<Content>`. So, why is Rust having an issue with this? And to answer it, we have to go into Rust **trait coherence**.

## Trait Coherence

Trait coherence is "[the property that there is at most one implementation of a trait for any given type](https://github.com/Ixrec/rust-orphan-rules/blob/4b2ccb102bd7c715c4dc2ec4bdeaa96c6662093c/README.md?plain=1#L9)." So, for example, you cannot have:

```rust
struct Content;
impl Eq for Content
impl Eq for Content
```

Once again, when presented like this, this seems reasonable. Despite my confusion, I have to agree that the Rust compiler is making a sane assumption here. The issue, however, quickly becomes more complicated once you allow generics and crates (i.e. third-party code and dependencies). There are two often cited rules to help the Rust compiler enforce trait coherence.

1. The Orphan Rule: Either the trait or the type for which you are implementing the trait must be local to your crate
2. The Overlapping Rule: For any two types A and B implementing a trait T, A must be different from B

The orphan rule seems much more controversial and discussed online, but I believe my issue actually stems from the overlapping rule. Now, previously I gave an obviously wrong example to motivate why the Rust compiler even has a concept of trait coherence. But to motivate the overlapping rule, let's give a [more realistic example](https://github.com/kennytm/rfcs/blob/a956323627bbc245dd3fe657f1dbc67060e77167/text/0000-negative-bounds.md) in order to steel-man the motivation. Let's say that we have a trait to get the average of two numbers.

```rust
trait Int {}

trait Float {}

trait Average {
    fn average(self, other: Self) -> Self;
}
```

We can implement this for `Int` types:

```rust
impl<T: Int> Average for T {
    fn average(self, other: Self) -> Self {
        if self >= other {
            other + (self - other) / 2
        } else {
            self + (other - self) / 2
        }
    }
}
```

And then for `Float` types:

```rust
impl<T: Float> Average for T {
    fn average(self, other: Self) -> Self {
        self * 0.5 + other * 0.5
    }
}
```

Uh-oh! Compiler error. The same as before. `conflicting implementations of trait Average`. But... why? We don't really have two *conflicting* implementations, as we have *two* implementations. It seems as though Rust is unnecessarily strict here since there is no issue yet. Well, you see, I actually sort of underplayed the overlapping rule earlier. It really should be

2. The Overlapping Rule: For any two types A and B implementing a trait T, A must be **provably** different from B **in every possible program**

This is called ["negative reasoning"](https://aturon.github.io/blog/2017/04/24/negative-chalk/). It is essentially the difference between saying "there does not exist any type which implements a trait twice *currently*" and "there will *never* exist any type which implements a trait twice." Hopefully now it is more clear what the actual issue is. There is nothing specifically **stopping** a type from implementing **both** `Int` and `Float`. In the absence of this guarantee, Rust errs on the side of caution. And to Rust's credit, this makes sense. Just look at the following, nothing seems wrong.

```rust
struct Real
// Is this error?
impl Int for Real
// Or this?
impl Float for Real
```

If the compiler instead decided to follow the approach I preferred (don't throw an error until a conflict actually exists), it would make implementating a trait an error. Not just an error, but a rather arbitrary error as well. In the example above, which implementation would you consider the "bad" one? There is no deterministic answer. This is not only unexpected, but would have breaking downstream consequences: a conflict may exist within other people's crates because of a trait conflict that does **not** exist on your system, but **does** exist on theirs. This would be a huge disaster for the Rust ecosystem of crates, not to mention seriously hinder Rust's changes of mainstream adoption. Looking at it from this perspective, it actually becomes clear why the Rust compiler decides to just play it safe and stop the problem at the root: a trait definition which could, theoretically, introduce an implementation conflict. This keeps the wider Rust ecosystem safe until a robust "fix"/solution is developed.

But back to my original issue. You might recall that, unlike the previous example, there *was no* more specific, conflicting example. I just wanted to implement

```rust
impl<P> TryFrom<P> for Content where P: AsRef<Path>
```

It is not so much that there exists "two implementations" for `TryFrom for Content`. It is moreso that currently Rust cannot guarantee that there will never be a type which satisfies two implementations. For example, if someone were to implement `TryFrom<String>`.

```rust
impl TryFrom<String> for Content
```

This would introduce two valid implementations: one for the generic type `P: AsRef<Path>` (which `String` satisfies) and another for the specific type `String`. Similarly, you could imagine we have a trait to calculate the last modified time of a file path.

```rust
trait LastModified {
    fn last_modified(&self) -> std::time::SystemTime;
}
```

For all regular files, this is simple. Just take the modified time (`mtime`).

```rust
impl<P: AsRef<Path>> LastModified for P {
    fn last_modified(&self) -> std::time::SystemTime {
        fs::metadata(self.as_ref())
            .and_then(|metadata| metadata.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    }
}
```

But for directories, the `mtime` might not be exactly what we want. It most likely will be the last time a file or subdirectory was added or removed from the directory, rather than the last time a file or subdirectory was actually *modified*. To fix this, we would have to make a special implementation just for directories.

```rust
impl LastModified for ReadDir {
    fn last_modified(&self) -> std::time::SystemTime {
        self.filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter_map(|path| fs::metadata(&path).ok())
            .filter_map(|metadata| metadata.modified().ok())
            .max()
            .unwrap_or(SystemTime::UNIX_EPOCH)
    }
}
```

So which one to choose? Currently, there is no way, and doing so would require a feature called "specialization".

There are a few options here:

1. Extend Rust to supportive "negative" trait bounds (i.e. `P: AsRef<Path> &! ContentNode: TryFrom<P>`) to allow the compiler to guarantee one implementation per type
2. Allow multiple implementations per type and design a mechanism for deciding between them ("specialization")
3. Allow users to specify a crate locality condition that this crate will never be used by any other crate and thus we can bypass this condition just between us
4. Just use `PathBuf` here instead of generics

For now, I'm sticking with number four.
