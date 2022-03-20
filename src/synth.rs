use crate::dsp as dsp;
use crate::voice as voice;
use crate::io as io;
use crate::midi as midi;


#[derive(Default)]
pub struct SynthVoice {
    pub osc1: dsp::Oscillator,
    pub osc2: dsp::Oscillator,
}

#[derive(Default)]
pub struct SynthPreset {
    osc1_waveform: dsp::WaveForm,
    osc2_waveform: dsp::WaveForm,
    detune: f64,
}

pub struct SynthEngine {
    pub preset: SynthPreset,

    voice_mgr: voice::VoiceManager<SynthVoice>,
    sample_rate: u32,
    time_step: f64,
}

impl SynthEngine {
    pub fn new() -> SynthEngine {
        return SynthEngine {
            preset: SynthPreset { //Simple fat saw patch
                osc1_waveform: dsp::WaveForm::Saw,
                osc2_waveform: dsp::WaveForm::Saw,
                detune: 0.1,
            },
            voice_mgr: voice::VoiceManager::new(30),
            sample_rate: 0,
            time_step: 0.0,
        };
    }
}

impl voice::VoiceProcessor<SynthVoice> for SynthEngine {

    fn process_voice(&mut self, voice: &mut voice::Voice<SynthVoice>, info: io::SampleInfo) -> f64 {
        let osc1 = dsp::OscilatorConfig {waveform: self.preset.osc1_waveform, freq: 440.0};
        let osc2 = dsp::OscilatorConfig {waveform: self.preset.osc2_waveform, freq: 220.0};

        return (voice.data.osc1.process(osc1, self.time_step) + voice.data.osc2.process(osc2, self.time_step)) * 0.5; //Mix both oscillators equally
    }

}

impl io::AudioMidiProcessor for SynthEngine {

    fn setup(&mut self, info: io::ProcessingInfo) {
        self.sample_rate = info.sample_rate;
        self.time_step = info.time_step;
    }

    fn process(&mut self, info: io::SampleInfo) -> f64 {
        return self.voice_mgr.process_voices(self, info);
    }

    fn recieve_midi(&mut self, msg: midi::MidiMessage, info: io::SampleInfo) {
        //Note on/off for voice manager
        match msg.message_type {
            midi::MidiMessageType::NoteOn => self.voice_mgr.press_note(*msg.note(), *msg.velocity(), info),
            midi::MidiMessageType::NoteOff => self.voice_mgr.release_note(*msg.note(), info),
        }
    }

}