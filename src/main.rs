use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Import from our library crate
use fm_synth::synth_core::{FMSynth, FMParams};
use fm_synth::synth_data::{get_presets, get_melodies, note_freq};


/// CLI interface
struct CLI {
    presets: Vec<(&'static str, FMParams)>,
    melodies: Vec<(&'static str, Vec<(&'static str, u64)>)>,
}

impl CLI {
    fn new() -> Self {
        Self {
            presets: get_presets(),
            melodies: get_melodies(),
        }
    }

    fn print_menu(&self) {
        println!("\n=== FM Synthesizer CLI ===");
        println!("Commands:");
        println!("  list presets  - Show all available presets");
        println!("  list melodies - Show all available melodies");
        println!("  play <preset> <melody> - Play a melody with a preset");
        println!("  demo - Play all presets with a scale");
        println!("  help - Show this menu");
        println!("  quit - Exit the program");
        println!();
    }

    fn list_presets(&self) {
        println!("\nAvailable Presets:");
        for (i, (name, _)) in self.presets.iter().enumerate() {
            println!("  {}. {}", i + 1, name);
        }
    }

    fn list_melodies(&self) {
        println!("\nAvailable Melodies:");
        for (i, (name, _)) in self.melodies.iter().enumerate() {
            println!("  {}. {}", i + 1, name);
        }
    }

    fn find_preset(&self, name: &str) -> Option<FMParams> {
        // Try by number first
        if let Ok(num) = name.parse::<usize>() {
            if num > 0 && num <= self.presets.len() {
                return Some(self.presets[num - 1].1.clone());
            }
        }
        
        // Try by name (case insensitive)
        self.presets.iter()
            .find(|(n, _)| n.to_lowercase() == name.to_lowercase())
            .map(|(_, p)| p.clone())
    }

    fn find_melody(&self, name: &str) -> Option<Vec<(&'static str, u64)>> {
        // Try by number first
        if let Ok(num) = name.parse::<usize>() {
            if num > 0 && num <= self.melodies.len() {
                return Some(self.melodies[num - 1].1.clone());
            }
        }
        
        // Try by name (case insensitive)
        self.melodies.iter()
            .find(|(n, _)| n.to_lowercase().contains(&name.to_lowercase()))
            .map(|(_, m)| m.clone())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn play_melody(preset: FMParams, melody: Vec<(&'static str, u64)>) -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("No output device available");
    
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    
    let synth = Arc::new(Mutex::new(FMSynth::new(sample_rate, preset.clone())));
    let synth_clone = Arc::clone(&synth);
    
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut synth = synth_clone.lock().unwrap();
                for sample in data.iter_mut() {
                    *sample = synth.next_sample();
                }
            },
            |err| eprintln!("Error in audio stream: {}", err),
            None,
        )?,
        _ => panic!("Unsupported sample format"),
    };
    
    stream.play()?;
    
    for (note, duration) in melody {
        let freq = note_freq(note);
        if freq > 0.0 {
            let mut params = preset.clone();
            let freq_ratio = freq / 440.0;
            params.carrier_freq *= freq_ratio;
            params.modulator_freq *= freq_ratio;
            
            {
                let mut synth = synth.lock().unwrap();
                synth.set_params(params);
                synth.note_on();
            }
            
            std::thread::sleep(Duration::from_millis(duration * 80 / 100));
            
            {
                let mut synth = synth.lock().unwrap();
                synth.note_off();
            }
            
            std::thread::sleep(Duration::from_millis(duration * 20 / 100));
        } else {
            std::thread::sleep(Duration::from_millis(duration));
        }
    }
    
    std::thread::sleep(Duration::from_millis(500));
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    // Initialize audio
    let host = cpal::default_host();
    let device = host.default_output_device()
        .expect("No output device available");
    
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    
    // Create synth with default parameters
    let params = FMParams::default();
    let synth = Arc::new(Mutex::new(FMSynth::new(sample_rate, params)));
    
    // Clone for audio callback
    let synth_clone = Arc::clone(&synth);
    
