use fon::chan::Ch16;
use fon::{Audio, Frame};
use twang::noise::White;
use twang::ops::Gain;
use twang::osc::Sine;
use twang::Synth;

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

#[derive(Clone, PartialEq)]
pub enum Chord {
    C3Minor,
    D3Minor,
    E3Minor,
    F3Minor,
    G3Minor,
    A3Minor,
    B3Minor,
}

pub fn pitches(chord: Chord) -> [f32; 3] {
    let base = match chord {
        Chord::C3Minor => 130.81,
        Chord::D3Minor => 146.83,
        Chord::E3Minor => 164.81,
        Chord::F3Minor => 174.61,
        Chord::G3Minor => 196.,
        Chord::A3Minor => 220.,
        Chord::B3Minor => 246.94,
    };
    [base, base * 32.0 / 27.0, base * 3.0 / 2.0]
}

// State of the synthesizer.
#[derive(Default)]
struct Processors {
    // White noise generator.
    white: White,
    // 10 harmonics for 3 pitches.
    piano: [[Sine; 10]; 3],
}

pub fn make_sample(chord: Chord, len_ms: usize) -> Audio<Ch16, 2> {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::with_silence(48_000, 48 * len_ms);
    // Create audio processors
    let mut proc = Processors::default();
    // Adjust phases of harmonics.
    for pitch in proc.piano.iter_mut() {
        for harmonic in pitch.iter_mut() {
            harmonic.shift(proc.white.step());
        }
    }

    // Build synthesis algorithm
    let mut synth = Synth::new(proc, move |proc, mut frame: Frame<_, 2>| {
        for (s, pitch) in proc.piano.iter_mut().zip(pitches(chord.clone()).iter()) {
            for ((i, o), v) in s.iter_mut().enumerate().zip(HARMONICS.iter()) {
                // Get next sample from oscillator.
                let sample = o.step(pitch * (i + 1) as f32);
                // Pan the generated harmonic center
                frame = frame.pan(Gain.step(sample, (v * VOLUME).into()), 0.0);
            }
        }
        frame
    });

    // Synthesize 5 seconds of audio
    synth.stream(audio.sink());

    audio
}
