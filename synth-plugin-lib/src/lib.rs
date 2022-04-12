use synth_lib::{audio::{AudioMidiProcessor, ProcessingInfo, ProcessingMode, SampleInfo}, synth::SynthEngine};

pub struct SetupProcessingParams {
    processing_mode: i32,
    sample_rate: i32,
}

pub struct ProcessingParams {
    processing_mode: i32,
}


pub struct RustDemoSynth {
    processor: Box<dyn AudioMidiProcessor>,
    info: ProcessingInfo,
    time: f64,
    sample_count: u64,
}

#[no_mangle]
pub extern "C" fn initialize(synth: &mut *mut RustDemoSynth) {
    *synth = Box::into_raw(Box::new(RustDemoSynth { processor: Box::new(SynthEngine::new()), info: ProcessingInfo::default(), time: 0.0, sample_count: 0 }))
}

pub unsafe extern "C" fn terminate(synth: *mut RustDemoSynth) {
    Box::from_raw(synth);
}

pub unsafe extern "C" fn setup_processing(synth: *mut RustDemoSynth, params: SetupProcessingParams) {
    //Create info
    let info = ProcessingInfo {
        sample_rate: params.sample_rate as u32,
        time_step: 1.0/(params.sample_rate as f64),
        processing_mode: match params.processing_mode {
            2 => ProcessingMode::Offline,
            _ => ProcessingMode::Realtime,
        },
    };
    //Setup
    match synth.as_mut() {
        Some(s) => {
            s.processor.setup(info);
            s.info = info;
            s.time = 0.0;
            s.sample_count = 0;
        },
        None => (),
    };
}

pub unsafe extern "C" fn process(synth: *mut RustDemoSynth, params: ProcessingParams, left: &mut f64, right: &mut f64) {
    match synth.as_mut() {
        Some(s) => {
            //Create info
            let info = SampleInfo {
                time: s.time,
                sample_count: s.sample_count,
                jitter: params.processing_mode != 0,
            };
            //Process
            (*left, *right) = s.processor.setup(info);
        },
        None => {
            (*left, *right) = (0, 0);
        },
    };
}