# 🧝 Elf - Advent of Code CLI

## What can this do?
- Create boilerplate for new AoC Solutions
- Load your puzzle input (requires a session cookie)
- Bootstrap a fully new Rust project with predefined structure and utility functions
- Add boilerplate for a new year of solutions inside a previously bootstrapped project
- Submit solutions (**NYI**)

## How do I use it?
<br>

> Elf tracks your current progress in the `elf.toml` file.<br>
> When you omit `--year` or `--day`, it will fall back to the values stored there.<br><br>
> Some examples:
> - `elf new bootstrap -l=rust "my_aoc"` will bootstrap a new, opinionated, cargo project named "my_aoc" in the current directory, with a module for the current year
> - `elf new` will create the stubs for the day after `elf.toml::day` in the year `elf.toml::year`
> - `elf new -d=11` will create the stubs for the day 11 in the year `elf.toml::year` and set `elf.toml::day` to 11
> - `elf new -y=2023` will create a new module for the 2023 AoC, including stubs for day 1, and adjust `elf.toml`

Elf always assumes to be run from the root of a bootstrapped project, except when using the `bootstrap` command.<br>
Available commands are:

### `elf bootstrap`:

| arg      | alt  | required  | effect                                                                     |
|----------|------|-----------|----------------------------------------------------------------------------|
| `<NAME>` | --   | **yes**   | Positional argument. The name of the project to be created.                |
| `--lang` | `-l` | **yes**   | The language you intend to use. Accepted values: "rust", "go" (NYI)        |
| `--year` | `-y` | no        | The year to create a module for. <br> Defaults to current year if missing. |


### `elf new`:
<br>

> If **--year** is set and **--day** is not, a new module for the given year will be created inside the bootstrapped project. <br> 
> In all other cases, year and day use their defaults to create new solution stubs.


| arg          | alt  | effect                                                       |
|--------------|------|--------------------------------------------------------------|
| `--year`     | `-y` | The year to create a solution stub for.                      |
| `--day`      | `-d` | The day to create a solution stub for.                       |
| `--template` | `-t` | Path to an optional template file to base the stub on. (NYI) |


### `elf submit`:
<br> 

> Stubs created by Elf write computed solutions into `elf.toml`, from which they are read when submitting. <br>

| arg      | alt  | effect                                                             |
|----------|------|--------------------------------------------------------------------|
| `--year` | `-y` | The year of the solution to submit.                                |
| `--day`  | `-d` | The day of the solution to submit.                                 |
| `--part` | `-p` | The part of the solution to submit. <br> Defaults to 1 if missing. |


