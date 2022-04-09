use crate::midi;

#[derive(Copy, Clone)]
pub struct ProcessingInfo {
    pub sample_rate: u32,
    pub time_step: f64,
}

#[derive(Copy, Clone)]
pub struct SampleInfo {
    pub sample_count: u64,
    pub time: f64,
}

pub trait AudioMidiProcessor {

    fn setup(&mut self, info: ProcessingInfo);

    fn process(&mut self, info: SampleInfo) -> f64;

    fn recieve_midi(&mut self, msg: midi::MidiMessage, info: SampleInfo);

}