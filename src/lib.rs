// src/lib.rs - WebAssembly library entry point

pub mod synth_core;
pub mod synth_data;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::AudioContext;
#[cfg(target_arch = "wasm32")]
use crate::synth_core::FMParams;
#[cfg(target_arch = "wasm32")]
use crate::synth_data::{get_melodies, get_presets, note_freq};

// WebAssembly exports
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebFMSynth {
    context: AudioContext,
    presets: Vec<(&'static str, synth_core::FMParams)>,
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
            presets: synth_data::get_presets(),
            melodies: synth_data::get_melodies(),
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
            let freq = synth_data::note_freq(note);
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

    fn play_note(&self, freq: f32, preset: &synth_core::FMParams, duration: f32) -> Result<(), JsValue> {
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
