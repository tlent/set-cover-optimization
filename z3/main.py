import z3
import time
import os

testcases = [
    "s-rg-8-10",
    "s-X-12-6",
    "s-k-20-30",
    "s-k-30-50",
    "s-rg-31-15",
    "s-rg-40-20",
    "s-k-40-60",
    "s-k-20-35",
    "s-rg-63-25",
    "s-k-30-55",
    "s-rg-118-30",
    "s-rg-109-35",
    "s-k-35-65",
    "s-rg-155-40",
    "s-rg-197-45",
    "s-rg-245-50",
    "s-k-40-80",
    "s-k-50-95",
    "s-rg-413-75",
    "s-k-150-225",
    "s-k-50-100",
    "s-rg-733-100",
    "s-k-200-300",
    "s-k-100-175",
    "s-k-150-250",
]

os.makedirs("output", exist_ok=True)
for testcase in testcases:
    print(testcase)
    with open(f"../testcases/{testcase}.txt", "r") as file:
        element_count = int(file.readline())
        set_count = int(file.readline())
        universe = set(range(1, element_count + 1))
        sets = [
            {int(element) for element in file.readline().split()}
            for _ in range(set_count)
        ]
    context = z3.Context()
    optimize = z3.Optimize(context)
    chosen_sets = [z3.Bool(f"set_{i}", context) for i, _ in enumerate(sets)]
    for elem in universe:
        optimize.add(z3.Or([chosen_sets[i] for i, s in enumerate(sets) if elem in s]))
    optimize.minimize(z3.Sum([z3.If(chosen_sets[i], 1, 0) for i, _ in enumerate(sets)]))
    start = time.time()
    result = optimize.check()
    end = time.time()
    if result == z3.sat:
        model = optimize.model()
        set_cover = [(i, s) for (i, s) in enumerate(sets) if model[chosen_sets[i]]]
        output = []
        output.append(
            f"Found minimum set cover containing {len(set_cover)} sets in {end - start:.3g} seconds."
        )
        output.append(f"Included sets: {[i + 1 for i, _ in set_cover]}")
        for i, s in set_cover:
            output.append(f"Set #{i + 1}: {sorted(list(s))}")
        s = "\n".join(output)
        print(s)
        with open(f"output/{testcase}.txt", "w") as file:
            file.write(s)
    else:
        print("No solution found.")
