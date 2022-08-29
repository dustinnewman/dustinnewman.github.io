---
title: "Poset Theory of Politics"
date: 2022-03-24T17:33:14-07:00
draft: false
tags: politics
---

I'm a semi-avid reader of Astral Codex Ten (ACX), and there was a [semi-recent post](https://astralcodexten.substack.com/p/justice-creep) on so-called "justice creep" in modern identity and issue politics (e.g. social justice, environmental justice, food justice, X-justice). This is not about that post, but rather the [RFC](https://astralcodexten.substack.com/p/highlights-from-the-comments-on-justice) following it.

While I found the original enlightening and nuanced on both sides of the X-justice coin, nothing really struck me enough to write my own post about, until now. One commentor applied the X-justice view to the "incel" (involuntarily celibate) group, as in "It is unjust that some people have sex more than others" evoking some sort of sexual communism (amazing phrase).

As I assume most people were (false consensus effect, perhaps), I was put off by not only the framing of sex as a "right", but by the sway and logical compulsion of this hypothetical. When taken on face value, it seems to be something completely aligned with the X-justice viewpoint, and yet - sociologically - belongs to a group diametrically opposed to said viewpoint. The resulting cognitive dissonance was so uncomfortable, I simply had to think and ponder some way out of this dilemma. While ACX does propose the objection that aligns most closely with my initial thoughts:

> It’s hard to think of a way to help him that doesn’t impinge on important freedoms in some way. Either the government would have to use force to coerce people to have sex with him, or use force to coerce people to give him their money so he could pay others to have sex. Both of these solutions seem to have enough ethical downsides not to be worth it.

I needed a more rigorous, or at least more formal idea to really argue against the incel equivalence.

## The Poset Theory of Politics

The theory hinges on the idea of a *partially-ordered set*: a *poset* as it is known. My first introduction to these actually stemmed not from mathematics but rather from linguistics, optimality theory to be specific. The idea behind optimality theory is that, actually, any thing you have ever communicated has broken some sort of "rule" (valid), but that what we call a "language" is merely a consistent (or relatively consistent across native speakers) ranking of these rules. There are some rules you can break willy-nilly and all the way to the bank, while there are others that, if broken, render your speech incomprehensible (relatable).

Optimality theory makes heavy use of Hasse diagrams, which allow one to rank various rules or constraints or, put differently, express a partial order relation on a poset. One example of a Hasse diagram is the representation of two being greater than one i.e. `1 <= 2`.

{{< rawhtml >}}
<svg viewBox="0 0 7 55" xmlns="http://www.w3.org/2000/svg" width="100%" height="100px"><g fill="none" fill-rule="evenodd"><text fill="var(--text-color, black)" font-family="EBGaramond-Regular, EB Garamond" font-size="14"><tspan x=".14" y="14">2</tspan></text><text fill="var(--text-color, black)" font-family="EBGaramond-Regular, EB Garamond" font-size="14"><tspan x=".14" y="51">1</tspan></text><path stroke="var(--text-color, black)" stroke-linecap="square" d="M3.5 18.5v19"/></g></svg>
{{< /rawhtml >}}

I will be using this idea for my construction of a poset theory of politics. Let's start simple: theft. Take a culture which views theft as morally wrong. We can represent this moral principle as a rule called `No-Steal` and apply it as follows:

| Action      | `No-Steal`    | Winner |
|:------------|:-------------:|:-------|
| Steal       | X             |        |
| Don't steal |               | X      |

Therefore, given the choice between stealing and not stealing, the obvious choice is to not steal.

But life is often much more complicated than this. So let's introduce a complication: hunger. Our morality has an aversion to suffering and will allow theft if it is to sate hunger, a principle going all the way back to the Bible. [^1] We will call this `No-Starve` and we will allow it to *out-rank* `No-Steal`. In poset notation, `No-Steal <= No-Starve`. This is simply a way to use our constraints to express the idea that stealing is okay if you are hungry.

Now our possible choices look like this:

