use hex_forge::*;

fn main() {
    let mut hf = HexForge::new(HexForgeConfig {
        player_speed: 250.0,
        camera_speed: 4.0,
        stroke_color: bevy::prelude::Color::srgb(1.0, 0.5, 0.0),
        fill_color: bevy::prelude::Color::srgba(1.0, 1.0, 1.0, 0.0),
        hover_color: bevy::prelude::Color::srgba(1.0, 1.0, 1.0, 0.1),
    });

    hf.start();
}