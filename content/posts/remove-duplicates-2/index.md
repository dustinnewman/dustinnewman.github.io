---
title: "Remove Duplicates 2"
date: 2024-06-17T22:42:50-07:00
draft: false
---

I have recently been brushing up on some interview prep with LeetCode. For one of the problems, despite only being a "medium" difficulty, I came up with an interesting solution that pulled from my experience developing cellular firmware that I would like to share. It is very common when dealing with frequency ranges for radio frequency (RF) bands and that these ranges are organized in tables. It is also common for special rules or regulations to apply when dealing with the "edge" ranges of these bands. For example, if we have band 1, it might have a low and high edge that has a lower or higher maximum transmissible power.

| Band | Frequency | Edge? |
|------|-----------|-------|
| 1    | 10        | Yes   |
| 1    | 11        | No    |
| 1    | 12        | No    |
| 1    | 13        | Yes   |
| 2    | 20        | No    |
| 3    | 24        | No    |

A common way to identify these edge regions is to compare the current band with the previous and next bands. If both the previous and next bands are the same as the current, then we are in an edge region. Else, we aren't. Then you can add some special handling for the first and last regions in the table, etc.

Now, the LeetCode problem is to remove duplicates from an array of integers, but allow up to two duplicates for any given value. So if you had the array `[1,1,1,2,2,3]`, the output would be `[1,1,2,2,3]`. On the face of it, this has nothing to do with the frequency range concept I just described. However, I have always visualized the edge region filtering as a sort of "catching the wave." Basically, if we turn our table into an array of only the band numbers, we can denote the start of the band with a forward slash / and the end of the band with a backwards slash. Non-edge regions will be denoted with a dash -.

```
 / - - \ / /
[1,1,1,1,2,3]
```

Note the slight difference from the table earlier. Here, when we first see a new band number, we somewhat optimistically label it the "rising edge" of the band. 

My key insight was realizing that removing the duplicates with an allowance of two actually reduced to "catching the wave." We want to take only the rising and falling edges of the frequency bands. This allowed me to apply both my RF knowledge and the corresponding embedded programming knowledge (where every byte counts) to come up with a quite minimal C++ solution.

```cpp
int removeDuplicates(vector<int>& nums) {
    int k = 1;
    for (size_t i = 1; i < nums.size(); i++) {
        if (nums[i] != nums[i - 1] || i == nums.size() - 1 || nums[i] != nums[i + 1]) {
            nums[k++] = nums[i];
        }
    }
    return k;
}
```

where `k` denotes the length of the de-duplicated array. (Don't judge me for using a signed integer for the return value. LeetCode forced the interface on me.) We initialize it to 1 to reflect the fact that every non-empty array (LeetCode guaranteed the arrays would be non-empty) will contain at least 1 element. We then iterate through the array and "catch the waves": if the current element is not equal to the previous or next elements, then we have either the rising or falling edges. And then there is an additional check for the final element of the array which should always be included (either because it is the falling edge of a run or because it is unique).
