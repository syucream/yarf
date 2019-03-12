# yarf; An yet another rust-fuse, focuses on just only libfuse highlevel API binding

[![Latest version](https://meritbadge.herokuapp.com/yarf)](https://crates.io/crates/yarf)

It's a [libfuse](https://github.com/libfuse/libfuse) binding in Rust. And it focuses on only libfuse highlevel API.


## Documentation

https://docs.rs/yarf/0.0.2/yarf/


## Usage

Require yarf crate in your `Cargo.toml`

```toml
[dependencies]
yarf = "0.0.2"
```

then you can write your filesystem as Rust struct on yarf, like below:

```rust
extern crate yarf;

...

use yarf::{FileSystem, FuseFileInfo, FuseFillDir};

...

struct MyFS;

impl FileSystem for MyFS {
    fn getattr(&self, path: String, stbuf: Option<&mut stat>) -> c_int {
        ...
    }

    ...
}

fn main() {
    let fs = Box::new(MyFS);
    yarf::yarf_main(fs);
}
```
