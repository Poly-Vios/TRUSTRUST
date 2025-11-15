use std::fmt;

// ============================================================================
// CORE DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pitch {
    midi_number: u8, // C4 = 60
}

impl Pitch {
    fn new(midi_number: u8) -> Self {
        Self { midi_number }
    }
    
    fn semitones(&self) -> i16 {
        self.midi_number as i16
    }
    
    fn name(&self) -> String {
        let names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (self.midi_number / 12) as i16 - 1;
        let note = names[(self.midi_number % 12) as usize];
        format!("{}{}", note, octave)
    }
}

#[derive(Debug, Clone)]
struct Voicing {
    soprano: Pitch,
    alto: Pitch,
    tenor: Pitch,
    bass: Pitch,
}

impl fmt::Display for Voicing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "S:{} A:{} T:{} B:{}", 
               self.soprano.name(), self.alto.name(), 
               self.tenor.name(), self.bass.name())
    }
}

#[derive(Debug)]
struct FiguredBassSymbol {
    bass: Pitch,
    chord_tones: Vec<Pitch>, // Already calculated from figures
}

// Voice ranges in MIDI numbers
const SOPRANO_MIN: u8 = 60; // C4
const SOPRANO_MAX: u8 = 79; // G5
const ALTO_MIN: u8 = 55;    // G3
const ALTO_MAX: u8 = 72;    // C5
const TENOR_MIN: u8 = 48;   // C3
const TENOR_MAX: u8 = 67;   // G4
const BASS_MIN: u8 = 40;    // E2
const BASS_MAX: u8 = 60;    // C4

// ============================================================================
// VOICING GENERATION
// ============================================================================

fn generate_voicings(symbol: &FiguredBassSymbol) -> Vec<Voicing> {
    let mut voicings = Vec::new();
    let bass = symbol.bass;
    
    // Get chord tones in various octaves for upper voices
    let soprano_notes = get_notes_in_range(&symbol.chord_tones, SOPRANO_MIN, SOPRANO_MAX);
    let alto_notes = get_notes_in_range(&symbol.chord_tones, ALTO_MIN, ALTO_MAX);
    let tenor_notes = get_notes_in_range(&symbol.chord_tones, TENOR_MIN, TENOR_MAX);
    
    // Generate all combinations
    for &soprano in &soprano_notes {
        for &alto in &alto_notes {
            for &tenor in &tenor_notes {
                let voicing = Voicing { soprano, alto, tenor, bass };
                
                // Basic validity checks
                if is_valid_voicing(&voicing, &symbol.chord_tones) {
                    voicings.push(voicing);
                }
            }
        }
    }
    
    voicings
}

fn get_notes_in_range(chord_tones: &[Pitch], min: u8, max: u8) -> Vec<Pitch> {
    let mut notes = Vec::new();
    
    for &tone in chord_tones {
        let pitch_class = tone.midi_number % 12;
        
        // Generate this pitch class in all octaves within range
        let mut midi = pitch_class;
        while midi < min {
            midi += 12;
        }
        while midi <= max {
            notes.push(Pitch::new(midi));
            midi += 12;
        }
    }
    
    notes.sort();
    notes.dedup();
    notes
}

fn is_valid_voicing(voicing: &Voicing, chord_tones: &[Pitch]) -> bool {
    // Check voices don't cross
    if voicing.soprano.midi_number < voicing.alto.midi_number {
        return false;
    }
    if voicing.alto.midi_number < voicing.tenor.midi_number {
        return false;
    }
    if voicing.tenor.midi_number < voicing.bass.midi_number {
        return false;
    }
    
    // Check spacing between upper voices (not more than an octave)
    if voicing.soprano.midi_number - voicing.alto.midi_number > 12 {
        return false;
    }
    if voicing.alto.midi_number - voicing.tenor.midi_number > 12 {
        return false;
    }
    
    // Check all chord tones are represented
    let voicing_pcs: Vec<u8> = vec![
        voicing.soprano.midi_number % 12,
        voicing.alto.midi_number % 12,
        voicing.tenor.midi_number % 12,
        voicing.bass.midi_number % 12,
    ];
    
    for tone in chord_tones {
        let pc = tone.midi_number % 12;
        if !voicing_pcs.contains(&pc) {
            return false;
        }
    }
    
    true
}

