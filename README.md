# sync_select

A short-circuiting (verbose) `std::thread::scope`.

## Example

```rust
use std::{thread::sleep, time::Duration};

use sync_select::*;

fn main() {
    let s = SyncSelect::new();

    // Task A (3rd to finish)
    s.spawn(|| sleep(Duration::from_secs(3)));

    // Task B (1st to finish => Subtask 1)
    s.spawn_with(|s| {
        // Task C (subtask of B) (1st to finish)
        s.spawn(|| sleep(Duration::from_secs(1)));

        // Task D (subtask of B) (2nd to finish)
        s.spawn(|| sleep(Duration::from_secs(2)));
    });

    // In this specific scenario, Task B is first to die,
    // because it short-circuits due to one of it's subtasks
    // finishing. This causes the root to short-circuit,
    // resulting in the program exiting.

    // Note: scopes automatically short-circuit when dropped.
}
```

## Notes
- There is no "borrow non-'static data" functionality.

## Todo
- Better ergonomics
    - `scope(...)` function or something.
    - Maybe build a macro `select!` similar to tokio's [select!](https://docs.rs/tokio/1.41.1/tokio/macro.select.html) macro.