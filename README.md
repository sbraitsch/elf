# Elf - Advent of Code CLI

## What can this do?
- Create boilerplate for new AoC Solutions
- Load your puzzle input (requires a session cookie)
- Submit solutions (**NYI**)
- Bootstrap a fully new Rust project with predefined structure and utility functions (**NYI**)
- Add boilerplate for a new year of solutions inside a previously bootstrapped project (**NYI**)

## How do I use it?

Elf always assumes to be run from the root of a bootstrapped project, except when using the `bootstrap` command.

> Elf tracks your current progress in the `elf.toml` file.<br>
> When you omit `--year` or `--day`, it will fall back to the values stored there.<br>
> E.g. `elf new` will create the stubs for the day after `elf.toml::day` in the year `elf.toml::year`. <br>
> And `elf new -d=11` will create the stubs for the day 11 in the year `elf.toml::year` and set `elf.toml::day` to 11.

Available commands are:

### `elf bootstrap`:

| arg      | alt  | effect                                                                           |
|----------|------|----------------------------------------------------------------------------------|
| `--year` | `-y` | The year to create a module for. <br> Defaults to current year if missing. (NYI) |


### `elf new`:

> If **--year** is set and **--day** is not, a new module for the given year will be created inside the bootstrapped project. <br> 
> In all other cases, year and day use their defaults to create new solution stubs.


| arg          | alt  | effect                                                       |
|--------------|------|--------------------------------------------------------------|
| `--year`     | `-y` | The year to create a solution stub for.                      |
| `--day`      | `-d` | The day to create a solution stub for.                       |
| `--template` | `-t` | Path to an optional template file to base the stub on. (NYI) |


### `elf submit`:

> Stubs created by Elf write computed solutions into `elf.toml`, from which they are read when submitting. <br>
> Submitting a specific day/year will not change the progress state in `elf.toml`.

| arg      | alt  | effect                                                             |
|----------|------|--------------------------------------------------------------------|
| `--year` | `-y` | The year of the solution to submit.                                |
| `--day`  | `-d` | The day of the solution to submit.                                 |
| `--part` | `-p` | The part of the solution to submit. <br> Defaults to 1 if missing. |