// ============================================================================
// SCORING FUNCTIONS
// ============================================================================

fn score_voicing(voicing: &Voicing, prev: Option<&Voicing>, root: Pitch) -> f32 {
    let mut score = 0.0;
    
    // Static scores
    score += doubling_score(voicing, root);
    score += spacing_score(voicing);
    score += range_comfort_score(voicing);
    
    // Dynamic scores (if there's a previous chord)
    if let Some(prev_voicing) = prev {
        score += parallel_motion_penalty(prev_voicing, voicing);
        score += voice_motion_score(prev_voicing, voicing);
        score += contrary_motion_bonus(prev_voicing, voicing);
    }
    
    score
}

fn doubling_score(voicing: &Voicing, root: Pitch) -> f32 {
    let root_pc = root.midi_number % 12;
    let mut score = 0.0;
    
    let voices = [
        voicing.soprano.midi_number % 12,
        voicing.alto.midi_number % 12,
        voicing.tenor.midi_number % 12,
        voicing.bass.midi_number % 12,
    ];
    
    for &voice_pc in &voices {
        if voice_pc == root_pc {
            score += 10.0; // Prefer doubling the root
        }
    }
    
    score
}

fn spacing_score(voicing: &Voicing) -> f32 {
    let mut score = 0.0;
    
    // Penalize large gaps in upper voices
    let sop_alto_gap = voicing.soprano.midi_number - voicing.alto.midi_number;
    let alto_tenor_gap = voicing.alto.midi_number - voicing.tenor.midi_number;
    
    if sop_alto_gap > 7 {
        score -= (sop_alto_gap - 7) as f32 * 2.0;
    }
    if alto_tenor_gap > 7 {
        score -= (alto_tenor_gap - 7) as f32 * 2.0;
    }
    
    score
}

fn range_comfort_score(voicing: &Voicing) -> f32 {
    let mut score = 0.0;
    
    // Prefer notes in the middle of each range
    let soprano_mid = (SOPRANO_MIN + SOPRANO_MAX) / 2;
    let alto_mid = (ALTO_MIN + ALTO_MAX) / 2;
    let tenor_mid = (TENOR_MIN + TENOR_MAX) / 2;
    
    score -= ((voicing.soprano.midi_number as i16 - soprano_mid as i16).abs() as f32) * 0.1;
    score -= ((voicing.alto.midi_number as i16 - alto_mid as i16).abs() as f32) * 0.1;
    score -= ((voicing.tenor.midi_number as i16 - tenor_mid as i16).abs() as f32) * 0.1;
    
    score
}

fn parallel_motion_penalty(v1: &Voicing, v2: &Voicing) -> f32 {
    let voices1 = [v1.soprano, v1.alto, v1.tenor, v1.bass];
    let voices2 = [v2.soprano, v2.alto, v2.tenor, v2.bass];
    
    for i in 0..4 {
        for j in (i+1)..4 {
            let interval1 = (voices1[i].semitones() - voices1[j].semitones()).abs();
            let interval2 = (voices2[i].semitones() - voices2[j].semitones()).abs();
            
            // Check for parallel perfect 5ths (7 semitones) or octaves (12 semitones)
            if (interval1 == 7 || interval1 == 12) && interval1 == interval2 {
                let motion1 = voices2[i].semitones() - voices1[i].semitones();
                let motion2 = voices2[j].semitones() - voices1[j].semitones();
                
                // Parallel motion (same direction)?
                if motion1 != 0 && motion2 != 0 && motion1.signum() == motion2.signum() {
                    return -1000.0; // Huge penalty!
                }
            }
        }
    }
    
    0.0
}

