# Atmpt

Pronounced _attempt_, Atmpt is a simple CLI program which lets you quickly
create a temporary project and open it in your chosen `$EDITOR` environment
variable.

This makes testing some quick lines of code easy without having to open a new
replit or creating a new project with boilerplate yourself.

## Running

After installing, you may run

```bash
atmpt <template>
```

where `<template>` is the name of the folder in atmpt's [data directory] which
you would like to clone. For example, if you have a `cpp` template for C++, you
would run

```bash
atmpt cpp
```

Afterwards, whatever you have set as your `$EDITOR`
environment variable will be run in the new directory.

## Installing Manually

Ensure you have [Rust] installed. After cloning this repo, `cd` into it and
compile with:

```bash
cargo build --release
```

Then you may either add the newly created `target/release` directory to your
PATH or, for Linux, you could move the resulting binaries to be used anywhere:

```bash
mv target/release/atmpt /usr/local/bin/
```

Finally, you can create any templates you would like to use in atmpt's
[data directory]. To get started with the default ones in this repository you
may copy them (example on Linux):

```bash
mkdir -p ~/.local/share/atmpt/
cp -r templates/* ~/.local/share/atmpt/
```

## License

Atmpt is licensed under the GPLv3, a free and open source license. For more
information, please read the [LICENSE] file in this repositories' root
directory.

[data directory]: https://docs.rs/directories-next/latest/directories_next/struct.ProjectDirs.html#method.data_dir
[Rust]: https://www.rust-lang.org
[LICENSE]: https://github.com/marcelohdez/Atmpt/blob/master/LICENSE
