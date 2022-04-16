#ifndef VST3_JAMBA_RustDemoPlugin_RUST_SYNTH_LIB_H
#define VST3_JAMBA_RustDemoPlugin_RUST_SYNTH_LIB_H

#include <inttypes.h>

struct RustDemoSynth;

struct SetupProcessingParams {
    int32_t processing_mode;
    int32_t sample_rate;
};

struct ProcessingParams {
    int32_t processing_mode;
};

struct NoteEvent {
    int32_t note;
    int32_t velocity;
};

extern "C" void demo_synth_initialize(RustDemoSynth*& synth);

extern "C" void demo_synth_setup_processing(RustDemoSynth* synth, SetupProcessingParams params);

extern "C" void demo_synth_process(RustDemoSynth* synth, ProcessingParams params, double& left, double& right);

extern "C" void demo_synth_note_event(RustDemoSynth* synth, ProcessingParams params, NoteEvent note);

extern "C" void demo_synth_terminate(RustDemoSynth* synth);

#endif 