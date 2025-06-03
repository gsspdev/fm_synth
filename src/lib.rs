// src/lib.rs - WebAssembly library entry point

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_sys::AudioContext;

/// FM Synthesizer parameters
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct FMParams {
    carrier_freq: f32,
    modulator_freq: f32,
    modulation_index: f32,
    amplitude: f32,
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

/// Note frequencies
#[allow(dead_code)]
fn note_freq(note: &str) -> f32 {
    match note {
        "C3" => 130.81, "C#3" => 138.59, "D3" => 146.83, "D#3" => 155.56, "E3" => 164.81,
        "F3" => 174.61, "F#3" => 185.00, "G3" => 196.00, "G#3" => 207.65, "A3" => 220.00,
        "A#3" => 233.08, "B3" => 246.94,
        "C4" => 261.63, "C#4" => 277.18, "D4" => 293.66, "D#4" => 311.13, "E4" => 329.63,
        "F4" => 349.23, "F#4" => 369.99, "G4" => 392.00, "G#4" => 415.30, "A4" => 440.00,
        "A#4" => 466.16, "B4" => 493.88,
        "C5" => 523.25, "C#5" => 554.37, "D5" => 587.33, "D#5" => 622.25, "E5" => 659.25,
        "F5" => 698.46, "F#5" => 739.99, "G5" => 783.99, "G#5" => 830.61, "A5" => 880.00,
        _ => 0.0, // Rest
    }
}

/// Preset definitions
#[allow(dead_code)]
fn get_presets() -> Vec<(&'static str, FMParams)> {
    vec![
        ("Bell", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 440.0,
            modulation_index: 7.0,
            amplitude: 0.3,
        }),
        ("Bass", FMParams {
            carrier_freq: 110.0,
            modulator_freq: 110.0,
            modulation_index: 1.5,
            amplitude: 0.5,
        }),
        ("Electric Piano", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 880.0,
            modulation_index: 3.0,
            amplitude: 0.4,
        }),
        ("Brass", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 440.0,
            modulation_index: 2.5,
            amplitude: 0.4,
        }),
        ("Organ", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 880.0,
            modulation_index: 1.0,
            amplitude: 0.4,
        }),
        ("Synth Lead", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 1320.0,
            modulation_index: 4.0,
            amplitude: 0.35,
        }),
        ("Marimba", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 440.0,
            modulation_index: 3.5,
            amplitude: 0.4,
        }),
        ("Strings", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 220.0,
            modulation_index: 0.8,
            amplitude: 0.3,
        }),
        ("Flute", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 440.0,
            modulation_index: 0.5,
            amplitude: 0.25,
        }),
        ("Metallic", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 567.0,
            modulation_index: 9.0,
            amplitude: 0.3,
        }),
        ("Glockenspiel", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 1760.0,
            modulation_index: 2.5,
            amplitude: 0.3,
        }),
        ("Wood Block", FMParams {
            carrier_freq: 440.0,
            modulator_freq: 300.0,
            modulation_index: 12.0,
            amplitude: 0.4,
        }),
    ]
}

/// Melody definitions
#[allow(dead_code)]
fn get_melodies() -> Vec<(&'static str, Vec<(&'static str, u64)>)> {
    vec![
        ("Twinkle Twinkle", vec![
            ("C4", 500), ("C4", 500), ("G4", 500), ("G4", 500),
            ("A4", 500), ("A4", 500), ("G4", 1000),
            ("F4", 500), ("F4", 500), ("E4", 500), ("E4", 500),
            ("D4", 500), ("D4", 500), ("C4", 1000),
        ]),
        ("Happy Birthday", vec![
            ("C4", 250), ("C4", 250), ("D4", 500), ("C4", 500),
            ("F4", 500), ("E4", 1000),
            ("C4", 250), ("C4", 250), ("D4", 500), ("C4", 500),
            ("G4", 500), ("F4", 1000),
        ]),
        ("Ode to Joy", vec![
            ("E4", 500), ("E4", 500), ("F4", 500), ("G4", 500),
            ("G4", 500), ("F4", 500), ("E4", 500), ("D4", 500),
            ("C4", 500), ("C4", 500), ("D4", 500), ("E4", 500),
            ("E4", 750), ("D4", 250), ("D4", 1000),
        ]),
        ("Mary Had a Little Lamb", vec![
            ("E4", 500), ("D4", 500), ("C4", 500), ("D4", 500),
            ("E4", 500), ("E4", 500), ("E4", 1000),
            ("D4", 500), ("D4", 500), ("D4", 1000),
            ("E4", 500), ("G4", 500), ("G4", 1000),
        ]),
        ("Chromatic Scale", vec![
            ("C4", 200), ("C#4", 200), ("D4", 200), ("D#4", 200),
            ("E4", 200), ("F4", 200), ("F#4", 200), ("G4", 200),
            ("G#4", 200), ("A4", 200), ("A#4", 200), ("B4", 200),
            ("C5", 400),
        ]),
        ("Major Arpeggio", vec![
            ("C4", 300), ("E4", 300), ("G4", 300), ("C5", 300),
            ("G4", 300), ("E4", 300), ("C4", 600),
        ]),
        ("Minor Pentatonic", vec![
            ("A3", 400), ("C4", 400), ("D4", 400), ("E4", 400),
            ("G4", 400), ("A4", 400), ("G4", 400), ("E4", 400),
            ("D4", 400), ("C4", 400), ("A3", 800),
        ]),
        ("Jazz Lick", vec![
            ("C4", 200), ("E4", 200), ("G4", 200), ("A#4", 200),
            ("A4", 400), ("F4", 200), ("D4", 400),
            ("G4", 200), ("E4", 200), ("C4", 600),
        ]),
        ("Bach Invention", vec![
            ("C4", 200), ("D4", 200), ("E4", 200), ("F4", 200),
            ("D4", 200), ("E4", 200), ("C4", 400),
            ("G4", 200), ("F4", 200), ("E4", 200), ("D4", 200),
            ("B3", 200), ("C4", 600),
        ]),
        ("Synth Demo", vec![
            ("C4", 150), ("E4", 150), ("G4", 150), ("C5", 150),
            ("E5", 150), ("G5", 150), ("E5", 150), ("C5", 150),
            ("G4", 150), ("E4", 150), ("C4", 300),
            ("REST", 300),
            ("F4", 150), ("A4", 150), ("C5", 150), ("F5", 150),
            ("C5", 150), ("A4", 150), ("F4", 300),
        ]),
    ]
}

