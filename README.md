# Purpose

Cli application to show the calendar week of today or a certain date

## How to install it

1. Clone this repository
2. Go into to the cloned folder
3. Type the following command into the terminal
```sh
cargo install --path=. --force
```

## Examples of subcommands

### today

Look up calendar week for today. Examples today is 04.07.2023. 

```sh
week_calendar today
```

Output:

```text
+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 27          | 2023-07-03 | 2023-07-09 |
+-------------+------------+------------+
```

---

### date

Look up calendar week for a given date, here 30th of June in 2023

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

---

### month

Look up all calendar weeks within current month. Example assumes today is 8th of august in 2023.

```sh
week_calendar month
```

Output:

```text
+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 31          | 2023-07-31 | 2023-08-06 |
+-------------+------------+------------+
| 32          | 2023-08-07 | 2023-08-13 |
+-------------+------------+------------+
| 33          | 2023-08-14 | 2023-08-20 |
+-------------+------------+------------+
| 34          | 2023-08-21 | 2023-08-27 |
+-------------+------------+------------+
| 35          | 2023-08-28 | 2023-09-03 |
+-------------+------------+------------+
```

---

Look up all calendar weeks for a given month in the current year. Here the given month is April in 2023 as current year.

```sh
week_calendar month 4
```

Output:

```text
+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 13          | 2023-03-27 | 2023-04-02 |
+-------------+------------+------------+
| 14          | 2023-04-03 | 2023-04-09 |
+-------------+------------+------------+
| 15          | 2023-04-10 | 2023-04-16 |
+-------------+------------+------------+
| 16          | 2023-04-17 | 2023-04-23 |
+-------------+------------+------------+
| 17          | 2023-04-24 | 2023-04-30 |
+-------------+------------+------------+
```

---

Look up all calendar weeks for a given month in a given year. Here the given month is May in 1990 as given year.

```sh
week_calendar month 3 1990
```

Output:

```text
+-------------+------------+------------+
| Week Number | From       | To         |
+-------------+------------+------------+
| 9           | 1990-02-26 | 1990-03-04 |
+-------------+------------+------------+
| 10          | 1990-03-05 | 1990-03-11 |
+-------------+------------+------------+
| 11          | 1990-03-12 | 1990-03-18 |
+-------------+------------+------------+
| 12          | 1990-03-19 | 1990-03-25 |
+-------------+------------+------------+
| 13          | 1990-03-26 | 1990-04-01 |
+-------------+------------+------------+
```

## Licenses

This appliaction can be used under MIT or Apache 2.0 at your choice
