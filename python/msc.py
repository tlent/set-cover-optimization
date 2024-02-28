import sys
import time
import cProfile

class Subset(frozenset):
    def __new__(cls, elements, set_number, original_elements):
        return frozenset.__new__(cls, elements)

    def __init__(self, elements, set_number, original_elements):
        self.set_number = set_number
        self.original_elements = original_elements

    def difference(self, other_subset):
        new_subset = Subset(self - other_subset, self.set_number, self.original_elements)
        return new_subset

    def __repr__(self):
        return "Set #{}: {}".format(self.set_number, self.original_elements)

def read_sets(filename):
    with open(filename, 'r') as file:
        element_count = int(file.readline())
        set_count = int(file.readline())
        sets = set()
        for number in range(1, set_count + 1):
            elements = [int(element) for element in file.readline().split()]
            subset = Subset(elements, number, elements)
            sets.add(subset)
        return sets, set(range(1, element_count + 1))

def select(sets, selected_set):
    return {element_set.difference(selected_set) for element_set in sets
        if element_set - selected_set}

def minimum_set_cover(sets, uncovered_elements):
    if not sets:
        return []
    for element_set in sets:
        for other_set in sets - {element_set}:
            if element_set < other_set:
                return minimum_set_cover(sets - {element_set}, uncovered_elements)
    for element in uncovered_elements:
        containing_sets = []
        for element_set in sets:
            if element in element_set:
                containing_sets.append(element_set)
            if len(containing_sets) > 1:
                break
        if len(containing_sets) == 1:
            element_set = containing_sets[0]
            return containing_sets + minimum_set_cover(select(sets, element_set),
                uncovered_elements - element_set)
    largest_set = max(sets, key=len)
    return min(minimum_set_cover(sets - {largest_set}, uncovered_elements), 
        [largest_set] + minimum_set_cover(select(sets, largest_set),
        uncovered_elements - largest_set), key=len)

if __name__ == "__main__":
    start = time.clock()
    sets, elements = read_sets(sys.argv[1])
    # cProfile.run("minimum_set_cover(sets)", sort='calls')
    set_cover = minimum_set_cover(sets, elements)
    print("Found minimum set cover containing {} sets in {:.3g} seconds.".format(
        len(set_cover), time.clock() - start))
    cover_elements = set()
    for subset in set_cover:
        for element in subset.original_elements:
            cover_elements.add(element)
    assert elements == cover_elements
    print("Included sets: {}".format(sorted([subset.set_number for subset in set_cover])))
    for element_set in sorted(set_cover, key=lambda subset: subset.set_number):
        print(element_set)
