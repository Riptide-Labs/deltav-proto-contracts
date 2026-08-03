# Security Policy

## Supported versions

Only the latest released minor version of `deltav-proto-contracts` receives security fixes.

## Reporting a vulnerability

Report vulnerabilities privately through GitHub's [private vulnerability reporting](https://github.com/Riptide-Labs/deltav-proto-contracts/security/advisories/new).
This opens a draft advisory visible only to you and the maintainers.

While this repository is private, GitHub does not offer private vulnerability reporting on it.
Until it is made public, contact a Riptide-Labs maintainer directly instead.

Do not open a public issue for a security problem, and do not disclose it publicly before a fix is available.

Please include:

- what the problem is and why it is exploitable
- the affected version or commit
- reproduction steps or a proof of concept
- the impact you expect

You will get an acknowledgement within 5 working days and an assessment within 10 working days.
We aim to ship a fix and publish an advisory within 90 days of the report, and will keep you updated if that slips.

## Scope

This repository publishes schemas and generated stubs; it has no running service.
The realistic surface is the build and release pipeline (workflow permissions, action pins, artifact signing) and dependency vulnerabilities in the generated Java and Rust artifacts.
Reports about the Delta-V services that consume these contracts belong in their own repositories.
