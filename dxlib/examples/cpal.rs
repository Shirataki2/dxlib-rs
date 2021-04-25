use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use parking_lot::Mutex;
use rustfft::{num_complex::Complex, FftPlanner};
use std::{collections::VecDeque, sync::Arc};

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let device = host.default_input_device().unwrap();

    println!("Input Device: {}", device.name()?);

    let config = device.default_input_config()?;

    println!("Input Configuration: {:?}", config);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1024);

    let queue = Arc::new(Mutex::new(VecDeque::new()));

    let queue2 = Arc::clone(&queue);

    use cpal::SampleFormat::*;
    let stream = match config.sample_format() {
        F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                let mut buf = vec![
                    Complex {
                        re: 0.0f32,
                        im: 0.0f32
                    };
                    1024
                ];
                data.iter().enumerate().for_each(|(i, d)| buf[i].re = *d);
                fft.process(&mut buf);
                queue2.lock().push_back(buf);
            },
            |err| eprintln!("{:?}", err),
        )?,
        I16 => device.build_input_stream(
            &config.into(),
            |data: &[i16], _| {
                println!("{:?}", data);
            },
            |err| eprintln!("{:?}", err),
        )?,
        U16 => device.build_input_stream(
            &config.into(),
            |data: &[u16], _| {
                println!("{:?}", data);
            },
            |err| eprintln!("{:?}", err),
        )?,
    };

    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    drop(stream);

    Ok(())
}
