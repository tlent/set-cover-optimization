#include <stdlib.h>

#define TYPE unsigned int

typedef struct {
    size_t capacity;
    size_t size;
    size_t bit_array_count;
    TYPE *bit_arrays;
} BitSet;

BitSet *bitset_init(size_t capacity);
int bitset_add(BitSet *bitset, int value);
int bitset_remove(BitSet *bitset, int value);
int bitset_contains(BitSet *bitset, int value);
int bitset_is_subset(BitSet *superset, BitSet *subset);
int bitset_is_empty(BitSet *bitset);
BitSet *bitset_difference(BitSet *a, BitSet *b);
BitSet *bitset_union(BitSet *a, BitSet *b);
BitSet *bitset_copy(BitSet *bitset);
void bitset_print(BitSet *bitset);
void bitset_free(BitSet *bitset);
