use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::Sample;

mod dsp;
mod voice;
mod synth;
mod midi;
mod io;

fn main() {
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
    let time_step: f32 = 1.0/(sample_rate as f32);
    let mut osc: dsp::Oscillator = dsp::Oscillator::new();
    let mut sample_count: u128 = 0;
    let mut s = 0.0;

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| { 
            for sample in data.iter_mut() {
                if sample_count % channels == 0 {
                    osc.process(time_step);
                    s = osc.synthesize() * 0.2;
                }
                *sample = Sample::from(&s);
                sample_count += 1;
            }
        },
        move |_err| {
            println!("Error while running audio thread!")
        },
    ).unwrap();
    stream.play().unwrap();

    //Audio
    let mut handler = io::AudioMidiHandler::new();
    handler.run();

    //Wait
    println!("Press ENTER to stop!");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}
