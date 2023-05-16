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

Execute a summed dice roll. ROLL argument should be standard dice notation, e.g. `3d100` or `2d6+1d10-3`. You can also use more complex arithmetic like `"(2d12+(3/(5*4d10)))"`.

Note: arguments with surrounding parentheses should be enclosed in double-quotes.

Exploding dice are supported, just use `!` at the end: `6d6!`

### `dicer pool [OPTIONS] <ROLL>`

Execute a pooled dice roll. ROLL argument should be dice notation (`3d6`, `2d10+4d6+2`), arithmetic (`1+2+3`), or a single integer.

Options:

- `-d, --die <NUMBER>`

  - _DEFAULT:_ 10
  - Set default number of die faces. Whenever an integer is used in argument, it represents amount of dice of this type.
  - Can be overridden by using dice notation.

- `-t, --threshold <FRACTION | DECIMAL | INTEGER>`

  - Set threshold for a successful roll. If higher than maximum die, results in 0 successes.
  - If less than 1, minimum for success equals `threshold * die_faces + 1` (rounded down). Ex. threshold of 3/4 for d10 requires 8 or higher for success.
  - If 1 or greater, represents dice target. Ex. threshold of 5 means all dice throws 5 or over will be successful.
  - Defaults to `die_faces / 2 + 1`
