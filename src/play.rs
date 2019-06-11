// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Synthesizer audio player.

use portaudio as pa;

use std::error::Error;

use crate::*;

/// Gather samples and post for playback.
pub fn play(mut samples: Stream<'static>) -> Result<(), Box<Error>> {

    // Create and initialize audio output.
    let out = pa::PortAudio::new()?;
    let mut settings = out.default_output_stream_settings(
        1, // 1 channel.
        SAMPLE_RATE as f64,
        0_u32, // Least possible buffer.
    )?;
    settings.flags = pa::stream_flags::CLIP_OFF;

    let mut stream =
        out.open_non_blocking_stream(settings, move |out| {
            for i in 0..out.frames {
                match samples.next() {
                    Some(s) => out.buffer[i] =
                        f32::floor(s * 32768.0f32) as i16,
                    None => {
                        for j in i..out.frames {
                            out.buffer[j] = 0;
                        }
                    },
                }
            }
            pa::Continue
        })?;

    stream.start()?;

    out.sleep(5000);

    stream.stop()?;
    stream.close()?;

    Ok(())
}
