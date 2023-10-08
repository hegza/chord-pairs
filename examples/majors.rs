use chord_pairs::samples::{make_sample, Chord, ChordKind, Note};
use rodio::buffer::SamplesBuffer;
use strum::IntoEnumIterator;

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    loop {
        // Initialize audio
        let mut samples = Note::iter()
            .map(|note| Chord {
                basenote: note,
                kind: ChordKind::Major,
            })
            .map(|ch| make_sample(ch, 100));

        // Play all samples
        for mut sample in &mut samples {
            let buf = SamplesBuffer::new(2, 48_000, sample.as_i16_slice());
            sink.append(buf);
        }
    }
}
