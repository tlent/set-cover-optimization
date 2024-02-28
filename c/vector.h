#include <stdlib.h>

typedef struct {
    size_t size;
    size_t capacity;
    void **data;
} Vector;

Vector *vector_init(size_t initial_capacity);
int vector_append(Vector *vector, void *element);
int vector_is_empty(Vector *vector);
void *vector_index(Vector *vector, size_t index);
int vector_set(Vector *vector, size_t index, void *element);
void *vector_remove(Vector *vector, size_t index);
void vector_free(Vector *vector);
