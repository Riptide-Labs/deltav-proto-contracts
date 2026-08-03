# Contributing

Thanks for helping improve `deltav-proto-contracts`.

This repository holds the canonical Protobuf schemas and gRPC contracts for Delta-V.
Every change here is a change to a published wire contract, so review is deliberate.

## Before you start

Work starts from an issue, not a drive-by pull request.
Open a bug or enhancement issue first so the change can be discussed before anyone writes code.
If you are already fixing something trivial, open the issue anyway and reference it.

## Developer Certificate of Origin

All commits must be signed off under the [Developer Certificate of Origin](https://developercertificate.org/).
Sign off by committing with `-s`:

```bash
git commit -s -m "fix(proto): ..."
```

This appends a `Signed-off-by:` trailer with your real name and email.
The trailer must name a human, never a tool or an AI agent.
By signing off you certify that you wrote the change or otherwise have the right to submit it under the project's license.

## AI-assisted contributions

AI assistance is welcome and must be disclosed.
Any commit produced with help from an AI agent carries an `Assisted-by:` trailer naming the agent and model:

```
Assisted-by: ClaudeCode:claude-opus-5
Signed-off-by: Jane Doe <jane@example.org>
```

The `Assisted-by:` trailer goes first, `Signed-off-by:` last.
Disclosure does not transfer responsibility.
The human who signs off remains accountable for reviewing the change, for its correctness, and for its license compliance.
Do not sign off on generated code you have not read.

## Commit messages

[Conventional Commits](https://www.conventionalcommits.org/), referencing the GitHub issue:

```
fix(proto): correct the metric provenance field number (#42)
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`, `build`, `revert`.
Breaking changes append `!` to the type and add a `BREAKING CHANGE:` footer explaining the migration.
The release version is derived from these types, so they matter.

## Branches and pull requests

`main` is protected. Branch, then open a pull request against `main`.

```bash
git switch -c fix/metric-provenance-field
```

Pull requests must include a `Closes #<issue>` line so the issue resolves on merge.
Pull requests are squash-merged, so the PR title becomes the commit on `main` and must itself be a valid Conventional Commit.

## Building and testing

`make` is the only entry point. CI runs exactly these targets.

```bash
make verify        # lint + build + test, the full gate
make lint          # buf lint, rustfmt, clippy
make breaking      # schema backwards-compatibility check against origin/main
make build         # Java and Rust artifacts
make test          # Java and Rust test suites
make help          # every target
```

You need JDK 21, a Rust toolchain, `protoc`, and [`buf`](https://buf.build/docs/installation).

## Changing a schema

`make breaking` guards backwards compatibility and runs on every pull request.
Additive changes are safe. Renaming or removing a field, changing a field number or type, and renaming an enum value are not.

If a break is genuinely required, say so explicitly in the pull request, explain why the compatible alternative does not work, and mark the commit with `BREAKING CHANGE:`.
Expect this to be the slowest kind of review in the repository.

## License headers

Every source file carries an SPDX header:

```
// Copyright (C) 2026 The Delta-V Authors
// SPDX-License-Identifier: Apache-2.0
```

Contributions are licensed under [Apache-2.0](LICENSE).
