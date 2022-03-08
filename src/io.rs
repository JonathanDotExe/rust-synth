use midir::{MidiInput, MidiInputConnection, Ignore};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::Sample;

pub struct ProcessingInfo {
    sample_rate: u32,
    time_step: f64,
}

pub trait AudioMidiProcessor {

    fn setup(info: ProcessingInfo);

    fn process(&mut self) -> f64;

}

pub struct AudioMidiHandler {
    _midiconn: MidiInputConnection<()>,
}

impl AudioMidiHandler {

    pub fn new<T: AudioMidiProcessor>(processor: T)-> AudioMidiHandler {
        //Audio
        // Create audio pipeline
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No device available!");
        let sample_rate = 48000;
        //TODO query configs
        /*let config = cpal::StreamConfig{
            channels: 2,
            buffer_size: cpal::BufferSize::Fixed(256),
            sample_rate: cpal::SampleRate(sample_rate),
        };*/
        let range = device.supported_output_configs().expect("Error!").next().expect("No config found!");
        println!("{} / {}", range.min_sample_rate().0, range.max_sample_rate().0);
        let channels = range.channels() as u128;
        let config = range.with_sample_rate(cpal::SampleRate(sample_rate)).config();
        let time_step: f64 = 1.0/(sample_rate as f64);

        let info = ProcessingInfo {sample_rate: sample_rate, time_step: time_step};
        let mut curr_ch = channels;
        let mut curr_s: f64 = 0.0;

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| { 
                for sample in data.iter_mut() {

                    if curr_ch > channels {
                        //Process
                        curr_s = processor.process();
                        curr_ch = 0;
                    }
                    *sample = curr_s as f32;
                    curr_ch += 1;
                }
            },
            move |_err| {
                println!("Error while running audio thread!")
            },
        ).unwrap();
        stream.play().unwrap();
        //MIDI
        let mut midiin = MidiInput::new("App-In").unwrap();
        midiin.ignore(Ignore::None);
        let ports = midiin.ports();
        if ports.len() <= 0 {
            panic!("No MIDI port found!");
        }
        let port = &ports[0];
        println!("Using port {}!", midiin.port_name(&port).unwrap());

        let midiconn = midiin.connect(port, "App-In", move |stamp, message, _| {
            println!("Midi Message {}: {:?} (len={}", stamp, message, message.len());
        }, ()).unwrap();

        return AudioMidiHandler {
            _midiconn: midiconn
        };
    }

}