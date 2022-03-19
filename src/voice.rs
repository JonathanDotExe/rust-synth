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
    pub release_time: f64,
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

    fn process_voice(&mut self, voice: &Voice<T>, data: &mut T, info: io::SampleInfo);

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

    fn find_next_slot(&mut self) -> usize {
        let released = false;
        let longest_index: usize = 0;
        let longest_time: f64 = f64::MAX;

        //TODO refactor, bad code when more states are added
        for i in 0..self.voices.len() {
            let voice: &Voice<T> = &self.voices[i];
            if voice.state == VoiceState::Incative {    // Note is free, use it
                return i;
            }
            if voice.state == VoiceState::Released {
                if released {  // Only counting released notes
                    if voice.release_time < longest_time {
                        longest_index = i;
                        longest_time = voice.release_time;
                    }
                }
                else {  // First released note, only count released notes from now on
                    longest_index = i;
                    longest_time = voice.release_time;
                    released = true;
                }
            }
            else if voice.press_time < longest_time{ // Check for pressed notes
                longest_index = i;
                longest_time = voice.press_time;
            }
        }

		return longest_index;
    }

    pub fn press_note(&mut self, note: u32, velocity: u32, info: io::SampleInfo) {
        let index = self.find_next_slot();
        self.voices[index].note = note;
        self.voices[index].velocity = note;
        self.voices[index].state = VoiceState::Pressed;
        self.voices[index].press_time = info.time;
        self.voices[index].release_time = 0.0;
    }

    pub fn release_note(&mut self, note: u32, velocity: u32, info: io::SampleInfo) {
        for mut voice in self.voices {
            if voice.note == note {     //Check if note is equal
                voice.state = VoiceState::Released;
                voice.release_time = info.time;
            }
        }
    }
  
    pub fn process_voices<E: VoiceProcessor<T>>(&mut self, proc: E, info: io::SampleInfo) {
        for voice in self.voices {
            if voice.state != VoiceState::Incative {
                proc.process_voice(&voice, &mut voice.data, info);
            }
        }
    }
}