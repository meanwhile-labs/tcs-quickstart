use serde::Deserialize;

use crate::{
    apply_patch::{apply_patch, verify_patch, Patch},
    messaging::error_log,
};
#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct Config {
    #[serde(default = "default_true")]
    pub skip_splash_screens: bool,
    #[serde(default = "default_false")]
    pub disable_main_menu_music: bool,
}

#[allow(dead_code)]
fn default_true() -> bool {
    true
}
#[allow(dead_code)]
fn default_false() -> bool {
    true
}

pub fn apply_patches(config: &Config) {
    if config.skip_splash_screens {
        let skip_splash_screens = Patch {
            offset: hex_number("ca6ce"),
            // `jp LEGOStarWarsSaga.exe+CAD10`
            original: hex_bytes("0f 8a 3c 06 00 00"),
            // `jmp LEGOStarWarsSaga.exe+CAD10 / nop`
            patch: hex_bytes("e9 3d 06 00 00 90"),
        };

        unsafe {
            let verified = verify_patch(&skip_splash_screens);
            if let Err(err) = verified {
                error_log!("Skip Splash Screens patch is invalid: {err}");
            } else if let Err(err) = apply_patch(&skip_splash_screens) {
                error_log!("Skip Splash Screens patch failed: {err}");
            }
        }
    }

    if config.disable_main_menu_music {
        let disable_main_menu_music = Patch {
            offset: hex_number("28b42"),
            // `mov byte ptr [esi + 0x6],0x1`
            original: hex_bytes("c6 46 06 01"),
            // `mov byte ptr [esi + 0x6],0x0`
            patch: hex_bytes("c6 46 06 00"),
        };

        unsafe {
            let verified = verify_patch(&disable_main_menu_music);
            if let Err(err) = verified {
                error_log!("Disable Main Menu Music patch is invalid: {err}");
            } else if let Err(err) = apply_patch(&disable_main_menu_music) {
                error_log!("Disable Main Menu Music patch failed: {err}");
            }
        }
    }
}

fn hex_number(input: &str) -> u32 {
    u32::from_str_radix(input, 16)
        .expect(&("invalid hex_number (this is a problem with the mod): ".to_owned() + input))
}

fn hex_bytes(input: &str) -> Vec<u8> {
    let without_whitespace = input.replace(" ", "");
    let chars = without_whitespace.chars().collect::<Vec<_>>();
    let pairs: Vec<&[char]> = chars.chunks(2).collect();
    let bytes = pairs
        .into_iter()
        .map(|pair| {
            let pair: String = pair.iter().collect();
            u8::from_str_radix(&pair, 16)
        })
        .collect::<Result<Vec<_>, _>>()
        .expect(&("invalid hex_bytes (this is a problem with the mod): ".to_owned() + input));
    bytes
}
