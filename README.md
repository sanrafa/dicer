# Dicer

**Dicer** is a dice-roller for your command line. It can return summed results or individual dice pools (a la World of Darkness). You can enter single commands, or start up a REPL to quickly switch between result types.

## Commands

### `dicer [OPTIONS]`

Open the Dicer REPL.

Options:

- `-p, --pool`
  - Enable POOL mode by default. Use `pool` command within REPL to activate.
  - Can be disabled by entering `roll` within REPL.

### `dicer roll <ROLL>`

Execute a summed dice roll. ROLL argument should be standard dice notation, e.g. `3d100` or `2d6+1d10-3`. Only addition and subtraction operations are allowed.

### `dicer pool [OPTIONS] <ROLL>`

Execute a pooled dice roll. ROLL argument should be dice notation (`3d6`, `2d10+4d6+2`), arithmetic (`1+2+3`), or a single integer. Only addition operations are allowed.

Options:

- `-d, --die <NUMBER>`

  - _DEFAULT:_ 10
  - Set default number of die faces. Whenever an integer is used in argument, it represents amount of dice of this type.
  - Can be overridden by using dice notation.

- `-t, --threshold <NUMBER>`

  - Set threshold for a successful roll. If higher than maximum die, results in 0 successes.
  - NOTE: this currently applies to the entire roll, regardless of types of dice used.
  - Defaults to `die_faces / 2 + 1`
