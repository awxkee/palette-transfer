/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */

use colorutils_rs::{lab_to_srgb, lab_with_alpha_to_rgba, rgb_to_lab, rgba_to_lab_with_alpha};

pub(crate) fn image_to_lab_rgb<const CHANNELS: usize>(
    src: &[u8],
    dst: &mut [f32],
    width: u32,
    height: u32,
) {
    if CHANNELS == 3 {
        rgb_to_lab(
            src,
            width * CHANNELS as u32,
            dst,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            width,
            height,
        );
    } else if CHANNELS == 4 {
        rgba_to_lab_with_alpha(
            src,
            width * CHANNELS as u32,
            dst,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            width,
            height,
        );
    }
}

pub(crate) fn lab_to_image_rgb<const CHANNELS: usize>(
    src: &[f32],
    dst: &mut [u8],
    width: u32,
    height: u32,
) {
    if CHANNELS == 3 {
        lab_to_srgb(
            src,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            dst,
            width * CHANNELS as u32,
            width,
            height,
        );
    } else if CHANNELS == 4 {
        lab_with_alpha_to_rgba(
            src,
            std::mem::size_of::<f32>() as u32 * width * CHANNELS as u32,
            dst,
            width * CHANNELS as u32,
            width,
            height,
        );
    }
}
