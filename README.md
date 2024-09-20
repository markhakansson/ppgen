# ppgen

A simple passphrase generator.

## Installation

```sh
cd ppgen/
cargo install --path .
```

## Usage

Generate a 5 word long random passphrase with wordlist:
```sh
ppgen -c 5 -w /usr/share/dict/words
```

Generate a 5 word long random passphrase with multiple wordlists:
```sh
ppgen -c 5 -w path/to/wordlists/*
```

### Wordlists

By default, the default wordlist on the system is used. To increase the passphrase entropy, it's
recommended to use other or additional wordlists.

There are many good wordlists for diceware passphrases that can be used https://theworld.com/~reinhold/diceware.html .

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.
