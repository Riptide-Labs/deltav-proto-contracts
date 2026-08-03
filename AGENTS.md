# AGENTS.md

Canonical Protobuf schemas and gRPC contracts for Delta-V. Schemas only: no service runs from this repo.

## Commands

`make` is the only entry point. CI runs exactly these targets, nothing else.

```bash
make verify                  # lint + build + test, the full gate
make lint                    # buf lint, rustfmt, clippy
make breaking                # backwards-compatibility check against origin/main
make build                   # Java (mvn) and Rust (cargo) artifacts
make test                    # both test suites
make help                    # every target
cargo test --locked <name>   # a single Rust test
mvn -B test -Dtest=<Class>   # a single Java test
```

Needs JDK 21, a Rust toolchain, `protoc`, and `buf`.

## Layout

`proto/deltav/{telemetry,nodes,alarms}/v1/` holds the schemas. Everything else is generated from them:
the Rust crate through `build.rs` + `tonic-build`, the Java artifact through `protobuf-maven-plugin`,
Java and Go stubs through `buf generate` (`buf.gen.yaml`) into `gen/`. `src/lib.rs` is only
`include_proto!` wrappers, so there is no hand-written logic to change here.

## Conventions

- Editing a schema changes a published wire contract. `make breaking` gates every PR; additive changes
  are safe, renames and field-number changes are not. A genuine break needs a `BREAKING CHANGE:` footer
  and an explicit migration path.
- `buf lint` runs the DEFAULT ruleset. Enum values need their enum name as a prefix
  (`METRIC_TYPE_GAUGE`, not `GAUGE`), and RPC request types must be named `<Method>Request`.
- The Java and Rust versions must agree. `pom.xml` and `Cargo.toml` carry the same number, bumped
  together in the release commit.
- protobuf-java stays on the 3.25.x line and must not move to 4.x. grpc-java declares 3.25.x through
  its latest release, and `protobuf.version` also selects protoc, so a 4.x bump ships gencode that
  consumers on 3.25.x cannot load. The full reasoning is in `pom.xml` next to the property.
- Workflow actions are pinned to a commit SHA with the full version in a trailing comment. zizmor and
  actionlint enforce this, so an unpinned `uses:` fails CI.
- Commits are Conventional Commits, signed off (`git commit -s`), with an `Assisted-by:` trailer when
  AI-assisted. See CONTRIBUTING.md.

## Gotchas

- In the Makefile, `#` must be escaped (`\#`). Unescaped it starts a comment and silently truncates
  `BUF_AGAINST`, which makes the breaking-change gate compare HEAD against itself and always pass.
