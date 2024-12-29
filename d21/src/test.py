from functools import cache

inputval = ""

posi = [
    ["7", "8", "9"],
    ["4", "5", "6"],
    ["1", "2", "3"],
    [None, "0", "A"],
]

arr_pads = [
    [None, "^", "A"],
    ["<", "v", ">"]
]

def get_pos(arr, code):
    for i, row in enumerate(arr):
        if code in row:
            return (i, row.index(code))

@cache
def shortest(start, end, layers):
    if start == "<" and end == ">":
        pass
    if isinstance(start, str):
        start = get_pos(arr_pads, start)
    if isinstance(end, str):
        end = get_pos(arr_pads, end)

    if layers == 0:
        return 1
    elif layers < 3:
        vert = None
        hori = None
        if end[0] < start[0]:
            vert = "^"
        elif end[0] > start[0]:
            vert = "v"
        if end[1] < start[1]:
            hori = "<"
        elif end[1] > start[1]:
            hori = ">"
        if not hori and not vert:
            return shortest("A", "A", layers - 1)
        elif not hori:
            return shortest("A", vert, layers - 1) + (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + shortest(vert, "A", layers - 1)
        elif not vert:
            return shortest("A", hori, layers - 1) + (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + shortest(hori, "A", layers - 1)
        else:
            if start[1] == 0:
                return shortest("A", hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, "A", layers - 1)
            elif end[1] == 0:
                return shortest("A", vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, "A", layers - 1)
            else:
                return min(
                    shortest("A", hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, "A", layers - 1),
                    shortest("A", vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, "A", layers - 1)
                )
    else:
        vert = None
        hori = None
        if end[0] < start[0]:
            vert = "^"
        elif end[0] > start[0]:
            vert = "v"
        if end[1] < start[1]:
            hori = "<"
        elif end[1] > start[1]:
            hori = ">"
        if not hori and not vert:
            return shortest("A", "A", layers - 1)
        elif not hori:
            return shortest("A", vert, layers - 1) + (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + shortest(vert, "A", layers - 1)
        elif not vert:
            return shortest("A", hori, layers - 1) + (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + shortest(hori, "A", layers - 1)
        else:
            if start[1] == 0 and end[0] == 3:
                return shortest("A", hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, "A", layers - 1)
            elif end[1] == 0 and start[0] == 3:
                return shortest("A", vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, "A", layers - 1)
            else:
                return min(
                    shortest("A", hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, "A", layers - 1),
                    shortest("A", vert, layers - 1) + \
                    (abs(end[0] - start[0]) - 1) * shortest(vert, vert, layers - 1) + \
                    shortest(vert, hori, layers - 1) + \
                    (abs(end[1] - start[1]) - 1) * shortest(hori, hori, layers - 1) + \
                    shortest(hori, "A", layers - 1)
                )

# Updated input values
ut = [
    # "480A",
    # "143A",
    # "983A",
    "382A",
    # "974A"
]

score = 0
for inputval in ut:
    intval = int(inputval[:3])
    total = 0
    for startp, endp in zip("A" + inputval[:3], inputval):
        total += shortest(get_pos(posi, startp), get_pos(posi, endp), 3)
    print(intval, total)
    score += intval * total
print(score)

def shortestPath(key1, key2, pad):
    (coords, gap) = pad
    (r1, c1) = coords[key1]
    (r2, c2) = coords[key2]

    ud = "v" * (r2 - r1) if r2 > r1 else "^" * (r1 - r2)
    lr = ">" * (c2 - c1) if c2 > c1 else "<" * (c1 - c2)

    if c2 > c1 and f"{r2},{c1}" != gap:
        return f"{ud}{lr}A"
    if f"{r1},${c2}" != gap:
        return f"{lr}{ud}A"
    return f"{ud}${lr}A"

def padLookup(padRows):
    coords = {}
    gap = "0,0" if len(padRows) == 2 else "3,0"
    for (ridx, row) in enumerate(padRows):
        for (cidx, key) in enumerate(row):
            if (key != " "):
                coords[key] = (ridx, cidx)
    return (coords, gap)

numPad = padLookup(["789", "456", "123", " 0A"])
dirPad = padLookup([" ^A", "<v>"])

def sequences(seq, pad):
    print(f"computing {seq}")
    keys = []
    prevKey = "A"
    for key in seq:
        keys.append(shortestPath(prevKey, key, pad));
        prevKey = key
    print(f"Computed {keys}")
    return keys

def complexity(codes, numDirRobots = 2):
    for code in codes:
        numpad = "".join(sequences(code, numPad))
        print(numpad)
        rb1 = ["".join(sequences(numpad_num, dirPad)) for numpad_num in numpad]
        print(rb1)
        rb2 = ["".join(sequences(rb1_num, dirPad)) for rb1_num in rb1]
        print(rb2)

def part2(inp):
    return complexity(inp.split("\n"))


print(part2("382A"))