fn voice_motion_score(v1: &Voicing, v2: &Voicing) -> f32 {
    let total_motion = 
        (v2.soprano.semitones() - v1.soprano.semitones()).abs() +
        (v2.alto.semitones() - v1.alto.semitones()).abs() +
        (v2.tenor.semitones() - v1.tenor.semitones()).abs();
    
    // Prefer less motion (common tone retention, stepwise motion)
    -0.5 * (total_motion as f32)
}

fn contrary_motion_bonus(v1: &Voicing, v2: &Voicing) -> f32 {
    let mut score = 0.0;
    
    let sop_motion = v2.soprano.semitones() - v1.soprano.semitones();
    let bass_motion = v2.bass.semitones() - v1.bass.semitones();
    
    // Bonus for contrary motion between outer voices
    if sop_motion != 0 && bass_motion != 0 && sop_motion.signum() != bass_motion.signum() {
        score += 5.0;
    }
    
    score
}

// ============================================================================
// MAIN REALIZATION ALGORITHM
// ============================================================================

fn realize_figured_bass(symbols: &[FiguredBassSymbol]) -> Vec<Voicing> {
    let mut result = Vec::new();
    
    for (i, symbol) in symbols.iter().enumerate() {
        let candidates = generate_voicings(symbol);
        
        if candidates.is_empty() {
            panic!("No valid voicings found for chord {}", i);
        }
        
        let prev = if i > 0 { Some(&result[i - 1]) } else { None };
        
        // Find best voicing
        let mut best_voicing = None;
        let mut best_score = f32::MIN;
        
        for candidate in &candidates {
            let score = score_voicing(candidate, prev, symbol.bass);
            if score > best_score {
                best_score = score;
                best_voicing = Some(candidate.clone());
            }
        }
        
        result.push(best_voicing.unwrap());
    }
    
    result
}

// ============================================================================
// EXAMPLE USAGE
// ============================================================================

fn main() {
    // Example: Simple progression in C major
    // C major -> F major (first inversion, bass on A) -> G major -> C major
    
    let progression = vec![
        FiguredBassSymbol {
            bass: Pitch::new(48), // C3
            chord_tones: vec![
                Pitch::new(48), // C
                Pitch::new(52), // E
                Pitch::new(55), // G
            ],
        },
        FiguredBassSymbol {
            bass: Pitch::new(53), // F3
            chord_tones: vec![
                Pitch::new(53), // F
                Pitch::new(57), // A
                Pitch::new(60), // C
            ],
        },
        FiguredBassSymbol {
            bass: Pitch::new(55), // G3
            chord_tones: vec![
                Pitch::new(55), // G
                Pitch::new(59), // B
                Pitch::new(62), // D
            ],
        },
        FiguredBassSymbol {
            bass: Pitch::new(48), // C3
            chord_tones: vec![
                Pitch::new(48), // C
                Pitch::new(52), // E
                Pitch::new(55), // G
            ],
        },
    ];
    
    println!("Realizing figured bass progression...\n");
    
    let voicings = realize_figured_bass(&progression);
    
    for (i, voicing) in voicings.iter().enumerate() {
        println!("Chord {}: {}", i + 1, voicing);
    }
    
    println!("\n--- Analysis ---");
    
    // Check for parallel fifths/octaves
    for i in 1..voicings.len() {
        let penalty = parallel_motion_penalty(&voicings[i-1], &voicings[i]);
        if penalty < 0.0 {
            println!("Warning: Parallel motion detected between chords {} and {}", i, i+1);
        }
    }
    
    // Calculate total voice motion
    let mut total_motion = 0;
    for i in 1..voicings.len() {
        total_motion += (voicings[i].soprano.semitones() - voicings[i-1].soprano.semitones()).abs();
        total_motion += (voicings[i].alto.semitones() - voicings[i-1].alto.semitones()).abs();
        total_motion += (voicings[i].tenor.semitones() - voicings[i-1].tenor.semitones()).abs();
    }
    println!("Total voice motion: {} semitones", total_motion);
}