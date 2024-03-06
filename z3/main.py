import z3
import time
import json

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

total_runtime = 0
testcase_outputs = []
for testcase in testcases:
    with open(f"../testcases/{testcase}.txt", "r") as file:
        element_count = int(file.readline())
        set_count = int(file.readline())
        universe = set(range(element_count))
        sets = [
            {int(element) - 1 for element in file.readline().split()}
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
        runtime = end - start
        total_runtime += runtime
        set_indices = [i for i, _ in enumerate(sets) if model[chosen_sets[i]]]
        set_count = len(set_indices)
        output = {
            "name": testcase,
            "runtime": runtime,
            "set_count": set_count,
            "set_indices": set_indices,
        }
        testcase_outputs.append(output)
        print(f"{testcase} {set_count} {runtime}")
    else:
        print(f"{testcase}: No solution found.")
        exit(1)
print(f"Completed in {total_runtime} s")
output = {"total_runtime": total_runtime, "testcase_outputs": testcase_outputs}
with open("output.json", "w") as file:
    json.dump(output, file)
