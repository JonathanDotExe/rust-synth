pub enum MidiMessageType {
    NoteOff,
    NoteOn,
    PolyphonicAftertouch,
    ControlChange,
    ProgramChange,
    MonophonicAftertouch,
    PitchBend,
    SysEx 
}

pub struct MidiMessage {
    message_type: MidiMessageType,
    channel: u64,
    first_data: u64,
    second_data: u64,
}

impl MidiMessage {
    pub fn note(&mut self) -> &mut u64 {
        return &mut self.first_data;
    }
    pub fn velocity(&mut self) -> &mut u64 {
        return &mut self.second_data;
    }
    pub fn polyphonic_aftertouch(&mut self) -> &mut u64 {
        return &mut self.second_data;
    }
    pub fn control(&mut self) -> &mut u64 {
        return &mut self.first_data;
    }
    pub fn value(&mut self) -> &mut u64 {
        return &mut self.second_data;
    }
    pub fn program(&mut self) -> &mut u64 {
        return &mut self.first_data;
    }
    pub fn monophonic_aftertouch(&mut self) -> &mut u64 {
        return &mut self.second_data;
    }
    pub fn pitch_bend(&mut self) -> &mut u64 {
        return &mut self.first_data;
    }
}
