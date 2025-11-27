/**
 * utils.h - Utility functions
 */

#ifndef UTILS_H
#define UTILS_H

#include "types.h"

/* Compare two integers */
int compare_int(const void *a, const void *b);

/* Compare two strings */
int compare_string(const void *a, const void *b);

/* Allocate and copy an integer */
int *int_dup(int value);

/* Allocate and copy a string */
char *string_dup(const char *str);

#endif /* UTILS_H */

