/*
 * test-queue.c - Simplified queue tests
 */

#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

#include "../src/queue.h"

static int var1 = 100, var2 = 200, var3 = 300;

void test_new_free(void)
{
    Queue *queue;
    
    printf("  test_new_free...");
    
    queue = queue_new();
    assert(queue != NULL);
    assert(queue_is_empty(queue));
    
    queue_free(queue);
    printf(" OK\n");
}

void test_push_pop_head(void)
{
    Queue *queue;
    
    printf("  test_push_pop_head...");
    
    queue = queue_new();
    
    /* Push values */
    assert(queue_push_head(queue, &var1));
    assert(queue_push_head(queue, &var2));
    assert(queue_push_head(queue, &var3));
    assert(!queue_is_empty(queue));
    
    /* Pop in reverse order (LIFO for head) */
    assert(queue_pop_head(queue) == &var3);
    assert(queue_pop_head(queue) == &var2);
    assert(queue_pop_head(queue) == &var1);
    assert(queue_is_empty(queue));
    
    /* Pop from empty queue */
    assert(queue_pop_head(queue) == QUEUE_NULL);
    
    queue_free(queue);
    printf(" OK\n");
}

void test_push_pop_tail(void)
{
    Queue *queue;
    
    printf("  test_push_pop_tail...");
    
    queue = queue_new();
    
    /* Push values to tail */
    assert(queue_push_tail(queue, &var1));
    assert(queue_push_tail(queue, &var2));
    assert(queue_push_tail(queue, &var3));
    
    /* Pop from head (FIFO queue behavior) */
    assert(queue_pop_head(queue) == &var1);
    assert(queue_pop_head(queue) == &var2);
    assert(queue_pop_head(queue) == &var3);
    
    queue_free(queue);
    printf(" OK\n");
}

void test_peek(void)
{
    Queue *queue;
    
    printf("  test_peek...");
    
    queue = queue_new();
    
    /* Peek empty queue */
    assert(queue_peek_head(queue) == QUEUE_NULL);
    assert(queue_peek_tail(queue) == QUEUE_NULL);
    
    /* Add values */
    queue_push_tail(queue, &var1);
    queue_push_tail(queue, &var2);
    
    /* Peek should not remove */
    assert(queue_peek_head(queue) == &var1);
    assert(queue_peek_head(queue) == &var1);  /* Still same */
    assert(queue_peek_tail(queue) == &var2);
    
    queue_free(queue);
    printf(" OK\n");
}

void test_double_ended(void)
{
    Queue *queue;
    
    printf("  test_double_ended...");
    
    queue = queue_new();
    
    /* Push to both ends */
    queue_push_head(queue, &var2);  /* Middle */
    queue_push_head(queue, &var1);  /* Front */
    queue_push_tail(queue, &var3);  /* Back */
    
    /* Should be: var1 -> var2 -> var3 */
    assert(queue_pop_head(queue) == &var1);
    assert(queue_pop_tail(queue) == &var3);
    assert(queue_pop_head(queue) == &var2);
    assert(queue_is_empty(queue));
    
    queue_free(queue);
    printf(" OK\n");
}

int main(void)
{
    printf("Running queue tests:\n");
    
    test_new_free();
    test_push_pop_head();
    test_push_pop_tail();
    test_peek();
    test_double_ended();
    
    printf("\nAll queue tests passed!\n");
    return 0;
}

