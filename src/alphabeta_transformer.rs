/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */

use crate::image_configuration::ImageConfiguration;
use colorutils_rs::{
    bgr_to_lalphabeta, bgra_to_lalphabeta, lalphabeta_to_bgr, lalphabeta_to_bgra,
    lalphabeta_to_rgb, lalphabeta_to_rgba, rgb_to_lalphabeta, rgba_to_lalphabeta, TransferFunction,
};

pub(crate) fn image_to_lalphabeta_rgb<const CHANNELS: u8>(
    src: &[u8],
    dst: &mut [f32],
    width: u32,
    height: u32,
) {
    let channels: ImageConfiguration = CHANNELS.into();

    let worker = match channels {
        ImageConfiguration::Rgb => rgb_to_lalphabeta,
        ImageConfiguration::Rgba => rgba_to_lalphabeta,
        ImageConfiguration::Bgra => bgra_to_lalphabeta,
        ImageConfiguration::Bgr => bgr_to_lalphabeta,
    };
    worker(
        src,
        width * channels.get_channels_count() as u32,
        dst,
        width * std::mem::size_of::<f32>() as u32 * channels.get_channels_count() as u32,
        width,
        height,
        TransferFunction::Srgb,
    );
}

pub(crate) fn lalpha_beta_to_image_rgb<const CHANNELS: u8>(
    src: &[f32],
    dst: &mut [u8],
    width: u32,
    height: u32,
) {
    let channels: ImageConfiguration = CHANNELS.into();

    let worker = match channels {
        ImageConfiguration::Rgb => lalphabeta_to_rgb,
        ImageConfiguration::Rgba => lalphabeta_to_rgba,
        ImageConfiguration::Bgra => lalphabeta_to_bgra,
        ImageConfiguration::Bgr => lalphabeta_to_bgr,
    };
    worker(
        src,
        width * std::mem::size_of::<f32>() as u32 * channels.get_channels_count() as u32,
        dst,
        width * channels.get_channels_count() as u32,
        width,
        height,
        TransferFunction::Srgb,
    );
}
