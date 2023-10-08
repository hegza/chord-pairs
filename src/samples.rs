use std::fmt;

use fon::{chan::Ch16, Audio, Frame};
use twang::{noise::White, ops::Gain, osc::Sine, Synth};

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Chord {
    C3Minor,
    D3Minor,
    E3Minor,
    F3Minor,
    G3Minor,
    A3Minor,
    B3Minor,
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chord::C3Minor => write!(f, "C3 minor"),
            Chord::D3Minor => write!(f, "D3 minor"),
            Chord::E3Minor => write!(f, "E3 minor"),
            Chord::F3Minor => write!(f, "F3 minor"),
            Chord::G3Minor => write!(f, "G3 minor"),
            Chord::A3Minor => write!(f, "A3 minor"),
            Chord::B3Minor => write!(f, "B3 minor"),
            Chord::C4 => write!(f, "C4"),
            Chord::D4 => write!(f, "D4"),
            Chord::E4 => write!(f, "E4"),
            Chord::F4 => write!(f, "F4"),
            Chord::G4 => write!(f, "G4"),
            Chord::A4 => write!(f, "A4"),
            Chord::B4 => write!(f, "B4"),
        }
    }
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
        Chord::C4 => 261.,
        Chord::D4 => 294.,
        Chord::E4 => 329.,
        Chord::F4 => 349.,
        Chord::G4 => 392.,
        Chord::A4 => 440.,
        Chord::B4 => 493.,
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
