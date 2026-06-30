# cenum

`cenum` is a Rokit-friendly Rust CLI that compiles YAML enum definitions into one
self-contained Luau module.

## Usage

```sh
cenum init
cenum build
cenum build cenum.yaml --solver new --output src/shared/Enums.luau
cenum build cenum.yaml
```

By default, `cenum init` writes `cenum.yaml`, and `cenum build` reads
`cenum.yaml`.

```yaml
output: src/shared/CEnums.luau
solver: old
use-basic: false
enums: {}
```

`solver` defaults to `old`, and `output` is
required unless `--output` is passed. Set `use-basic: true` to emit simple
primitive enum values using the `old-solver/uenum.luau` style instead of the
full enum-item runtime.

Generated modules include lightweight enum serialization helpers:

```luau
local Enums = require(path.to.Enums)

local payload = Enums:serialize(Enums.TransactionStatus.Completed)
local status = Enums:deserialize(payload)
```

Enum items serialize to tagged plain data like
`{ __syncType = "CustomEnum", enumName = "TransactionStatus", value = 2 }`.
Tables are serialized recursively, and tables with enum/table keys are encoded
as tagged maps so they can be sent through remotes or saved in DataStores
without enum metatables, cycles, or back-references.

When `use-basic: true` is enabled, generated modules do not include
`serialize` or `deserialize`; enum values are numbers keyed by enum item name
and can be sent across the network directly.

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

If a release already exists, run the `Release` workflow manually with the existing
tag, such as `v0.1.0`. The workflow will rebuild every platform archive and
upload them to that release with replacement enabled.
