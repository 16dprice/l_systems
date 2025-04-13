pub struct ReplacementRule<'a> {
    pub from: char,
    pub to: &'a str,
}

pub struct LSystemConfiguration<'a> {
    pub axiom: &'a str,
    pub replacement_rules: Vec<ReplacementRule<'a>>,
}

#[allow(dead_code)]
pub enum PresetLSystemConfiguration {
    Crystal, DragonCurve, BinaryTree, FractalPlant, Sierpinski, Hilbert, Tree, Koch,
    My1, My2, My3, My4, My5, My6, My7, My8
}

pub fn get_preset_l_system_configuration<'a>(config: PresetLSystemConfiguration) -> LSystemConfiguration<'a> {
    match config {
        PresetLSystemConfiguration::Crystal => {
            return LSystemConfiguration {
                axiom: "X",
                replacement_rules: vec![
                    ReplacementRule { from: 'X', to: "F[+X][-X]FX" },
                    ReplacementRule { from: 'F', to: "FF" }
                ]
            }
        }
        PresetLSystemConfiguration::DragonCurve => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![
                    ReplacementRule { from: 'F', to: "F+G" },
                    ReplacementRule { from: 'G', to: "F-G" }
                ]
            }
        }
        PresetLSystemConfiguration::BinaryTree => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![
                    ReplacementRule { from: 'F', to: "G[-F]+F" },
                    ReplacementRule { from: 'G', to: "GG" }
                ]
            }
        }
        PresetLSystemConfiguration::FractalPlant => {
            return LSystemConfiguration {
                axiom: "-X",
                replacement_rules: vec![
                    ReplacementRule { from: 'X', to: "F+[[X]-X]-F[-FX]+X" },
                    ReplacementRule { from: 'F', to: "FF" }
                ]
            }
        }
        PresetLSystemConfiguration::Sierpinski => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![
                    ReplacementRule { from: 'F', to: "G-F-G" },
                    ReplacementRule { from: 'G', to: "F+G+F" }
                ]
            }
        }
        PresetLSystemConfiguration::Hilbert => {
            return LSystemConfiguration {
                axiom: "A",
                replacement_rules: vec![
                    ReplacementRule { from: 'A', to: "+BF-AFA-FB+" },
                    ReplacementRule { from: 'B', to: "-AF+BFB+FA-" },
                ]
            }
        }
        PresetLSystemConfiguration::Tree => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F[+F]F[-F]F" } ]
            };
        }
        PresetLSystemConfiguration::Koch => {
            return LSystemConfiguration {
                axiom: "F", 
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F-F+F+F-F" } ]
            };
        }
        PresetLSystemConfiguration::My1 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F[+F[-F[+F[-F]]]]" } ]
            };
        }
        PresetLSystemConfiguration::My2 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F[+F-F][-F+F]F" } ]
            };
        }
        PresetLSystemConfiguration::My3 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F[-F-F][+F+F]F" } ]
            };
        }
        PresetLSystemConfiguration::My4 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F-F+F" /* looks like dragon curve around 2.09" */ } ]
            };
        }
        PresetLSystemConfiguration::My5 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F[F-F+F+F-F]F[F+F-F-F+F]F" } ]
            };
        }
        PresetLSystemConfiguration::My6 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F[-F+F+F-]F" } ]
            };
        }
        PresetLSystemConfiguration::My7 => {
            return LSystemConfiguration {
                axiom: "F--F--F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "F+F--F+F" } ]
            };
        }
        PresetLSystemConfiguration::My8 => {
            return LSystemConfiguration {
                axiom: "F",
                replacement_rules: vec![ ReplacementRule { from: 'F', to: "FF[+++F][---F]" } ]
            };
        }
    }
}
