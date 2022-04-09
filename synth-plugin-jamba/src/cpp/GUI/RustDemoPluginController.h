#ifndef VST3_JAMBA_RustDemoPlugin_GUI_CONTROLLER_H
#define VST3_JAMBA_RustDemoPlugin_GUI_CONTROLLER_H

#include <pongasoft/VST/GUI/GUIController.h>
#include "../Plugin.h"

namespace JonathanDotExe::VST::RustDemoPlugin::GUI {

using namespace pongasoft::VST::GUI;

//------------------------------------------------------------------------
// RustDemoPluginController - Main GUI Controller
//------------------------------------------------------------------------
class RustDemoPluginController : public GUIController
{
public:
  //------------------------------------------------------------------------
  // UUID() method used to create the controller
  //------------------------------------------------------------------------
  static inline ::Steinberg::FUID UUID() { return RustDemoPluginControllerUID; };

  //------------------------------------------------------------------------
  // Factory method used to create the controller
  //------------------------------------------------------------------------
  static FUnknown *createInstance(void * /*iContext */) { return (IEditController *) new RustDemoPluginController(); }

public:
  // Constructor
  explicit RustDemoPluginController();

  // Destructor -- overridden for debugging purposes only
  ~RustDemoPluginController() override;

  // getGUIState
  GUIState *getGUIState() override { return &fState; }

protected:
  // initialize
  tresult PLUGIN_API initialize(FUnknown *context) override;

private:
  // The controller gets a copy of the parameters (defined in Plugin.h)
  RustDemoPluginParameters fParams;

  // The state accessible in the controller and views
  RustDemoPluginGUIState fState;
};

}

#endif // VST3_JAMBA_RustDemoPlugin_GUI_CONTROLLER_H

