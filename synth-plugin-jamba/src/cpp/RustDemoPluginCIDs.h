#ifndef VST3_JAMBA_RustDemoPlugin_CIDS_H
#define VST3_JAMBA_RustDemoPlugin_CIDS_H

#include <pluginterfaces/base/funknown.h>
#include <pluginterfaces/vst/vsttypes.h>

namespace JonathanDotExe::VST::RustDemoPlugin {

//------------------------------------------------------------------------
// These 2 IDs are used in RustDemoPlugin_VST2.cpp and RustDemoPlugin_VST3.cpp to create
// the processor (RT) and controller (GUI). Those IDs are unique and have
// been generated automatically. Using different ids for Debug/Release
// targets so that both plugins can live side by side.
//------------------------------------------------------------------------
#ifndef NDEBUG
static const ::Steinberg::FUID RustDemoPluginProcessorUID(0xbeab41c5, 0x4dd54f67, 0xb20ff239, 0xd7852792);
static const ::Steinberg::FUID RustDemoPluginControllerUID(0x9acc15b4, 0xbcaa4450, 0x9185ee8d, 0x3008a3ed);
#else
static const ::Steinberg::FUID RustDemoPluginProcessorUID(0x0fa74bc0, 0x5d6442ec, 0x8e7f6a62, 0x0b8d0c5b);
static const ::Steinberg::FUID RustDemoPluginControllerUID(0x1e4768b6, 0x31324582, 0x9b203aa3, 0x58c451c7);
#endif

//------------------------------------------------------------------------
// Parameters and Custom view ids
//------------------------------------------------------------------------
enum ERustDemoPluginParamID : Steinberg::Vst::ParamID
{
  // although NOT a requirement, I like to start at 1000 so that they are all 4 digits.
  // the grouping and numbering is arbitrary and you can use whatever makes sense for your case.

  // the bypass parameter which has a special meaning to the host
  kBypass = 1000,
};

}

#endif // VST3_JAMBA_RustDemoPlugin_CIDS_H