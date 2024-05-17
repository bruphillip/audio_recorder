use std::{f32::consts::PI, time::Duration};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    build_cpal();
}


fn build_cpal() {
    let host = cpal::default_host();

    let input_device = host
        .default_input_device()
        .expect("Failed to get default input device");

    
    println!("Input device: {:?}", input_device.name());

    let config = input_device
        .default_input_config()
        .expect("Failed to get default input config");


    println!("F32");

    let spec = hound::WavSpec {
        channels: config.channels() as u16,
        sample_rate: config.sample_rate().0,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create("output6.wav", spec).unwrap();


    let err_fn = |err: cpal::StreamError| eprintln!("An error occurred on the input audio stream: {}", err);

    println!("stream started");

    let stream = input_device.build_input_stream::<f32, _, _>(
        &config.config(),
        move |data: &[f32], _| {
            for sample in data.iter() {
                let sample = (sample * 2.0 * PI).sin();
                let amplitude = 2.0 as f32 ;
                writer.write_sample::<f32>(sample * amplitude).unwrap();
            }
        },
        err_fn,
        Some(Duration::from_secs(10)),
    ).unwrap();

    stream.play().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    stream.pause().unwrap();
        

}
