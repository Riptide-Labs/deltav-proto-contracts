# Support

This repository holds the Protobuf schemas and gRPC contracts for Delta-V.
It publishes no service, so most questions are about the schemas themselves, the generated Java and Rust artifacts, or the build.

## Where to ask

| You want to | Go to |
|---|---|
| Report something broken in the schemas, the generated code, or the build | [Bug report](https://github.com/Riptide-Labs/deltav-proto-contracts/issues/new?template=bug_report.yml) |
| Propose a schema change or a new message, field, or service | [Enhancement](https://github.com/Riptide-Labs/deltav-proto-contracts/issues/new?template=enhancement.yml) |
| Report a security vulnerability | [Private advisory](https://github.com/Riptide-Labs/deltav-proto-contracts/security/advisories/new), never a public issue. See [SECURITY.md](SECURITY.md) |
| Ask how to consume the artifacts | An issue using the bug form, or the [README](README.md) Quick Start first |

Discussions are not enabled. Issues are the only channel, which keeps every answer searchable.

## Before you open an issue

- Search the [existing issues](https://github.com/Riptide-Labs/deltav-proto-contracts/issues?q=is%3Aissue). Schema questions repeat.
- Say which version you are on: a release tag, or the commit SHA.
- Include the command you ran and its output, not a description of it.

## Questions about a Delta-V service

Problems with a service that consumes these contracts belong in that service's own repository.
Only open an issue here if the schema itself is wrong or missing something.

## Response times

This is a small maintainer team. Expect a first response within a few working days.
Security reports are the exception and follow the timelines in [SECURITY.md](SECURITY.md).

## Contributing a fix

Faster than waiting, but work starts from an issue here too: open one, then the pull request that references it.
[CONTRIBUTING.md](CONTRIBUTING.md) covers the rest: DCO sign-off, the Conventional Commit format, and the backwards-compatibility rules that govern schema changes.
