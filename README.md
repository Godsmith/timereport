# timereport

A small command-line utility for reporting working time and displaying it in different formats.

## Usage

It is recommended to assign an alias to timereport. Below the alias `t` is assumed.

```
$ t start 08:30 stop 16:00
+-------+------------+------------+------------+------------+------------+
|       | 2024-04-22 | 2024-04-23 | 2024-04-24 | 2024-04-25 | 2024-04-26 |
+-------+------------+------------+------------+------------+------------+
| start |            |            |            |            | 08:30      |
+-------+------------+------------+------------+------------+------------+
| stop  |            |            |            |            | 16:00      |
+-------+------------+------------+------------+------------+------------+

$ t start 08:30 --weekend
+-------+------------+------------+------------+------------+------------+------------+------------+
|       | 2024-04-29 | 2024-04-30 | 2024-05-01 | 2024-05-02 | 2024-05-03 | 2024-05-04 | 2024-05-05 |
+-------+------------+------------+------------+------------+------------+------------+------------+
| start |            |            |            |            |            |            | 08:30      |
+-------+------------+------------+------------+------------+------------+------------+------------+
| stop  |            |            |            |            |            |            |            |
+-------+------------+------------+------------+------------+------------+------------+------------+

$ t start 08:30 2024-04-26
+-------+------------+------------+------------+------------+------------+
|       | 2024-04-22 | 2024-04-23 | 2024-04-24 | 2024-04-25 | 2024-04-26 |
+-------+------------+------------+------------+------------+------------+
| start |            |            |            |            | 08:30      |
+-------+------------+------------+------------+------------+------------+
| stop  |            |            |            |            |            |
+-------+------------+------------+------------+------------+------------+
```
