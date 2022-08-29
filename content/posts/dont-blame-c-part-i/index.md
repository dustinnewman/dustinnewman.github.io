---
title: "Don't Blame C Part I"
date: 2021-07-21T06:03:00-07:00
draft: false
tags: tech, c
---

Yet another [security vulnerability](https://www.openwall.com/lists/oss-security/2021/07/20/1) has been disclosed in the Linux kernel, this one dealing with gaining root access from a user land program. The root cause is a `size_t` argument being passed as a (signed) `int`. This has people maligning C for being unsafe, unstable, and unreliable, all of which are true. But I feel compelled to defend the elderly in all things, especially when they are programming languages, and so I will do so here. Your logic may vary. 

The way I see it, a parameter named "buflen" should **never**, _ever_, **ever** have a signed type. You hear that, Linus? Better get grepping, you old Finn. Why should it? Length can never be a negative value, only zero. So why would any sane API accept -1 as a valid length? Of course, when I say it in such plain terms this may seem obvious. I'm sure the writer of this phony API would never agree to a statement like "length can be negative." And yet… that is precisely what you are saying when you write:

```c
char *dentry_path(struct dentry *dentry, char *buf, int buflen)
```

Ah, god, it hurts my eyes to just look at this. I advise you to scroll away for your own posterity, dear reader. Such affronts belong not in a civilized society. Of course, the correct way to write this is with `size_t buflen` and then we wouldn't be having this issue and then Twitter wouldn't be collectively jumping around a bonfire burning the limbs of C in an effigy and then I could get some sleep at night. Well, not since I got this strange doll from the thrift store. 

This should be standard in every embedded or kernel development interview. I guess I blame… people teaching C. Why does every C tutorial and even published book start with `int x` when you never see negative values assigned? For example, K&R, perhaps _the_ standard C reference, has on page 18:

```c
long nc;
nc = 0;
while (getchar() != EOF)
    ++nc;
```

How baffling that K&R thought using the slightly more efficient pre-increment operator is more important than using the correct type. Especially to beginners. `nc` has no business being `long`. I get it: "int" is fewer characters and you don't want to scare away beginners using incantations like "unsigned" or "size underscore t." Everybody already knows what an "integer" is, or has at least heard of one. However, I think C instructors are forgetting one key fact:

A hostage audience. 

No one learning C really has a choice. Either they're taking CS 101 or they're job hunting and want an extra tag in the "skills" box. Since you have them between a rock and a hard place, you can kind of teach them whatever you want. I'm not suggesting we start with `char (*(*x())[5])()` but maybe getting it in early that "types have meanings" would be a good thing. Maybe, just maybe, it would prevent issues like the Linux kernel bug from being written in the first place. Maybe we could even turn the tide against "int as default" which is easily one of the worst trends in computer science.

Of course, it's much easier to blame C though. Dang nabbit you, Dennis Ritchie!