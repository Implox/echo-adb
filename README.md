## Command-line tool for echoing lines of text to an Android device using ADB.

### Installation

You can install this with cargo with the command

```
cargo install echo_adb
```

You can install cargo with

```
curl https://sh.rustup.rs -sSf | sh
```

### Usage

If you just want a specific string echoed:

```
$ echo-adb "Sample text"
```

Or you can use pipes:

```
$ "Sample text" | echo-adb
```
