from collections import defaultdict

# https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/ketzp94/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button


def part_1(input: str) -> int:
    graph = defaultdict(set)
    for line in input.splitlines():
        parent, _, children = line.partition(": ")
        for child in children.split():
            graph[parent].add(child)
            graph[child].add(parent)

    def count(node):
        return len(graph[node] - seen)

    seen = set(graph)
    while sum(map(count, seen)) != 3:
        seen.remove(max(seen, key=count))

    a = len(seen)
    b = len(graph) - a
    return a * b


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 54


def test_part_1_input():
    with open("../test_data/year_2023/day_25.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 612945


EXAMPLE_1 = """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"""
