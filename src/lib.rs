//! # BAE_Resampler
//!
//! Crate containing the ability to resample from one sampling rate to another.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bae_resampler/0.14.0")]

use bae_sf::{ SampleFormat };
use bae_types::*;

pub mod mono_resampler_fast;
pub use mono_resampler_fast::*;

/// Trait defining the interface for any types that take a given audio data set
/// and resample it from its original sample rate to a given new sample rate.
pub trait Resampler<S: SampleFormat> {
    /// Type representing the container the original data is stored in. This
    /// data will be moved from it's original location to the constructed
    /// Resampler-implementing type rather than borrowed.
    ///
    /// DEVELOPER NOTE: This may be considered for refactoring to borrows and
    /// lifetimes later.
    type Data;

    /// Creates a new Resampler-implementing object.
    ///
    /// # Parameters
    ///
    /// * `data` - The container of the original audio data to resample.
    /// * `output_sample_rate` - The sampling rate to resample to.
    /// * `input_sample_rate` - The sample rate `data` was initially sampled at.
    /// * `loop_start` - The start point of looping.
    /// * `loop_end` - The end point of looping. If this value is 0 then it is
    ///   assumed there is no looping.
    ///
    /// If `loop_end` is less than `loop_start`, they shall be swapped. If
    /// `loop_end` is 0 and `loop_end` is >0 then they are swapped and those
    /// loop points are used, rather than disabling looping.
    ///
    /// The range of the looping is [`loop_start`, `loop_end`). This means that
    /// if you want to loop at the end of the data container then `data.len()`
    /// should be passed for the `loop_end` parameter, and if you'd like to loop
    /// a single sample, `sam` and `sam+1` should be used for the loop points.
    fn new(
        data: Self::Data,
        output_sample_rate: Math,
        input_sample_rate: Math,
        loop_start: usize,
        loop_end: usize,
    ) -> Self;

    /// Calculates and returns the next sample.
    fn process(&mut self) -> S;
}

/// Extends the [`Resampler`][0] trait with the ability to process a batch of samples at a time.
///
/// [0]: trait.Resampler.html
pub trait BlockResampler<S: SampleFormat>:
    Resampler<S>
{
    /// Calculates a batch of samples in one call. As audio code requires
    /// efficiency, the processed sample is saved into the passed mutable slice.
    /// For optimal efficiency (in order of decreasing efficiency), it is best
    /// to pass a slice with a size that is a power of 2 or a multiple of 16 or
    /// 4.
    fn process_block(&mut self, out: &mut[S]);
}
