mod gui;
mod io;

use synth_lib;

fn main() {
    //Audio
    let synth: Box<synth_lib::synth::SynthEngine> = Box::new(synth_lib::synth::SynthEngine::new());
    let mut _handler = io::AudioMidiHandler::new(synth);

    //GUI
    gui::launch_ui();
}
