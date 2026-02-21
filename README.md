# Quickstart for LEGO Star Wars: The Complete Saga

Makes the game start up faster by skipping the splash screens.

## Compatability

So far tested with:

- GOG version

It would probably work with the Steam version as well, but I haven't tested this yet.

## Installation

1. Add [Ultimate ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader) to the game folder (`dinput8.dll` will work fine).
2. In the game folder, create a `plugins` folder.
4. Download the latest [`quickstart.asi`](https://github.com/meanwhile-labs/tcs-quickstart/releases) file from this project and place it in `plugins`.
5. Copy the [`config/quickstart.toml`](./config/quickstart.toml) file from this project
6. Paste it in `plugins` next to `quickstart.asi`.
7. Configure it as desired (see below)

## Configuration:

There are currently only two settings:

### `skip_splash_screens`

If `true`, skips the beginning splash screens. You probably want this enabled; otherwise why even install the mod?

### `disable_main_menu_music`

If `true`, turns off the main menu music when the game first launches.

This is off by default, but it's a nice little bonus feature for modders who might want to listen to their own music as they repeatedly launch and relaunch the game to test their changes.

## How it works

See `src/patches.rs` for the actual executable patches, and the `src/apply_patch.rs` file for the code that makes those changes happen. The rest is mostly housekeeping, plus some code to display log messages if needed.

## Development

DLL must be built with `cargo build --target i686-pc-windows-msvc` (i.e. 32-bit Windows) to actually work with the game! Then rename it to `quickstart.asi` to work with Ultimate ASI Loader.

## Roadmap/ideas

- Make the main menu appear and be interactive faster (currently it waits for the Star Wars logo to fully zoom out)
- Keep the mouse cursor active (useful for DxWnd users)
- A version for LEGO Batman 1, which has a very similar engine