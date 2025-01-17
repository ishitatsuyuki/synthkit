// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Harmonizer demo example using synthkit-rs.
use synthkit::*;

fn main() {
    let wav = std::env::args().nth(1).unwrap();
    let sound = get_sample(&wav).unwrap();
    let sloop = Loop::new(&sound);
    let root = 440.0;
    let third = root * f32::powf(2.0, 4.0 / 12.0);
    let octaves_down = root / 4.0;
    let mixer = Box::new(Mixer::with_streams(vec![
        sloop.iter(root),
        sloop.iter(third),
        sloop.iter(octaves_down),
    ]));
    play(mixer).unwrap();
}
