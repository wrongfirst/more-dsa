You are given two integer arrays `nums1` and `nums2` of size `m` and `n` respectively, where each is sorted in ascending order. Return the [median](https://en.wikipedia.org/wiki/Median) value among all elements of the two arrays.

Your solution should run in $O(log (m+n))$ time.

<br>

**Example 1:**

```java
Input: nums1 = [1,2], nums2 = [3]

Output: 2.0
```

Explanation: Among `[1, 2, 3]` the median is `2`.

<br>

**Example 2:**

```java
Input: nums1 = [1,3], nums2 = [2,4]

Output: 2.5
```

Explanation: Among `[1, 2, 3, 4]` the median is `(2 + 3) / 2 = 2.5`.

<br>

**Constraints:**
* `nums1.length == m`
* `nums2.length == n`
* `0 <= m <= 1000`
* `0 <= n <= 1000`
* `1 <= m + n <= 2000`
* `-10^6 <= nums1[i], nums2[i] <= 10^6`


<br>
