# Purpose

Cli application to show the calendar week of today or a certain date

## How to install it

1. Clone this repository
2. Go into to the cloned folder
3. Type the following command into the terminal
```sh
cargo install --path=. --force
```

## Examples

Look up calendar week for today 

```sh
week_calendar today
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

## Licenses

This appliaction can be used under MIT or Apache 2.0 at your choice
