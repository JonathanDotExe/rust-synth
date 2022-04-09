#include "RustDemoPluginController.h"

namespace JonathanDotExe::VST::RustDemoPlugin::GUI {

//------------------------------------------------------------------------
// Constructor
//------------------------------------------------------------------------
RustDemoPluginController::RustDemoPluginController() : GUIController("RustDemoPlugin.uidesc"), fParams{}, fState{fParams}
{
  DLOG_F(INFO, "RustDemoPluginController()");
}

//------------------------------------------------------------------------
// Destructor
//------------------------------------------------------------------------
RustDemoPluginController::~RustDemoPluginController()
{
  DLOG_F(INFO, "~RustDemoPluginController()");
}

//------------------------------------------------------------------------
// RustDemoPluginController::initialize
//------------------------------------------------------------------------
tresult RustDemoPluginController::initialize(FUnknown *context)
{
  tresult res = GUIController::initialize(context);

  //------------------------------------------------------------------------
  // In debug mode this code displays the order in which the GUI parameters
  // will be saved
  //------------------------------------------------------------------------
#ifndef NDEBUG
  if(res == kResultOk)
  {
    using Key = Debug::ParamDisplay::Key;
    DLOG_F(INFO, "GUI Save State - Version=%d --->\n%s",
           fParams.getGUISaveStateOrder().fVersion,
           Debug::ParamTable::from(getGUIState(), true).keys({Key::kID, Key::kTitle}).full().toString().c_str());
  }
#endif

  return res;
}

}
