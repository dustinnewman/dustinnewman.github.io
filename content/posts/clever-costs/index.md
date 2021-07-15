---
title: "Clever Costs"
subtitle: "When one-liners become extra-liners"
date: 2020-12-17T02:44:41-08:00
tags: []
categories: []
---

I was going through basic LeetCode problems recently after hitting a frustrating snag in a real project and I came across a Problem. An easy problem. But an interesting one. Well, this is LeetCode we’re talking about so maybe “illustrative” is a better word. It’s an illustrative problem. 

I’m talking about the “Buy and Sell II” problem which essentially boils down to finding the sum of increasing intervals. Ah, I have just the solution for this! We’ll just walk through the array in pairs of two, summing the difference if it’s positive.

```cpp
#include <vector>

int max_profit(std::vector<int> prices) {
    int r = 0;
    for (int i = 0; i < prices.len(); i++) {
        if (prices[i] > prices[i - 1]) {
            r += prices[i] - prices[i - 1];
        }
    }
    return r;
}
```

Great! But my DRY detector is kicking in and it is not happy with that repetition of `prices[i - 1]` business. It’s ugly! Amateur even! Are you sure you’re a *real* computer programmer? Uh-oh, now imposter syndrome is chiming in too. This is getting out of hand. Luckily, I have just the clever one-liner to satisfy them both (for now).

```cpp
#include <vector>

int max_profit(std::vector<int> prices) {
    int r = 0;
    for (int i = 0; i < prices.len(); i++) {
        r += std::max(0, prices[i] - prices[i - 1]);
    }
    return r;
}
```

Beautiful! Linus will be pleased I saved an indentation. Happy and satiated, I submitted my concise little automaton.

24 MILLISECONDS? I didn’t even chart on Leetcode’s runtime bulletin of shame. Unacceptable, I spent a whole brain cycle beautifying this function. I made an offering to DRY and I find myself now banished from the Leetcode chart? With what gold stars am I to feed to my ego now? Curious, I put both implementations into the online C++ assembler on Godbolt to see what was going on in the assembly version. A whole extra ten instructions were cluttering up my std::max one-liner. Instructions... for std::max itself. An extra function which I called explicitly.

Now, this seems obvious in hindsight, but at the time of writing, I was so pumped on the high of development optimization that I forgot the other, very real dimension to optimization: time. You know, that little thing. It’s very easy as humans to make the connection between fewer lines of code to shorter runtimes. It makes intuitive sense. At a grocery store, you take the line with the fewest number of people in the belief that fewer people means less checkout time overall. But even that isn’t entirely true, is it? You can have one cashier faster than the other, customers with more items, technical difficulties, the pilgrims insisting to pay with cash, all manners of things complicate the relationship between line length and checkout time. But it’s often good enough that we make these calls every day because they work most of the time. Computers aren’t the same. At least not that straightforward. You have to take into account that most compiler optimization development goes to speeding up the majority of code. And most people don’t jump to the standard library when there’s a perfectly good implementation immediately obvious. Case in point, my first, unenlightened attempt took only 8%, placing me well within my vanity bracket on Leetcode. 

I guess the moral of this post is functions have overhead, a lot more overhead than extra keystrokes. Clever costs. 
