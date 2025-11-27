/*
 * slist.c - Singly-linked list implementation
 */

#include <stdlib.h>
#include "slist.h"

/* Internal structure */
struct _SListEntry {
    SListValue data;
    SListEntry *next;
};

void slist_free(SListEntry *list)
{
    SListEntry *entry = list;
    
    while (entry != NULL) {
        SListEntry *next = entry->next;
        free(entry);
        entry = next;
    }
}

SListEntry *slist_prepend(SListEntry **list, SListValue data)
{
    SListEntry *newentry = malloc(sizeof(SListEntry));
    
    if (newentry == NULL) {
        return NULL;
    }
    
    newentry->data = data;
    newentry->next = *list;
    *list = newentry;
    
    return newentry;
}

SListEntry *slist_append(SListEntry **list, SListValue data)
{
    SListEntry *newentry = malloc(sizeof(SListEntry));
    
    if (newentry == NULL) {
        return NULL;
    }
    
    newentry->data = data;
    newentry->next = NULL;
    
    if (*list == NULL) {
        *list = newentry;
    } else {
        SListEntry *rover;
        for (rover = *list; rover->next != NULL; rover = rover->next);
        rover->next = newentry;
    }
    
    return newentry;
}

SListValue slist_data(SListEntry *listentry)
{
    return listentry->data;
}

SListEntry *slist_next(SListEntry *listentry)
{
    return listentry->next;
}

SListEntry *slist_nth_entry(SListEntry *list, unsigned int n)
{
    SListEntry *entry = list;
    unsigned int i;
    
    for (i = 0; i < n; ++i) {
        if (entry == NULL) {
            return NULL;
        }
        entry = entry->next;
    }
    
    return entry;
}

SListValue slist_nth_data(SListEntry *list, unsigned int n)
{
    SListEntry *entry = slist_nth_entry(list, n);
    
    if (entry == NULL) {
        return SLIST_NULL;
    }
    return entry->data;
}

unsigned int slist_length(SListEntry *list)
{
    SListEntry *entry = list;
    unsigned int length = 0;
    
    while (entry != NULL) {
        ++length;
        entry = entry->next;
    }
    
    return length;
}

int slist_remove_entry(SListEntry **list, SListEntry *entry)
{
    if (*list == NULL || entry == NULL) {
        return 0;
    }
    
    if (*list == entry) {
        *list = entry->next;
    } else {
        SListEntry *rover = *list;
        
        while (rover != NULL && rover->next != entry) {
            rover = rover->next;
        }
        
        if (rover == NULL) {
            return 0;
        }
        
        rover->next = entry->next;
    }
    
    free(entry);
    return 1;
}

SListEntry *slist_find_data(SListEntry *list, SListEqualFunc callback, SListValue data)
{
    SListEntry *rover;
    
    for (rover = list; rover != NULL; rover = rover->next) {
        if (callback(rover->data, data) != 0) {
            return rover;
        }
    }
    
    return NULL;
}

/* Internal sort function using quicksort */
static SListEntry *slist_sort_internal(SListEntry **list, SListCompareFunc compare_func)
{
    SListEntry *pivot;
    SListEntry *rover;
    SListEntry *less_list = NULL, *more_list = NULL;
    SListEntry *less_list_end, *more_list_end;
    
    if (*list == NULL || (*list)->next == NULL) {
        return *list;
    }
    
    pivot = *list;
    rover = (*list)->next;
    
    while (rover != NULL) {
        SListEntry *next = rover->next;
        
        if (compare_func(rover->data, pivot->data) < 0) {
            rover->next = less_list;
            less_list = rover;
        } else {
            rover->next = more_list;
            more_list = rover;
        }
        
        rover = next;
    }
    
    less_list_end = slist_sort_internal(&less_list, compare_func);
    more_list_end = slist_sort_internal(&more_list, compare_func);
    
    *list = less_list;
    
    if (less_list == NULL) {
        *list = pivot;
    } else {
        less_list_end->next = pivot;
    }
    
    pivot->next = more_list;
    
    return (more_list == NULL) ? pivot : more_list_end;
}

void slist_sort(SListEntry **list, SListCompareFunc compare_func)
{
    slist_sort_internal(list, compare_func);
}

