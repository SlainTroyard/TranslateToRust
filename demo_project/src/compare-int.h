/*
 * compare-int.h - Integer comparison functions
 */

#ifndef COMPARE_INT_H
#define COMPARE_INT_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Check if two integers are equal.
 * Returns non-zero if equal, zero if not.
 */
int int_equal(void *location1, void *location2);

/**
 * Compare two integers.
 * Returns: negative if a < b, positive if a > b, zero if equal.
 */
int int_compare(void *location1, void *location2);

#ifdef __cplusplus
}
#endif

#endif /* COMPARE_INT_H */

