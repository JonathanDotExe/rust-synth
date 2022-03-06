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
    type: MidiMessageType,
    channel: u32,
    first_byte: u32,
    second_byte: u32,
}
