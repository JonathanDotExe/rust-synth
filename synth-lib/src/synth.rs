use crate::audio::SampleInfo;
use crate::dsp as dsp;
use crate::voice as voice;
use crate::audio;
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
    voice_mgr: voice::VoiceManager<SynthVoice>,
    proc: SynthProcessor
}

struct SynthProcessor {
    pub preset: SynthPreset,
    sample_rate: u32,
    time_step: f64,
}

impl SynthEngine {
    pub fn new() -> SynthEngine {
        return SynthEngine {
            voice_mgr: voice::VoiceManager::new(30),
            proc: SynthProcessor {
                preset: SynthPreset { //Simple fat saw patch
                    osc1_waveform: dsp::WaveForm::Saw,
                    osc2_waveform: dsp::WaveForm::Saw,
                    detune: dsp::note_to_freq_transpose(0.1),
                },
                sample_rate: 0,
                time_step: 0.0,
            }
        };
    }
}

impl voice::VoiceProcessor<SynthVoice> for SynthProcessor {

    fn process_voice(&mut self, voice: &mut voice::Voice<SynthVoice>, _info: audio::SampleInfo) -> f64 {
        let osc1 = dsp::OscilatorConfig {waveform: self.preset.osc1_waveform, freq: voice.data.freq};
        let osc2 = dsp::OscilatorConfig {waveform: self.preset.osc2_waveform, freq: voice.data.freq * self.preset.detune};

        let sample = (voice.data.osc1.process(osc1, self.time_step) + voice.data.osc2.process(osc2, self.time_step)) * 0.5;

        return sample; //Mix both oscillators equally
    }

    fn voice_on(&mut self, voice: &mut voice::Voice<SynthVoice>, _info: audio::SampleInfo) {
        voice.data.freq = dsp::note_to_freq(voice.note as f64);
    }

}

impl audio::AudioMidiProcessor for SynthEngine {

    fn setup(&mut self, info: audio::ProcessingInfo) {
        self.proc.sample_rate = info.sample_rate;
        self.proc.time_step = info.time_step;
        let i = SampleInfo {
            sample_count: 0,
            time: 0.0,
            jitter: false,
        };
        self.voice_mgr.reset(&mut self.proc, i);
        //self.voice_mgr.press_note(&mut self.proc, 60, 127, i);
    }

    fn process(&mut self, info: audio::SampleInfo) -> (f64, f64) {
        let sample = self.voice_mgr.process_voices(&mut self.proc, info);
        return (sample, sample);
    }

    fn recieve_midi(&mut self, msg: midi::MidiMessage, info: audio::SampleInfo) {
        //Note on/off for voice manager
        match msg.message_type {
            midi::MidiMessageType::NoteOn => self.voice_mgr.press_note(&mut self.proc, msg.note(), msg.velocity(), info),
            midi::MidiMessageType::NoteOff => self.voice_mgr.release_note(&mut self.proc, msg.note(), info),
            _ => {},
        }
    }

}