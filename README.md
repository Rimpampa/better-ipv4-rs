# Better IPv4

An attempt to improve the IPv4 Rust APIs

# History

This repo is kind of a meme that started in a conversation on the Rust Italia telegram group:
@giulio-Joshi was wondering if it could be possible to express IPv4 addresses using the dotted decimal syntax by leveraging the tuple field access syntax.

This led to me making this post on reddit titled “[IPv4 dotted decimal tuple of hell]”
which cought the attention of none other than @joshtriplett who in response made this
[tweet](https://twitter.com/josh_triplett/status/1505458500016640002?s=20&t=4EQ1Cxsh2VPeCfQS2uVKfA).

Josh made the implementation a lot better (compilable in reasonable times) by using const-generics.
I then noticed that his implementation was not limited to tuples,
so I made a follow up post on reddit titled “[IPv4 "dotted english numerals" struct of hell]”,
following the idea of a [comment] in the first post,
which applied the same mechanism to a struct containing fields named as english numerals.

In the end I made this repo with the result of all those novelty ways to express IPv4 addresses
and, who knows, maybe one day even IPv6 will get the same special treatment.

---

_I'm aware that this README came a bit late (the first post was made 7 months ago) and that's because I'm very lazy_

[IPv4 dotted decimal tuple of hell]: https://www.reddit.com/r/rust/comments/ti0qpi/ipv4_dotted_decimal_tuple_of_hell/?utm_source=share&utm_medium=web2x&context=3
[IPv4 "dotted english numerals" struct of hell]: https://www.reddit.com/r/rust/comments/tikqfl/ipv4_dotted_english_numerals_struct_of_hell/?utm_source=share&utm_medium=web2x&context=3
[comment]: https://www.reddit.com/r/rust/comments/ti0qpi/comment/i1b9e1d/?utm_source=reddit&utm_medium=web2x&context=3
