#ifndef QUIBBLE_RS_INNER_H
#define QUIBBLE_RS_INNER_H

#pragma once

#include "stdint.h"
#include "stdbool.h"


namespace nirgendwo {

struct OperatingSystem {
  const uint8_t *display_name;
  const uint8_t *system_path;
  const uint8_t *options;
};

struct QuibbleOptions {
  uint64_t timeout;
  const uint8_t *default_os;
  const OperatingSystem *operating_systems;
  size_t operating_systems_len;
  size_t operating_systems_capacity;
};


extern "C" {

uint64_t add_10(uint64_t i);

void efi_free(void *ptr);

char *efi_malloc(size_t size);

void operating_system_destroy(OperatingSystem self);

QuibbleOptions parse_quibble_options(const uint8_t *data, size_t len);

void quibble_options_destroy(QuibbleOptions self);

} // extern "C"

} // namespace nirgendwo

#endif // QUIBBLE_RS_INNER_H
