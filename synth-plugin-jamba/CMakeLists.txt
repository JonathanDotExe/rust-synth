cmake_minimum_required(VERSION 3.17)

set(RUST_SYNTH_LIB_LOCATION "../../target/debug/synth_plugin_lib" CACHE FILEPATH "Location of the rust synth library!")

# build VST 2?
option(JAMBA_ENABLE_VST2 "Use VST2" OFF)

# build Audio Unit?
option(JAMBA_ENABLE_AUDIO_UNIT "Enable Audio Unit" ON)

# Download VST SDK if not installed?
option(JAMBA_DOWNLOAD_VSTSDK "Download VST SDK if not installed" OFF)

# Sets the deployment target for macOS
set(JAMBA_MACOS_DEPLOYMENT_TARGET "10.14" CACHE STRING "macOS deployment target")

# To use local jamba install, uncomment the following line (no download)
set(JAMBA_ROOT_DIR "")
#set(JAMBA_ROOT_DIR "${CMAKE_CURRENT_LIST_DIR}/../../pongasoft/jamba")

# download jamba framework
include(fetch_jamba.cmake)

# Determine proper architecture for the project
include("${JAMBA_ROOT_DIR}/cmake/JambaSetArchitecture.cmake")

# Plugin version
set(PLUGIN_MAJOR_VERSION 1)
set(PLUGIN_MINOR_VERSION 0)
set(PLUGIN_PATCH_VERSION 0)
set(PLUGIN_VERSION "${PLUGIN_MAJOR_VERSION}.${PLUGIN_MINOR_VERSION}.${PLUGIN_PATCH_VERSION}")

# Project
project("RustDemoPlugin")

# To use local googletest install, uncomment the following line (no download) and modify the path accordingly
set(GOOGLETEST_ROOT_DIR "")
#set(GOOGLETEST_ROOT_DIR "${CMAKE_CURRENT_LIST_DIR}/../../google/googletest")

# Include Jamba
include("${JAMBA_ROOT_DIR}/jamba.cmake")

set(CPP_SOURCES "src/cpp")

# Generating the version.h header file which contains the plugin version (to make sure it is in sync with the version
# defined here)
set(VERSION_DIR "${CMAKE_BINARY_DIR}/generated")
configure_file("${CPP_SOURCES}/version.h.in" "${VERSION_DIR}/version.h")

# Source files for VST2
if (JAMBA_ENABLE_VST2)
  set(vst2_sources
      "${CPP_SOURCES}/RustDemoPlugin_VST2.cpp"
      )
endif ()

# Source files for VST3
set(vst_sources
    "${CPP_SOURCES}/RustDemoPlugin_VST3.cpp"

    "${CPP_SOURCES}/Plugin.h"
    "${CPP_SOURCES}/RustDemoPluginCIDs.h"

    "${CPP_SOURCES}/RT/RustDemoPluginProcessor.h"
    "${CPP_SOURCES}/RT/RustDemoPluginProcessor.cpp"

    "${CPP_SOURCES}/GUI/RustDemoPluginController.h"
    "${CPP_SOURCES}/GUI/RustDemoPluginController.cpp"
    )

# Location of resources
set(RES_DIR "${CMAKE_CURRENT_LIST_DIR}/resource")

# List of resources (images)
set(vst_resources
#    "${RES_DIR}/background.png"
)

# Location of the test cases
set(TEST_DIR "${CMAKE_CURRENT_LIST_DIR}/test/cpp")

# List of test cases
set(test_case_sources
    "${TEST_DIR}/test-RustDemoPlugin.cpp"
)

# Finally invoke jamba_add_vst_plugin
jamba_add_vst_plugin(
    TARGET                "JonathanDotExe_RustDemoPlugin" # name of CMake target for the plugin
    RELEASE_FILENAME      "RustDemoPlugin" # filename for the plugin (RustDemoPlugin.vst3)
    RELEASE_SNAPSHOTS     "${RES_DIR}/0FA74BC05D6442EC8E7F6A620B8D0C5B_snapshot.png" "${RES_DIR}/0FA74BC05D6442EC8E7F6A620B8D0C5B_snapshot_2.0x.png"
    ARCHIVE_ARCHITECTURE  "${JAMBA_ARCHIVE_ARCHITECTURE}"
    TARGETS_PREFIX        "jmb_" # prefix used by all targets directly linked to this plugin
    VST_SOURCES           "${vst_sources}" "${vst2_sources}" # the source files that compose the plugin
    INCLUDE_DIRECTORIES   "${VERSION_DIR}" # we add the version folder to the list of includes
    UIDESC                "${RES_DIR}/RustDemoPlugin.uidesc" # the main xml file for the GUI
    RESOURCES             "${vst_resources}" # the resources for the GUI (png files)
    TEST_CASE_SOURCES     "${test_case_sources}" # the source files containing the test cases
#    TEST_SOURCES          "${CPP_SOURCES}/xxx.cpp" # individual files required for compiling tests but we could add ${vst_sources} if we needed more
    TEST_LINK_LIBRARIES   "jamba" # the libraries needed for linking the tests (if your tests don't depend on Jamba you can comment this out)
    LINK_LIBRARIES        "${RUST_SYNTH_LIB_LOCATION}"
)

target_link_libraries("JonathanDotExe_RustDemoPlugin" PUBLIC "wsock32" "ws2_32" "userenv" "bcrypt") # These are important or otherwise the static rust library won't load