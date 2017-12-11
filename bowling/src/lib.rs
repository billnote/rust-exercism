use std::collections::VecDeque;

pub struct BowlingGame {
    frame_size: usize,
    frames: VecDeque<BowlingFrame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frame_size: 10,
            frames: VecDeque::with_capacity(10),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), ()> {
        if pins > 10 {
            return Err(());
        }

        let has_frames = self.has_frames();
        let mut frame = self.frames.pop_back().unwrap_or(BowlingFrame::new());

        if frame.throws == 0 {
            if has_frames {
                self.frames.push_back(frame);
                frame = BowlingFrame::new();
            } else {
                if frame.next_throws <= 0 || frame.score % 10 + pins > 10 {
                    return Err(());
                }
                frame.score += pins;
                frame.next_throws -= 1;

                self.complete_pre_frames(pins);
                self.frames.push_back(frame);

                return Ok(());
            }
        }

        frame.score += pins;
        if frame.score > 10 {
            return Err(());
        } else if frame.score == 10 {
            frame.next_throws = frame.throws;
            frame.throws = 0;
        } else {
            frame.throws -= 1;
        }

        self.complete_pre_frames(pins);
        self.frames.push_back(frame);

        Ok(())
    }

    pub fn score(&self) -> Result<u16, ()> {
        if self.is_complete() {
            Ok(self.frames.iter().fold(
                0u16,
                |acc, frame| acc + frame.score,
            ))
        } else {
            Err(())
        }
    }

    fn complete_pre_frames(&mut self, pins: u16) {
        let mut pop_frames: VecDeque<BowlingFrame> = VecDeque::new();
        self.calc_pre_frames(pins, &mut pop_frames);
        self.frames.append(&mut pop_frames);
    }

    fn calc_pre_frames(&mut self, pins: u16, pop_frames: &mut VecDeque<BowlingFrame>) {
        while let Some(pre_frame) = self.calc_pre_frame_iter(pins) {
            if pre_frame.is_complete() {
                pop_frames.push_front(pre_frame);
                break;
            }
            pop_frames.push_front(pre_frame);
        }
    }

    fn calc_pre_frame_iter(&mut self, pins: u16) -> Option<BowlingFrame> {
        match self.frames.pop_back() {
            None => None,
            Some(mut pre_frame) => {
                if !pre_frame.is_complete() {
                    pre_frame.score += pins;
                    pre_frame.next_throws -= 1;
                }
                Some(pre_frame)
            }
        }
    }

    fn has_frames(&self) -> bool {
        self.frames.len() < self.frame_size
    }

    fn is_complete(&self) -> bool {
        self.frames.len() == self.frame_size && self.frames.back().unwrap().is_complete()
    }
}

pub struct BowlingFrame {
    throws: u8,
    next_throws: u8,
    score: u16,
}

impl BowlingFrame {
    const FRAME_THROWS_COUNT: u8 = 2;

    pub fn new() -> Self {
        BowlingFrame {
            throws: BowlingFrame::FRAME_THROWS_COUNT,
            next_throws: 0,
            score: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.throws == 0 && self.next_throws == 0
    }
}
