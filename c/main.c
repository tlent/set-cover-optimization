#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/time.h>
#include "vector.h"
#include "set.h"

static const char *usage = "Usage: %s input_filename\n";
static const int max_line_length = 1000;

void usage_error(const char *errorMessage, const char *argv[]) {
    fprintf(stderr, "%s\n\n", errorMessage);
    fprintf(stderr, usage, argv[0]);
}

int read_data_from_file(const char *filename, Vector *sets,
        size_t *element_count, size_t *set_count) {
    if (!filename || !sets || !element_count || !set_count)
        return 0;
    FILE *fp;
    if ((fp = fopen(filename, "r"))) {
        int success = 1;
        if (success && !(fscanf(fp, "%zu\n", element_count) 
                && fscanf(fp, "%zu\n", set_count))) {
            success = 0;
        }
        char line[max_line_length];
        for (unsigned int i = 0; i < *set_count && success; i++) {
            Set *set = set_init(i + 1, *element_count, NULL);
            if (!set)
                success = 0;
            if (!fgets(line, max_line_length, fp))
                success = 0;
            const char *delimeters = " \n";
            char *elementStr = strtok(line, delimeters);
            while (elementStr && success) {
                int element = atoi(elementStr);
                if(!bitset_add(set->elements, element))
                    success = 0;
                elementStr = strtok(NULL, delimeters);
            }
            if (!vector_append(sets, set))
                success = 0;
        }
        fclose(fp);
        return success;
    }
    return 0;
}

void print_set_cover(BitSet *set_cover, Vector *original_sets, float run_time) {
    printf("Found minimum set cover containing %zu sets", set_cover->size);
    if (run_time)
        printf(" in %.6f seconds", run_time);
    printf(".\n");
    printf("Included sets: ");
    bitset_print(set_cover);
    for (size_t i = 0; i < original_sets->size; i++) {
        Set *set = vector_index(original_sets, i);
        if (bitset_contains(set_cover, set->set_number)) {
            set_print(set);
        }
    }
}

Vector *copy_sets(Vector *sets) {
    Vector *new_vector = vector_init(sets->capacity);
    for (size_t i = 0; i < sets->size; i++) {
        Set *set = vector_index(sets, i);
        if (!set)
            return NULL;
        Set *new_set = set_copy(set);
        if (!new_set)
            return NULL;
        if (!vector_append(new_vector, new_set)) {
            set_free(new_set);
            return NULL;
        }
    }
    return new_vector;
}

void free_sets(Vector *sets) {
    for (size_t i = 0; i < sets->size; i++) {
        set_free(vector_index(sets, i));
    }
    vector_free(sets);
}

int select_set(Vector *sets, Set *selected_set) {
    for (size_t i = 0; i < sets->size; i++) {
        Set *set = vector_index(sets, i);
        Set *diff = set_difference(set, selected_set);
        if (!diff)
            return 0;
        if (!bitset_is_empty(diff->elements)) {
            if (!vector_set(sets, i, diff))
                return 0;
        } else {
            Set *removed_set = vector_remove(sets, i);
            if (!removed_set) 
                return 0;
            set_free(removed_set);
            set_free(diff);
        }
        set_free(set);
    }
    return 1;
}

