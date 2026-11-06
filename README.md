# aktujo models

The models [aktujo](https://aktujo.dev) builds in – OCR for finding and reading text – as Rust crates, one crate per
model, so that `cargo` fetches them like any other dependency.

| Crate | Model | Size |
|---|---|---|
| [`aktujo-extract-det`](crates/aktujo-extract-det) | PP-OCRv3 text detection: where text stands on a page | 2.4 MB |
| [`aktujo-extract-latin`](crates/aktujo-extract-latin) | PP-OCRv5 Latin text recognition with its dictionary | 7.9 MB |

The crates hold the bytes only; [`aktujo-extract`](https://crates.io/crates/aktujo-extract) runs them, and
`aktujo-sdk` builds them in with its feature `ocr`. Each crate has a version of its own: a model is published again
only when it changes. More scripts come as crates of their own (`aktujo-extract-<script>`).

## Where the models come from

PaddleOCR's models, converted to ONNX, from [monkt/paddleocr-onnx](https://huggingface.co/monkt/paddleocr-onnx) at
commit `7b02d0a30a07ba2b92ad1ff5a8941ae2c633de65` – unchanged; each crate's `NOTICE` names the files and
`SHA256SUMS` their hashes. How they were chosen and measured: aktujo's
[`docs/spikes/ocr.md`](https://github.com/aktujo/aktujo/blob/main/docs/spikes/ocr.md).

## A new model

A crate under `crates/` with the files, `SHA256SUMS`, `NOTICE` (where the model comes from, under which licence),
`README.md`, the licence and constants for the bytes and hashes; a crate on crates.io may be 10 MiB at most, packed
(CI checks it). Commits are signed off (`git commit -s`, [DCO](https://developercertificate.org)).

## Releasing

Raise the crate's version, then tag the commit of `main` `<crate>-v<version>` (for example
`aktujo-extract-latin-v1.1.0`). CI publishes that crate through crates.io's Trusted Publishing, once the maintainer
approves the environment `crates-io`. A crate's first version is published by hand.

## Licence

Apache-2.0 ([LICENSE](LICENSE)), the models and the code around them; REUSE compliant.