    // Build output stream
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut synth = synth_clone.lock().unwrap();
                for sample in data.iter_mut() {
                    *sample = synth.next_sample();
                }
            },
            |err| eprintln!("Error in audio stream: {}", err),
            None,
        )?,
        _ => panic!("Unsupported sample format"),
    };
    
    stream.play()?;
    
    println!("FM Synthesizer Demo");
    println!("==================");
    
    // Choose demo mode: 1 for presets, 2 for melody
    let demo_mode = 1; // Change this to switch between demos
    
    match demo_mode {
        1 => {
            // Demo 1: Play through all presets
            println!("Playing preset sounds...\n");
            
            let presets = get_presets();
            let note_freqs = vec![220.0, 440.0, 330.0, 440.0]; // A3, A4, E4, A4
            
            for (name, mut preset_params) in presets {
                println!("Preset: {}", name);
                
                for &freq in &note_freqs {
                    // Scale frequencies proportionally
                    let freq_ratio = freq / 440.0;
                    preset_params.carrier_freq *= freq_ratio;
                    preset_params.modulator_freq *= freq_ratio;
                    
                    println!("  Note at {:.1}Hz", preset_params.carrier_freq);
                    
                    {
                        let mut synth = synth.lock().unwrap();
                        synth.set_params(preset_params.clone());
                        synth.note_on();
                    }
                    
                    std::thread::sleep(Duration::from_millis(600));
                    
                    {
                        let mut synth = synth.lock().unwrap();
                        synth.note_off();
                    }
                    
                    std::thread::sleep(Duration::from_millis(200));
                }
                
                println!();
                std::thread::sleep(Duration::from_millis(500));
            }
        }
        2 => {
            // Demo 2: Play a melody with custom parameters
            println!("Playing a sequence of FM tones...\n");
            
            let notes = vec![
                (440.0, 880.0, 2.0),   // A4 with 2:1 ratio
                (523.25, 1046.5, 3.0), // C5 with 2:1 ratio
                (659.25, 659.25, 5.0), // E5 with 1:1 ratio (bell-like)
                (440.0, 220.0, 8.0),   // A4 with 1:2 ratio (sub-harmonic)
            ];
            
            for (carrier, modulator, mod_index) in notes {
                println!("Playing: Carrier={:.1}Hz, Modulator={:.1}Hz, Index={:.1}", 
                         carrier, modulator, mod_index);
                
                {
                    let mut synth = synth.lock().unwrap();
                    synth.set_params(FMParams {
                        carrier_freq: carrier,
                        modulator_freq: modulator,
                        modulation_index: mod_index,
                        amplitude: 0.3,
                    });
                    synth.note_on();
                }
                
                std::thread::sleep(Duration::from_millis(800));
                
                {
                    let mut synth = synth.lock().unwrap();
                    synth.note_off();
                }
                
                std::thread::sleep(Duration::from_millis(700));
            }
        }
        _ => {
            println!("Invalid demo mode");
        }
    }
    
    println!("\nDone!");
    let cli = CLI::new();
    cli.print_menu();
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "list" => {
                if parts.len() > 1 {
                    match parts[1] {
                        "presets" => cli.list_presets(),
                        "melodies" => cli.list_melodies(),
                        _ => println!("Unknown list command. Use 'list presets' or 'list melodies'"),
                    }
                } else {
                    println!("Usage: list <presets|melodies>");
                }
            }
            "play" => {
                if parts.len() >= 3 {
                    let preset_name = parts[1];
                    let melody_name = parts[2..].join(" ");
                    
                    match (cli.find_preset(preset_name), cli.find_melody(&melody_name)) {
                        (Some(preset), Some(melody)) => {
                            println!("Playing '{}' melody with '{}' preset...", melody_name, preset_name);
                            play_melody(preset, melody)?;
                            println!("Done!");
                        }
                        (None, _) => println!("Preset '{}' not found. Use 'list presets' to see available options.", preset_name),
                        (_, None) => println!("Melody '{}' not found. Use 'list melodies' to see available options.", melody_name),
                    }
                } else {
                    println!("Usage: play <preset> <melody>");
                    println!("Example: play bell twinkle");
                    println!("Example: play 1 3");
                }
            }
            "demo" => {
                println!("Playing demo with all presets...");
                let scale = vec![
                    ("C4", 300), ("D4", 300), ("E4", 300), ("F4", 300),
                    ("G4", 300), ("A4", 300), ("B4", 300), ("C5", 600),
                ];
                for (name, preset) in &cli.presets {
                    println!("  Playing: {}", name);
                    play_melody(preset.clone(), scale.clone())?;
                }
                println!("Demo complete!");
            }
            "help" => cli.print_menu(),
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Unknown command. Type 'help' for available commands."),
        }
    }
    
    Ok(())
}

