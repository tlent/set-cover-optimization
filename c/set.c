#include "set.h"
#include <stdio.h>

Set *set_init(unsigned int set_number, size_t capacity, BitSet *elements) {
    Set *set = malloc(sizeof(Set));
    if (!set)
        return NULL;
    set->set_number = set_number;
    if (!elements)
        set->elements = bitset_init(capacity);
    else
        set->elements = elements;
    if (!set->elements)
        return NULL;
    return set;
}

int set_add(Set *set, int element) {
    if (!set)
        return 0;
    return bitset_add(set->elements, element);
}

int set_contains(Set *set, int element) {
    if (!set)
        return 0;
    return bitset_contains(set->elements, element);
}

int set_is_subset(Set *a, Set *b) {
    if (!a || !b)
        return 0;
    return bitset_is_subset(a->elements, b->elements);
}

int set_is_empty(Set *set) {
    if (!set)
        return 1;
    return bitset_is_empty(set->elements);
}

Set *set_difference(Set *a, Set *b) {
    if (!a || !b)
        return NULL;
    BitSet *set_difference = bitset_difference(a->elements, b->elements);
    if (!set_difference)
        return NULL;
    return set_init(a->set_number, a->elements->capacity, set_difference);
}

Set *set_union(Set *a, Set *b) {
    if (!a || !b)
        return NULL;
    BitSet *set_union = bitset_union(a->elements, b->elements);
    if (!set_union)
        return NULL;
    return set_init(a->set_number, a->elements->capacity, set_union);
}

Set *set_copy(Set *set) {
    if (!set)
        return NULL;
    BitSet *new_bitset = bitset_copy(set->elements);
    if (!new_bitset)
        return NULL;
    Set *new_set = set_init(set->set_number, set->elements->capacity, new_bitset);
    if (!new_set) {
        bitset_free(new_bitset);
        return NULL;
    }
    return new_set;
}

void set_print(Set *set) {
    if (set) {
        printf("Set #%d: ", set->set_number);
        bitset_print(set->elements);
    }
}

void set_free(Set *set) {
    bitset_free(set->elements);
    free(set);
}
