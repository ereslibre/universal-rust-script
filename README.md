# A universal Rust script

- Install [nix](https://nixos.org/download/)
- Run

  ```
  ❯ ./greet.rs
  error: the following required arguments were not provided:
    --name <NAME>

  Usage: greet.rs --name <NAME>

  For more information, try '--help'.
  ```


  ````
  ❯ echo "Hi friend" | ./greet.rs --name Ferris
  Waiting for a letter in stdin...
  Hello Ferris! Here's a letter for you:
  ```
  Hi friend
  ```
  ````

That's it!
