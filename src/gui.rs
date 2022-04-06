use eframe::{egui, epi};
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

impl epi::App for SynthApp {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        //Center panel
        egui::CentralPanel::default().show(ctx, |ui| {
            //Top bar with synth controls
            egui::Grid::new("synth_grid").show(ui, |ui| {
                //Heading
                ui.label("Osc 1");
                ui.label("Osc 2");
                ui.label("Filter");
                //Oscillator 1
                egui::Grid::new("osc_1_grid").show(ui, |ui| {
                    //Volume
                    ui.add(egui::Slider::new(&mut self.model.volume1, 0.0..=1.0).text("Volume"));
                    //Waveform
                });
            });
        });
    }

    fn name(&self) -> &str {
        return "Epic Rust Synth"
    }
}

pub fn launch_ui() {

}