# Purpose

[![Build status](https://github.com/BoolPurist/week_calendar_cli/actions/workflows/ci_check.yml/badge.svg)](https://github.com/boolpurist/week_calendar_cli/actions)

**week_calendar** is a command line tool which shows calendar weeks. 
It provides the number of week and from which day it starts and on which day a week ends.
Different intervals can be chosen. Year, month, or just one specific week.

## How to install it

1. Clone this repository
2. Go into to the cloned folder
3. Type the following command into the terminal

```sh
cargo install --path=. --force
```

## Examples

Here little demonstration of the appliaction 


```sh
week_calendar date 2023 6 30
```

Output:

```text
+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 26          | 2023-06-26 | 2023-07-02 |
+-------------+------------+------------+
```

More examples for usage can be found here [./Examples.md].

## Licenses

This appliaction can be used under MIT or Apache 2.0 at your choice
