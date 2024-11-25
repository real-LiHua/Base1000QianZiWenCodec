#include <iostream>
#include <string>

#include <unicode/umachine.h>
#include <unicode/unistr.h>
#include <unicode/utypes.h>
std::string encode(std::string text) {

  icu::UnicodeString qzw1 = icu::UnicodeString::fromUTF8(QZW1);
  std::string result;
  icu::UnicodeString(qzw1).toUTF8String(result);
  std::cout << result << std::endl;
  return text;
};
