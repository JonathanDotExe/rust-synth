use crate::dsp as dsp;
use crate::voice as voice;
use crate::io as io;

#[derive(Default)]
pub struct SynthVoice {
    pub osc1: dsp::Oscillator,
    pub osc2: dsp::Oscillator,
}

#[derive(Default)]
pub struct SynthPreset {
    osc1: dsp::OscilatorConfig,
    osc2: dsp::OscilatorConfig,
    detune: f64,
}

pub struct SynthEngine {
    voice_mgr: voice::VoiceManager<SynthVoice>,

    sample_rate: u32,
    time_step: f64,
}

impl SynthEngine {
    pub fn new() -> SynthEngine {
        return SynthEngine {
            voice_mgr: voice::VoiceManager::new(30),

            sample_rate: 0,
            time_step: 0.0,
        };
    }
}

impl io::AudioMidiProcessor for SynthEngine {

    fn setup(&mut self, info: io::ProcessingInfo) {

    }

    fn process(&mut self) -> f64 {
        
    }

    fn recieve_midi(&mut self, msg: midi::MidiMessage) {

    }

}