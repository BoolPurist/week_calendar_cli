# Purpose

[![Crates.io](https://img.shields.io/crates/v/week_calendar.svg)](https://crates.io/crates/week_calendar)
![Spelling_status](https://github.com/BoolPurist/week_calendar_cli/actions/workflows/spelling.yml/badge.svg)
![CI_status](https://github.com/BoolPurist/week_calendar_cli/actions/workflows/ci_check.yml/badge.svg)

"week_calendar" is a command line tool which shows calendar weeks. 

It provides the number and start/end date of a week.

Different intervals can be chosen. Year, month, or just one specific week.
You can also convert week numbers into their respective weeks.


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

## Rust version 

1.64.0 

## Application version

0.1.3 

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

This application is also published on this [page](https://crates.io/crates/week_calendar) of crates.io 

You can install via 
```sh
cargo install week_calendar
```

### Compile from source and install it

1. Clone this repository
2. Go into to the cloned folder
3. Type the following command into the terminal

```sh
cargo install --path=. --force
```
or via
```sh
just local-install
```

## Changelog 

Changelog can be found [here](./CHANGELOG.md)

## Licenses

This application can be used under MIT or Apache 2.0 at your choice

## Detailed Manual with Examples

```txt
"week_calendar" is a command line tool which shows calendar weeks. 

It provides the number and start/end date of a week.

Different intervals can be chosen. Year, month, or just one specific week.
You can also convert week numbers into their respective weeks.


Usage: week_calendar [OPTIONS] <COMMAND>

Commands:
  today        Shows calendar week for today [aliases: t]
  date         Shows calendar week of a given date. Given date must be a valid date. Example: 30.02.2023 is not valid [aliases: d]
  month        List all calendar weeks in a given month in a given year [aliases: m]
  year         List all calendar weeks in a given year. If no year is provided then the current year is assumed [aliases: y]
  week-number  Converts given week numbers into their respective week calendars [aliases: w]
  help         Print this message or the help of the given subcommand(s)

Options:
  -f, --for-machine
          Data entry about week calendar are outputted with spaces between. This is intended to make parsing of the date easier
          
          [env: WEEK_CALENDAR_FOR_MACHINE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

SOURCE CODE:

https://github.com/BoolPurist/week_calendar_cli

LICENSE:

This application can be used under MIT or Apache 2.0 at your choice
For more details see the license file within source code base 
on GitHub.

----------------------------------------------------------------------------------------------------

Shows calendar week for today

Usage: week_calendar today

Options:
  -h, --help
          Print help (see a summary with '-h')

EXAMPLES:

Show current week calendar 

Command:
    # today is 04.07.2023. here
    week_calendar today

Output:

+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 27          | 2023-07-03 | 2023-07-09 |
+-------------+------------+------------+

----------------------------------------------------------------------------------------------------

Shows calendar week of a given date. Given date must be a valid date. Example: 30.02.2023 is not valid

Usage: week_calendar date <YEAR> <MONTH> <DAY>

Arguments:
  <YEAR>   Year of a date
  <MONTH>  Month of a date. Must be between 1 and 12
  <DAY>    Day of a date. Must be between 1 and 31

Options:
  -h, --help  Print help

----------------------------------------------------------------------------------------------------

List all calendar weeks in a given month in a given year

Usage: week_calendar month [MONTH] [YEAR]

Arguments:
  [MONTH]  Month of a date. Must be between 1 and 12. If not provided then the current month and year are assumed
  [YEAR]   Day of a date. Must be between 1 and 31. If not provided then the current year is assumed

Options:
  -h, --help  Print help

----------------------------------------------------------------------------------------------------

List all calendar weeks in a given year. If no year is provided then the current year is assumed

Usage: week_calendar year [YEAR]

Arguments:
  [YEAR]  Given year. If omitted then the current year is assumed

Options:
  -h, --help  Print help

----------------------------------------------------------------------------------------------------

Converts given week numbers into their respective week calendars.

Value for week numbers are between 1 and 53. Note: Some years only have 52 week numbers. If for such year 53 is given then that number is treated as 52.

Usage: week_calendar week-number [OPTIONS] <START> [END]

Arguments:
  <START>
          Single week number to convert if no end week number is given

  [END]
          End week number within a year

Options:
  -y, --year <YEAR>
          Given year. If omitted then the current year is assumed

  -h, --help
          Print help (see a summary with '-h')

EXAMPLES 

Showing the calendar week with week number 10 of 2024, assuming the year 2024

Command:
    week_calendar week-number 10

Output:

+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 10          | 2024-03-04 | 2024-03-10 |
+-------------+------------+------------+
```
