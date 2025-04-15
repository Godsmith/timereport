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

## Installation

Timereport stores both configuration and time report data in a file `timereport.json`, by default at .

1. Run `cargo install --path .`
2. Set the `TIMEREPORT_PATH` environment variable to where you want to create the .json file containing settings and time report data. Default is `C:\Users\$USERNAME\Dropbox\timereport.json`.
3. Run `timereport`. `timereport.json` will be created in the chosen location.
4. Set the `working_time_per_day` variable in the json file to the appropriate value in seconds. Default is 27900 seconds (7 hours and 45 minutes).
5. Add projects as appropriate (see [Adding Projects](#adding-projects)).

## Usage

### General

It is recommended to assign an alias to timereport. Below the alias `t` is assumed.

Saturdays and Sundays will only be shown if they are affected. To always show them, use `--weekend`.

### Adding Data

```
$ t 2025-02-08 start 08:30
$ t monday lunch 45m 
$ t last monday stop 16:45
```

If date is omitted, the current date will be used:

```
$ t start 08:30
$ t start 08:30 stop 16:00 lunch 45m
```

### Projects

#### Adding Projects

```
$ t add myproject
$ t add "Project containing spaces"
```

#### Reporting Time on Projects

These are all equivalent, provided that myproject is number 2 in the list:

```
$ t project myproject 8
$ t project myproject 8:00
$ t project 2 8:00
```

### Showing Data

Data can be shown one week or one month at a time, either in the terminal or in a web browser.

Add `last` to show the previous week.

#### Console

```
$ t
$ t show week
$ t show last week
$ t show january
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

### Running Tests

```
cargo test
```

### Running Tests with Coverage

```
cargo llvm-cov [--html]
```

### Publishing to Github and Crates.io

1. Update version in Cargo.toml
2. Add version description to Changelog below
3. Commit
4. `git tag x.y.z; git push --tags`
5. `cargo publish`

## Changelog

### 0.2.4

First working version

### 0.2.8

Add workflow to deploy for Windows

### 0.3.0

Bold formatting for changed cells

### 0.3.1

Fix flex display in `show week html`
