use crate::synth_core::FMParams;

/// Note frequencies
pub fn note_freq(note: &str) -> f32 {
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
pub fn get_presets() -> Vec<(&'static str, FMParams)> {
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
pub fn get_melodies() -> Vec<(&'static str, Vec<(&'static str, u64)>)> {
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