BitSet *minimum_set_cover(Vector *sets, BitSet *covered_elements, 
                            size_t set_count, size_t element_count) {
    if (!sets || !covered_elements)
        return NULL;
    if (vector_is_empty(sets))
        return bitset_init(set_count);
    // Prune all sets that are subsets of another set
    for (size_t i = 0; i < sets->size; i++) {
        for (size_t j = 0; j < sets->size; j++) {
            if (i != j) {
                Set *a = vector_index(sets, i);
                Set *b = vector_index(sets, j);
                if (!a || !b) {
                    return NULL;
                }
                if (bitset_is_subset(a->elements, b->elements)) {
                    Vector *new_sets = copy_sets(sets);
                    if (!new_sets)
                        return NULL;
                    Set *removed_set = vector_remove(new_sets, j);
                    if (!removed_set) {
                        free_sets(new_sets);
                        return NULL;
                    }
                    set_free(removed_set);
                    BitSet *msc = minimum_set_cover(new_sets, covered_elements,
                        set_count, element_count);
                    free_sets(new_sets);
                    return msc;
                }
            }
        }
    }
    // Select any sets which contain an element not found in any other set
    for (unsigned int i = 1; i < element_count + 1; i++) {
        if (!bitset_contains(covered_elements, i)) {
            int containing_sets = 0;
            size_t last_containing_index = 0;
            for (size_t index = 0; index < sets->size; index++) {
                Set *set = vector_index(sets, index);
                if (bitset_contains(set->elements, i)) {
                    last_containing_index = index;
                    ++containing_sets;
                }
                if (containing_sets > 1)
                    break;
            }
            if (containing_sets == 0) { // an uncovered element is not in any set
                return NULL;
            }
            if (containing_sets == 1) {
                Set *containing_set = vector_index(sets, last_containing_index);
                BitSet *new_covered_elements = bitset_union(covered_elements,
                    containing_set->elements);
                if (!new_covered_elements) {
                    return NULL;
                }
                Vector *new_sets = copy_sets(sets);
                if (!new_sets) {
                    return NULL;
                }
                Set *removed_set = vector_remove(new_sets, last_containing_index);
                if (!removed_set) {
                    free_sets(new_sets);
                    return NULL;
                }
                set_free(removed_set);
                if (!select_set(new_sets, containing_set)) {
                    return NULL;
                }
                BitSet *msc = minimum_set_cover(new_sets, new_covered_elements,
                    set_count, element_count);
                free_sets(new_sets);
                bitset_free(new_covered_elements);
                if (!msc) {
                    return NULL;
                }
                if (!bitset_add(msc, containing_set->set_number)) {
                    return NULL;
                }
                return msc;
            }
        }
    }
    // Test both selecting and not selecting the largest remaining set. Choose
    // the option that creates a smaller cover.
    size_t largest_set_index = 0;
    Set *largest_set = vector_index(sets, largest_set_index);
    Set *cursor;
    for (size_t i = 1; i < sets->size; i++) {
        cursor = vector_index(sets, i);
        if (cursor->elements->size > largest_set->elements->size) {
            largest_set_index = i;
            largest_set = cursor;
        } 
    }
    // MSC without largest set
    Vector *new_sets_without_largest = copy_sets(sets);
    if (!new_sets_without_largest)
        return NULL;
    Set *removed_set = vector_remove(new_sets_without_largest, largest_set_index);
    if (!removed_set) {
        free_sets(new_sets_without_largest);
        return NULL;
    }
    set_free(removed_set);
    BitSet *msc_without_largest = minimum_set_cover(new_sets_without_largest,
        covered_elements, set_count, element_count);
    free_sets(new_sets_without_largest);
    if (!msc_without_largest)
        return NULL;
    // MSC with largest set
    BitSet *new_covered_elements = bitset_union(covered_elements,
        largest_set->elements);
    if (!new_covered_elements) {
        bitset_free(msc_without_largest);
        return NULL;
    }
    Vector *new_sets_with_largest = copy_sets(sets);
    if (!new_sets_with_largest) {
        bitset_free(msc_without_largest);
        return NULL;
    }
    removed_set = vector_remove(new_sets_with_largest, largest_set_index);
    if (!removed_set) {
        free_sets(new_sets_with_largest);
        bitset_free(msc_without_largest);
        return NULL;
    }
    set_free(removed_set);
    if (!select_set(new_sets_with_largest, largest_set)) {
        free_sets(new_sets_with_largest);
        bitset_free(msc_without_largest);
        return NULL;
    }
    BitSet *msc_with_largest = minimum_set_cover(new_sets_with_largest,
        new_covered_elements, set_count, element_count);
    free_sets(new_sets_with_largest);
    bitset_free(new_covered_elements);
    if (!msc_with_largest) {
        bitset_free(msc_without_largest);
        return NULL;
    }
    if (!bitset_add(msc_with_largest, largest_set->set_number)) {
        bitset_free(msc_without_largest);
        return NULL;
    }
    // Compare the 2 MSCs and choose the one with fewer sets.
    if (msc_with_largest->size < msc_without_largest->size){
        bitset_free(msc_without_largest);
        return msc_with_largest;
    }
    bitset_free(msc_with_largest);
    return msc_without_largest;
}

int main(int argc, const char *argv[]) {
    struct timeval start, end;
    if (argc < 2) {
        usage_error("Missing required filename argument.", argv);
        return 1;
    } else if (argc > 2) {
        usage_error("Too many arguments supplied.", argv);
        return 1;
    } else {
        const char *filename = argv[1];
        size_t element_count;
        size_t set_count;
        Vector *sets = vector_init(16);
        if (!sets)
            return 1;
        if (!read_data_from_file(filename, sets, &element_count, &set_count)) {
            printf("Error reading file.\n");
            vector_free(sets);
            return 1;
        }
        Vector *original_sets = copy_sets(sets);
        BitSet *covered_elements = bitset_init(element_count);
        if (!covered_elements || !original_sets) {
            printf("Memory error.\n");
            free_sets(sets);
            if (covered_elements)
                bitset_free(covered_elements);
            if (original_sets)
                free_sets(original_sets);
            return 1;
        }
        gettimeofday(&start, NULL);
        BitSet *set_cover = minimum_set_cover(sets, covered_elements,
            set_count, element_count);
        gettimeofday(&end, NULL);
        float run_time = 0;
        run_time = (float) (end.tv_usec - start.tv_usec) / 1000000 +
            (float) (end.tv_sec - start.tv_sec);
        bitset_free(covered_elements);
        free_sets(sets);
        if (!set_cover) {
            printf("Error finding minimum set cover.\n");
            free_sets(original_sets);
            return 1;
        }
        print_set_cover(set_cover, original_sets, run_time);
        bitset_free(set_cover);
        free_sets(original_sets);
    }
    return 0;
}
