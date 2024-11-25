#include "config.h"
#include "decode.h"
#include "encode.h"
#include <CLI/CLI.hpp>
#include <iostream>
#include <unicode/umachine.h>
#include <unicode/unistr.h>
#include <unicode/utypes.h>
int main(int argc, char **argv) {
  CLI::App app{PACKAGE_NAME};
  std::string text;
  bool flag{false};
  auto e = app.add_flag("-e,--encode", flag, "编码输入文本");
  auto d = app.add_flag("-d,--decode", "解码输入文本")->excludes(e);
  app.add_option("text", text, "文本")->required();
  CLI11_PARSE(app, argc, argv);
  if (flag) {
    std::cout << encode(text) << std::endl;
  } else {
    std::cout << decode(text) << std::endl;
  }
  return 0;
}
