# 🧝 Elf - Advent of Code CLI

## What can this do?

- Scaffold a new Rust project with predefined structure and utility functions
- Create boilerplate for new AoC Solutions
- Option to define a template to use
- Load your puzzle input (requires a session cookie)
- Create modules for a new year of solutions inside a previously bootstrapped project
- Submit solutions

## How do I use it?

<br>

> Elf tracks your current progress in the `elf.toml` file.<br>
> Usage examples:
>
> - `elf new <project>` will create a new cargo project named \<project> in the current directory
> - `elf add 2023` will create a new module for the 2023 AoC, including stubs for day 1, and adjust `elf.toml`
> - `elf add 06` will create the stubs for day 6 in the year `elf.toml::year` and set `elf.toml::day` to 06
> - `elf next` will create the stubs for the day after `elf.toml::day` in the year `elf.toml::year`
> - `elf run` does the same as `cargo run`, but also writes the solution into `elf.toml`


Elf assumes it will to be run from the root of an elf-project, except when using the `new` command.<br>
Available commands are:

### `elf new`:

<br>

| arg      | alt  | required | effect                                                                                                                                               |
| -------- |------|----------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| `<NAME>` | --   | **yes**  | Positional argument. The name of the project to be created.                                                                                          |
| `--lang` | `-l` | no       | The language you intend to use. Defaults to Rust.<br><br> Accepted values: <br>[ 'rust', 'rs', 'c++', 'cpp', 'cc', 'kotlin', 'kt', 'go', 'golang' ]  |

### `elf next`:

Creates the solution stubs for the _next_ puzzle, based on the values in `elf.toml`.

### `elf add`:

<br>

Expands the existing scaffolding by either a new submodule or a solution stub.

> If you want to add a specific day to a specific year, you can change `year` in the `elf.toml`.
> Note that the module for that year needs to exist. It will not be auto-created.

| arg      | alt | effect                                                                                                                                          |
|----------|-----|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `<DATE>` | --  | Depending on the format, adds either that year as a new module, or that day in the configured year<br/>[2015-2029] for years, [01-25] for days. |

### `elf submit`:

<br>

> Stubs created by Elf write computed solutions into `elf.toml`, from which they are read when submitting. <br>
> Either specify year AND day, or neither to use the values from `elf.toml`

| arg      | alt  | effect                                                            |
| -------- | ---- | ----------------------------------------------------------------- |
| `--year` | `-y` | The year of the solution to submit. Defaults to `elf.toml::year`. |
| `--day`  | `-d` | The day of the solution to submit. Defaults to `elf.toml::day`.   |
| `--part` | `-p` | The part of the solution to submit. <br> Defaults to 1.           |

### `elf set`:

<br>

> Convenience command to manipulate values in `elf.toml`, e.g. to switch the current year context.
> If template is set, that file will be used for new days instead of the default.

| arg          | alt  | effect                    |
| ------------ | ---- | ------------------------- |
| `--year`     | `-y` | Sets `elf.toml::year`     |
| `--day`      | `-d` | Sets `elf.toml::day`      |
| `--session`  | `-s` | Sets `elf.toml::session`  |
| `--template` | `-t` | Sets `elf.toml::template` |

### `elf run`:

<br>

> Internally calls `cargo run` and writes part of the `stdout` into `elf.toml`.
> It assumes one line per solution and that the solution value is the first word of the output.
> Takes `[ -r | --release ]` as optional flag and passes it to `cargo`
