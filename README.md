### Environment

Let's say relatively recent nightly toolchain.
OS is not important, this should work on any major one.

### Project structure

```
src/                    <-- crate "tbd"
  lib.rs                <-- lib compilation unit "tbd"
  bin/
    hw.rs               <-- bin compilation unit "hw"
scratches/
  username/             <-- crate "username"
    src/
      lib.rs            <-- lib compilation unit "username"
      bin/
        my_script.rs    <-- bin compilation unit "my_script"
```

Crate "tbd" (whatever is in `src/`) is what we used to call "production".
Things defined in bin compilation units can't be used elsewhere.
So all reusable type definitions and subroutines go to lib compilation unit "tbd" (obsiously it doesn't have to be a single `lib.rs` file [link](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files)).

Each crate has exactly one lib compilation unit and any number of bin compilation units.

Compilation units inside crate "tbd" can use external dependencies declared in `Cargo.toml`. Bin compilation units inside crate "tbd" also can use lib compilation unit "tbd".

Compilation units inside crate "username" can use external dependencies declared in `scratches/username/Cargo.toml`. Crate "username" also declares "tbd" as an external dependency. This means it can use lib compilation unit "tbd".

<!--
To update, run
    dot compilation_untis.dot -Tpng -o compilation_units.png
-->
![mess](compilation_units.png)

### How to run

```

cargo test          # run all tests in crate "tbd"
cargo test --all    # run all tests in all crates
cargo run --bin hw  # run hw.rs in crate "tbd"
cargo run -p username --bin my_script   # run myscript.rs in crate "username"

```
