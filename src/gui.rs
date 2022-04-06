use eframe::{egui::{self, Ui}, epi};
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
                //Functions
                fn waveform_dropdown(waveform: &mut dsp::WaveForm, ui: &mut egui::Ui) {
                    egui::ComboBox::from_label("Waveform")
                        .selected_text(format!("{:?}", waveform))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(waveform, dsp::WaveForm::Sine, "Sine");
                            ui.selectable_value(waveform, dsp::WaveForm::Saw, "Saw");
                            ui.selectable_value(waveform, dsp::WaveForm::Square, "Square");
                        });
                }
                //Heading
                ui.label("Osc 1");
                ui.label("Osc 2");
                ui.label("Filter");
                //Oscillator 1
                egui::Grid::new("osc_1_grid").show(ui, |ui| {
                    //Volume
                    ui.add(egui::Slider::new(&mut self.model.volume1, 0.0..=1.0).vertical().text("Volume"));
                    //Waveform
                    waveform_dropdown(&mut self.model.waveform1, ui);

                    ui.end_row();
                });
                //Oscillator 2
                egui::Grid::new("osc_2_grid").show(ui, |ui| {
                    //Volume
                    ui.add(egui::Slider::new(&mut self.model.volume2, 0.0..=1.0).vertical().text("Waveform"));
                    //Waveform
                    waveform_dropdown(&mut self.model.waveform2, ui);
                    //UI
                    ui.add(egui::Slider::new(&mut self.model.detune, 0.0..=1.0).vertical().text("Detune"));
                    ui.end_row();
                });
                //Filter
                egui::Grid::new("filter_grid").show(ui, |ui| {
                    //Cutofff
                    ui.add(egui::Slider::new(&mut self.model.filter_cutoff, 14.0..=21000.0).vertical().logarithmic(true).text("Waveform"));
                    ui.end_row();
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