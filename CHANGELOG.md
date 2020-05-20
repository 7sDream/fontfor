# Changelog

## Unreleased

## 0.3.0

- Browser preview page gets a new wonderful UI (Thanks [@Tiierr](https://github.com/Tiierr), PR #5)

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