| Action                    | `No-Starve` | `No-Steal` | Winner |
|:--------------------------|:-----------:|:----------:|:-------|
| Steal (but don't starve)  |             | X          | X      |
| Starve (but don't steal)  | X           |            |        |

Here we can see that, even though stealing is wrong (`No-Steal` says so), it is acceptable and even expected that one will do so if hungry, as required by `No-Starve`.

Now that we've got introductions out of the way, we can address the topic of X-justice with some more clarity. ACX used the example of climate justice vs murdering so-called climate villains. On one hand, we want to preserve the environment and minimize or even reverse our greenhouse gas emissions: `No-Pollute`. On the other hand, some people are polluting - or perhaps more accurately, leading companies which *do* pollute - the environment. One suggestion is to simply murder these people.

However, our pesky morality has another constraint: `No-Murder`. Even more unfortunately, `No-Murder` *outranks* `No-Pollute` i.e. it is worse to murder someone than it is to pollute the environment.

Our poset politics predict exactly what we would: you can't murder the guys in charge.

| Action                        | `No-Murder` | `No-Pollute` | Winner |
|:------------------------------|:-----------:|:------------:|:-------|
| Murder climate villain        | X           |              |        |
| Do not murder climate villain |             | X            | X      |

While this result may be disappointing to some leftists, it does at least make some sense: killing a CEO won't destroy a company and it certainly won't fix the climate crisis. Without changing the underlying system which allows pollution and allows perverse capital incentives to outweigh environmental costs, it doesn't matter if Company Alpha is dismantled because in a few years, Beta will take its place.

Of course, this is an extremely simplified model. While the loss of life in WWII was unimaginable, it would be a far worse tragedy if the Third Reich were to march on unimpeded. Revolutions almost always demand the murder of the old guard but may also expand the rights and freedoms of society in the long-term, outweighing the finite cost of human life in the short-term. While the calculus may be upsetting, the alternative is more so.

For now, we need not establish a flawless and universal moral mathematics, however. We only need to be better than the incel equivalence.

The argument hinges on a principle called "sexual justice" i.e. everyone having an equal amount (the quality/satisfaction is not addressed in the original argument, so we will keep it simple for now) of sex. In the world as-is, sex can be modeled as a (consensual) agreement between any number of people. While this is akin in many ways to any other agreement (e.g. a contract), there is something specific to sex which gives it a stronger expectation of freedom: *bodily autonomy*. In its most limited sense, bodily autonomy is the freedom to resist others' actions against your body. Someone cannot force you to cut your hair, stab you, or - most pertinently - have sex with you against your will because doing so would infringe upon your bodily autonomy.

Grounding our discussion in reality, many cultures, including America, have a strong sense of bodily autonomy. In poset terms, the rule `Bodily-Autonomy` outranks many other such rules. Before we raise objections here and because bodily autonomy can be a loose concept, I am specifically referring to bodily autonomy in the sense of resisting external coercion against your body. I am not using the stronger, so-called "positive rights" model which defines the right to *do* something as opposed as the right to *resist* something. For example, the positive rights model supports the *right to have* an abortion, whereas the negative rights model I am using here supports *freedom from* a coerced abortion. Using this subset makes things a bit simpler for now.

Since "sexual justice" is not an established moral principle (I have never heard of such a formulation before this post), we are going to use the root cause moral principle here: `Equality` i.e. the equality in occurrences of sex. Thus, our final framing of this issue is that the rule of `Bodily-Autonomy` outranks the rule of `Equality`. In fact, `Bodily-Autonomy` ranks so high that it even justifies murder in the case of self-defense (you have the right resist someone trying to murder you). Note further than the solution of "sexual justice" here is framed as "state mandated gfs/bfs" or "candidates who will mandate sex" and *not* as legalized prostitution.

| Action           | `Bodily-Autonomy` | `Equality` | Winner |
|:-----------------|:-----------------:|:----------:|:-------|
| Sexual injustice |                   | X          | X      |
| Mandated sex     | X                 |            |        |

Here we have it: a somewhat formal, mostly arbitrary way of saying what ACX already said. Of course, the framing of any and all of this as "justice" and moral mathematics is the initial step down this slippery slope in the first place. I was originally considering a sort of equality axis and freedom axis where we could instead frame the issue as continuous, as merely a preference in a slope on a graph. Unsurprisingly, trying to reduce the issues of "equality" and "freedom" to one-dimensional units was not so easy and, in particular, those damn Nordic countries posed too great of a challenge to model in one day.

[^1]: Proverbs, 6:30.
