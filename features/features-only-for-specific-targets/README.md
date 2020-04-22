### A crate should be only enable for Linux, and we want to control features of it

#### How to Test It

- Run follow commands in Linux:

  ```bash
  cd bar
  # compile should be ok, but 'feature-c' shouldn't be enabled
  cargo run
  # compile should be ok, but 'feature-c' should be enabled
  cargo run --features feature-c
  ```

- Run follow commands in Windows:

  ```bash
  cd bar
  # compile should be ok, no feature should be enabled
  cargo run
  # compile should be failed
  cargo run --features feature-c
  ```
