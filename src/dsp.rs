pub enum WaveForm {
    Sine,
    Saw,
    Square,
}

impl WaveForm {
    fn synthesize(&self, phase: f32) -> f32{
        let f: f32 = 0.0;
        match self {
            WaveForm::Sine => f = (phase * std::f64::consts::PI as f32).sin(),
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

pub struct Oscillator {
    pub waveform: WaveForm,
    pub freq: f32,
    phase: f32,
}

impl Oscillator {

    pub fn new() -> Oscillator {
        return Oscillator {
            waveform: WaveForm::Sine,
            freq: 220.0,
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

    pub fn synthesize(&self) -> f32{
        return self.waveform.synthesize(self.phase);
    }

}