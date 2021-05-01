# codedj-examples-ecoop-2021

A crate containing ![Djanco](https://github.com/PRL-PRG/djanco) queries.

This crate executes examples from the paper:

Petr Maj. Konrad Siek. Alexander Kovalenko. Jan Vitek.
*Reproducible Queries over Large-Scale Software Repositories.*
Submitted to ECOOP 2021.

There are 7 project selection queries in the paper:

* *Stars*: Pick projects with most stars. Rationale: starred projects are
  popular and thus likely to be well written and maintained. 
* *Touched Files*: compute #files changed by commits, pick projects that
  changed the most files. Rationale: indicative of projects where commits
  represent larger units of work.
* *Experienced Author*: experienced developers are those on GitHub for at least
  two years; pick a sample of projects with at least one experienced
  contributor. Rationale: less likely to be throw-away projects.
* *50% Experienced*: projects with two or more developers, half of which 
  experienced. Rationale: focus on larger teams.
* *Message Size*: Compute size in bytes of commit messages; pick projects with
  the largest size. Rationale: empty or trivial commit messages indicate
  uninteresting projects.
* *Number of Commits*: Compute the number of commits; pick projects with the
  most commits. Rationale: larger projects are more mature.
* *Issues*: Pick projects with the most issues. Rationale: issues indicate a
  more structured development process.
  
## Djanco

Djanco is a query system for querying GitHub datasets downloaded by 
![Parasite](https://github.com/PRL-PRG/codedj-parasite) as part of the CodeDJ
project.

## Running the queries

To generate a harness for executing the queries in this crate install 
![cargo-djanco](https://github.com/PRL-PRG/cargo-djanco):

```bash
cargo install --git https://github.com/PRL-PRG/cargo-djanco
```

Then, generate the harness:

```bash
cargo djanco
``` 

This generates s source file `src/bin/djanco.rs`, which you can run to execute all your queries:

```bash
cargo run --bin djanco --release -- --dataset-path DATASET_LIVES_HERE --output-path WRITE_RESULTS_HERE 
```

# Template

The template file for the codedj-examples-ecoop-2021 crate comes from 
![here](https://github.com/PRL-PRG/djanco-query-template). 

To create a new crate from the template, first install 
![cargo-generate](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate
```

Then, use the `generate` command in cargo to create a new crate from the 
template:

```bash
cargo generate --git https://github.com/PRL-PRG/djanco-query-template --name my-query-crate
```

Then you can add your query functions. There's an example function in 
`src/lib.rs` to get you started.

