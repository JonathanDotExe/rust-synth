//------------------------------------------------------------------------------------------------------------
// This file contains the standard boilerplate code that VST3 sdk requires to instantiate the plugin
// components
//------------------------------------------------------------------------------------------------------------
#include "RustDemoPluginCIDs.h"

#include "version.h"
#include "RT/RustDemoPluginProcessor.h"
#include "GUI/RustDemoPluginController.h"

#include <pongasoft/VST/PluginFactory.h>

using namespace pongasoft::VST;

//------------------------------------------------------------------------
//  Module init/exit
//------------------------------------------------------------------------

//------------------------------------------------------------------------
// called after library was loaded
/*bool InitModule()
{
  return true;
}*/

//------------------------------------------------------------------------
// called after library is unloaded
/*bool DeinitModule()
{
  return true;
}*/

//------------------------------------------------------------------------
//  VST3 Plugin Main entry point
//------------------------------------------------------------------------
SMTG_EXPORT_SYMBOL Steinberg::IPluginFactory* PLUGIN_API GetPluginFactory()
{
  return JambaPluginFactory::GetVST3PluginFactory<
    JonathanDotExe::VST::RustDemoPlugin::RT::RustDemoPluginProcessor, // processor class (Real Time)
    JonathanDotExe::VST::RustDemoPlugin::GUI::RustDemoPluginController // controller class (GUI)
  >("JonathanDotExe", // company/vendor
    "https://jonathandotexe/github.com", // url
    "support@JonathanDotExe.com", // email
    stringPluginName, // plugin name
    FULL_VERSION_STR, // plugin version
    Vst::PlugType::kInstrument // plugin category (can be changed to other like kInstrument, etc...)
   );
}
