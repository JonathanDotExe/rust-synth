use druid::{Widget, Data, WindowDesc, AppLauncher, widget::{Label, Flex}, Env};

use synth_lib::dsp;

#[derive(Default, Clone, Data)]
struct SynthModel {
    #[data(same_fn = "PartialEq::eq")]
    pub waveform1: dsp::WaveForm,
    #[data(same_fn = "PartialEq::eq")]
    pub waveform2: dsp::WaveForm,
    pub volume1: f64,
    pub volume2: f64,
    pub detune: f64,
    pub filter_cutoff: f64,
}

fn build_main_ui() -> impl Widget<SynthModel> {
    //Top Panel
    let waveform1_label = Label::dynamic(|model: &SynthModel, _env| {
        return format!("Waveform 1: {}", model.waveform1);
    });

    let layout = Flex::column()
        .with_child(waveform1_label);
    return layout;
}

pub fn launch_ui() {
    let window = WindowDesc::new(build_main_ui).title("Simple Rust Synth");
    let model = SynthModel::default();
    AppLauncher::with_window(window).launch(model).expect("Failed to launch app!");
}