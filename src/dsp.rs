pub enum WaveForm {
    Sine,
    Saw,
    Square,
}

impl Default for WaveForm {
    fn default() -> Self {
        return WaveForm::Sine;
    }
}

impl WaveForm {
    fn synthesize(&mut self, phase: f32) -> f32{
        let f: f32;
        match self {
            WaveForm::Sine => f = (phase * (std::f64::consts::PI as f32) * 2.0).sin(),
            WaveForm::Square => {
                if phase < 0.5 {
                    f = 1.0;
                }
                else {
                    f = -1.0;
                }   
            },
            WaveForm::Saw => f = phase * 2.0 - 1.0,
        }
        return f;
    }
}

#[derive(Default)]
pub struct Oscillator {
    pub waveform: WaveForm,
    pub freq: f32,
    phase: f32,
}

impl Oscillator {

    pub fn new() -> Oscillator {
        return Oscillator {
            waveform: WaveForm::Sine,
            freq: 440.0,
            phase: 0.0,
        }
    }

    pub fn process(&mut self, time_step: f32) {
        self.phase += time_step * self.freq;
        //Modulo
        while self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }

    pub fn synthesize(&mut self) -> f32{
        let sample = self.waveform.synthesize(self.phase);
        return sample;
    }

}