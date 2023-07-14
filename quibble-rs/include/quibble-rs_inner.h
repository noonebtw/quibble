#ifndef QUIBBLE_RS_INNER_H
#define QUIBBLE_RS_INNER_H

#pragma once

#include "stdint.h"
#include "stdbool.h"


namespace nirgendwo {

struct OperatingSystem {
  const char *display_name;
  const int16_t *display_namew;
  const char *system_path;
  char *options;
};

struct QuibbleOptions {
  uint64_t timeout;
  const uint8_t *default_os;
  OperatingSystem *operating_systems;
  size_t operating_systems_len;
  size_t operating_systems_capacity;
};


extern "C" {

uint64_t add_10(uint64_t i);

void efi_free(void *ptr);

char *efi_malloc(size_t size);

void operating_system_destroy(OperatingSystem this_);

const QuibbleOptions *parse_quibble_options(const uint8_t *data, size_t len);

void quibble_options_destroy(const QuibbleOptions *this_);

} // extern "C"

} // namespace nirgendwo

#endif // QUIBBLE_RS_INNER_H
