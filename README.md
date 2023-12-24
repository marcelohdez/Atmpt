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

## Building

_(Currently the only installation method)_

Ensure you have [Rust] installed. After cloning this repo, `cd` into it and
install by running:

```bash
cargo install --path .
```

This should place the `atmpt` binary in `$HOME/.cargo/bin`, but you will not
have any templates to use. You may either create them in atmpt's
[data directory] _(along with the directory itself if it does not exist
already)_, or you could use the default ones included in this repo _(example
given for Unix-based systems like Linux and MacOS)_:

```bash
mkdir -p $(atmpt --list-template-dir)
cp -r templates/* $(atmpt --list-template-dir)
```

Additionally, autocompletion files for several shells are created in a new
`completions` directory. These should be placed in your respective shell's
autocompletion directory, for example, `zsh` reads completion files from any
directory in your `$fpath` environment variable.

## Data Directory

Many times above you may have seen talk about a _data directory_, this is
where your templates should be stored, but is dependent on the OS. Therefore,
Atmpt offers an option to print it out on your system (you may have seen its
output be used in the [installing] section):

```bash
atmpt --list-template-dir
```

_(The option could be shortened to `-d`)_

In the printed directory you may put folders for atmpt to clone as temporary
projects when run with their name as input.

## License

Atmpt is licensed under the GPLv3, a free and open source license. For more
information, please read the [LICENSE] file in this repositories' root
directory.

[installing]: https://github.com/marcelohdez/Atmpt/#building
[data directory]: https://github.com/marcelohdez/Atmpt/#data-directory
[Rust]: https://www.rust-lang.org
[LICENSE]: https://github.com/marcelohdez/Atmpt/blob/master/LICENSE
