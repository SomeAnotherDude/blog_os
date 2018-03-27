# What Is this?

This is my implementation of
[this step-by-step guide](https://os.phil-opp.com/second-edition/)
to write an OS in [Rust](https://www.rust-lang.org/).
Mostly it's a self-education project, which involves a bit (a lot)
of copy-paste with my useful (mirror) changes.
Therefore you probably are about to move along.

**NOTE**: All the scripts have been written an tested for Linux
(more specifically Kubuntu 17.10, should be OK with any Debian-based OS).

# Quick How to

All the scripts support a `--release` flag.
This flag builds a project in release mode but preserves debugging symbols.

## Dependencies

### Rust compiler
The easiest way is to follow instructions on
[official site](https://www.rust-lang.org/en-US/install.html).
It offers you to run an [install bash script](https://sh.rustup.rs):

---

**NOTE**: The script requires a `curl` command
([this is about to change](https://github.com/rust-lang-nursery/rustup.rs/pull/1373))
which is not installed on Ubuntu by default so you have to install it first.


```bash
$ sudo apt install curl -y
```
---

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

This command will install a [`rustup`](https://github.com/rust-lang-nursery/rustup.rs/) tool.
We need a nightly channel:

```bash
$ rustup update nightly
```

and `rust-src`

```bash
$ rustup component add rust-src
```

### Xargo

(This step will be [omitted in future](https://github.com/japaric/xargo/issues/193)).

```bash
$ cargo install xargo
```

### Bootimage
[`bootimage`](https://github.com/rust-osdev/bootimage) is a
[bootloader](https://en.wikipedia.org/wiki/Booting#BOOT-LOADER) for my os.
Actually, it's a tool which makes a hard-drive image with OS image attached
back to [this](https://github.com/rust-osdev/bootloader) bootloader.
When the machine starts (all the rest is very much simplified,
just too lazy to explain, future me, I'm so sorry...)
it runs this bootloader which initializes hardware, finds a `_start` symbol and  jumps to it.

```bash
$ cargo install bootimage
```

### Qemu
Technically nothing forces me to use `qemu`, `virtualbox` should be OK,
but all the scripts use `qemu-x86_64` so

```bash
sudo apt install qemu-x86 -y
```

## How to build

```bash
$ ./build [--release]
```

## How to run
```bash
$ ./run [--release]
```

## How to run under debugger
```bash
$ ./debug [--release]
```
