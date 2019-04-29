# async-native-timer

Async/await timers with native implementations.

```rust
use async_native_timer::Delay;
use std::time::Duration;

async fn main() {
  let timer = Delay::new(Duration::from_secs(5));

  await!(timer);
  println!("Hello World!");
}
```
