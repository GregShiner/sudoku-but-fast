SIZE = 81
def floor(num, div):
    return (num // div) * div

def get_row(idx):
    return list(range(floor(idx, 9), floor(idx, 9) + 8 + 1, 1))

def get_col(idx):
    return list(range(idx % 9, (idx % 9) + 81, 9))

def get_box_row(idx):
    return list(range(floor(idx, 3), floor(idx, 3) + 3, 1))

def get_box(idx):
    top_box = (floor(idx, 27) + (idx %9))
    return get_box_row(top_box) + get_box_row(top_box + 9) + get_box_row(top_box + 18)

# Tests that all elements in a row, col, or box, all resolve to the same row, col, box
for i in range(SIZE):
    row = get_row(i)
    for j in row:
        assert row == get_row(j)

    col = get_col(i)
    for j in col:
        assert col == get_col(j)

    box = get_box(i)
    for j in box:
        assert box == get_box(j)

def get_neighbors(idx):
    return list(sorted(set(get_row(idx) + get_col(idx) + get_box(idx)) - {idx}))

# RED = "\033[31m"
# GREEN = "\033[32m"
# WHITE = "\033[37m"
# RESET = "\033[0m"
# for i in range(SIZE):
#     neighbors = get_neighbors(i)
#
#     for j in range(SIZE):
#         if j == i:
#             color = RED
#         elif j in neighbors:
#             color = GREEN
#         else:
#             color = WHITE
#         print(f"{color}{j : 3}{RESET}", end="")
#         if j % 9 == 8:
#             print()

with open("generated/checks.rs", "w") as f:
    f.write("#[rustfmt::skip]\npub const CHECKS: [[u8; 20]; SIZE] = [\n")
    for i in range(SIZE):
        f.write("   " + str(get_neighbors(i)) + ",\n")
    f.write("];")

