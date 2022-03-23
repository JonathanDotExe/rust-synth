#[derive(PartialEq, Debug)]
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

#[derive(Debug)]
pub struct MidiMessage {
    pub message_type: MidiMessageType,
    pub channel: u32,
    pub first_data: u32,
    pub second_data: u32,
}

impl MidiMessage {

    pub fn new(data: &[u8]) -> Result<MidiMessage, &'static str> {
        let mut message_type = MidiMessageType::SysEx;
        let mut channel: u32 = 0;
        let mut first_data: u32 = 0;
        let mut second_data: u32 = 0;

        //Message type
        if data.len() > 0 {
            match data[0] & 0xF0 {
                0x80 => message_type = MidiMessageType::NoteOff,
                0x90 => message_type = MidiMessageType::NoteOn,
                0xA0 => message_type = MidiMessageType::PolyphonicAftertouch,
                0xB0 => message_type = MidiMessageType::ControlChange,
                0xC0 => message_type = MidiMessageType::ProgramChange,
                0xD0 => message_type = MidiMessageType::MonophonicAftertouch,
                0xE0 => message_type = MidiMessageType::PitchBend,
                0xF0 => message_type = MidiMessageType::SysEx,
                _ =>  return Err("Invalid message type!"),
            }
            channel = (data[0] & 0x0F) as u32;
        }
        else {
            return Err("Empty message!")
        }

        //Data
        if data.len() > 1 {
            first_data = data[1] as u32;
        }
        if data.len() > 2 {
            second_data = data[2] as u32;
        }

        //Note on with zero velocity => Note off
        if message_type == MidiMessageType::NoteOn && second_data == 0 {
            message_type = MidiMessageType::NoteOff;
        }

        return Ok(MidiMessage {
            message_type: message_type,
            channel: channel,
            first_data: first_data,
            second_data: second_data,
        });
    }

    pub fn note(&self) -> u32 {
        return self.first_data;
    }
    pub fn velocity(&self) -> u32 {
        return self.second_data;
    }
    pub fn polyphonic_aftertouch(&self) -> u32 {
        return self.second_data;
    }
    pub fn control(&self) -> u32 {
        return self.first_data;
    }
    pub fn value(&self) -> u32 {
        return self.second_data;
    }
    pub fn program(&self) -> u32 {
        return self.first_data;
    }
    pub fn monophonic_aftertouch(&self) -> u32 {
        return self.second_data;
    }
    pub fn pitch_bend(&self) -> u32 {
        return self.first_data;
    }

    pub fn note_mut(&mut self) -> &mut u32 {
        return &mut self.first_data;
    }
    pub fn velocity_mut(&mut self) -> &mut u32 {
        return &mut self.second_data;
    }
    pub fn polyphonic_aftertouch_mut(&mut self) -> &mut u32 {
        return &mut self.second_data;
    }
    pub fn control_mut(&mut self) -> &mut u32 {
        return &mut self.first_data;
    }
    pub fn value_mut(&mut self) -> &mut u32 {
        return &mut self.second_data;
    }
    pub fn program_mut(&mut self) -> &mut u32 {
        return &mut self.first_data;
    }
    pub fn monophonic_aftertouch_mut(&mut self) -> &mut u32 {
        return &mut self.second_data;
    }
    pub fn pitch_bend_mut(&mut self) -> &mut u32 {
        return &mut self.first_data;
    }
}
