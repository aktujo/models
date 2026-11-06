// SPDX-License-Identifier: Apache-2.0
//! The model with which aktujo reads Latin script: PP-OCRv5 Latin text recognition, converted to ONNX, with the
//! dictionary of its characters (Apache-2.0; where it comes from is in `NOTICE`). It holds the bytes only;
//! `aktujo-extract` runs it, after the text was found with `aktujo-extract-det`.
#![cfg_attr(not(test), no_std)]

/// The model, an ONNX file.
pub const MODEL: &[u8] = include_bytes!("../rec.onnx");

/// The characters the model tells apart, one per line, in the order of its output.
pub const DICT: &str = include_str!("../dict.txt");

/// The SHA-256 hash of [`MODEL`], in lowercase hex – the same as in `SHA256SUMS`.
pub const MODEL_SHA256: &str = "614ffc2d6d3902d360fad7f1b0dd455ee45e877069d14c4e51a99dc4ef144409";

/// The SHA-256 hash of [`DICT`], in lowercase hex – the same as in `SHA256SUMS`.
pub const DICT_SHA256: &str = "3c0a8a79b612653c25f765271714f71281e4e955962c153e272b7b8c1d2b13ff";

#[cfg(test)]
mod tests {
    #[test]
    fn hashes_as_in_the_sums() {
        let sums = include_str!("../SHA256SUMS");
        for (hash, file) in [(super::MODEL_SHA256, "rec.onnx"), (super::DICT_SHA256, "dict.txt")] {
            assert!(sums.lines().any(|l| l.split_whitespace().eq([hash, file])), "{file}");
        }
        assert!(!super::MODEL.is_empty());
        assert!(super::DICT.lines().count() > 100);
    }
}
