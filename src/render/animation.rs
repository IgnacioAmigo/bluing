use std::{time::{Duration, Instant}};

use super::{texture::Texture};

pub struct Animation<'a>
{
    frames: &'a [Texture],
    current_frame: usize,
    frame_duration: Duration,
    last_frame_time: Instant,
}

impl<'a> Animation<'a>
{
    pub fn new(frames: &'a [Texture], frame_duration: Duration) -> Self {
        Animation {
            frames,
            current_frame: 0,
            frame_duration,
            last_frame_time: Instant::now(),
        }
    }

    pub fn get_frame(&mut self) -> &Texture {
        let now = Instant::now();
        if now.duration_since(self.last_frame_time) >= self.frame_duration {                
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_frame_time = now;
        }

        &self.frames[self.current_frame]
    }
}