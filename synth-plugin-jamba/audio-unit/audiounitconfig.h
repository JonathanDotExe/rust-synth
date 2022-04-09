#include "version.h"

// Check https://developer.apple.com/library/archive/documentation/General/Conceptual/ExtensibilityPG/AudioUnit.html for various types

/* Bundle Identifier */
#define kAudioUnitBundleIdentifier	JonathanDotExe.vst3.RustDemoPlugin.audiounit

/* Version Number (computed in version.h from version in CMakeLists.txt) */
#define kAudioUnitVersion			AU_VERSION_INT

/* Company Name + Effect Name */
#define kAUPluginName 				JonathanDotExe: RustDemoPlugin

/* A product name for the audio unit, such as TremoloUnit */
#define kAUPluginDescription 		RustDemoPlugin

/*
  The specific variant of the Audio Unit. The four possible types and their values are:
  Effect (aufx), Generator (augn), Instrument (aumu), and Music Effect (aufm).
 */
#define kAUPluginType 				aufx

/* A subtype code for the audio unit, such as tmlo. This value must be exactly 4 alphanumeric characters. */
#define kAUPluginSubType 			unkw

/* A manufacturer code for the audio unit, such as Aaud. This value must be exactly 4 alphanumeric characters.
 * This value should be unique across audio units of the same manufacturer
 * Manufacturer OSType should have at least one non-lower case character */
#define kAUPluginManufacturer 		Jojo

// Definitions for the resource file
#define kAudioUnitName				      "JonathanDotExe: RustDemoPlugin" // same as kAUPluginName
#define kAudioUnitDescription	      "RustDemoPlugin" // same as kAUPluginDescription
#define kAudioUnitType				      'aufx' // same as kAUPluginType
#define kAudioUnitComponentSubType	'unkw' // same as kAUPluginSubType
#define kAudioUnitComponentManuf    'Jojo' // same as kAUPluginManufacturer

#define kAudioUnitCarbonView		1		// if 0 no Carbon view support will be added
