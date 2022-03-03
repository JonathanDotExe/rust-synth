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
    pub note: u8,
    pub data: T,
}


impl<T> Voice<T> where T: Default {
    fn new() -> Voice<T> {
        return Voice::default();
    }
}

pub struct VoiceManager<T> where T: Default{
    voices: Vec<Voice<T>>,
}

impl<T> VoiceManager<T> where T: Default {
    //Init voice manager with a specific amount of polyphony
    fn new(size: usize) -> VoiceManager<T> {
        let mut mgr: VoiceManager<T> = VoiceManager {
            voices: Vec::new()
        };
        for _i in 0..size {
            let voice: Voice<T> = Voice::default();
            mgr.voices.push(voice);
        }
        return mgr;
    }
}