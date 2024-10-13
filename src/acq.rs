/*
 * // Copyright 2024 (c) the Radzivon Bartoshyk. All rights reserved.
 * //
 * // Use of this source code is governed by a BSD-style
 * // license that can be found in the LICENSE file.
 */
use crate::fast_average::Average;
use crate::fast_std_dev::StdDev;
use crate::image_configuration::ImageConfiguration;
use crate::TransferColorspace;

#[inline]
fn lerp(a: f32, b: f32, f: f32) -> f32 {
    a + f * (b - a)
}

fn copy_palette_impl<const SOURCE_IMAGE_CONFIGURATION: u8>(
    src: &[u8],
    src_width: u32,
    src_height: u32,
    target: &mut [u8],
    target_width: u32,
    target_height: u32,
    intensity: f32,
    perceptual_transformer: fn(&[u8], &mut [f32], u32, u32),
    display_transformer: fn(&[f32], &mut [u8], u32, u32),
) -> Result<(), String> {
    let source_configuration: ImageConfiguration = SOURCE_IMAGE_CONFIGURATION.into();
    let destination_configuration: ImageConfiguration = SOURCE_IMAGE_CONFIGURATION.into();
    if src.len()
        != src_width as usize * src_height as usize * source_configuration.get_channels_count()
    {
        return Err(format!(
            "Source image must be {} but it is {}",
            src_width as usize * src_height as usize * source_configuration.get_channels_count(),
            src.len()
        ));
    }
    if target.len()
        != target_width as usize
            * target_height as usize
            * destination_configuration.get_channels_count()
    {
        return Err(format!(
            "Target image must be {} but it is {}",
            target_width as usize
                * target_height as usize
                * destination_configuration.get_channels_count(),
            target.len()
        ));
    }

    let mut perceptual_source: Vec<f32> =
        vec![
            0.;
            src_width as usize * src_height as usize * source_configuration.get_channels_count()
        ];

    let mut perceptual_target: Vec<f32> = vec![
        0.;
        target_width as usize
            * target_height as usize
            * destination_configuration.get_channels_count()
    ];

    perceptual_transformer(src, &mut perceptual_source, src_width, src_height);
    perceptual_transformer(target, &mut perceptual_target, target_width, target_height);

    let mut source_lightness: Vec<f32> = vec![0.; src_width as usize * src_height as usize];
    let mut source_alpha: Vec<f32> = vec![0.; src_width as usize * src_height as usize];
    let mut source_beta: Vec<f32> = vec![0.; src_width as usize * src_height as usize];
    let mut source_display_alpha: Vec<f32> = vec![0.; src_width as usize * src_height as usize];

    let mut target_lightness: Vec<f32> = vec![0.; target_width as usize * target_height as usize];
    let mut target_alpha: Vec<f32> = vec![0.; target_width as usize * target_height as usize];
    let mut target_beta: Vec<f32> = vec![0.; target_width as usize * target_height as usize];
    let mut target_display_alpha: Vec<f32> =
        vec![0.; target_width as usize * target_height as usize];

    for ((((perceptual_chunk, lightness_chunk), alpha_chunk), beta_chunk), display_alpha_chunk) in
        perceptual_source
            .chunks_exact(source_configuration.get_channels_count())
            .zip(&mut source_lightness)
            .zip(&mut source_alpha)
            .zip(&mut source_beta)
            .zip(&mut source_display_alpha)
    {
        unsafe {
            *lightness_chunk = *perceptual_chunk.get_unchecked(0);
            *alpha_chunk = *perceptual_chunk.get_unchecked(1);
            *beta_chunk = *perceptual_chunk.get_unchecked(2);
            if source_configuration.has_alpha() {
                *display_alpha_chunk = *perceptual_chunk.get_unchecked(3);
            }
        }
    }

    for ((((perceptual_chunk, lightness_chunk), alpha_chunk), beta_chunk), display_alpha_chunk) in
        perceptual_target
            .chunks_exact(source_configuration.get_channels_count())
            .zip(&mut target_lightness)
            .zip(&mut target_alpha)
            .zip(&mut target_beta)
            .zip(&mut target_display_alpha)
    {
        unsafe {
            *lightness_chunk = *perceptual_chunk.get_unchecked(0);
            *alpha_chunk = *perceptual_chunk.get_unchecked(1);
            *beta_chunk = *perceptual_chunk.get_unchecked(2);
            if source_configuration.has_alpha() {
                *display_alpha_chunk = *perceptual_chunk.get_unchecked(3);
            }
        }
    }

    let mean_source_lightness = source_lightness.average();

    let std_dev_source_lightness = source_lightness.std_dev(mean_source_lightness);

    let mean_source_alpha = source_alpha.average();

    let std_dev_source_alpha = source_alpha.std_dev(mean_source_alpha);

    let mean_source_beta = source_beta.average();

    let std_dev_source_beta = source_beta.std_dev(mean_source_beta);

    let mean_target_lightness = target_lightness.average();

    let std_dev_target_lightness = target_lightness.std_dev(mean_target_lightness);

    let mean_target_alpha = target_alpha.average();

    let std_dev_target_alpha = target_alpha.std_dev(mean_target_alpha);

    let mean_target_beta = target_beta.average();

    let std_dev_target_beta = target_beta.std_dev(mean_target_beta);

    let op_scale_lightness = (std_dev_source_lightness / std_dev_target_lightness) as f32;
    let op_scale_alpha = (std_dev_source_alpha / std_dev_target_alpha) as f32;
    let op_scale_beta = (std_dev_source_beta / std_dev_target_beta) as f32;

    let is_nan_lightness =
        op_scale_lightness.is_nan() || op_scale_lightness.is_infinite() || op_scale_lightness == 0.;
    let is_nan_alpha =
        op_scale_alpha.is_nan() || op_scale_alpha.is_infinite() || op_scale_alpha == 0.;
    let is_nan_beta = op_scale_beta.is_nan() || op_scale_beta.is_infinite() || op_scale_beta == 0.;

    let mut perceptual_dest: Vec<f32> = vec![
        0.;
        target_width as usize
            * target_height as usize
            * source_configuration.get_channels_count()
    ];

    for ((((perceptual_chunk, target_lightness), target_alpha), target_beta), display_alpha) in
        perceptual_dest
            .chunks_exact_mut(source_configuration.get_channels_count())
            .zip(target_lightness)
            .zip(target_alpha)
            .zip(target_beta)
            .zip(target_display_alpha)
    {
        unsafe {
            if !is_nan_lightness {
                let new_lightness = op_scale_lightness
                    * (target_lightness - mean_target_lightness as f32)
                    + mean_source_lightness as f32;
                let c0 = lerp(target_lightness, new_lightness, intensity);
                *perceptual_chunk.get_unchecked_mut(0) = c0;
            } else {
                *perceptual_chunk.get_unchecked_mut(0) = target_lightness;
            }

            if !is_nan_alpha {
                let new_alpha = op_scale_alpha * (target_alpha - mean_target_alpha as f32)
                    + mean_source_alpha as f32;
                let c1 = lerp(target_alpha, new_alpha, intensity);
                *perceptual_chunk.get_unchecked_mut(1) = c1;
            } else {
                *perceptual_chunk.get_unchecked_mut(1) = target_alpha;
            }

            if !is_nan_beta {
                let new_beta = op_scale_beta * (target_beta - mean_target_beta as f32)
                    + mean_source_beta as f32;
                let c2 = lerp(target_beta, new_beta, intensity);
                *perceptual_chunk.get_unchecked_mut(2) = c2;
            } else {
                *perceptual_chunk.get_unchecked_mut(2) = target_beta;
            }

            if source_configuration.has_alpha() {
                *perceptual_chunk.get_unchecked_mut(3) = display_alpha;
            }
        }
    }

    display_transformer(&perceptual_dest, target, target_width, target_height);

    Ok(())
}

