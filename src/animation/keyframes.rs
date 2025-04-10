use crate::configurations::runtime_configuration::RuntimeConfiguration;

struct LSystemKeyFrame {
    runtime_configuration: RuntimeConfiguration,
}

struct LSystemAnimation {
    keyframes: [LSystemKeyFrame; 2],
    total_frames: usize,
    
}
