//------------------------------------------------------------------------
// Copyright(c) 2022 JonathanDotExe.
//------------------------------------------------------------------------

#pragma once

#include "pluginterfaces/base/funknown.h"
#include "pluginterfaces/vst/vsttypes.h"

namespace JonathanDotExe__DemoSynth {
//------------------------------------------------------------------------
static const Steinberg::FUID kRustDemoVSTProcessorUID (0x907872F8, 0x97645895, 0x96C6302D, 0x7C0EDB63);
static const Steinberg::FUID kRustDemoVSTControllerUID (0x72709BC2, 0x2E465608, 0xBD0C92AB, 0xAA020E00);

#define RustDemoVSTVST3Category "Instrument"

//------------------------------------------------------------------------
} // namespace JonathanDotExe__DemoSynth
