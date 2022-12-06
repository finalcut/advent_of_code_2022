#Intro
Since I'm learning rust and I'm forgetful.  Some tips.

## start a new project (day)

```rust
cargo new dayx
```
where x is the day we're starting.

## run the program

```rust
cargo run
```

you can do a `cargo build` but that is clunkier for our purposes here. We're trying to build and execute quickly.  So `cargo run` works


## test the program

```rust
cargo test
```

need to add a block of code like this at the end of main.rs:

```rust#[cfg(test)]
mod tests {
  // super::* makes sure the methods in the non-test section are available to test
  use super::*;

    #[test]
    fn test_input_parsing_test() {


      let result0 = no_duplicates_exists(['a','b','c','d','a'].to_vec());
      assert_eq!(result0, false);

      let result1 = no_duplicates_exists(['a','b','c','d','e'].to_vec());
      assert_eq!(result1, true);
    }
}