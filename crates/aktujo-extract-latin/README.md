# aktujo-extract-latin

The model with which [aktujo](https://aktujo.dev) reads Latin script – PP-OCRv5 Latin text recognition, converted to
ONNX, about 7.9 MB, with the dictionary of its characters. It reads English, German, French, Spanish, Italian,
Portuguese, Dutch, Polish, Czech, Turkish and other languages in Latin script. The crate holds the bytes only;
`aktujo-extract` runs the model on the text that
[`aktujo-extract-det`](https://crates.io/crates/aktujo-extract-det) found.

```rust
let model: &[u8] = aktujo_extract_latin::MODEL;
let dictionary: &str = aktujo_extract_latin::DICT;
```

`MODEL_SHA256` and `DICT_SHA256` are the hashes of the files, as in `SHA256SUMS`.

Apache-2.0 – where the model comes from is in [NOTICE](NOTICE). Source:
[github.com/aktujo/models](https://github.com/aktujo/models).
