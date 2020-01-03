# FontFor

Find fonts which can show a specified character and preview them in browser.

## Usage

### Basic

```bash
$ fontfor א
Fonts support the character א [U+05D0, 1488, 0xd790]: 
.Arial Hebrew Desk Interface with 3 styles
.LastResort                  with 1 style
Arial                        with 4 styles
Arial Hebrew                 with 3 styles
Arial Hebrew Scholar         with 3 styles
Arial Unicode MS             with 1 style
Corsiva Hebrew               with 2 styles
Courier New                  with 4 styles
DejaVu Sans                  with 8 styles
FreeMono                     with 4 styles
FreeSans                     with 4 styles
FreeSerif                    with 4 styles
HanaMinA                     with 1 style
Lucida Grande                with 2 styles
Microsoft Sans Serif         with 1 style
New Peninim MT               with 4 styles
Raanana                      with 2 styles
TW-Kai                       with 1 style
TW-Sung                      with 1 style
Tahoma                       with 2 styles
Times New Roman              with 4 styles
```

### Character input format

You can use the following formats for the character:

- Character: `א`
- Unicode scalar value
  - `U+XXXXXX`: `U+5d0`, `U+05d0`, `U+0005d0`
  - Direct input
    - dec format: `1488`
    - oct format: `0o2720`
    - binary format: `0b010111010000`
- UTF8 bytes: `0xd790`

### Preview

Add `-p` option to enable preview:

![preview]

## Install

```bash
cargo install --force --git https://git.7sdre.am/7sDream/fontfor-rs.git
```

## LICENSE

GPLv3 or later.

See [COPYING][COPYING-file].

[preview]: https://rikka.7sdre.am/files/81994541-9e44-4e96-827f-ddc960c03b26.png
[COPYING-file]: https://git.7sdre.am/7sDream/fontfor-rs/src/branch/master/COPYING
