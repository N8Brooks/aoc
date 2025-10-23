# Python Solutions

This directory contains the Python implementations of the Advent of Code problems.
It now uses [uv](https://docs.astral.sh/uv/latest/) for Python and environment management.

## Requirements

- uv 0.9 or newer
- Python 3.10+ (installable through uv)

## Setup

1. Install the desired interpreter, for example `uv python install 3.12`.
2. Optionally pin that interpreter for this project with `uv python pin 3.12`.
3. Create the virtual environment (there are no third-party dependencies, but this ensures the venv exists) using `uv sync`.

## Running Solutions

Each module exposes `part_1` and `part_2` callables that expect the raw puzzle input.
You can execute a specific day with `uv run`, for example:

```bash
uv run python - <<'PY'
from pathlib import Path
from year_2023 import day_01

data = Path("../test_data/year_2023/day_01.txt").read_text()
print("Part 1:", day_01.part_1(data))
print("Part 2:", day_01.part_2(data))
PY
```

Replace `year_2023`/`day_01` and the input path to target other days.

## Testing

The files include `pytest`-style tests that validate the expected answers when the personal input files in `../test_data` are present.
You can run them ad hoc through uv's tool runner, which avoids adding `pytest` to the project dependencies:

```bash
uvx pytest year_2023/day_01.py
```

Call `uvx pytest` with additional files or directories to test more solutions at once.
