# rustic_rosalind
Solutions to problems from rosalind.info in the rust language. A learner's project.

## Getting started

Clone the project and compile with `cargo build`. This will produce a binary at
`./target/debug/rosalind` (or `./target/release/rosalind`if you use
`cargo build --release`).

## How to use

Each problem from Rosalind has a signifying ID, like "dna"
(http://rosalind.info/problems/dna/) or "rna"
(http://rosalind.info/problems/rna/). These will be the names of subcommands for
the `rosalind` command line interface. Want to see the options? Run `rosalind`
with `--help`. You can also get help for subcommands (`rosalind dna --help`)
but they should all be basically the same format: `rosalind <problem-id> <input>`.

Rosalind presents you with file inputs to use as parameters for a problem, so
that you can't just look up the answers ahead of time. Thus `rosalind` expects
you to provide it with an input file for each subcommand. I haven't worried much
about input validation and error handling, so give it the right input or you
might get odd results!

## Design of the project

The command line interface is implemented using `clap` and `yaml`, with the
nicely extensible `./src/cli.yaml` file. Whenever a new problem solver is
implemented, I have been adding a new subcommand for it. I am sure that there
is a clever-er way to do this, but for now it will do.

The `main.rs` file is basically just responsible for pulling together the CLI
and the modules for solvers.

The `lib.rs` file exposes the solver interfaces.

Each problem is given a module file. So the solver for "dna" can be found in
`./src/dna.rs`. Each module should provide a file parser that produces suitable
results for ingestion by the solver function that it also provides. The solver
in turn should produce a result that implements `std::fmt::Display` such that
the printed output is a solution that can be uploaded to Rosalind. Most of these
modules should also have some tests!

## About the author

**Paul Barton <pablo.barton@gmail.com>**

Coming from a background that is primarily focused on Python, this project is a
first practical foray into Rust.

I am still learning to walk before I can run, but compared to Python it is like
walking _really fast!_ 

## Contributing

If you are interesting in contributing or have any advice for coding style and
design, let me know.
