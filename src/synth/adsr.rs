enum ADSRState {
    Attack,
    Decay,
    Sustain,
    Release,
    Dead,
}

pub struct ADSR {
    state: ADSRState,
    amplitude: f32,
    slope: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl ADSR {
    pub fn new(
        attack: f32, 
        decay: f32, 
        sustain: f32, 
        release: f32,
    ) -> Self {
        Self {
            state: ADSRState::Attack,
            amplitude: 0.0,
            slope: 1.0 / attack,
            decay,
            sustain,
            release,
        }
    }

    pub fn next_amplitude(&mut self, time_increment: f32) -> f32 {
        self.amplitude += time_increment * self.slope;

        match self.state {
            ADSRState::Attack => {
                if self.amplitude >= 1.0 {
                    self.enter_decay();
                }
            },
            ADSRState::Decay => {
                if self.amplitude <= self.sustain {
                    self.enter_sustain();
                }
            },
            ADSRState::Sustain => {},
            ADSRState::Release => {
                if self.amplitude <= 0.0 {
                    self.enter_dead();
                }
            },
            ADSRState::Dead => {},
        }

        self.amplitude
    }

    pub fn release(&mut self) {
        if matches!(self.state, 
            ADSRState::Attack |
            ADSRState::Decay |
            ADSRState::Sustain
        ) {
            self.enter_release();  
        }
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.state, ADSRState::Dead)
    }

    fn enter_decay(&mut self) {
        self.state = ADSRState::Decay;
        self.amplitude = 1.0;
        self.slope = - (1.0 - self.sustain) / self.decay;
    }

    fn enter_sustain(&mut self) {
        self.state = ADSRState::Sustain;
        self.amplitude = self.sustain;
        self.slope = 0.0;
    }

    fn enter_release(&mut self) {
        self.state = ADSRState::Release;
        self.slope = - self.amplitude / self.release;
    }

    fn enter_dead(&mut self) {
        self.state = ADSRState::Dead;
        self.amplitude = 0.0;
        self.slope = 0.0;
    }
}
