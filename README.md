# atmpt

Pronounced _attempt_, atmpt is a simple CLI program which lets you quickly
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
you would like to clone and open with the binary set as your `$VISUAL`
environment variable.

After your editor exits you will be asked if you would like to keep or delete
the attempt (by default they are stored in your system's [temporary directory],
which should get deleted in time if the folder is not moved manually). If you
decide to keep it, you can reopen the attempt with `--previous`:

```bash
atmpt -p
```

Whether the attempt was saved or not, you can start a new attempt with the same
template with `--retry`:

```bash
atmpt -r
```

To view all options please see the help page:

```bash
atmpt --help
```

## Installing

> [!IMPORTANT]
>
> - Ensure you have [Rust] installed.
> - Unless you bring your own templates, atmpt is useless without also
>   installing the default ones. See the [templates] section.

atmpt is available on [crates.io]:

```bash
cargo install atmpt
```

_Or_ you can build manually; After cloning this repo, `cd` into it and if you
would like cargo to install into its default directory run:

```bash
cargo install --path .
```

Otherwise, to place in your `$PATH` yourself, you can build with:

```bash
cargo build -r
```

With the resulting binary being `./target/release/atmpt` (or `atmpt.exe` on
Windows).

### Templates

atmpt uses any folders in your [data directory] as templates to clone. You may
install the default ones (under `/templates`) by running the following commands
after cloning and `cd`ing into this repo _(example given for MacOS/Linux)_:

```bash
mkdir -p $(atmpt -d)
cp -r templates/* $(atmpt -d)
```

## Data Directory

This is where your templates will be stored but is different depending on the
system. atmpt offers the `--list-template-dir` option to print it out on your
system (you may have used it while setting up your [templates]):

```bash
atmpt -d
```

In the printed directory you may put folders for atmpt to clone as temporary
projects when run with their name as input.

## License

atmpt is licensed under the GPLv3, a free and open source license. For more
information, please read the [LICENSE] file in this repositories' root
directory.

[installing]: https://github.com/marcelohdez/atmpt/#installing
[data directory]: https://github.com/marcelohdez/atmpt/#data-directory
[Rust]: https://www.rust-lang.org
[temporary directory]: https://en.wikipedia.org/wiki/Temporary_folder
[crates.io]: https://crates.io/crates/atmpt
[templates]: https://github.com/marcelohdez/atmpt/#templates
[LICENSE]: https://github.com/marcelohdez/atmpt/blob/master/LICENSE
