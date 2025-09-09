# depot-rs

A TUI for managing crates, all at one place.

## Features

- A dashboard to view all installed crates with their metadata 
- Pick a crate to uninstall from the dashboard
- Shows which crates are outdated and update them at will

## Installation

> [!NOTE]
> You need to install [Rust](https://www.rust-lang.org/tools/install) before you can proceed.

```zsh
cargo install depot-rs --locked
depot
```

## Usage

To get started, run the binary:

```
depot
```

It should bring you to a loading screen. This may take a while if it is the first time running it. After it has finished fetching all the metadata, you should be able to see a menu.

### Dashboard

Press <kbd>c</kbd> from the menu to open the dashboard. Use vim-keybindings or arrow keys to navigate up and down, and press <kbd>d</kbd> to uninstall a crate.

### Update a crate

Press <kbd>u</kbd> from the menu to see what crates are outdated. Use vim-keybindings or arrow keys to navigate up and down, and press <kbd>ENTER</kbd> to update a crate.

## Motivation

I tend to forget about a crate I'd installed the other day after trying it out once or twice. If a crate has an update, it often goes unnoticed unless I'm following their release page closely. This makes me want to build a tool that helps me manage the crates I've installed at one place and not forget about them.

## License

Copyright (c) Moreen Ho <pigeon@quietpigeon.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
