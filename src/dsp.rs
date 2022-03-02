pub trait Synthesizable {
    fn synthesize(phase: f32) -> f32;
}

pub enum WaveForm {
    Sine,
    Saw,
    Square,
}

impl Synthesizable for WaveForm::Sine {
    fn synthesize(phase: f32) {
        return (phase * std::f64::consts::PI as f32).sin();
    }
}

impl Synthesizable for WaveForm::Saw {
    fn synthesize(phase: f32) {
        return phase * 2.0 - 1.0;
    }
}

impl Synthesizable for WaveForm::Square {
    fn synthesize(phase: f32) {
        if (phase < 0.5) {
            return 1.0;
        }
        else {
            return -1.0;
        }
    }
}

pub struct Oscillator {
    pub waveform: WaveForm = Sine;
    pub freq: f32 = 220.0;

    phase: f32 = 0;
}

impl Oscillator {

    fn process(&self, time_step: f32) {
        self.phase += time_step * freq;
        //Modulo
        while self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }

    fn synthesize(&self) -> f32{
        return waveform.synthesize(self.phase);
    }

}