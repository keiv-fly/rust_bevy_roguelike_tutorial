# Setting Up

## Prior knowledge

This tutorial assumes that a reader is familiar with Rust, but of course you can try to read it without knowing Rust.
## Installing Rust

In this tutorial I will be installing the latest Rust version and also Rust nightly build for faster development on Windows. The nightly build allows to compile Bevy in a dynamic library once and then reuse it for all the following compilations.

[Download Rust from here.](https://www.rust-lang.org/tools/install) I recommend using a 64-bit version. For Windows the button has a title "DOWNLOAD RUSTUP-INIT.EXE (64-BIT)". I used the default settings in the installation.

## Creating a project
I created a new project by executing the following command:
```
cargo new bevy_roguelike_tutorial
```

and go inside this folder:
```
cd bevy_roguelike_tutorial
```
This created the following folder structure:
```
bevy_roguelike_tutorial
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    └── ...
```

With the main code in `src/main.rs`
```rust
fn main() {
    println!("Hello, world!");
}
```

Let's compile the code and see that this Rust program is working.

In the command line execute:
```
cargo run
```
And you should have the following output:
```
   Compiling bevy_roguelike_tutorial v0.1.0 (C:\...\bevy_roguelike_tutorial)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
     Running `target\debug\bevy_roguelike_tutorial.exe`
Hello, world!
```

## Install Bevy

Here I am setting setting up to use Rust nightly build and use Bevy as a dynamic library.

To install Rust nightly build execute the following command:
```
rustup install nightly
```

Create a new file `bevy_roguelike_tutorial/rust-toolchain` with the following content:
```
nightly
```
This step is specifically for Windows users. On Linux the build should work without it:

Create a directory `bevy_roguelike_tutorial/.cargo` and put a file with name `config.toml` with the following content:
```
[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=off"]
```

A new line should be added to `Cargo.toml` after `[dependencies]`:
```
[dependencies]
bevy = { version = "0.7.0", features = ["dynamic"] }
```

After that run
```
cargo run
```

After several minutes of compiling you should see the following output:
```
    Finished dev [unoptimized + debuginfo] target(s) in 4m 14s
     Running `target\debug\bevy_roguelike_tutorial.exe`
Hello, world!
```

Everything is compiled and Bevy is added. Goal achieved!



