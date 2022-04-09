use crate::dsp as dsp;

#[derive(Default)]
struct SynthModel {
    pub waveform1: dsp::WaveForm,
    pub waveform2: dsp::WaveForm,
    pub volume1: f64,
    pub volume2: f64,
    pub detune: f64,
    pub filter_cutoff: f64,
}

#[derive(Default)]
struct SynthApp {
    pub model: SynthModel
}


pub fn launch_ui() {
    
}