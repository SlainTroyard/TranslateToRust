/*
 * slist.h - Singly-linked list interface
 * 
 * A simple singly-linked list implementation for demonstration.
 */

#ifndef SLIST_H
#define SLIST_H

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declaration */
typedef struct _SListEntry SListEntry;

/* Value type stored in the list */
typedef void *SListValue;

/* Null value */
#define SLIST_NULL ((void *) 0)

/* Comparison function type */
typedef int (*SListCompareFunc)(SListValue value1, SListValue value2);

/* Equality function type */
typedef int (*SListEqualFunc)(SListValue value1, SListValue value2);

/**
 * Free an entire list.
 */
void slist_free(SListEntry *list);

/**
 * Prepend a value to the start of a list.
 * Returns the new entry, or NULL on failure.
 */
SListEntry *slist_prepend(SListEntry **list, SListValue data);

/**
 * Append a value to the end of a list.
 * Returns the new entry, or NULL on failure.
 */
SListEntry *slist_append(SListEntry **list, SListValue data);

/**
 * Get the next entry in the list.
 */
SListEntry *slist_next(SListEntry *listentry);

/**
 * Get the data stored at an entry.
 */
SListValue slist_data(SListEntry *listentry);

/**
 * Get the entry at index n.
 */
SListEntry *slist_nth_entry(SListEntry *list, unsigned int n);

/**
 * Get the data at index n.
 */
SListValue slist_nth_data(SListEntry *list, unsigned int n);

/**
 * Get the length of the list.
 */
unsigned int slist_length(SListEntry *list);

/**
 * Remove an entry from the list.
 * Returns non-zero if successful.
 */
int slist_remove_entry(SListEntry **list, SListEntry *entry);

/**
 * Find an entry by value.
 */
SListEntry *slist_find_data(SListEntry *list, SListEqualFunc callback, SListValue data);

/**
 * Sort the list.
 */
void slist_sort(SListEntry **list, SListCompareFunc compare_func);

#ifdef __cplusplus
}
#endif

#endif /* SLIST_H */

