# Security Policy

## Supported versions

| Version | Supported |
| ------- | --------- |
| 1.0.x   | yes       |
| 0.6.x   | security fixes only |
| < 0.6   | no        |

## Reporting

ContextCrucible reads local files and writes packs/manifests locally - no
network calls. The spark-test reports secrets; it never transmits them. If you
find a vulnerability (path traversal in scan mode, unsafe manifest parsing),
open a private security advisory rather than a public issue. Expect a first
response within 7 days.
