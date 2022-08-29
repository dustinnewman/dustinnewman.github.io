---
title: "Maybe Optional, Definitely String"
date: 2022-03-09T21:35:29-08:00
draft: false
tags: tech, programming-languages
---

In parsing structured files, there are many structures which share fields between various types of "things" and depending on the thing type, these fields may or may not be optional. It is not that they are `Option<Option<T>>`, it is that they are `T | Option<T>`. That is, some type of the "thing" must absolutely have a T, while other types of that "thing" may or may not have a T. 

For example, let's say we have a structure defining a font:

```rust
struct Font {
	weight: FontWeightEnum,
	italic: bool,
	// snip! many other fields
	name: Option<String>
	// snip! many other, other fields
}
```

The problem here is that `name` field. Let's say we have two types of fonts: strict and squishy. Strict fonts must always have a name, while squishy fonts can, but are not required to. "What's the problem then?" you ask. "Strict fonts will always be `Some(name)`." But see, then our program is not modelling reality correctly. There is an invariant here that strict fonts must always have a name, and yet they use the same `Option` that squishy fonts do. This would allow you to create a "strict" font which has no name, and the type system does nothing to enforce this. We then have to rely on either the programmer's careful diligence, or some `new` function to enforce the invariant, in which case we are using an `Option` when we don't have to and will never use the `None` case.

```rust
impl Font {
    pub fn new_strict(name: String, ...) -> Self {
        Self {
            // ...
            name: Some(name),
            // This is redundant since we will never ever
            // have a strict font with None for name
        }
    }
}
```

The first suggestion here is to break up the fonts into two new structs:

```rust
struct StrictFont {
	weight: FontWeightEnum,
	italic: bool,
	// snip! many other fields
	name: String
	// snip! many other, other fields
}

struct SquishyFont {
	weight: FontWeightEnum,
	italic: bool,
	// snip! many other fields
	name: Option<String>
	// snip! many other, other fields
}
```

But this requires a huge duplication of data, most of which *is* common between the two: `weight`, `italic`, `stretch`, etc. Surely, there must be a better way. Perhaps we could encapsulate all the same fields into one type and then layer on those which are unique:

```rust
struct Font {
	weight: FontWeightEnum,
	italic: bool,
    // snip! many other fields
}

struct StrictFont {
	font: Font,
	name: String
}

struct SquishyFont {
	font: Font,
	name: Option<String>
}
```

However, this implies that there is some inner font kernel inside of either a strict or squishy font. This is not the case. A strict font does not *have* a font; it *is* a font! Not to mention that this could grow quite complex the more variable types we have. Consider if we have a third type of font which does not require a name, but does have a unicode map which no other font has. Instead we could use a type parameter to define a font:

```rust
struct Font<NameType> {
	weight: FontWeightEnum,
	italic: bool,
	name: NameType,
}

type StrictFont = Font<String>;
type SquishyFont = Font<Option<String>>;
```

But then this also allows introducing invariants. This would allow you to define some type `NotReallyAFont = Font<bool>` which then defines the name to be a boolean! This is never possible obviously, but our program above allows it. Further, suppose that we actually many different fields that will also vary between strict and squishy fonts. Bounding boxes, maps, ascents, x-heights, etc. "Well, if they're so different, then just split them up entirely!" While this is possible, it still results in the same duplication of data mentioned earlier, and is not fully modelling the problem. It is not as though there are two entirely different concepts, strict fonts and squishy fonts, which are as different as apples and lions. There really is one concept - a font - and there just so happens they differ on various fields and if those fields are required. A correct model of the problem would capture this relationship.

Alas, even product types do not come to our rescue here because they would not allow for us to define the fact that strict fonts are `String` while squishy fonts are `Option<String>`. In pseudo-Haskell, consider:

```haskell
data Font = RecordType
    { weight :: Float
    , italic :: Bool
    , name :: String | Maybe String
    }
```

In the example above, this does capture the similarities between the two font types, but leaves us no way to specify that the `String` type should be strict and the `Maybe String` should be squishy.

That brings me to what I consider the true, accurate, and impossible solution to the problem:

```rust
struct Font<NameType> {
	weight: FontWeightEnum,
	italic: bool,
	name: NameType<String>
}

type StrictFont = Font<()>;
type SquishyFont = Font<Option>;
```

This solution allows us to both guarantee that strict fonts must have a name, while squishy fonts are free to choose; enforce that a name, whether optional or not, must be a string; and avoid data duplication at the same time. The downside to this is that it will not scale if there are many other field constructors (similar to the earlier solution) and could quickly become a nightmare of tracking type parameters. Oh, and the fact that it's impossible. That's another downside. For now, it seems the best solution is to be somewhat less neurotic: just use `Option<String>` and learn to live with it.
