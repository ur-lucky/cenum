# cenum

`cenum` is a Rokit-friendly Rust CLI that compiles YAML enum definitions into one
self-contained Luau module.

## Usage

```sh
cenum build
cenum build cenum.yaml --solver new --output src/shared/Enums.luau --use-const
cenum build cenum.yaml --no-use-const
```

By default, `cenum build` reads `cenum.yaml`.

```yaml
output: src/shared/Enums.luau
solver: old
use-const: false
enums:
  TransactionType:
    - Robux
    - Tickets
    - Diamonds
    - Gold
  TransactionStatus:
    - Pending
    - Completed
    - Failed
```

`solver` defaults to `old`, and `use-const` defaults to `false`. `output` is
required unless `--output` is passed.

## Rokit

After publishing release artifacts for this repository, consumers can add it to a
Rokit project:

```sh
rokit add ur-lucky/cenum@<version>
cenum build
```

Release archives should contain the standalone `cenum` executable for each target
platform Rokit should install.

## Releasing

The GitHub Actions release workflow builds Rokit-compatible `.zip` archives for:

- `windows-x86_64`
- `windows-aarch64`
- `linux-x86_64`
- `linux-aarch64`
- `macos-x86_64`
- `macos-aarch64`

To create a release, push a version tag:

```sh
git tag v0.1.0
git push origin v0.1.0
```

The workflow creates a draft GitHub release with assets named like
`cenum-0.1.0-windows-x86_64.zip`.
