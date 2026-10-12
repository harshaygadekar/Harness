# Size metrics: Harness vs fx

Measured SHA: `08faa76` (origin/main). Date: 2026-10-12. Host: Linux x86_64 box.

## Tool versions

- cargo 1.85.1, rustc 1.85.1
- cloc 2.04
- cargo-bloat 0.12.1

## Table

| Item | Code LOC (no tests) | Code LOC (with tests) | Binary size | Direct normal deps | Total normal deps |
|---|---|---|---|---|---|
| Harness (workspace, `harness` bin) | 2,865 | 3,803 | 3,590,776 B stripped; 10,406,608 B unstripped | 6 (harness-cli) | 69 (harness-cli); 72 (workspace) |
| harness-core (core loop) | 488 | 597 | about 38.9 KiB of .text (cargo bloat estimate) | 3 | 17 |
| harness-cli | 834 | 921 | (holds the bin) | 6 | 69 |
| harness-model | 660 | 1,191 | about 40.7 KiB of .text | n/a | n/a |
| harness-tools | 883 | 1,094 | about 47.4 KiB of .text | n/a | n/a |
| fx 0.0.13 | Cannot measure | Cannot measure | 13,180,752 B (single file) | Cannot measure | Cannot measure |

Notes:

- "Total" counts from `cargo tree` include the root crate line itself.
- Tests are inline `#[cfg(test)]` modules. Each one is at the end of its file. For "no tests", we cut each file at the `#[cfg(test)]` line. Then we ran cloc. cloc already skips comments and blanks.
- The release profile sets `strip = true`, `lto = true`, `opt-level = "z"`. The unstripped build sets `CARGO_PROFILE_RELEASE_STRIP=false`.
- cargo bloat numbers are estimates (the tool says so). Total .text is 1.7 MiB. harness-core is 2.2% of .text.

## fx limits

- The binary is at `/tmp/fxbin/fx` (`which fx`). The installer gets one prebuilt archive from `releases.fx.sh`. The install dir holds only the binary. `~/.fx` holds only settings and backups.
- No source is installed. So we cannot count LOC.
- No lockfile or package manifest is installed. So we cannot count dependencies.
- We do not guess these values.

## Build and binary details

`file` is not installed on the box. We used `readelf` (binutils) instead.

### Harness

- Toolchain: rustc 1.85.1, target `x86_64-unknown-linux-gnu` (host triple, from `rustc -vV`).
- Release profile in `native/Cargo.toml`: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true`. `panic` is not set, so it is the default (`unwind`).
- The release binary is an ELF64 x86-64 PIE. It is dynamic: it needs `libc.so.6` and `libgcc_s.so.1`. Its interpreter is `/lib64/ld-linux-x86-64.so.2`. It is stripped (no `.symtab`).
- The unstripped size comes from the same profile with `CARGO_PROFILE_RELEASE_STRIP=false`.

### fx

- 13,180,752 B is the prebuilt release binary of fx v0.0.13. The official install script downloaded it. We did not build it.
- Install script URL pattern: `https://releases.fx.sh/<version>/fx-<os>-<arch>.tar.gz`. On this box that is `fx-linux-x86_64.tar.gz`. The script takes `fx` out of the archive and installs it. We measured the binary, not the archive.
- The binary is an ELF64 x86-64 executable. It is static: it has no dynamic section and no `INTERP` segment. It is stripped (no `.symtab`, 8 section headers).

Commands:

```bash
rustc -vV
readelf -h -l -d target/release/harness
readelf -S target/release/harness | grep symtab
readelf -h -l -d -S /tmp/fxbin/fx
```

## Commands

```bash
cd native
# LOC with tests
cloc --quiet --csv --include-lang=Rust crates/<crate>
# LOC without tests
for f in $(find crates -name '*.rs'); do mkdir -p /tmp/notest/$(dirname $f)
  awk '/#\[cfg\(test\)\]/{exit} {print}' $f > /tmp/notest/$f; done
cloc --quiet --csv --include-lang=Rust /tmp/notest/crates/<crate>
# Binary size
cargo build --release && stat -c %s target/release/harness
CARGO_PROFILE_RELEASE_STRIP=false cargo build --release --target-dir /tmp/wt-size-nostrip
stat -c %s /tmp/wt-size-nostrip/release/harness
# Dependencies
cargo tree --depth 1 -e normal -p harness-cli
cargo tree -e normal --prefix none -p harness-cli | sort -u | wc -l
cargo tree -e normal --prefix none --workspace | sort -u | wc -l
cargo tree --depth 1 -e normal -p harness-core
cargo tree -e normal --prefix none -p harness-core | sort -u | wc -l
# Per-crate binary share
cargo install cargo-bloat --locked
CARGO_PROFILE_RELEASE_STRIP=false cargo bloat --release --crates -n 15 -p harness-cli --target-dir /tmp/wt-size-nostrip
# fx
which fx && stat -c %s "$(which fx)" && fx --version
```
