use synth_lib::{audio::{AudioMidiProcessor, ProcessingInfo, ProcessingMode}, synth::SynthEngine};

pub struct ProcessingParams {
    processing_mode: i32,
    sample_rate: i32,
}

pub struct RustDemoSynth {
    processor: Box<dyn AudioMidiProcessor>,
}

#[no_mangle]
pub extern "C" fn initialize(synth: &mut *mut RustDemoSynth) {
    *synth = Box::into_raw(Box::new(RustDemoSynth { processor: Box::new(SynthEngine::new()) }))
}

pub unsafe extern "C" fn terminate(synth: *mut RustDemoSynth) {
    Box::from_raw(synth);
}

pub unsafe extern "C" fn setup_processing(synth: *mut RustDemoSynth, params: ProcessingParams) {
    let info = ProcessingInfo {
        sample_rate: params.sample_rate as u32,
        time_step: 1.0/(params.sample_rate as f64),
        processing_mode: match params.processing_mode { //TODO mode changes in realtime
            0 => ProcessingMode::Realtime,
            1 => ProcessingMode::Preload,
            2 => ProcessingMode::Offline,
            _ => ProcessingMode::Realtime,
        },
    };
    match synth.as_mut() {
        Some(s) => s.processor.setup(info),
        None => (),
    };
}