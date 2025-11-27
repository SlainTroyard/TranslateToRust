# Demo Project - C Data Structures

A minimal C data structures library for demonstrating Rustify translation.

## 📁 Structure

```
demo_project/
├── src/
│   ├── slist.h        # Singly-linked list header
│   ├── slist.c        # Singly-linked list implementation
│   ├── queue.h        # Double-ended queue header
│   ├── queue.c        # Double-ended queue implementation
│   ├── compare-int.h  # Integer comparison functions header
│   └── compare-int.c  # Integer comparison functions
├── test/
│   ├── test-slist.c   # Singly-linked list tests
│   └── test-queue.c   # Queue tests
├── Makefile
└── README.md
```

## 📊 Statistics

- **Source files**: 6 (3 modules × 2 files each)
- **Test files**: 2
- **Total lines**: ~500

## 🔨 Build & Test

```bash
# Build all
make

# Run tests
make test

# Clean
make clean
```

## 🦀 Translation with Rustify

```bash
# Translate to Rust
rustify translate ./demo_project/src ./demo_project_rs

# Incremental translation after changes
rustify translate ./demo_project/src ./demo_project_rs --incremental
```

## 📋 Features Demonstrated

### Singly-Linked List (slist)
- Prepend/append operations
- Index-based access
- Element removal
- Search with callback
- Sorting (quicksort)

### Queue (double-ended)
- Push/pop from both ends
- Peek without removal
- Empty check
- FIFO and LIFO usage

### Compare Functions
- Integer equality check
- Integer comparison for sorting

