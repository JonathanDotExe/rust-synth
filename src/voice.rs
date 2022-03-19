use crate::io as io;

#[derive(PartialEq)]
pub enum VoiceState {
    Incative,
    Pressed,
    Released
}

impl Default for VoiceState {
    fn default() -> Self {
        return VoiceState::Incative
    }
}

#[derive(Default)]
pub struct Voice<T> where T: Default{
    pub state: VoiceState,
    pub note: u32,
    pub velocity: u32,
    pub press_time: f64,
    pub data: T,
}


impl<T> Voice<T> where T: Default {
    pub fn new() -> Voice<T> {
        return Voice::default();
    }
}

pub struct VoiceManager<T> where T: Default{
    pub voices: Vec<Voice<T>>,
}

pub trait VoiceProcessor<T> where T: Default{

    fn process(&mut self, voice: &Voice<T>, data: &mut T, info: io::SampleInfo);

}

impl<T> VoiceManager<T> where T: Default {
    //Init voice manager with a specific amount of polyphony
    pub fn new(size: usize) -> VoiceManager<T> {
        let mut mgr: VoiceManager<T> = VoiceManager {
            voices: Vec::new()
        };
        for _i in 0..size {
            let voice: Voice<T> = Voice::default();
            mgr.voices.push(voice);
        }
        return mgr;
    }

    fn find_next_slot(&mut self) -> u32 {

		return 0;
    }

    pub fn press_note(note: u32, velocity: u32) {

    }

    pub fn process_voices<E: VoiceProcessor<T>>(&mut self, proc: E, info: io::SampleInfo) {
        for voice in self.voices {
            if voice.state != VoiceState::Incative {
                proc.process(&voice, &mut voice.data, info);
            }
        }
    }
}