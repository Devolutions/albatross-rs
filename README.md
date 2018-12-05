
## Installing albatross

Download, compile and install albatross:

```
git clone https://github.com/devolutions/albatross-rs.git
cd albatross-rs
cargo install
```

Add required environment variables to ~/.bashrc (Linux) or ~/.profile (macOS):

```
export ALBATROSS_HOME=$HOME/.albatross
export PATH=$ALBATROSS_HOME/toolchain/llvm/bin:$PATH
```

## Installing toolchain

Download and install clang+llvm 7.0.0 from the official website:

```
albatross toolchain install
```

## Installing sysroots

Build and install sysroots (requires docker):

```
cd albatross-rs
albatross sysroot ubuntu-14.04-i386
albatross sysroot ubuntu-14.04-amd64
```

## CMake cross-compilation:

```
mkdir build-i386 && cd build-i386
cmake -DCMAKE_TOOLCHAIN_FILE=$ALBATROSS_HOME/cmake/ubuntu-14.04-i386.cmake ..

mkdir build-amd64 && cd build-amd64
cmake -DCMAKE_TOOLCHAIN_FILE=$ALBATROSS_HOME/cmake/ubuntu-14.04-amd64.cmake ..
```

## Common issues

### missing glib-compile-resources

When cross-compiling Linux projects using glib from a macOS host, you may need to install the glib resource compiler using `brew install glib`.
