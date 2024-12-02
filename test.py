def parse_input(filename: str) -> tuple[list[int], list[int]]:
    with open(filename, "r") as f:
        input = f.readlines()
        input = [
            [int(val.split(maxsplit=1)[0]), int(val.rsplit(maxsplit=1)[-1])]
            for val in input
        ]
    output_l, output_r = [val[0] for val in input], [val[1] for val in input]
    return output_l, output_r


if __name__ == "__main__":
    list_l, list_r = parse_input("data/inputs/01.txt")
    list_l.sort()
    list_r.sort()
    difference = sum([abs(l - r) for l, r in zip(list_l, list_r)])

    print("Solution is: {}".format(difference))
