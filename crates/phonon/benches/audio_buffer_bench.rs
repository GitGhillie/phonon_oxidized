use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phonon::audio_buffer::AudioBuffer;

fn mix_buffers(in1: AudioBuffer<1, 200>) -> AudioBuffer<1, 200> {
    let mut in2: AudioBuffer<1, 200> = AudioBuffer::new();
    let mut in3: AudioBuffer<1, 200> = AudioBuffer::new();

    in2[0][0] = 3.0;
    in2[0][1] = 4.0;
    in3[0][0] = 7.0;
    in3[0][1] = 9.0;

    let mut out: AudioBuffer<1, 200> = AudioBuffer::new();

    out.mix(&in1);
    out.mix(&in2);
    out.mix(&in3);

    out
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mix buffers", |b| {
        b.iter(|| mix_buffers(black_box(AudioBuffer::new())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
