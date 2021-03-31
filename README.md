# {{project-name}}

A crate containing ![Djanco](https://github.com/PRL-PRG/djanco) queries.

This crate... *TODO*

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

Now you can execute the query with:

```bash
cargo --bin djanco --release -- --dataset-path DATASET_LIVES_HERE --output-path WRITE_RESULTS_HERE 
```

# Template

The template file for the {{project-name}} crate comes from 
![here](https://github.com/PRL-PRG/djanco-query-template). 

To create a new crate from the template, first install 
![cargo-generate](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate
```

Then, use the `generate` command in cargo to create a new crate from the 
template:

```bash
cargo generate --git https://github.com/PRL-PRG/djanco-query-template --name my_query_crate
```

