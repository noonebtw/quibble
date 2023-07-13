#include "stdint.h"
#include "wchar.h"
#include "stdbool.h"


namespace nirgendwo {

struct MyType {
  uint64_t i;
  float f;
};


extern "C" {

uint64_t add_10(uint64_t i);

MyType my_type_new();

} // extern "C"

} // namespace nirgendwo
