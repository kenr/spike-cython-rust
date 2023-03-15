## Info

This is only tested on macOS, check the path in src/main.rs regarding the library path to use

Compile the python project with:

```
cd python
pip3 install cython
python3 setup.py build
```

You have to have a Rust compiler installed. See https://www.rust-lang.org/tools/install for more information.

Compile and run the rust project with:

```
cargo run
```
