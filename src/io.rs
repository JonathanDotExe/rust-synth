use midir::{MidiInput, Ignore};

struct AudioMidiHandler {
    midiin: Box<MidiInput>,
}

impl AudioMidiHandler {

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
        println!("Using port {}!", self.midiin.port_name(&port).unwrap());

        let conn = &midiin.connect(port, "App-In", move |stamp, message, _| {
            println!("Midi Message {}: {:?} (len={}", stamp, message, message.len());
        }, ()).unwrap();
    }

}