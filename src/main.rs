mod dsp;
mod voice;
mod synth;
mod midi;
mod io;

fn main() {
    //Audio
    let synth = Box::new(synth::SynthEngine::new());
    let mut _handler = io::AudioMidiHandler::new(synth);

    //Wait
    println!("Press ENTER to stop!");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}
