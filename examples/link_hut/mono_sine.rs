pub struct MonoSine {
    buffer: Vec<f32>,
    pos: usize,
}

impl MonoSine {
    pub fn new(sample_rate: u32, pitch_freq: f32) -> Self {
        let mut buffer: Vec<f32> = Vec::with_capacity(sample_rate as usize);

        for sample in 0..sample_rate as u32 {
            buffer.push(
                ((sample as f32 / sample_rate as f32) * pitch_freq * 2.0 * std::f32::consts::PI)
                    .sin(),
            );
        }

        Self { buffer, pos: 0 }
    }
}

impl Iterator for MonoSine {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = Some(self.buffer[self.pos]);
        self.pos = (self.pos + 1) % self.buffer.len();
        value
    }
}
