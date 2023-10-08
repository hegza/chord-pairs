use std::collections::HashMap;

use crate::samples::{make_sample, Chord};
use fon::{chan::Ch16, Audio};
use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle};

pub const GLOBAL_MUTE: bool = false;

pub struct ChordPlayer {
    sink: rodio::Sink,
    _handle: OutputStreamHandle,
    _stream: OutputStream,
    chords: HashMap<Chord, Audio<Ch16, 2>>,
}

impl ChordPlayer {
    pub fn from_chords(chords: impl Iterator<Item = Chord>) -> Self {
        let (_stream, _handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&_handle).unwrap();

        let chords = chords
            .map(|chord| (chord.clone(), make_sample(chord, 500)))
            .collect();

        Self {
            sink,
            _handle,
            _stream,
            chords,
        }
    }

    pub fn play_chord(&self, chord: &Chord) {
        let sample = self.chords.get(chord).expect(&format!(
            "chord {:?} not registered with chord player",
            chord
        ));
        let mut nsample = Audio::<Ch16, 2>::with_audio::<Ch16, 2>(48_000, sample);
        self.play_sample(&mut nsample);
    }

    fn play_sample(&self, sample: &mut Audio<Ch16, 2>) {
        if !GLOBAL_MUTE {
            let buf = SamplesBuffer::new(2, 48_000, sample.as_i16_slice());
            self.sink.append(buf);
        }
    }
}
