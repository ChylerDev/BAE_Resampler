extern crate bae_debug;

#[test]
fn test_mono_resampler_fast() {
    use bae_resampler::*;
    
    use bae_debug::*;
    use bae_sf::{Mono, SampleFormat};
    use bae_types::*;

    const SAMPLE_RATE:usize = 48000;

    let sam = vec![
        Mono::from_sample(0.0.into()),
        Mono::from_sample(1.0.into()),
        Mono::from_sample(2.0.into()),
        Mono::from_sample(3.0.into()),
    ];

    let mut r = MonoResamplerFast::new(
        sam.clone(),
        (SAMPLE_RATE as AccurateMath).into(),
        (SAMPLE_RATE as AccurateMath / 2.0).into(),
        0,
        0,
    );
    for i in 0..7 {
        let s = r.process();

        assert!(float_equal(s.mono.0, i as FastMath / 2.0, FastMath::EPSILON, FastMath::abs));
    }

    let mut r = MonoResamplerFast::new(
        sam.clone(),
        (SAMPLE_RATE as AccurateMath).into(),
        (SAMPLE_RATE as AccurateMath * 2.0).into(),
        0,
        0,
    );
    for i in 0..2 {
        let s = r.process();

        assert!(float_equal(s.mono.0, (i * 2) as FastMath, FastMath::EPSILON, FastMath::abs));
    }
}
