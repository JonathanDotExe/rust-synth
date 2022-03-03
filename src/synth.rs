mod dsp;
mod voice;

pub trait SoundEngine {
    
    fn process(time: f32, time_step: f32) -> f32;

}

#[derive(Default)]
pub struct SynthVoice {
    dsp::Oscillator osc;
}

pub struct SynthEngine {
    VoiceManager<SynthVoice> voice_mgr;
}

impl SynthEngine {
    fn new() -> SynthEngine {
        return SynthEngine {
            voice_mgr: VoiceManager::new(30),
        };
    }
}

impl SoundEngine for SynthEngine {

    fn process(time: f32, time_step: f32) -> f32 {
        //TODO process voices
    };

}