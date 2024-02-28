#include <stdlib.h>
#include "bitset.h"

typedef struct {
    unsigned int set_number;
    BitSet *elements;
} Set;

Set *set_init(unsigned int set_number, size_t capacity, BitSet *elements);
Set *set_difference(Set *a, Set *b);
Set *set_union(Set *a, Set *b);
Set *set_copy(Set *set);
void set_print(Set *set);
void set_free(Set *set);
