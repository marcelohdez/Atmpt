# Atmpt

Pronounced _attempt_, Atmpt is a simple CLI program which lets you quickly
create a temporary project and open it in your chosen `$EDITOR` environment
variable.

This makes testing some quick lines of code easy without having to open a new
replit or creating a new project with boilerplate yourself.

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

Afterwards, whatever you have set as your `$EDITOR`
environment variable will be run in the new directory.

## Installing Manually

Ensure you have [Rust] installed. After cloning this repo, `cd` into it and
compile with:

```bash
cargo build --release
```

Then you may either add the newly created `target/release` directory to your
PATH or, **for Linux**, you could move the resulting binaries to be used
anywhere:

```bash
mv target/release/atmpt /usr/local/bin/
```

Finally, you can create any templates you would like to use in atmpt's
[data directory]. To get started with the default ones in this repository you
may copy them:

```bash
mkdir -p $(atmpt --template-dir)
cp -r templates/* $(atmpt --template-dir)
```

## Data Directory

Many times above you may have seen talk about a _data directory_, this is
where your templates should be stored, but is dependent on the OS. Therefore,
Atmpt offers an option to print it out on your system (you may have seen its
output be used in the [installing] section):

```bash
atmpt --template-dir
```

In the printed directory you may put folders for atmpt to clone as temporary
projects when run with their name as input.

## License

Atmpt is licensed under the GPLv3, a free and open source license. For more
information, please read the [LICENSE] file in this repositories' root
directory.

[installing]: https://github.com/marcelohdez/Atmpt/#installing-manually
[data directory]: https://github.com/marcelohdez/Atmpt/#data-directory
[Rust]: https://www.rust-lang.org
[LICENSE]: https://github.com/marcelohdez/Atmpt/blob/master/LICENSE
