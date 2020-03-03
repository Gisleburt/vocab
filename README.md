# Vocab

A command line application for learning vocabulary in new languages

## Usage:

### Installation

```shell
cargo install vocab
```

### Initialising:

`init` will create a bew vocab.sqlite file in the current directory. We recommend you use
directories to label which language you're learning.

Example:

```shell
mkdir japanese
cd japanese
vocab init
```

This will help you separate and organise multiple languages.

### Adding new words

`add <local> <foreign>` will let you add a new word as you learn it. `<local>` should be the
word in your own language, `<foreign>` should be the word in the language you are learning.

Example:

```shell
vocab add japan 日本
```

### Try a single word

You can try guessing a single word at a time

```shell
vocab single
```

### Endless Mode

When you run the program with no other arguments it will enter endless mode (use ctrl+c to quit)

```shell
vocab
```

### Export the database

You can export the database to csv file, either by naming it or via stdout

```shell
vocab export -f my_japanese_backup.csv
```
or
```shell
vocab export > my_japanese_backup.csv
```

### Import your backup

You can import you old csv file in much the same way

```shell
vocab import -f my_japanese_backup.csv
```
or
```shell
cat my_japanese_backup.csv | vocab export
```

If the database already contains the vocabulary in the csv file it will attempt to reconcile
the differences, choosing whichever set has more guesses against it.
