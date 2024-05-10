# Changelog

## Unreleased

## 0.4.3

- Add `-f` option to filter font list by family name, and a search box in TUI mode to change the filter word (Issue [#64](https://github.com/7sDream/fontfor/issues/64), PR [#71](https://github.com/7sDream/fontfor/pull/71))
- Remove `#[deny(warnings)]` in source code, add it in CI (Issue [#70](https://github.com/7sDream/fontfor/issues/70))
- Fix build for upcomming Rust 1.79 new lints
- Update deps

## 0.4.2

- Fix build for Rust 1.75. (by PR [#69](https://github.com/7sDream/fontfor/pull/69), thanks [@mfrischknecht].)
- Update deps

## 0.4.1

- Use family + subfamily as a fallback of missing full name, instead of postscript name (Issue [#67](https://github.com/7sDream/fontfor/issues/67), fixed by PR [#68](https://github.com/7sDream/fontfor/pull/68))
- Add musl libc build for Linux release, static link to crt for Windows release
- Max release profile performance, reduce binary size to around 1M

## 0.4.0

- Remove dependency of fontconfig and freetype lib
- Add `-vv` option to show font file location and face index
- ASCII mode render result now not narrow (Issue [#61](https://github.com/7sDream/fontfor/issues/61), fixed by PR [#63](https://github.com/7sDream/fontfor/pull/63))
- Support custom font paths `-I/--include <PATH>` (Issue [#62](https://github.com/7sDream/fontfor/issues/62), fixed by PR [#65](https://github.com/7sDream/fontfor/pull/65)), and load system fonts can be skipped by `--no-system`.
- Now release contains ia32/x64/arm64 binary for Windows
- Now release contains x64/arm64 binary for macOS
- Now release contains x64/arm64/armhf binary Linux

## 0.3.1

- Display help message directly when `char` arg are missing (Issue [#11](https://github.com/7sDream/fontfor/issue/11), Pr [#12](https://github.com/7sDream/fontfor/pull/12))

## 0.3.0

- Browser preview page gets a new wonderful UI (Thanks [@Tiierr](https://github.com/Tiierr), PR [#5](https://github.com/7sDream/fontfor/pull/5))

## 0.2.2

- Thanks to tui's upgrade, mono render don't need do a huge bitmap copy to render char, this will improve some pref
- Upgrade deps to latest version

## 0.2.1

- Fix tui help text typo

## 0.2.0

- Add TUI Mode for preview font directly in Terminal

## 0.1.3

- Remove dependency of `tokio` and `hyper`. Because I only need a simple static single page HTTP server, use them is too heavy for me
- Add a simple signal thread HTTP server for preview in browser
- Update `once_cell` to `1.3.1`

## 0.1.2

- Remove prefix dot(`.`) in output font name

## 0.1.1

- Remove dependency of `unicode-width` due to only output font family name in english
- Adjust character output format

## 0.1.0

- Init release, has same features as [which_fonts_support][which_fonts_support-github] script

[which_fonts_support-github]: https://github.com/7sDream/which_fonts_support
[@mfrischknecht]: https://github.com/mfrischknecht
