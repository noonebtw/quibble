#ifndef QUIBBLE_RS_H
#define QUIBBLE_RS_H

#pragma once

#include "quibble-rs_inner.h"

namespace nirgendwo {

	template<typename T>
	auto free(T* ptr) -> void {
		efi_free(reinterpret_cast<char*>(ptr));
	}

	template <typename T = uint8_t> auto malloc(size_t size) -> T * {
		return reinterpret_cast<T*>(efi_malloc(size));
	}
	}

#endif /* QUIBBLE_RS_H */
