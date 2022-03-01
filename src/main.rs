use cpal::traits::HostTrait;

fn process(sample: mut& f32) {
    //TODO audio processing
}

fn main() {
    // Create audio pipeline
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No device available!");
    //TODO query configs
    let config = cpal::StreamConfig{
        channels: 2,
        buffer_size: cpal::BufferSize::Fixed(64),
        sample_rate: cpal::SampleRate(44100),
    };

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _ &cpal::OutputCallbackInfo| {
            for (sample in data.iter_mut()) {
                process(mut& sample);
            }
        },
        move |err| {
            printLn!("Error while running audio thread!")
        }
    )

    //Wait
    printLn!("Press ENTER to stop!")
    let mut line = String::new();
    std::io::stdin().read_line().unwrap(&mut line);
}
