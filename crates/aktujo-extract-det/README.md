# aktujo-extract-det

The model with which [aktujo](https://aktujo.dev) finds where text stands on a page – PP-OCRv3 text detection,
converted to ONNX, about 2.4 MB. The crate holds the bytes only; `aktujo-extract` runs the model, and
[`aktujo-extract-latin`](https://crates.io/crates/aktujo-extract-latin) then reads what it found.

```rust
let model: &[u8] = aktujo_extract_det::MODEL;
```

`MODEL_SHA256` is the hash of the file, as in `SHA256SUMS`. aktujo uses the detection of PP-OCRv3: that of
PP-OCRv5 is about 84 MB, too large for a crate.

Apache-2.0 – where the model comes from is in [NOTICE](NOTICE). Source:
[github.com/aktujo/models](https://github.com/aktujo/models).