/// Copies palette from one RGB image to another RGB image
pub fn copy_palette_rgb(
    src: &[u8],
    src_width: u32,
    src_height: u32,
    target: &mut [u8],
    target_width: u32,
    target_height: u32,
    intensity: f32,
    transfer_colorspace: TransferColorspace,
) -> Result<(), String> {
    copy_palette_impl::<{ ImageConfiguration::Rgb as u8 }>(
        src,
        src_width,
        src_height,
        target,
        target_width,
        target_height,
        intensity,
        transfer_colorspace.get_perceptual_transformer(ImageConfiguration::Rgb),
        transfer_colorspace.get_display_transformer(ImageConfiguration::Rgb),
    )
}

/// Copies palette from one RGBA image to another RGBA image
pub fn copy_palette_rgba(
    src: &[u8],
    src_width: u32,
    src_height: u32,
    target: &mut [u8],
    target_width: u32,
    target_height: u32,
    intensity: f32,
    transfer_colorspace: TransferColorspace,
) -> Result<(), String> {
    copy_palette_impl::<{ ImageConfiguration::Rgba as u8 }>(
        src,
        src_width,
        src_height,
        target,
        target_width,
        target_height,
        intensity,
        transfer_colorspace.get_perceptual_transformer(ImageConfiguration::Rgba),
        transfer_colorspace.get_display_transformer(ImageConfiguration::Rgba),
    )
}
