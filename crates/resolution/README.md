# Resolution

This library contains traits and types which represent periods in discrete time.


## Zero copy etc
- Types are all just a wrapper around i32
    - Means they all meet the criteria that "all bit patterns are valid for the type"
    - _However_ just because the type can be constructed, doesn't mean all methods are infallible
    - Or should we go for another tradeoff -> types validate internally, and all methods are infallible?
    - Third option: all methods that _could_ fail return option/result