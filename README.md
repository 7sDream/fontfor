# FontFor

![license-badge] ![version-badge]

Find fonts which can show a specified character and preview them in browser.

This is port from my early python script called [which_fonts_support][which_fonts_support-github], but with some improvements:

- Write in Rust, safe and faster
- Use `fontconfig` library instead of depends on `fc-list` command installed
- Support preview in shell, no GUI or browser needed (in plan)

## Install

```bash
cargo install fontfor
```

## Usage

### Basic

```bash
$ fontfor א
Font(s) support the character "א"(U+05D0, 1488, 0xD7 0x90):
Arial                       with 4 styles
Arial Hebrew                with 3 styles
Arial Hebrew Desk Interface with 3 styles
Arial Hebrew Scholar        with 3 styles
Arial Unicode MS            with 1 style
Corsiva Hebrew              with 2 styles
Courier New                 with 4 styles
DejaVu Sans                 with 8 styles
FreeMono                    with 4 styles
FreeSans                    with 4 styles
FreeSerif                   with 4 styles
HanaMinA                    with 1 style
LastResort                  with 1 style
Lucida Grande               with 2 styles
Microsoft Sans Serif        with 1 style
New Peninim MT              with 4 styles
Raanana                     with 2 styles
TW-Kai                      with 1 style
TW-Sung                     with 1 style
Tahoma                      with 2 styles
Times New Roman             with 4 styles
```

### Character input format

You can use the following formats for the character:

- Character: `א`
- Unicode scalar value
  + `U+XXXXXX`: `U+5d0`, `U+05d0`, `U+0005d0`
  + Direct input
    * Dec format: `1488`
    * Oct format: `0o2720`
    * Binary format: `0b010111010000`
- UTF8 bytes: `0xd790`

### Preview

Add `-p` flag to enable browser preview:

![preview]

## LICENSE

GPLv3 or later.

See [COPYING][COPYING-file].

[license-badge]: https://img.shields.io/crates/l/fontfor?style=flat-square
[version-badge]: https://img.shields.io/crates/v/fontfor?style=flat-square
[which_fonts_support-github]: https://github.com/7sDream/which_fonts_support
[preview]: https://rikka.7sdre.am/files/81994541-9e44-4e96-827f-ddc960c03b26.png
[COPYING-file]: https://github.com/7sDream/fontfor/blob/master/COPYING
