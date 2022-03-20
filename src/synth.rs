use crate::dsp as dsp;
use crate::voice as voice;
use crate::io as io;
use crate::midi as midi;


#[derive(Default)]
pub struct SynthVoice {
    pub osc1: dsp::Oscillator,
    pub osc2: dsp::Oscillator,
    pub freq: f64,
}

#[derive(Default, Copy, Clone)]
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
                detune: dsp::note_to_freq_transpose(0.1),
            },
            voice_mgr: voice::VoiceManager::new(30),
            sample_rate: 0,
            time_step: 0.0,
        };
    }
}

impl voice::VoiceProcessor<SynthVoice> for SynthEngine {

    fn process_voice(&mut self, voice: &mut voice::Voice<SynthVoice>, info: io::SampleInfo) -> f64 {
        let osc1 = dsp::OscilatorConfig {waveform: self.preset.osc1_waveform, freq: voice.data.freq};
        let osc2 = dsp::OscilatorConfig {waveform: self.preset.osc2_waveform, freq: voice.data.freq * self.preset.detune};

        return (voice.data.osc1.process(osc1, self.time_step) + voice.data.osc2.process(osc2, self.time_step)) * 0.5; //Mix both oscillators equally
    }

    fn voice_on(&mut self, voice: &mut voice::Voice<SynthVoice>, info: io::SampleInfo) {
        voice.data.freq = dsp::note_to_freq(voice.note as f64);
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
            midi::MidiMessageType::NoteOn => self.voice_mgr.press_note(self, *msg.note(), *msg.velocity(), info),
            midi::MidiMessageType::NoteOff => self.voice_mgr.release_note(self, *msg.note(), info),
            _ => {},
        }
    }

}