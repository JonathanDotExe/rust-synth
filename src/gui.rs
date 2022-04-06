use eframe::{egui, epi};
use crate::dsp as dsp;


struct SynthApp {
    pub waveform1: dsp::WaveForm,
    pub waveform2: dsp::WaveForm,
    pub volume1: f64,
    pub volume2: f64,
    pub detune: f64,
    pub filter_cutoff: f64,
}

impl epi::App for SynthApp {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        
    }

    fn name(&self) -> &str {
        return "Epic Rust Synth"
    }
}

pub fn launch_ui() {

}