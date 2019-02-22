[![Build Status](https://travis-ci.org/danylaporte/typed_fixedbitset.svg?branch=master)](https://travis-ci.org/danylaporte/typed_fixedbitset)

A typed bitset container for Rust.

## Documentation
[API Documentation](https://danylaporte.github.io/typed_fixedbitset/typed_fixedbitset)

## Example

```rust
use typed_fixedbitset::TypedFixedBitSet;

fn main() {
    let set = vec![S(0), S(2)]
        .into_iter()
        .collect::<TypedFixedBitSet<_>>();
    assert!(set.contains(S(0)));
    assert!(!set.contains(S(1)));
    assert!(!set.is_empty());
}

struct S(u32);

impl From<S> for usize {
    fn from(s: S) -> Self {
        s.0 as usize
    }
}
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0) or the MIT license
[http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT), at your
option. This file may not be copied, modified, or distributed
except according to those terms.