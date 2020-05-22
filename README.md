# FontFor

[![license-badge]][license-file] [![version-badge]][crates-io-page] [![ci-badge]][github-actions-page]

Find fonts which can show a specified character and preview them in browser.

This is port from my early python script called [which_fonts_support][which_fonts_support-github], but with some improvements:

- Write in Rust, safe and faster
- Use [`Fontconfig`][fontconfig-home] library instead of depends on `fc-list` command installed
- Support preview in shell (powered by [`FreeType`][free-type-home]), no browser needed

## Dependencies

This program needs `Fontconfig` and `FreeType` library to run, and some of their deps needs `cmake` to compile, so we need to install them before compile:

- macOS: `brew install cmake freetype fontconfig`
- Linux: Please refer to the docs of your Linux distribution to figure out how to install them
- Win: Do not support Windows for now

## Install or Update

```bash
cargo install -f fontfor
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

### Character Input Format

You can use the following formats for the character:

- Character: `א`
- Unicode scalar value
  - `U+XXXXXX`: `U+5d0`, `U+05d0`, `U+0005d0`
  - Direct input
    - Dec format: `1488`
    - Oct format: `0o2720`
    - Binary format: `0b010111010000`
- UTF8 bytes: `0xd790`

### Show All Font Styles

add `-v` flag to show all font style.

We don't show [screenshot picture][verbose-mode-screenshot] here because it's a bit long.

### Preview in Browser

Add `-p` flag to enable browser preview:

![browser-preview-screenshot]

### Preview in Shell

Add `-t` flag to enter tui mode for preview in shell:

![tui-mono-mode]

And you can change render mode to ASCII(10 Level):

![tui-ascii-10-mode]

ASCII(70 Level):

![tui-ascii-70-mode]

Or [`moon-render`][moon-render-github] mode:

![tui-moon-mode]

## LICENSE

GPLv3 or later.

See [COPYING][COPYING-file].

[license-badge]: https://img.shields.io/crates/l/fontfor?style=flat-square
[license-file]: https://github.com/7sDream/fontfor/blob/master/COPYING
[version-badge]: https://img.shields.io/crates/v/fontfor?style=flat-square
[crates-io-page]: https://crates.io/crates/fontfor
[ci-badge]: https://github.com/7sDream/fontfor/workflows/CI/badge.svg
[github-actions-page]: https://github.com/7sDream/fontfor/actions
[fontconfig-home]: https://www.freedesktop.org/wiki/Software/fontconfig/
[free-type-home]: https://www.freetype.org/
[which_fonts_support-github]: https://github.com/7sDream/which_fonts_support
[verbose-mode-screenshot]: https://rikka.7sdre.am/files/22ea7500-525b-47ba-9c4e-6ef963999983.png
[browser-preview-screenshot]: https://rikka.7sdre.am/files/8f27f97b-a9b5-4fac-b922-594d188f648c.png
[tui-mono-mode]: https://rikka.7sdre.am/files/c7a7a685-3966-4fb4-8e9e-0cc53636e406.png
[tui-ascii-10-mode]: https://rikka.7sdre.am/files/77d4b267-00e4-4ab2-abe3-dc0569769566.png
[tui-ascii-70-mode]: https://rikka.7sdre.am/files/59262531-7d4c-4228-be2f-3149c14c86d2.png
[tui-moon-mode]: https://rikka.7sdre.am/files/8128a291-27a7-42bd-813e-1136f971cdb3.png
[moon-render-github]: https://github.com/7sDream/moon-render
[COPYING-file]: https://github.com/7sDream/fontfor/blob/master/COPYING
