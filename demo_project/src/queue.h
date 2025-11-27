/*
 * queue.h - Double-ended queue interface
 */

#ifndef QUEUE_H
#define QUEUE_H

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declaration */
typedef struct _Queue Queue;

/* Value type */
typedef void *QueueValue;

/* Null value */
#define QUEUE_NULL ((void *) 0)

/**
 * Create a new queue.
 */
Queue *queue_new(void);

/**
 * Free the queue.
 */
void queue_free(Queue *queue);

/**
 * Add value to head of queue.
 */
int queue_push_head(Queue *queue, QueueValue data);

/**
 * Remove and return value from head.
 */
QueueValue queue_pop_head(Queue *queue);

/**
 * Peek at head value without removing.
 */
QueueValue queue_peek_head(Queue *queue);

/**
 * Add value to tail of queue.
 */
int queue_push_tail(Queue *queue, QueueValue data);

/**
 * Remove and return value from tail.
 */
QueueValue queue_pop_tail(Queue *queue);

/**
 * Peek at tail value without removing.
 */
QueueValue queue_peek_tail(Queue *queue);

/**
 * Check if queue is empty.
 */
int queue_is_empty(Queue *queue);

#ifdef __cplusplus
}
#endif

#endif /* QUEUE_H */

