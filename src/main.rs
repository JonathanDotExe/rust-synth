use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::Sample;
use math::round;

fn saw_wave(freq: f32, time: f32) -> f32 {
    let a = time * freq;
    let prog: f32 = a - (round::floor(a as f64, 0) as f32);
    return prog * 2.0 - 1.0;
}

fn square_wave(freq: f32, time: f32) -> f32 {
    let a = time * freq;
    let prog: f32 = a - (round::floor(a as f64, 0) as f32);
    if (prog < 0.5) {
        return 1.0;
    }
    else {
        return -1.0;
    }
}

fn sine_wave(freq: f32, time: f32) -> f32 {
    let a = time * freq * std::f64::consts::PI as f32;
    return a.sin();
}

fn process(time: f32) -> f32 {
    return sine_wave(220.0, time) * 0.2;
}

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
    let config = range.with_sample_rate(cpal::SampleRate(sample_rate)).config();
    let mut time: f32 = 0.0;
    let time_step: f32 = 1.0/(sample_rate as f32);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = Sample::from(&process(time));
                time += time_step;
            }
        },
        move |_err| {
            println!("Error while running audio thread!")
        },
    ).unwrap();
    stream.play().unwrap();

    //Wait
    println!("Press ENTER to stop!");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}
