/* Copyright (c) Mark Harmstone 2020
 *
 * This file is part of Quibble.
 *
 * Quibble is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public Licence as published by
 * the Free Software Foundation, either version 3 of the Licence, or
 * (at your option) any later version.
 *
 * Quibble is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public Licence for more details.
 *
 * You should have received a copy of the GNU Lesser General Public Licence
 * along with Quibble.  If not, see <http://www.gnu.org/licenses/>. */

#pragma once

#include <efibind.h>
#include <efidef.h>
#include <efidevp.h>
#include <efiprot.h>
#include <eficon.h>
#include <efiapi.h>
#include <efierr.h>
#include <stddef.h>

extern EFI_SYSTEM_TABLE* systable;

#ifdef __cplusplus
extern "C" {
#endif

void itow(int v, wchar_t* w);
#ifdef _MSC_VER
int wcsicmp(const wchar_t* s1, const wchar_t* s2);
int stricmp(const char* s1, const char* s2);
int strnicmp(const char* s1, const char* s2, int n);
#endif
char*       stpcpy(char* dest, const char* src);
char*       stpcpy_utf16(char* dest, const wchar_t* src);
char*       hex_to_str(char* s, uint64_t v, unsigned int min_length = 1);
char*       dec_to_str(char* s, uint64_t v);
EFI_STATUS  utf8_to_utf16(wchar_t*      dest,
                          unsigned int  dest_max,
                          unsigned int* dest_len,
                          const char*   src,
                          unsigned int  src_len);
EFI_STATUS  utf16_to_utf8(char*          dest,
                          unsigned int   dest_max,
                          unsigned int*  dest_len,
                          const wchar_t* src,
                          unsigned int   src_len);
const char* error_string(EFI_STATUS Status);

#ifdef __cplusplus
}
#endif

#include <quibble-rs.h>

auto operator new(size_t size) -> void*;
auto operator new[](size_t size) -> void*;
auto operator new(size_t size, std::align_val_t al) -> void*;
auto operator new[](size_t size, std::align_val_t al) -> void*;
auto operator delete(void* ptr) -> void;
auto operator delete[](void* ptr) -> void;
auto operator delete(void* ptr, size_t al) -> void;
auto operator delete[](void* ptr, size_t al) -> void;
