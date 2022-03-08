use crate::dsp as dsp;
use crate::voice as voice;


pub trait SoundEngine {
    
    fn process(time: f64, time_step: f64) -> f64;

}

#[derive(Default)]
pub struct SynthVoice {
    pub osc: dsp::Oscillator,
}

pub struct SynthEngine {
    pub voice_mgr: voice::VoiceManager<SynthVoice>,
}

impl SynthEngine {
    pub fn new() -> SynthEngine {
        return SynthEngine {
            voice_mgr: voice::VoiceManager::new(30),
        };
    }
}

impl SoundEngine for SynthEngine {

    fn process(time: f64, time_step: f64) -> f64 {
        //TODO process voices
        return 0.0;
    }

}