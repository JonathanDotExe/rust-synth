#include <pongasoft/logging/logging.h>
#include <gtest/gtest.h>

namespace JonathanDotExe::VST::RustDemoPlugin::Test {

// PluginTest - Test1
TEST(PluginTest, Test1)
{
  DLOG_F(INFO, "Demonstrating test capability");

  ASSERT_EQ(4, 2+2);
}

}
