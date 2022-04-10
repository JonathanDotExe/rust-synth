cmake_minimum_required(VERSION 3.14.0)

set(vst3sdk_SOURCE_DIR C:/UserProgramme/VST_SDK/vst3sdk)
if(NOT vst3sdk_SOURCE_DIR)
    message(FATAL_ERROR "Path to VST3 SDK is empty!")
endif()

project(RustDemoVST)

set(SMTG_VSTGUI_ROOT "${vst3sdk_SOURCE_DIR}")

add_subdirectory(${vst3sdk_SOURCE_DIR} ${PROJECT_BINARY_DIR}/vst3sdk)
smtg_enable_vst3_sdk()

smtg_add_vst3plugin(RustDemoVST
    source/version.h
    source/myplugincids.h
    source/mypluginprocessor.h
    source/mypluginprocessor.cpp
    source/myplugincontroller.h
    source/myplugincontroller.cpp
    source/mypluginentry.cpp
)

#- VSTGUI Wanted ----
if(SMTG_ADD_VSTGUI)
    target_sources(RustDemoVST
        PRIVATE
            resource/myplugineditor.uidesc
    )
    target_link_libraries(RustDemoVST
        PRIVATE
            vstgui_support
    )
    smtg_target_add_plugin_resources(RustDemoVST
        RESOURCES
            "resource/myplugineditor.uidesc"
    )
endif(SMTG_ADD_VSTGUI)
# -------------------

smtg_target_add_plugin_snapshots (RustDemoVST
    RESOURCES
        resource/907872F89764589596C6302D7C0EDB63_snapshot.png
        resource/907872F89764589596C6302D7C0EDB63_snapshot_2.0x.png
)

target_link_libraries(RustDemoVST
    PRIVATE
        sdk
)

if(SMTG_MAC)
    set(CMAKE_OSX_DEPLOYMENT_TARGET 10.12)
    smtg_target_set_bundle(RustDemoVST
        BUNDLE_IDENTIFIER at.jonathandotexe.rustsynth
        INFOPLIST "${CMAKE_CURRENT_LIST_DIR}/resource/Info.plist" PREPROCESS
    )
    smtg_target_set_debug_executable(RustDemoVST
        "/Applications/VST3PluginTestHost.app"
        "--pluginfolder;$(BUILT_PRODUCTS_DIR)"
    )
elseif(SMTG_WIN)
    target_sources(RustDemoVST PRIVATE 
        resource/win32resource.rc
    )
    if(MSVC)
        set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT RustDemoVST)

        smtg_target_set_debug_executable(RustDemoVST
            "$(ProgramW6432)/Steinberg/VST3PluginTestHost/VST3PluginTestHost.exe"
            "--pluginfolder \"$(OutDir)/\""
        )
    endif()
endif(SMTG_MAC)
