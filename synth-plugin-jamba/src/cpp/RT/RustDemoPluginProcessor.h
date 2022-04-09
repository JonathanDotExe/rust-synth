#ifndef VST3_JAMBA_RustDemoPlugin_RT_PROCESSOR_H
#define VST3_JAMBA_RustDemoPlugin_RT_PROCESSOR_H

#include <pongasoft/VST/RT/RTProcessor.h>
#include "../Plugin.h"

namespace JonathanDotExe::VST::RustDemoPlugin::RT {

using namespace pongasoft::VST::RT;

//------------------------------------------------------------------------
// RustDemoPluginProcessor - Real Time Processor
//------------------------------------------------------------------------
class RustDemoPluginProcessor : public RTProcessor
{
public:
  //------------------------------------------------------------------------
  // UUID() method used to create the processor
  //------------------------------------------------------------------------
  static inline ::Steinberg::FUID UUID() { return RustDemoPluginProcessorUID; };

  //------------------------------------------------------------------------
  // Factory method used to create the processor
  //------------------------------------------------------------------------
  static FUnknown *createInstance(void * /*iContext*/) { return (IAudioProcessor *) new RustDemoPluginProcessor(); }

public:
  // Constructor
  explicit RustDemoPluginProcessor();

  // Destructor
  ~RustDemoPluginProcessor() override;

  // getRTState
  RTState *getRTState() override { return &fState; }

  /** Called at first after constructor (setup input/output) */
  tresult PLUGIN_API initialize(FUnknown *context) override;

  // Called at the end before destructor
  tresult PLUGIN_API terminate() override;

  // This is where the setup happens which depends on sample rate, etc..
  tresult PLUGIN_API setupProcessing(ProcessSetup &setup) override;

protected:

  // genericProcessInputs<SampleType>
  template<typename SampleType>
  tresult genericProcessInputs(ProcessData &data);

  // processInputs32Bits
  tresult processInputs32Bits(ProcessData &data) override { return genericProcessInputs<Sample32>(data); }

  // processInputs64Bits
  tresult processInputs64Bits(ProcessData &data) override { return genericProcessInputs<Sample64>(data); }

private:
  // The processor gets a copy of the parameters (defined in Plugin.h)
  RustDemoPluginParameters fParams;

  // The state
  RustDemoPluginRTState fState;
};

}

#endif // VST3_JAMBA_RustDemoPlugin_RT_PROCESSOR_H

