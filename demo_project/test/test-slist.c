/*
 * test-slist.c - Simplified singly-linked list tests
 */

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

#include "../src/slist.h"
#include "../src/compare-int.h"

static int var1 = 10, var2 = 20, var3 = 30, var4 = 40;

/* Helper: create a test list with 4 elements */
static SListEntry *create_test_list(void)
{
    SListEntry *list = NULL;
    
    slist_append(&list, &var1);
    slist_append(&list, &var2);
    slist_append(&list, &var3);
    slist_append(&list, &var4);
    
    return list;
}

void test_append(void)
{
    SListEntry *list = NULL;
    
    printf("  test_append...");
    
    assert(slist_append(&list, &var1) != NULL);
    assert(slist_append(&list, &var2) != NULL);
    assert(slist_length(list) == 2);
    
    assert(slist_nth_data(list, 0) == &var1);
    assert(slist_nth_data(list, 1) == &var2);
    
    slist_free(list);
    printf(" OK\n");
}

void test_prepend(void)
{
    SListEntry *list = NULL;
    
    printf("  test_prepend...");
    
    assert(slist_prepend(&list, &var1) != NULL);
    assert(slist_prepend(&list, &var2) != NULL);
    assert(slist_length(list) == 2);
    
    /* Prepend adds to front, so var2 is first */
    assert(slist_nth_data(list, 0) == &var2);
    assert(slist_nth_data(list, 1) == &var1);
    
    slist_free(list);
    printf(" OK\n");
}

void test_length(void)
{
    SListEntry *list;
    
    printf("  test_length...");
    
    list = create_test_list();
    assert(slist_length(list) == 4);
    assert(slist_length(NULL) == 0);
    
    slist_free(list);
    printf(" OK\n");
}

void test_nth_data(void)
{
    SListEntry *list;
    
    printf("  test_nth_data...");
    
    list = create_test_list();
    
    assert(slist_nth_data(list, 0) == &var1);
    assert(slist_nth_data(list, 1) == &var2);
    assert(slist_nth_data(list, 2) == &var3);
    assert(slist_nth_data(list, 3) == &var4);
    assert(slist_nth_data(list, 4) == NULL);  /* Out of bounds */
    
    slist_free(list);
    printf(" OK\n");
}

void test_remove(void)
{
    SListEntry *list;
    SListEntry *entry;
    
    printf("  test_remove...");
    
    list = create_test_list();
    
    /* Remove middle element */
    entry = slist_nth_entry(list, 2);
    assert(slist_remove_entry(&list, entry) != 0);
    assert(slist_length(list) == 3);
    
    /* Remove first element */
    entry = slist_nth_entry(list, 0);
    assert(slist_remove_entry(&list, entry) != 0);
    assert(slist_length(list) == 2);
    
    slist_free(list);
    printf(" OK\n");
}

void test_find(void)
{
    SListEntry *list;
    SListEntry *found;
    int val;
    
    printf("  test_find...");
    
    list = create_test_list();
    
    /* Find existing value */
    val = 20;
    found = slist_find_data(list, int_equal, &val);
    assert(found != NULL);
    assert(slist_data(found) == &var2);
    
    /* Find non-existing value */
    val = 999;
    found = slist_find_data(list, int_equal, &val);
    assert(found == NULL);
    
    slist_free(list);
    printf(" OK\n");
}

void test_sort(void)
{
    SListEntry *list = NULL;
    int values[] = {50, 10, 40, 20, 30};
    int expected[] = {10, 20, 30, 40, 50};
    unsigned int i;
    
    printf("  test_sort...");
    
    /* Create unsorted list */
    for (i = 0; i < 5; ++i) {
        slist_append(&list, &values[i]);
    }
    
    slist_sort(&list, int_compare);
    
    /* Verify sorted order */
    for (i = 0; i < 5; ++i) {
        int *val = (int *)slist_nth_data(list, i);
        assert(*val == expected[i]);
    }
    
    slist_free(list);
    printf(" OK\n");
}

int main(void)
{
    printf("Running slist tests:\n");
    
    test_append();
    test_prepend();
    test_length();
    test_nth_data();
    test_remove();
    test_find();
    test_sort();
    
    printf("\nAll slist tests passed!\n");
    return 0;
}

