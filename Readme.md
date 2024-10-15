# rust uinput

Goal is to automate tests using of [uinput](https://kernel.org/doc/html/v4.19/input/uinput.html) module

* Generate yaml file with test cases and run this one with it

## Building

```rust
cargo build
```

## Usage

* Write own scenario file. As example see [Key_Events](./data/key_events.yaml) or [Key_Events](./data/mouse_events.yaml)
* Set enough rights for writing into /dev/uinput. For example with:

```shell
sudo chmod 777 /dev/uinput
```

* Start with cargo run or directly giving in command line the test scenario fil
* Link on youtube: [youtube](https://youtu.be/uz3DGyu6lOc)
