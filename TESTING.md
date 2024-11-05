# Benchmarking & Testing

## Simulation framework (Simpa)

Logging in `calicod` and `simpa` can be [filtered](https://docs.rs/env_logger/0.10.0/env_logger/#filtering-results)
by either:

The current codebase supports a full in-process network simulation,
building an actual DAG over virtual time with virtual delay and
benchmarking validation time (following the simulation generation).
To see the available commands.

```bash
cargo run --release --bin simpa -- --help
```

The following command will run a simulation to produce 1000 blocks
with communication delay of 2 seconds and 8 BPS (blocks per second)
while attempting to fill each block with up to 200 transactions.

```bash
cargo run --release --bin simpa -- -t=200 -d=2 -b=8 -n=1000
```

## Heap Profiling

Heap-profiling in `calicod` and `simpa` can be done by enabling
`heap` feature and profile using the `--features` argument.

```bash
cargo run --bin calicod --profile heap --features=heap
```

It will produce `{bin-name}-heap.json` file in the root of the workdir,
that can be inspected by the [dhat-viewer](https://github.com/unofficial-mirror/valgrind/tree/master/dhat)

## Tests

Run unit and most integration tests:

```bash
cd rusty-calico
cargo test --release
// or install nextest and run
```

Using nextest:

```bash
cd rusty-calico
cargo nextest run --release
```

## Benchmarks

```bash
cd rusty-calico
cargo bench
```

## Logging

Logging in `calicod` and `simpa` can be [filtered](https://docs.rs/env_logger/0.10.0/env_logger/#filtering-results)
by either:

1. Defining the environment variable `RUST_LOG`
2. Adding the --loglevel argument like in the following example:

   ```
   (cargo run --bin calicod -- --loglevel info,calico_rpc_core=trace,calico_grpc_core=trace,consensus=trace,calico_core=trace) 2>&1 | tee ~/rusty-calico.log
   ```

   In this command we set the `loglevel` to `INFO`.
