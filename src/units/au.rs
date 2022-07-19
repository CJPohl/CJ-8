use sdl2::{
    audio::AudioSpecDesired,
    audio::{AudioCallback, AudioDevice},
    Sdl,
};

// Audio Unit
pub struct AU {
    pub device: AudioDevice<SquareWave>,
}

impl AU {
    pub fn new(context: &Sdl) -> AU {
        let audio_subsystem = context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                println!("{:?}", spec);

                // init callback device
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        AU { device }
    }
}

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // gen square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
