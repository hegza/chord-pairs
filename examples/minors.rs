use rodio::buffer::SamplesBuffer;
use sounds::sound::{make_sample, Chord};

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    loop {
        // Initialize audio
        use Chord as C;
        let mut samples = [
            C::C3Minor,
            C::D3Minor,
            C::E3Minor,
            C::F3Minor,
            C::G3Minor,
            C::A3Minor,
            C::B3Minor,
        ]
        .into_iter()
        .map(|ch| make_sample(ch, 100));

        // Play all samples
        for mut sample in &mut samples {
            let buf = SamplesBuffer::new(2, 48_000, sample.as_i16_slice());
            sink.append(buf);
        }
    }
}
