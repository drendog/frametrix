use crate::device::led_matrix::PatternId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub patterns: HashMap<String, Pattern>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Pattern {
    pub id: PatternId,
    pub label: String,
}

// TODO: Avoid the need for Default implementation and hardcoding
impl Default for Config {
    fn default() -> Self {
        Self {
            patterns: HashMap::from_iter([
                (
                    "all_off".to_string(),
                    Pattern {
                        id: 0,
                        label: "All Off".to_string(),
                    },
                ),
                (
                    "gradient".to_string(),
                    Pattern {
                        id: 1,
                        label: "Gradient".to_string(),
                    },
                ),
                (
                    "double_gradient".to_string(),
                    Pattern {
                        id: 2,
                        label: "Double Gradient".to_string(),
                    },
                ),
                (
                    "lotus_sideways".to_string(),
                    Pattern {
                        id: 3,
                        label: "Lotus Sideways".to_string(),
                    },
                ),
                (
                    "zigzag".to_string(),
                    Pattern {
                        id: 4,
                        label: "Zigzag".to_string(),
                    },
                ),
                (
                    "all_on".to_string(),
                    Pattern {
                        id: 5,
                        label: "All On".to_string(),
                    },
                ),
                (
                    "panic".to_string(),
                    Pattern {
                        id: 6,
                        label: "Panic".to_string(),
                    },
                ),
                (
                    "lotus_top_down".to_string(),
                    Pattern {
                        id: 7,
                        label: "Lotus Top Down".to_string(),
                    },
                ),
            ]),
        }
    }
}
