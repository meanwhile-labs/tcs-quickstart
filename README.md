# ttlego-jitpatcher

Patches the code of classic (LEGO Batman 1 and older) Traveler's Tales LEGO Games without modifying the EXE!

## Compatability

So far tested with:

- LEGO Star Wars: The Complete Saga (GOG version)

Please let me know if you find other games that work!

## Installation

1. Add [Ultimate ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader) to the game folder (`dinput8.dll` will work fine).
2. In the game folder, create a `plugins/jitpatcher` folder.
4. Download the latest [`jitpatcher.asi`](https://github.com/meanwhile-labs/ttlego-jitpatcher/releases) file from this project and place it in `plugins/jitpatcher`.
5. Next to the ASI file, create a `patches` folder.
6. Add `.toml` files to the `patches` folder. You can find examples in the [examples](./examples/) folder in this project. Any of those patches will be automatically applied when the game starts.

## Creating patches

All patch files should end in `.toml` and be in the following format:

```toml
name = "[Human readable name]"
enabled = true

[[patches]]
offset = "000000" # hex offset of address to modify
original = "00 00 00 00" # the original hex value that lives at that offset
patch =    "00 00 00 00" # the hex value you want to replace it with

[[patches]] # you can add multiple patches in a single file
offset = "0000ff"
original = "00 00 00 00 00 00 00"
patch =    "00 00 00 00 00 00 00"
```

Hex values aren't case sensitive; uppercase or lowercase letters work.

The spaces also aren't strictly required in `original` or `patch`, but highly recommended.

### `offset`

This is the hex address that will be modified. This is relative to the start of the game code, so...

- If you see something like `LEGOStarWarsSaga.exe+CAD10` in a tool like Cheat Engine, then `CAD10` will be your offset.
- If you see something like `004cad10` in a tool like Ghidra, then subtract hex `00400000` (or 4,194,304 in decimal / normal numbers) to get your address, because the game code starts there (at least in my testing, and it seems to be pretty standard - let me know if there are any games that don't work like that!)
  - Note that there might be addresses above `00500000`, so it's not just a matter of removing the `4`. If you see something like `005cccc`, that would be an offset of `1cccc`.
- If you have an address from a hex editor looking at the executable, that actually is the offset; no further transformation required. So `0x000CA6CE` would be `ca6ce`.

### `original`

These are the bytes, written in hexadecimal, that are expected to already exist in the game code. This lets the patcher make sure it has the right place before modifying the game code - otherwise unpredictable but probably crash-related things could happen.

This can be any length.

### `patch`

The new bytes, again written in hexadecimal, to replace the bytes in `original`. This must be the same length as `original`.

Remember that game code is stored in little-endian format. If you don't know what that means, be careful when modifying numbers - probably best to test your changes in something like Cheat Engine before making a patch.

## How it works

See the `src/apply_patch.rs` file for the meaty stuff. The rest is mostly housekeeping, plus some code to display log messages if needed.

## Development

DLL must be built with `cargo build --target i686-pc-windows-msvc` to actually work with the game! Then rename it to `jitpatcher.asi` to work with Ultimate ASI Loader.

## Roadmap/ideas

- More documentation, especially of the internals
- More testing on various games
- More example patch files for more games
- Auto-detect different versions of the same game (ex. Steam vs. GOG) and adjust patches to fit
- Support for code caves, to support more ambitious modifications
