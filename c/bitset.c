#include <math.h>
#include <string.h>
#include <stdio.h>
#include "bitset.h"

size_t ARRAY_BITS = sizeof(TYPE) * 8;

BitSet *bitset_init(size_t capacity) {
    BitSet *bitset = malloc(sizeof(BitSet));
    if (!bitset)
        return NULL;
    size_t necessary_arrays = 1 + capacity / ARRAY_BITS;
    bitset->bit_arrays = calloc(necessary_arrays, sizeof(TYPE));
    bitset->bit_array_count = necessary_arrays;
    bitset->capacity = capacity;
    bitset->size = 0;
    return bitset;
}

int bitset_add(BitSet *bitset, int value) {
    if (!bitset || value > bitset->capacity)
        return 0;
    size_t array = value / ARRAY_BITS;
    int position = value % ARRAY_BITS;
    TYPE bit = 1;
    bitset->bit_arrays[array] |= (bit << position);
    ++bitset->size;
    return 1;
}

int bitset_remove(BitSet *bitset, int value) {
    if (!bitset || value > bitset->capacity)
        return 0;
    size_t array = value / ARRAY_BITS;
    int position = value % ARRAY_BITS;
    TYPE bit = 1;
    bitset->bit_arrays[array] &= ~(bit << position);
    --bitset->size;
    return 1;
}

int bitset_contains(BitSet *bitset, int value) {
    if (!bitset || value > bitset->capacity || bitset_is_empty(bitset))
        return 0;
    size_t array = value / ARRAY_BITS;
    int position = value % ARRAY_BITS;
    TYPE bit = 1;
    TYPE bit_array = bitset->bit_arrays[array];
    return bit_array != 0 && (bit_array & (bit << position)) != 0;
}

int bitset_is_subset(BitSet *superset, BitSet *subset) {
    if (!superset || !subset || superset->capacity != subset->capacity
            || subset->size > superset->size)
        return 0;
    for (size_t i = 0; i < superset->bit_array_count; i++) {
        TYPE superset_array = superset->bit_arrays[i];
        if ((superset_array | subset->bit_arrays[i]) != superset_array)
            return 0;
    }
    return 1;
}

int bitset_is_empty(BitSet *bitset) {
    if (!bitset)
        return 1;
    return bitset->size == 0;
}

BitSet *bitset_difference(BitSet *a, BitSet *b) {
    if (!a || !b || a->capacity != b->capacity)
        return NULL;
    BitSet *difference_bitset = bitset_init(a->capacity);
    if (!difference_bitset)
        return NULL;
    size_t total_bit_count = 0;
    for (size_t i = 0; i < difference_bitset->bit_array_count; i++) {
        TYPE a_array = a->bit_arrays[i];
        TYPE b_array = b->bit_arrays[i];
        TYPE diff_array = a_array & ~b_array;
        difference_bitset->bit_arrays[i] = diff_array;
        TYPE bit_array = diff_array;
        TYPE bit_count;
        for (bit_count = 0; bit_array; bit_count++) {
            bit_array &= bit_array - 1;
        }
        total_bit_count += bit_count;
    }
    difference_bitset->size = total_bit_count;
    return difference_bitset;
}

BitSet *bitset_union(BitSet *a, BitSet *b) {
    if (!a || !b || a->capacity != b->capacity)
        return NULL;
    BitSet *union_bitset = bitset_init(a->capacity);
    if (!union_bitset)
        return NULL;
    size_t total_bit_count = 0;
    for (size_t i = 0; i < union_bitset->bit_array_count; i++) {
        TYPE a_array = a->bit_arrays[i];
        TYPE b_array = b->bit_arrays[i];
        union_bitset->bit_arrays[i] = a_array | b_array;
        TYPE bit_array = union_bitset->bit_arrays[i];
        TYPE bit_count;
        for (bit_count = 0; bit_array; bit_count++) {
            bit_array &= bit_array - 1;
        }
        total_bit_count += bit_count;
    }
    union_bitset->size = total_bit_count;
    return union_bitset;
}

BitSet *bitset_copy(BitSet *bitset) {
    if (!bitset)
        return NULL;
    BitSet *new_bitset = bitset_init(bitset->capacity);
    if (!new_bitset)
        return NULL;
    new_bitset->size = bitset->size;
    memcpy(new_bitset->bit_arrays, bitset->bit_arrays,
        sizeof(TYPE) * (bitset->bit_array_count));
    return new_bitset;
}

void bitset_print(BitSet *bitset) {
    if (bitset) {
        printf("[");
        int first_print = 1;
        for (size_t i = 0; i <= bitset->capacity; i++) {
            if (bitset_contains(bitset, i)) {
                if (first_print) {
                    printf("%zu", i);
                    first_print = 0;
                } else
                    printf(", %zu", i);
            }
        }
        printf("]\n");
    }
}

void bitset_free(BitSet *bitset) {
    free(bitset->bit_arrays);
    free(bitset);
}
