### Overview

A toy project demonstrating an approach to solve the following problem:

Implement basic function to split some generic computational work between threads.
Split should only occur on some threshold: if the amount of computational work (input length) is below this threshold then no split takes place and no threads are created.

Input is considered to be:
- ```Vec<T>```
- Function ```f(t: T) -> R```

Output:
- ```Vec<R>```

### Running tests

```cargo test -- --nocapture```
