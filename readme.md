<img src="https://raw.githubusercontent.com/aryadaroui/Favy/main/docs/assets/favy-icon-small.webp" width="160" height="160" alt="Favy icon" align="right">

# Favy

A simple, cross-platform photo organizer.

Quickly go through your photos, rate them, and filter them in your filesystem.

![Favy screenshot](https://raw.githubusercontent.com/aryadaroui/Favy/main/docs/assets/screenshot.webp)

## Usage

*Currently useable, but not officially distributed yet!* You can check progress on the [kanban board](https://github.com/users/aryadaroui/projects/2/views/1).

To test development version:
```zsh
npm run tauri dev
```

To build:
```zsh
npm run tauri build # executable found in ./src-tauri/target/
```

## Technology credits

Favy is built with [Rust](https://www.rust-lang.org/), [Svelte](https://svelte.dev/), and [TypeScript](https://www.typescriptlang.org/) (RuST? 🤔) with the frameworks:

- [Tauri](https://tauri.app/) (Rust)
- [SvelteKit](https://kit.svelte.dev/) (Svelte / TypeScript)

Other notable packages:

- Rust
  - [fast image resize](https://github.com/Cykooz/fast_image_resize)
  - [zune image (zune-jpeg)](https://github.com/etemesi254/zune-image)
  - [kamadak-exif](https://github.com/kamadak/exif-rs)
  - [window-vibrancy](https://github.com/tauri-apps/window-vibrancy)
  - [Serde](https://serde.rs/)

- TypeScript
  - [Vite](https://vitejs.dev/)
  - [Sass (SCSS)](https://sass-lang.com/)
  - [Prettier](https://prettier.io/)
  - [ESlint](https://eslint.org/)


## License

Copyright 2023 - Arya "Dee" Daroui

Don't redistribute this project. *Otherwise*, MIT/Apache v2.0 Licenses apply.

In other words, you are welcome to...

- Fork, edit, and build this project for personal use, publicly or privately
- Contribute your changes back to this repository
- Reuse and rework *parts* of this codebase for anything else

But...

- Do not redistribute this project or its executables--especially not for money
- Do not take significant enough parts of the codebase that you are effectively redistributing it :-/

The intent is that this repository remain the official home of this project. Please see the [license.txt](https://github.com/aryadaroui/Favy/blob/main/license.txt) for more.
