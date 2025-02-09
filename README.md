# timereport

A small command-line utility for reporting working time and displaying it in different formats.

```
+--------------------+------------+------------+------------+------------+------------+
|                    | 2025-02-03 | 2025-02-04 | 2025-02-05 | 2025-02-06 | 2025-02-07 |
+--------------------+------------+------------+------------+------------+------------+
|                    | Monday     | Tuesday    | Wednesday  | Thursday   | Friday     |
+--------------------+------------+------------+------------+------------+------------+
| start              |            |            | 08:00      |            |            |
+--------------------+------------+------------+------------+------------+------------+
| stop               |            |            | 17:00      |            |            |
+--------------------+------------+------------+------------+------------+------------+
| lunch              |            |            | 00:45      |            |            |
+--------------------+------------+------------+------------+------------+------------+
| 1. Default project |            |            | 06:15      |            |            |
+--------------------+------------+------------+------------+------------+------------+
| 2. Project A       |            |            | 02:15      |            |            |
+--------------------+------------+------------+------------+------------+------------+
| 3. Project B       |            |            |            |            |            |
+--------------------+------------+------------+------------+------------+------------+
```

## Usage

It is recommended to assign an alias to timereport. Below the alias `t` is assumed.

To show weekend data, add `--weekend` to the command.

### Adding data

```
$ t 2025-02-08 start 08:30
$ t monday stop 16:00
```

If date is omitted, the current date will be used:

```
$ t start 08:30
$ t start 08:30 stop 16:00 lunch 45m
```

### Projects

#### Adding projects

```
$ t add myproject
```

#### Reporting time on projects

These are all equivalent, provided that myproject is number 2 in the list:

```
$ t project myproject 8
$ t project myproject 8:00
$ t project 2 8:00
```

### Showing data

#### Console

```
$ t
$ t show week
```

#### Browser

```
$ t show week html
```

#### Weekend

Add the `--weekend` flag to any command to show Saturday and Sunday:

```
$ t --weekend
$ t show week html --weekend
$ t start 8:30 --weekend
```

## Development

### Running tests

```
cargo test --features mock-open
```

The mock-open feature is necessary to add so that the test does not open 
a lot of browser windows.

### Running tests with coverage

```
cargo llvm-cov --features mock-open [--html]
```
