#include <string.h>
#include "vector.h"

Vector *vector_init(size_t initial_capacity) {
    Vector *vector = malloc(sizeof(Vector));
    if (!vector)
        return NULL;
    vector->data = malloc(sizeof(void *) * initial_capacity);
    if (!vector->data)
        return NULL;
    vector->size = 0;
    vector->capacity = initial_capacity;
    return vector;
}

int vector_append(Vector *vector, void *element) {
    if (!vector || !element)
        return 0;
    if (vector->capacity == vector->size) {
        void **new_data = realloc(vector->data, 
            2 * vector->capacity * sizeof(void *));
        if (!new_data)
            return 0;
        vector->data = new_data; 
        vector->capacity = 2 * vector->capacity;
    }
    vector->data[vector->size] = element;
    vector->size++;
    return 1;
}

int vector_is_empty(Vector *vector) {
    return vector->size == 0;
}

void *vector_index(Vector *vector, size_t index) {
    if (!vector || index >= vector->size)
        return NULL;
    return vector->data[index];
}

int vector_set(Vector *vector, size_t index, void *element) {
    if (!vector || index > vector->size)
        return 0;
    vector->data[index] = element;
    return 1;
}

void *vector_remove(Vector *vector, size_t index) {
    if (!vector || index > vector->size)
        return 0;
    void *element = vector->data[index];
    for (size_t i = index; i < vector->size - 1; i++)
        vector->data[i] = vector->data[i + 1];
    vector->size--;
    return element;
}

void vector_free(Vector *vector) {
    free(vector->data);
    free(vector);
}
