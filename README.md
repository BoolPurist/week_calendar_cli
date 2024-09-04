# Purpose

[![Build status](https://github.com/BoolPurist/week_calendar_cli/actions/workflows/ci_check.yml/badge.svg)](https://github.com/boolpurist/week_calendar_cli/actions)
[![Crates.io](https://img.shields.io/crates/v/week_calendar.svg)](https://crates.io/crates/week_calendar)

**week_calendar** is a command line tool which shows calendar weeks. 
It provides the number of week and from which date it starts and another ends.
Different intervals can be chosen. Year, month, or just one specific week.
Can also convert week numbers into their respective weeks.

## Demonstration 

### Getting the week in which the date 2023-06-30 resides

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

### Getting the week for the week number 9 in year 2023

```sh
week_calendar week-number 9 --year 2023
```

Output:

```text
+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 9           | 2023-02-27 | 2023-03-05 |
+-------------+------------+------------+
```

## Manual and more examples

More examples and more detailed usage can be found in this [manual](./MANUAL.txt)

## Current version

Current version is 0.1.3

Version on crates.io and the latest git 
[tag](https://github.com/BoolPurist/week_calendar_cli/releases/tag/v0.1.3) 
correspond to the current version. 

## How to install it

### Download a pre-built binary

There are pre-built binaries for the following targets 
under the latest [release](https://github.com/BoolPurist/week_calendar_cli/releases/tag/v0.1.3):

- x86_64-unknown-linux-gnu => Linux 64 Bit

After downloading the archive and unpacking it. Put the binary to a location 
which is in your PATH environmental variable.

### Install from crates.io

This appliaction is also published on this [page](https://crates.io/crates/week_calendar) of crates.io 

You can install via 
```sh
cargo install week_calendar
```

### Install it from source

1. Clone this repository
2. Go into to the cloned folder
3. Type the following command into the terminal

```sh
cargo install --path=. --force
```

## Changelog 

Changelog can be found [here](./CHANGELOG.md)

## Licenses

This application can be used under MIT or Apache 2.0 at your choice
