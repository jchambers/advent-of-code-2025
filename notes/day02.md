| Value   | Digits | ilog10 | left_min |
|---------|--------|--------|----------|
| 11      | 2      | 1      | 1        |
| 998     | 3      | 2      | 10       |
| 222220  | 6      | 5      | 222      |
| 1698522 | 7      | 6      | 1000     |

For odd-length values, seems like we can do something smarter. 998-1012 is an interesting case because it DOES contain an invalid ID at 1010. For longer ranges, rounding down to a previous digit could be really inefficient since we'd need to crawl up by 10^N to hit the first candidate digit.

Can just jump to the next power of 10, though!
