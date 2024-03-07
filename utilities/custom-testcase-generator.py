import sys
import random
import argparse

directory = "testcases/custom/"

def generate_unpadded_sets(element_count, set_count):
    sets = []
    uncovered_elements = set(element_range)
    for _ in range(set_count):
        elements = random.sample(element_range, 10)
        sets.append(elements)
        uncovered_elements -= set(elements)
    if uncovered_elements:
        for uncovered_element in uncovered_elements:
            sets[random.randint(len(sets))].append(uncovered_element)
    return sets

def generate_padded_sets(element_count, set_count):
    sets = []
    for _ in range(set_count - element_count):
        elements = random.sample(element_range, 10)
        sets.append(elements)
    for element in range(1, element_count + 1):
        sets.append([element])
    return sets

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate testcases for minimum set cover.")
    parser.add_argument('elements', type=int, default=100)
    parser.add_argument('sets', type=int)
    parser.add_argument('-p','--padded', action='store_true')
    args = parser.parse_args()
    element_count = args.elements
    set_count = args.sets
    element_range = range(1, element_count + 1)
    if args.padded:
        padded_status = "-padded"
        sets = generate_padded_sets(element_count, set_count)
    else:
        padded_status = ""
        sets = generate_unpadded_sets(element_count, set_count)
    filename = "s-c{}-{}-{}".format(padded_status, element_count, set_count)
    output = [str(element_count), str(set_count)]
    for element_set in sets:
        output.append(' '.join((str(element) for element in sorted(element_set))))
    with open(directory + filename, 'w') as file:
        file.write('\n'.join(output))
