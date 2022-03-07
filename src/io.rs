use midir::{MidiInput, Ignore};

pub struct AudioMidiHandler {
    
}

impl AudioMidiHandler {

    pub fn new() -> AudioMidiHandler {
        return AudioMidiHandler {};
    }

    pub fn run(&mut self) {
        //Audio
        //TODO

        //Midi-Input
        let mut midiin = MidiInput::new("App-In").unwrap();
        midiin.ignore(Ignore::None);
        let ports = midiin.ports();
        if ports.len() <= 0 {
            panic!("No MIDI port found!");
        }
        let port = &ports[0];
        println!("Using port {}!", midiin.port_name(&port).unwrap());

        let _conn = &midiin.connect(port, "App-In", move |stamp, message, _| {
            println!("Midi Message {}: {:?} (len={}", stamp, message, message.len());
        }, ()).unwrap();
    }

}