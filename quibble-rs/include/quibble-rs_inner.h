#ifndef QUIBBLE_RS_INNER_H
#define QUIBBLE_RS_INNER_H

#pragma once

#include "stdint.h"
#include "stdbool.h"


namespace nirgendwo {

struct MyType {
  uint64_t i;
  float f;
};


extern "C" {

uint64_t add_10(uint64_t i);

void efi_free(void *ptr);

char *efi_malloc(size_t size);

MyType my_type_new();

void say_hello();

} // extern "C"

} // namespace nirgendwo

#endif // QUIBBLE_RS_INNER_H
