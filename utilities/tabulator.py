import json
import re


def format_runtime(runtime):
    if runtime == float("inf"):
        return runtime
    if runtime >= 1:
        return f"{runtime:.2f} s"
    elif runtime >= 1e-3:
        return f"{runtime * 1e3:.2f} ms"
    elif runtime >= 1e-6:
        return f"{runtime * 1e6:.2f} µs"
    else:
        return f"{runtime * 1e9:.2f} ns"


with open("../rust/output.json", "r") as file:
    rust_output = json.load(file)

with open("../z3/output.json", "r") as file:
    z3_output = json.load(file)

testcases = [output["name"] for output in rust_output["testcase_outputs"]]
c_output = {"total_runtime": 0.0, "testcase_outputs": []}
for testcase in testcases:
    with open(f"../c/output/{testcase}.txt", "r") as file:
        content = file.read()
        if content.lower().strip() == "did not finish":
            c_output["testcase_outputs"].append(
                {
                    "name": testcase,
                    "runtime": float("inf"),
                }
            )
            c_output["total_runtime"] += float("inf")
            continue
        match = re.search(
            r"Found minimum set cover containing (\d+) sets in ([\d.]+) seconds.",
            content,
        )
        set_count = int(match.group(1))
        runtime = float(match.group(2))
        included_sets_match = re.search(r"Included sets: \[([\d, ]+)\]", content)
        included_sets = [
            int(set_number) - 1
            for set_number in included_sets_match.group(1).split(",")
        ]
        c_output["testcase_outputs"].append(
            {
                "name": testcase,
                "runtime": runtime,
                "set_count": set_count,
                "set_indices": included_sets,
            }
        )
        c_output["total_runtime"] += runtime

headers = ["Test Case", "C", "Rust", "Z3"]
data = [
    [
        c["name"],
        format_runtime(c["runtime"]),
        format_runtime(rust["runtime"]),
        format_runtime(z3["runtime"]),
    ]
    for (c, rust, z3) in zip(
        c_output["testcase_outputs"],
        rust_output["testcase_outputs"],
        z3_output["testcase_outputs"],
    )
]
data.append(
    [
        "Total",
        format_runtime(c_output["total_runtime"]),
        format_runtime(rust_output["total_runtime"]),
        format_runtime(z3_output["total_runtime"]),
    ]
)

# Calculate maximum width for each column
column_widths = [max(map(len, map(str, col))) for col in zip(headers, *data)]

# Create the header row
markdown_table = (
    "| "
    + " | ".join(f"{header: <{width}}" for header, width in zip(headers, column_widths))
    + " |\n"
)

# Create the separator row
markdown_table += (
    "|" + "|".join([f"{'-' * (width + 2)}" for width in column_widths]) + "|\n"
)

# Create the data rows
for row in data:
    markdown_table += (
        "| "
        + " | ".join(f"{value: <{width}}" for value, width in zip(row, column_widths))
        + " |\n"
    )

print(markdown_table)