// WebAssembly exports
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebFMSynth {
    context: AudioContext,
    presets: Vec<(&'static str, FMParams)>,
    melodies: Vec<(&'static str, Vec<(&'static str, u64)>)>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebFMSynth {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebFMSynth, JsValue> {
        // Set panic hook for better error messages
        console_error_panic_hook::set_once();
        
        let context = AudioContext::new()?;
        Ok(WebFMSynth {
            context,
            presets: get_presets(),
            melodies: get_melodies(),
        })
    }

    pub fn list_presets(&self) -> String {
        self.presets.iter()
            .enumerate()
            .map(|(i, (name, _))| format!("{}. {}", i + 1, name))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn list_melodies(&self) -> String {
        self.melodies.iter()
            .enumerate()
            .map(|(i, (name, _))| format!("{}. {}", i + 1, name))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub async fn play_melody(&self, preset_idx: usize, melody_idx: usize) -> Result<(), JsValue> {
        if preset_idx >= self.presets.len() || melody_idx >= self.melodies.len() {
            return Err(JsValue::from_str("Invalid preset or melody index"));
        }

        let preset = &self.presets[preset_idx].1;
        let melody = &self.melodies[melody_idx].1;

        for (note, duration) in melody {
            let freq = note_freq(note);
            if freq > 0.0 {
                self.play_note(freq, preset, *duration as f32 / 1000.0)?;
            }
            
            // Wait for note duration
            let promise = js_sys::Promise::new(&mut |resolve, _| {
                let window = web_sys::window().unwrap();
                window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    &resolve,
                    *duration as i32,
                ).unwrap();
            });
            wasm_bindgen_futures::JsFuture::from(promise).await?;
        }

        Ok(())
    }

    fn play_note(&self, freq: f32, preset: &FMParams, duration: f32) -> Result<(), JsValue> {
        let current_time = self.context.current_time();
        
        // Create carrier oscillator
        let carrier = self.context.create_oscillator()?;
        carrier.frequency().set_value(freq);
        
        // Create modulator oscillator
        let modulator = self.context.create_oscillator()?;
        let freq_ratio = freq / 440.0;
        modulator.frequency().set_value(preset.modulator_freq * freq_ratio);
        
        // Create modulation gain
        let mod_gain = self.context.create_gain()?;
        mod_gain.gain().set_value(preset.modulation_index * freq);
        
        // Create output gain with envelope
        let output_gain = self.context.create_gain()?;
        let gain_param = output_gain.gain();
        
        // ADSR envelope
        gain_param.set_value_at_time(0.0, current_time)?;
        gain_param.linear_ramp_to_value_at_time(preset.amplitude, current_time + 0.01)?;
        gain_param.exponential_ramp_to_value_at_time(preset.amplitude * 0.7, current_time + 0.1)?;
        gain_param.linear_ramp_to_value_at_time(0.001, current_time + duration as f64)?;
        
        // Connect FM synthesis chain
        modulator.connect_with_audio_node(&mod_gain)?;
        mod_gain.connect_with_audio_param(&carrier.frequency())?;
        carrier.connect_with_audio_node(&output_gain)?;
        output_gain.connect_with_audio_node(&self.context.destination())?;
        
        // Start oscillators
        modulator.start()?;
        carrier.start()?;
        
        // Stop oscillators after duration
        let stop_time = current_time + duration as f64 + 0.1;
        modulator.stop_with_when(stop_time)?;
        carrier.stop_with_when(stop_time)?;
        
        Ok(())
    }
}
