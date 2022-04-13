#include <pongasoft/VST/AudioBuffer.h>
#include <pongasoft/VST/Debug/ParamTable.h>
#include <pongasoft/VST/Debug/ParamLine.h>


#include "RustDemoPluginProcessor.h"

#include "version.h"
#include "jamba_version.h"
#include "../RustSynthLib.h"

namespace JonathanDotExe::VST::RustDemoPlugin::RT {

//------------------------------------------------------------------------
// Constructor
//------------------------------------------------------------------------
RustDemoPluginProcessor::RustDemoPluginProcessor() : RTProcessor(RustDemoPluginControllerUID), fParams{}, fState{fParams}
{
  DLOG_F(INFO, "[%s] RustDemoPluginProcessor() - jamba: %s - plugin: v%s (%s)",
         stringPluginName,
         JAMBA_GIT_VERSION_STR,
         FULL_VERSION_STR,
         BUILD_ARCHIVE_ARCHITECTURE);
    demo_synth_initialize(synth);
  // in Debug mode we display the parameters in a table
#ifndef NDEBUG
  DLOG_F(INFO, "Parameters ---> \n%s", Debug::ParamTable::from(fParams).full().toString().c_str());
#endif
}

//------------------------------------------------------------------------
// Destructor - purely for debugging purposes
//------------------------------------------------------------------------
RustDemoPluginProcessor::~RustDemoPluginProcessor()
{
  DLOG_F(INFO, "~RustDemoPluginProcessor()");
  demo_synth_terminate(synth);
  synth = nullptr;
}

//------------------------------------------------------------------------
// RustDemoPluginProcessor::initialize - define input/outputs
//------------------------------------------------------------------------
tresult RustDemoPluginProcessor::initialize(FUnknown *context)
{
  DLOG_F(INFO, "RustDemoPluginProcessor::initialize()");

  tresult result = RTProcessor::initialize(context);
  if(result != kResultOk)
  {
    return result;
  }

  //------------------------------------------------------------------------
  // This is where you define inputs and outputs
  //------------------------------------------------------------------------
  //addAudioInput(STR16 ("Stereo In"), SpeakerArr::kStereo);
  addAudioOutput(STR16 ("Stereo Out"), SpeakerArr::kStereo);

  //------------------------------------------------------------------------
  // Displays the order in which the RT parameters will be saved (debug only)
  //------------------------------------------------------------------------
#ifndef NDEBUG
  using Key = Debug::ParamDisplay::Key;
  DLOG_F(INFO, "RT Save State - Version=%d --->\n%s",
         fParams.getRTSaveStateOrder().fVersion,
         Debug::ParamTable::from(getRTState(), true).keys({Key::kID, Key::kTitle}).full().toString().c_str());
#endif

  return result;
}

//------------------------------------------------------------------------
// RustDemoPluginProcessor::terminate - purely for debugging purposes
//------------------------------------------------------------------------
tresult RustDemoPluginProcessor::terminate()
{
  DLOG_F(INFO, "RustDemoPluginProcessor::terminate()");

  return RTProcessor::terminate();
}

//------------------------------------------------------------------------
// RustDemoPluginProcessor::setupProcessing
//------------------------------------------------------------------------
tresult RustDemoPluginProcessor::setupProcessing(ProcessSetup &setup)
{
  tresult result = RTProcessor::setupProcessing(setup);

  if(result != kResultOk)
    return result;


  SetupProcessingParams params;
  params.processing_mode = setup.processMode;
  params.sample_rate = setup.sampleRate;
  demo_synth_setup_processing(synth, params);

  DLOG_F(INFO,
         "RustDemoPluginProcessor::setupProcessing(%s, %s, maxSamples=%d, sampleRate=%f)",
         setup.processMode == kRealtime ? "Realtime" : (setup.processMode == kPrefetch ? "Prefetch" : "Offline"),
         setup.symbolicSampleSize == kSample32 ? "32bits" : "64bits",
         setup.maxSamplesPerBlock,
         setup.sampleRate);

  return result;
}

//------------------------------------------------------------------------
// RustDemoPluginProcessor::genericProcessInputs
// Implementation of the generic (32 and 64 bits) logic.
//------------------------------------------------------------------------
template<typename SampleType>
tresult RustDemoPluginProcessor::genericProcessInputs(ProcessData &data)
{
  if(data.numOutputs == 0)
  {
    // nothing to do
    return kResultOk;
  }

  AudioBuffers<SampleType> out(data.outputs[0], data.numSamples);
  SampleType* left = out.getLeftChannel().getBuffer();
  SampleType* right = out.getRightChannel().getBuffer();

  for (size_t i = 0; i < out.getNumSamples(); ++i) {
	  double l = 0;
	  double r = 0;
	  ProcessingParams params;
	  params.processing_mode = data.processMode;
	  demo_synth_process(synth, params, l, r);
	  if (left) {
		  left[i] = l;
	  }
	  if (right) {
		  right[i] = r;
	  }
  }

  // TODO use fState.fBypass to disable plugin effect...

  return kResultOk;
}

}
