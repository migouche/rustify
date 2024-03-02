use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, SampleFormat, SizedSample, StreamConfig,
};
use fundsp::{hacker::sine_hz, prelude::AudioUnit64};

fn main() {
    let sine = Box::new(sine_hz(442.0));

    run_output(sine);

    std::thread::sleep(std::time::Duration::from_secs(30));

    println!("Hello, world!");
}

fn run_output(graph: Box<dyn AudioUnit64>) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find default output");
    let config = device.default_output_config().unwrap();
    match config.sample_format() {
        SampleFormat::F32 => run_synth::<f32>(graph, device, config.into()),
        SampleFormat::I16 => run_synth::<i16>(graph, device, config.into()),
        SampleFormat::U16 => run_synth::<u16>(graph, device, config.into()),

        _ => panic!("Unsupported format"),
    }
}

fn run_synth<T: SizedSample + FromSample<f64>>(
    mut audio_graph: Box<dyn AudioUnit64>,
    device: Device,
    config: StreamConfig,
) {
    std::thread::spawn(move || {
        let sample_rate = config.sample_rate.0 as f64;
        audio_graph.set_sample_rate(sample_rate);

        let mut next_value = move || audio_graph.get_stereo();

        let channels = config.channels as usize;
        let err_fn = |err| eprintln!("an error occurred on stream: {err}");
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value)
                },
                err_fn,
                None,
            )
            .unwrap();
        stream.play().unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}

fn write_data<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f64, f64),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right }
        }
    }
}
