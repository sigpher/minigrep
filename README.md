# Chapter 1:Getting Started

## 1.1.Installation

run "rustup doc" to open documentation in your browser.

```bash
rustup doc
```

## 1.2.Hello,World!

```bash
mkdir ~/projects

cd ~/projects

mkdir hello_world

cd hello_world
```

Filename: main.rs

```rust
fn main(){
	println!("Hello, world!");
}
```

### 1.3.Hello, Cargo

Let’s recap what we’ve learned so far about Cargo:

* We can create a project using `cargo new`.
* We can build a project using `cargo build`.
* We can build and run a project in one step using `cargo run`.
* We can build a project without producing a binary to check for errors using `cargo check`.
* Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.

# Chapter 2:Progamming a Guessing Game

First we add the line `use rand::Rng;`. The `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Chapter 10 will cover traits in detail.

Another neat feature of Cargo is that running the `cargo doc --open` command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the `rand` crate, for example, run `cargo doc --open` and click `rand` in the sidebar on the left.
