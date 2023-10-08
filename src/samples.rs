use std::fmt;

use fon::{chan::Ch16, Audio, Frame};
use strum::EnumIter;
use twang::{noise::White, ops::Gain, osc::Sine, Synth};

/// First ten harmonic volumes of a piano sample (sounds like electric piano).
const HARMONICS: [f32; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];
/// Volume of the piano
const VOLUME: f32 = 1.0 / 3.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter, strum::Display)]
pub enum Note {
    C3,
    D3,
    E3,
    F3,
    G3,
    A3,
    B3,
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum ChordKind {
    Minor,
    Major,
}

impl Note {
    pub fn freq(&self) -> f64 {
        match self {
            Note::C3 => 130.81,
            Note::D3 => 146.83,
            Note::E3 => 164.81,
            Note::F3 => 174.61,
            Note::G3 => 196.,
            Note::A3 => 220.,
            Note::B3 => 246.94,
            Note::C4 => 261.,
            Note::D4 => 294.,
            Note::E4 => 329.,
            Note::F4 => 349.,
            Note::G4 => 392.,
            Note::A4 => 440.,
            Note::B4 => 493.,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chord {
    pub basenote: Note,
    pub kind: ChordKind,
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.basenote)?;
        match self.kind {
            ChordKind::Minor => write!(f, " minor"),
            ChordKind::Major => write!(f, ""),
        }
    }
}

pub fn get_chord(note: Note, chord_kind: ChordKind) -> [f32; 3] {
    let base = match note {
        Note::C3 => 130.81,
        Note::D3 => 146.83,
        Note::E3 => 164.81,
        Note::F3 => 174.61,
        Note::G3 => 196.,
        Note::A3 => 220.,
        Note::B3 => 246.94,
        Note::C4 => 261.,
        Note::D4 => 294.,
        Note::E4 => 329.,
        Note::F4 => 349.,
        Note::G4 => 392.,
        Note::A4 => 440.,
        Note::B4 => 493.,
    };
    match chord_kind {
        ChordKind::Minor => [base, base * 32.0 / 27.0, base * 3.0 / 2.0],
        ChordKind::Major => [base, base * 5. / 4., base * 3.0 / 2.0],
    }
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
    let Chord { basenote, kind } = chord;

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
        for (s, pitch) in proc
            .piano
            .iter_mut()
            .zip(get_chord(basenote.clone(), kind.clone()).iter())
        {
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
