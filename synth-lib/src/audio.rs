use crate::midi;

#[derive(Copy, Clone)]
pub enum ProcessingMode {
    Realtime,   //Must respond in real time
    Offline,    //Processing is offline (e.g. rendering in a DAW)
}

impl Default for ProcessingMode {
    fn default() -> Self {
        return ProcessingMode::Realtime;
    }
}

#[derive(Copy, Clone, Default)]
pub struct ProcessingInfo {
    pub sample_rate: u32,
    pub time_step: f64,
    pub processing_mode: ProcessingMode,
}

#[derive(Copy, Clone, Default)]
pub struct SampleInfo {
    pub sample_count: u64,
    pub time: f64,
    pub jitter: bool, //Indicates wether the sample function is called at a fixed (false) or variable (true) pace
}

pub trait AudioMidiProcessor {

    fn setup(&mut self, info: ProcessingInfo);

    fn process(&mut self, info: SampleInfo) -> (f64, f64);

    fn recieve_midi(&mut self, msg: midi::MidiMessage, info: SampleInfo);

}