# Makefile for demo_project

CC = gcc
CFLAGS = -Wall -Wextra -g -I./src

SRC_DIR = src
TEST_DIR = test

# Source files
SOURCES = $(SRC_DIR)/slist.c \
          $(SRC_DIR)/queue.c \
          $(SRC_DIR)/compare-int.c

# Test executables
TEST_SLIST = $(TEST_DIR)/test-slist
TEST_QUEUE = $(TEST_DIR)/test-queue

.PHONY: all clean test

all: $(TEST_SLIST) $(TEST_QUEUE)

$(TEST_SLIST): $(TEST_DIR)/test-slist.c $(SOURCES)
	$(CC) $(CFLAGS) -o $@ $^

$(TEST_QUEUE): $(TEST_DIR)/test-queue.c $(SOURCES)
	$(CC) $(CFLAGS) -o $@ $^

test: all
	@echo "Running slist tests..."
	@./$(TEST_SLIST)
	@echo ""
	@echo "Running queue tests..."
	@./$(TEST_QUEUE)

clean:
	rm -f $(TEST_SLIST) $(TEST_QUEUE)
	rm -f $(SRC_DIR)/*.o $(TEST_DIR)/*.o

