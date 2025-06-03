use std::f32::consts::PI;

/// FM Synthesizer parameters
#[derive(Clone, Debug)]
pub struct FMParams {
    pub carrier_freq: f32,      // Carrier frequency in Hz
    pub modulator_freq: f32,    // Modulator frequency in Hz
    pub modulation_index: f32,  // Modulation depth
    pub amplitude: f32,         // Output amplitude (0.0 - 1.0)
}

impl Default for FMParams {
    fn default() -> Self {
        Self {
            carrier_freq: 440.0,
            modulator_freq: 220.0,
            modulation_index: 2.0,
            amplitude: 0.3,
        }
    }
}

/// FM Synthesizer oscillator
pub struct FMOscillator {
    sample_rate: f32,
    carrier_phase: f32,
    modulator_phase: f32,
    params: FMParams,
}

impl FMOscillator {
    pub fn new(sample_rate: f32, params: FMParams) -> Self {
        Self {
            sample_rate,
            carrier_phase: 0.0,
            modulator_phase: 0.0,
            params,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let modulator = (2.0 * PI * self.modulator_phase).sin();
        let modulated_freq = self.params.carrier_freq * 
            (1.0 + self.params.modulation_index * modulator);
        let carrier = (2.0 * PI * self.carrier_phase).sin();
        
        self.carrier_phase += modulated_freq / self.sample_rate;
        self.modulator_phase += self.params.modulator_freq / self.sample_rate;
        
        if self.carrier_phase >= 1.0 {
            self.carrier_phase -= 1.0;
        }
        if self.modulator_phase >= 1.0 {
            self.modulator_phase -= 1.0;
        }
        
        carrier * self.params.amplitude
    }

    pub fn set_params(&mut self, params: FMParams) {
        self.params = params;
    }
}

/// ADSR Envelope state
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

/// ADSR Envelope generator
pub struct Envelope {
    pub attack: f32,   // Attack time in seconds
    pub decay: f32,    // Decay time in seconds
    pub sustain: f32,  // Sustain level (0.0 - 1.0)
    pub release: f32,  // Release time in seconds
    sample_rate: f32,
    state: EnvelopeState,
    level: f32,
    time: f32,
}

impl Envelope {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain: 0.7,
            release: 0.5,
            sample_rate,
            state: EnvelopeState::Idle,
            level: 0.0,
            time: 0.0,
        }
    }

    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
        self.time = 0.0;
    }

    pub fn release(&mut self) {
        if self.state != EnvelopeState::Idle {
            self.state = EnvelopeState::Release;
            self.time = 0.0;
        }
    }

    pub fn process(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;
        
        match self.state {
            EnvelopeState::Idle => {
                self.level = 0.0;
            }
            EnvelopeState::Attack => {
                self.level = self.time / self.attack;
                if self.time >= self.attack {
                    self.state = EnvelopeState::Decay;
                    self.time = 0.0;
                }
            }
            EnvelopeState::Decay => {
                self.level = 1.0 - ((1.0 - self.sustain) * (self.time / self.decay));
                if self.time >= self.decay {
                    self.state = EnvelopeState::Sustain;
                    self.time = 0.0;
                }
            }
            EnvelopeState::Sustain => {
                self.level = self.sustain;
            }
            EnvelopeState::Release => {
                self.level = self.sustain * (1.0 - (self.time / self.release));
                if self.time >= self.release {
                    self.state = EnvelopeState::Idle;
                    self.level = 0.0;
                }
            }
        }
        
        self.time += dt;
        self.level
    }
}

/// FM Synthesizer with envelope
pub struct FMSynth {
    oscillator: FMOscillator,
    envelope: Envelope,
}

impl FMSynth {
    pub fn new(sample_rate: f32, params: FMParams) -> Self {
        Self {
            oscillator: FMOscillator::new(sample_rate, params.clone()),
            envelope: Envelope::new(sample_rate),
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let osc_out = self.oscillator.next_sample();
        let env_out = self.envelope.process();
        osc_out * env_out
    }

    pub fn note_on(&mut self) {
        self.envelope.trigger();
    }

    pub fn note_off(&mut self) {
        self.envelope.release();
    }

    pub fn set_params(&mut self, params: FMParams) {
        self.oscillator.set_params(params);
    }
}
