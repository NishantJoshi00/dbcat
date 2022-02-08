# DBcat
[![Crates.io](https://img.shields.io/crates/v/dbcat)](https://crates.io/crates/dbcat)
[![Documentation](https://docs.rs/dbcat/badge.svg)](https://docs.rs/dbcat)
[![License](https://img.shields.io/crates/l/dbcat)](https://github.com/nishantjoshi00/dbcat/LICENSE)

A *cat(1)* equivalent for viewing sqlite database.


## Installation

- cargo: `cargo install dbcat`

### Usage

Read the database simply by running `dbcat <filename>.db`.

![Basic Usage](./assets/basic.gif)


---

Change the format of the output by choice or automatically if the output is a tty or not.

![Atty Usage](./assets/atty_example.gif)

---

Access individual tables by running `dbcat [-t <table>] <filename>.db`.

![Tables Access](./assets/tables.gif)
---

Pass a filter query to filter the tables by running `dbcat [-t <table>] -f <query> <filename>.db`.

![Filter Command](./assets/query.gif)
---
## Help

Use `dbcat --help` to see the help.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).