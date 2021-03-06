[![Current version][crate-badge]][crates-io]
[![Current documentation][doc-badge]][docs]
[![Build status][build-badge]][builds]

# Shaku

Shaku is a Rust dependency injection library. See the [docs] for more details,
including a getting started guide.

## Example
```rust
use shaku::{Component, ContainerBuilder, Interface};
use std::sync::Arc;

trait IOutput: Interface {
    fn write(&self, content: String);
}

trait IDateWriter: Interface {
    fn write_date(&self);
}

#[derive(Component)]
#[shaku(interface = IOutput)]
struct ConsoleOutput;

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{}", content);
    }
}

#[derive(Component)]
#[shaku(interface = IDateWriter)]
struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn IOutput>,
    today: String,
    year: usize,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        self.output.write(format!("Today is {}, {}", self.today, self.year));
    }
}

fn main() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<ConsoleOutput>();
    builder
        .register_type::<TodayWriter>()
        .with_named_parameter("today", "Jan 26".to_string())
        .with_typed_parameter::<usize>(2020);
    let container = builder.build().unwrap();

    let writer: &dyn IDateWriter = container.resolve_ref().unwrap();
    writer.write_date();
}
```

## Minimum Supported Rust Version
Shaku currently supports the latest stable release of Rust, plus the previous
version. This range will probably increase to the previous two versions by the
time Shaku reaches 1.0.

Latest stable version: 1.41.0

Minimum supported version: 1.40.0

## Acknowledgements
This library started off as "he_di" (later renamed to "shaku") under the
guidance of [@bgbahoue] and [@U007D]. Their work inspired the current maintainer
([@Mcat12]) to continue the library from where they left off.

[crates-io]: https://crates.io/crates/shaku
[docs]: https://docs.rs/shaku
[builds]: https://circleci.com/gh/Mcat12/shaku
[crate-badge]: https://img.shields.io/crates/v/shaku.svg
[doc-badge]: https://docs.rs/shaku/badge.svg
[build-badge]: https://circleci.com/gh/Mcat12/shaku.svg?style=shield
[@bgbahoue]: https://github.com/bgbahoue
[@U007D]: https://github.com/U007D
[@Mcat12]: https://github.com/Mcat12
