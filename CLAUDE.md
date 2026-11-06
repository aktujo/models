# CLAUDE.md – aktujo models

`aktujo/models`: the models aktujo builds in, as Rust crates, one per model. This file is public: nothing personal
goes here. aktujo itself lives in `aktujo/aktujo`, checked out next to this one as `../aktujo`; its `aktujo-sdk`
depends on these crates (feature `ocr`), `aktujo-extract` runs the models.

- **English** in files and commits; talk to the maintainer in German. Commits signed off (`git commit -s`).
- **The models are unchanged copies** of their source; each crate's `NOTICE` names the source (with its commit) and
  the licence, `SHA256SUMS` the hashes, and the constants in `src/lib.rs` repeat them (a test compares them).
- **No dependencies** in the crates – bytes and hashes only, `no_std`.
- **One version per crate**, raised only when its model changes; released by a tag `<crate>-v<version>` (README).
  A packed crate stays under 10 MiB (crates.io's limit; CI checks it).
- **Never delete (your own actions):** move files to `_to_delete/` (git-ignored) instead.
- **Verified before claimed:** mark what is not verified as `OPEN`. Licence Apache-2.0, REUSE compliant.
