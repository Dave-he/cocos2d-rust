use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn generate_beep(path: &str, frequency: f32, duration: f32) -> bool {
    let sample_rate: u32 = 44100;
    let amplitude: i16 = 8000;
    
    let num_samples: usize = (sample_rate as f32 * duration) as usize;
    let mut samples: Vec<i16> = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * amplitude as f32;
        samples.push(sample as i16);
    }
    
    write_wav(path, sample_rate, &samples)
}

pub fn generate_click(path: &str) -> bool {
    let sample_rate: u32 = 44100;
    let num_samples: usize = (sample_rate / 10) as usize;
    let mut samples: Vec<i16> = Vec::with_capacity(num_samples);
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let envelope = if t < 0.001 {
            t / 0.001
        } else if t > 0.009 {
            (0.01 - t) / 0.001
        } else {
            1.0
        };
        let sample = (rand::random::<f32>() * 2.0 - 1.0) * 16000.0 * envelope;
        samples.push(sample as i16);
    }
    
    write_wav(path, sample_rate, &samples)
}

fn write_wav(path: &str, sample_rate: u32, samples: &[i16]) -> bool {
    let file = match File::create(Path::new(path)) {
        Ok(f) => f,
        Err(_) => return false,
    };
    
    let mut writer = BufWriter::new(file);
    
    let data_size = samples.len() * 2;
    let file_size = 36 + data_size;
    
    let header = [
        b'R', b'I', b'F', b'F',
        (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8,
        ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8,
        b'W', b'A', b'V', b'E',
        b'f', b'm', b't', b' ',
        16, 0, 0, 0,
        1, 0,
        1, 0,
        (sample_rate & 0xFF) as u8, ((sample_rate >> 8) & 0xFF) as u8,
        ((sample_rate >> 16) & 0xFF) as u8, ((sample_rate >> 24) & 0xFF) as u8,
        ((sample_rate * 2) & 0xFF) as u8, (((sample_rate * 2) >> 8) & 0xFF) as u8,
        (((sample_rate * 2) >> 16) & 0xFF) as u8, (((sample_rate * 2) >> 24) & 0xFF) as u8,
        2, 0,
        16, 0,
        b'd', b'a', b't', b'a',
        (data_size & 0xFF) as u8, ((data_size >> 8) & 0xFF) as u8,
        ((data_size >> 16) & 0xFF) as u8, ((data_size >> 24) & 0xFF) as u8,
    ];
    
    if writer.write_all(&header).is_err() {
        return false;
    }
    
    for &sample in samples {
        let bytes = sample.to_le_bytes();
        if writer.write_all(&bytes).is_err() {
            return false;
        }
    }
    
    true
}
