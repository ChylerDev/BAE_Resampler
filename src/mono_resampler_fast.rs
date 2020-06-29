//! # MonoResamplerFast
//!
//! Resamples the given monophonic audio signal from its source sampling rate to
//! a given sampling rate.

use super::*;
use bae_sf::{MonoTrackT, SampleFormat, Mono};

/// Type used for fractional indexing.
type Index = FastMath;

/// Struct tracking all of the data required for resampling, with some extra
/// features like playback speed and looping.
///
/// This resampler is implemented using linear interpolation to easily enable
/// fast resampling.
pub struct MonoResamplerFast {
    data: MonoTrackT,
    x1ind: Index,
    x2ind: Index,
    inc: Sample,
    loop_start: usize,
    loop_end: usize,
}

impl Resampler<Mono> for MonoResamplerFast {
    type Data = MonoTrackT;

    fn new(
        data: MonoTrackT,
        output_sample_rate: Math,
        input_sample_rate: Math,
        mut loop_start: usize,
        mut loop_end: usize,
    ) -> Self {
        if loop_end < loop_start {
            std::mem::swap(&mut loop_start, &mut loop_end);
        }

        let inc = Sample((input_sample_rate.0 * (1.0 / output_sample_rate.0)) as FastMath);

        MonoResamplerFast {
            data,
            x1ind: Default::default(),
            x2ind: inc.0,
            inc,
            loop_start,
            loop_end,
        }
    }

    fn process(&mut self) -> Mono {
        // If the x1 index is larger than the saved data set and there's no
        // looping, then return a 0.0 sample.
        if self.x1ind as usize >= self.data.len() && self.loop_end == 0 {
            return Default::default();
        }

        // Save sample values for interpolation. In the case of `x2`, we need to
        // double check that its index is not beyond the bounds of the data
        // array.
        let x1 = self.data[self.x1ind.trunc() as usize].into_sample();
        let x2 = if self.x2ind as usize >= self.data.len() {
            Default::default()
        } else {
            self.data[self.x2ind.trunc() as usize].into_sample()
        };

        // Interpolate the samples.
        let y = x1.0 + self.x1ind.fract() as FastMath * (x2.0 - x1.0);

        // Increase the indices.
        self.x1ind += self.inc.0;
        self.x2ind += self.inc.0;

        // If looping is enabled, check if the indices need to be looped.
        if self.loop_end != 0 {
            if self.x1ind as usize >= self.loop_end {
                self.x1ind -= (self.loop_end - self.loop_start) as Index;
            }
            if self.x2ind as usize >= self.loop_end {
                self.x2ind -= (self.loop_end - self.loop_start) as Index;
            }
        }

        // Return the sample value.
        Mono::from(y.into())
    }
}

impl BlockResampler<Mono> for MonoResamplerFast {
    fn process_block(&mut self, out: &mut[Mono]) {
        for s in out {
            // If the x1 index is larger than the saved data set and there's no
            // looping, then return a 0.0 sample.
            if self.x1ind as usize >= self.data.len() && self.loop_end == 0 {
                return Default::default();
            }

            // Save sample values for interpolation. In the case of `x2`, we need to
            // double check that its index is not beyond the bounds of the data
            // array.
            let x1 = self.data[self.x1ind.trunc() as usize].into_sample();
            let x2 = if self.x2ind as usize >= self.data.len() {
                Default::default()
            } else {
                self.data[self.x2ind.trunc() as usize].into_sample()
            };

            // Interpolate the samples.
            let y = x1.0 + self.x1ind.fract() as FastMath * (x2.0 - x1.0);

            // Increase the indices.
            self.x1ind += self.inc.0;
            self.x2ind += self.inc.0;

            // If looping is enabled, check if the indices need to be looped.
            if self.loop_end != 0 {
                if self.x1ind as usize >= self.loop_end {
                    self.x1ind -= (self.loop_end - self.loop_start) as Index;
                }
                if self.x2ind as usize >= self.loop_end {
                    self.x2ind -= (self.loop_end - self.loop_start) as Index;
                }
            }

            // Save the sample value.
            *s = Mono::from(y.into());
        }
    }
}

impl Clone for MonoResamplerFast {
    fn clone(&self) -> Self {
        MonoResamplerFast {
            data: self.data.clone(),
            x1ind: Default::default(),
            x2ind: self.inc.0,
            inc: self.inc,
            loop_start: self.loop_start,
            loop_end: self.loop_end,
        }
    }
}
