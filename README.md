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
> - `elf new -l=rust "my_aoc"` will create a new, opinionated, cargo project named "my_aoc" in the current directory, with a module for the current year
> - `elf next` will create the stubs for the day after `elf.toml::day` in the year `elf.toml::year`
> - `elf add -d=06` will create the stubs for day 6 in the year `elf.toml::year` and set `elf.toml::day` to 06
> - `elf add -y=2023` will create a new module for the 2023 AoC, including stubs for day 1, and adjust `elf.toml`

Elf always assumes to be run from the root of a bootstrapped project, except when using the `bootstrap` command.<br>
Available commands are:

### `elf new`:
<br>

| arg      | alt  | required  | effect                                                                     |
|----------|------|-----------|----------------------------------------------------------------------------|
| `<NAME>` | --   | **yes**   | Positional argument. The name of the project to be created.                |
| `--lang` | `-l` | **yes**   | The language you intend to use. Accepted values: "rust", "go" (NYI)        |
| `--year` | `-y` | no        | The year to create a module for. <br> Defaults to current year if missing. |


### `elf next`:

Creates the solution stubs for the *next* puzzle, based on the values in `elf.toml`.


### `elf add`:
<br>

Expands the existing scaffolding by either a new submodule or a solution stub.

> For a cleaner API, `--year` and `--day` are mutually exclusive.<br>
> If you want to add a specific day to a specific year, you can edit `year` in the `elf.toml`.


| arg          | alt  | effect                                                       |
|--------------|------|--------------------------------------------------------------|
| `--year`     | `-y` | The year to add a module for.                                |
| `--day`      | `-d` | The day to create a solution stub for. Left pad with 0.      |
| `--template` | `-t` | Path to an optional template file to base the stub on. (NYI) |


### `elf submit`:
<br> 

> Stubs created by Elf write computed solutions into `elf.toml`, from which they are read when submitting. <br>

| arg      | alt  | effect                                                             |
|----------|------|--------------------------------------------------------------------|
| `--year` | `-y` | The year of the solution to submit.                                |
| `--day`  | `-d` | The day of the solution to submit. Left pad with 0.                |
| `--part` | `-p` | The part of the solution to submit. <br> Defaults to 1 if missing. |

### `elf set`:
<br>

Convenience command to manipulate year/day in `elf.toml`, e.g. to switch the current year context. 

| arg      | alt  | effect                          |
|----------|------|---------------------------------|
| `--year` | `-y` | Sets `elf.toml::year` to <YEAR> |
| `--day`  | `-d` | Sets `elf.toml::day` to <DAY>   |
