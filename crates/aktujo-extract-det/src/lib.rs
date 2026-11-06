// SPDX-License-Identifier: Apache-2.0
//! The model with which aktujo finds where text stands on a page: PP-OCRv3 text detection, converted to ONNX
//! (Apache-2.0; where it comes from is in `NOTICE`). It holds the bytes only; `aktujo-extract` runs it.
#![cfg_attr(not(test), no_std)]

/// The model, an ONNX file.
pub const MODEL: &[u8] = include_bytes!("../det.onnx");

/// The SHA-256 hash of [`MODEL`], in lowercase hex – the same as in `SHA256SUMS`.
pub const MODEL_SHA256: &str = "ee40e80071ba3a320d4efda75f3e22047a7d049e9bf7bcaaf9daea23fc21b935";

#[cfg(test)]
mod tests {
    #[test]
    fn hash_as_in_the_sums() {
        let sums = include_str!("../SHA256SUMS");
        assert!(sums.lines().any(|l| l.split_whitespace().eq([super::MODEL_SHA256, "det.onnx"])));
        assert!(!super::MODEL.is_empty());
    }
}
