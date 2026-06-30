<div align="center">
    <h1>cproj</h1>

C/C++ CMake project bootstrapping tool written in Rust (cuz why not)
</div>

Exactly what it sounds like. You run it in the terminal, it asks you a couple of questions and sets up a new CMake project for you, complete with `git init`, `.clang-format`, and a fairly customized `CMakeLists.txt`.

As of now, you can provide:

- `folder name`: name of the folder where the project will be created in.
- `project name`: name used in the CMake `project()` command, defaults to `folder name`.
- `project version`: version used in the CMake `project()` command, defaults to `1.0.0`.
- `project languages`: either C, CXX, or both. Used to configure other options and they're automatically given to the CMake `project()` command.
- `c standard`: integer value for the C standard to be used in the project. Only requested if C is among `project languages`. Used to set up `CMAKE_C_STANDARD`.
- `cxx standard`: integer value for the CXX standard to be used in the project. Only requested if CXX is among `project languages`. Used to set up `CMAKE_CXX_STANDARD`.
- `main file`: main source file created inside `src/`. Will provide a hello world snippet for C and C++.
- `CMake version`: used in the CMake `cmake_minimum_required()` command.
- `"use Mika's code formatting style?"`: Whether to include the `.clang-format` that I personally use. It's based in Mozilla's style. Check it out [here](template/.clang-format).
- `"create a cmake target?"` Whether to automatically add a target to the generated `CMakeLists.txt`. If you say yes, then you'll be asked:
    - `target name`: name of the target.
    - `target type`: executable for `add_executable()`, static for `add_library(... STATIC)`, you get the idea. `executable, static, shared, and interface` are supported. Extra boilerplate will be generated for you if available, namely `set_target_properties()` and a few `target_compile_options()`.
- `"run git init?"`: Whether to run `git init` in the project's folder. If you say yes, a `.gitignore` will also be generated for you.

And that's all.

# How to Use

It's in [crates.io](https://crates.io/crates/cpoj), so you can install it through `cargo`:

```bash
cargo install cproj
```

or clone the repo and install it from there:

```bash
cargo install --path wherever/you/cloned/this/thing
```

There's a silly feature that just enables some silly messages as a response when you say no to a particular question. Enable it by passing `--features silly_stuff`

Whenever you want to use it, just run it in your terminal. No flags, no nothing, just `cproj`.

# Building
... It's a rust crate.
```bash
cargo build
```
why did I even write this section

# License

[MIT License](LICENSE).
