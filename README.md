# DBcat

A *cat(1)* equivalent for viewing sqlite database.


## Installation

- cargo: `cargo install dbcat`

### Usage

Read the database simply by running `dbcat <filename>.db`.

<!-- Asset -->

---

Change the format of the output by choice or automatically if the output is a tty or not.

<!-- Asset  -->
---

Access individual tables by running `dbcat [-t <table>] <filename>.db`.

<!-- Asset -->
---

Pass a filter query to filter the tables by running `dbcat [-t <table>] -f <query> <filename>.db`.

<!-- Asset -->
---
## Help

Use `dbcat --help` to see the help.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).