from math import prod
from operator import sub


def part_1(input: str) -> int:
    workflows, _, ratings = input.partition("\n\n")
    workflows = parse_workflows(workflows)

    total = 0
    for rating in map(parse_rating, ratings.splitlines()):
        node = "in"
        while node not in "AR":
            items, default = workflows[node]
            node = next(
                (
                    result
                    for index, condition, value, result in items
                    if (
                        rating[index] > value
                        if condition == ">"
                        else rating[index] < value
                    )
                ),
                default,
            )
        if node == "A":
            total += sum(rating)

    return total


def part_2(input: str) -> int:
    workflows_str, _, _ = input.partition("\n\n")
    workflows = parse_workflows(workflows_str)

    total = 0
    stack: list[tuple[str, list[int], list[int]]] = [("in", [1] * 4, [4001] * 4)]

    while stack:
        name, starts1, ends1 = stack.pop()

        if name == "A":
            total += prod(map(sub, ends1, starts1))
            continue
        elif name == "R":
            continue

        items, default = workflows[name]
        for index, condition, value, result in items:
            if condition == ">":
                if starts1[index] < value <= ends1[index]:
                    starts2 = starts1[:]
                    starts2[index] = value + 1
                    stack.append((result, starts2, ends1[:]))
                    ends1[index] = value + 1
            else:
                if starts1[index] < value <= ends1[index]:
                    ends2 = ends1[:]
                    ends2[index] = value
                    stack.append((result, starts1[:], ends2))
                    starts1[index] = value

        stack.append((default, starts1, ends1))

    return total


def parse_workflows(input: str):
    return {
        name: parse_workflow(workflow.removesuffix("}"))
        for name, _, workflow in (line.partition("{") for line in input.splitlines())
    }


def parse_workflow(input: str):
    *items_str, default = input.split(",")
    items = list(map(parse_item, items_str))
    return items, default


def parse_item(input: str):
    prefix, _, result = input.partition(":")
    index = "xmas".index(prefix[0])
    condition = prefix[1]
    value = int(prefix[2:])
    return index, condition, value, result


def parse_rating(rating: str):
    return tuple(
        int(item.partition("=")[2])
        for item in rating.removeprefix("{").removesuffix("}").split(",")
    )


def test_part_1_example_1():
    assert part_1(EXAMPLE_1) == 19114


def test_part_1_input():
    with open("../testdata/year_2023/day_19.txt", "r") as f:
        assert part_1(f.read().rstrip()) == 402185


def test_part_2_example_1():
    assert part_2(EXAMPLE_1) == 167409079868000


def test_part_2_input():
    with open("../testdata/year_2023/day_19.txt", "r") as f:
        assert part_2(f.read().rstrip()) == 130291480568730


EXAMPLE_1 = """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"""
