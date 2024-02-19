# atmpt

Pronounced _attempt_, Atmpt is a simple CLI program which lets you quickly
create a temporary project and open it in your chosen `$VISUAL` environment
variable.

This makes testing some quick lines of code easy without having to open a new
replit or creating a new project with boilerplate yourself.

## Showcase

_(`$VISUAL` variable set to `nvim` for Neovim)_

[![asciicast](https://asciinema.org/a/628728.svg)](https://asciinema.org/a/628728)

## Running

After [installing], you may run

```bash
atmpt <TEMPLATE>
```

where `<TEMPLATE>` is the name of the folder in atmpt's [data directory] which
you would like to clone. For example, if you have a `cpp` template for C++, you
would run

```bash
atmpt cpp
```

Afterwards, whatever you have set as your `$VISUAL`
environment variable will be run in the new directory.

## Installing

**Ensure you have [Rust] installed.**

You can install from `crates.io` using `cargo`:

```bash
cargo install atmpt
```

_Or_ you can build manually; After cloning this repo, `cd` into it and install by
running:

```bash
cargo install --path .
```

### Templates

Both of the methods above should place `atmpt` in `$HOME/.cargo/bin`, but you
will not have any templates. You may either create them in the [data directory],
or you could use the default ones included in this repo _(example given for
Unix-like systems e.g. Linux or MacOS)_:

```bash
mkdir -p $(atmpt -d)
cp -r templates/* $(atmpt -d)
```

Additionally, _autocompletion files_ for several shells are created in cargo's
`$OUTDIR` directory, which should have been printed out by cargo during
compilation, something like: `./target/release/build/atmpt-<hash>/out/`. These
should be placed in your respective shell's autocompletion directory, for
example, `zsh` reads completion files from any directory in your `$fpath`
environment variable.

## Data Directory

Many times above you may have seen talk about a _data directory_, this is
where your templates should be stored, but is dependent on the OS. Therefore,
Atmpt offers an option to print it out on your system (you may have seen its
output be used in the [installing] section):

```bash
atmpt --list-template-dir
```

_(This option can be shortened to `-d`, as seen in examples)_

In the printed directory you may put folders for atmpt to clone as temporary
projects when run with their name as input.

## License

Atmpt is licensed under the GPLv3, a free and open source license. For more
information, please read the [LICENSE] file in this repositories' root
directory.

[installing]: https://github.com/marcelohdez/Atmpt/#installing
[data directory]: https://github.com/marcelohdez/Atmpt/#data-directory
[Rust]: https://www.rust-lang.org
[LICENSE]: https://github.com/marcelohdez/Atmpt/blob/master/LICENSE
