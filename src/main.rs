mod gui;
mod dsp;
mod voice;
mod synth;
mod midi;
mod io;

fn main() {
    //Audio
    let synth = Box::new(synth::SynthEngine::new());
    let mut _handler = io::AudioMidiHandler::new(synth);

    //GUI
    gui::launch_ui();
}
