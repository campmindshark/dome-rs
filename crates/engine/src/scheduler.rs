//! Allocation-light scheduler semantics mirroring Spectrum's `Operator`.

/// Visualizer metadata needed by the scheduler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualizerSpec {
    /// Stable visualizer name.
    pub name: &'static str,
    /// Current priority. Priority 0 is never selected. Priority -1 is always-run.
    pub priority: i32,
    /// Whether all required inputs are enabled.
    pub inputs_enabled: bool,
}

/// Active visualizer names for one output frame.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScheduledFrame {
    /// Highest selected non-negative priority.
    pub top_priority: i32,
    /// Visualizers selected for this frame.
    pub active: Vec<&'static str>,
}

/// Input metadata used by the full frame scheduler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputSpec {
    /// Stable input name.
    pub name: &'static str,
    /// Whether the input is enabled in config/hardware.
    pub enabled: bool,
    /// Whether the input should run whenever the operator runs.
    pub always_active: bool,
}

/// Visualizer metadata with explicit input dependencies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullVisualizerSpec {
    /// Stable visualizer name.
    pub name: &'static str,
    /// Current priority.
    pub priority: i32,
    /// Required input names.
    pub inputs: &'static [&'static str],
}

/// Output metadata used by the full frame scheduler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputSpec {
    /// Stable output name.
    pub name: &'static str,
    /// Whether the output is enabled by hardware or simulation.
    pub enabled: bool,
    /// Visualizers registered for this output.
    pub visualizers: Vec<FullVisualizerSpec>,
}

/// Full operator-frame schedule.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OperatorFrame {
    /// Active input names, in update order.
    pub active_inputs: Vec<&'static str>,
    /// Active visualizer names, in update order.
    pub active_visualizers: Vec<&'static str>,
    /// Active output names, in update order.
    pub active_outputs: Vec<&'static str>,
}

/// Select visualizers using Spectrum's priority rules.
#[must_use]
pub fn schedule_visualizers(specs: &[VisualizerSpec]) -> ScheduledFrame {
    let mut top_priority = 1;
    let mut active = Vec::new();
    let mut always = Vec::new();

    for spec in specs {
        if !spec.inputs_enabled {
            continue;
        }
        if spec.priority == -1 {
            always.push(spec.name);
        } else if spec.priority > top_priority {
            top_priority = spec.priority;
            active.clear();
            active.push(spec.name);
        } else if spec.priority == top_priority {
            active.push(spec.name);
        }
    }

    active.extend(always);
    ScheduledFrame {
        top_priority,
        active,
    }
}

/// Build a full operator-frame schedule from outputs, visualizers, and inputs.
#[must_use]
pub fn schedule_operator_frame(inputs: &[InputSpec], outputs: &[OutputSpec]) -> OperatorFrame {
    let mut active_outputs = Vec::new();
    let mut active_visualizers = Vec::new();

    for output in outputs {
        if !output.enabled {
            continue;
        }

        let candidates: Vec<_> = output
            .visualizers
            .iter()
            .map(|visualizer| VisualizerSpec {
                name: visualizer.name,
                priority: visualizer.priority,
                inputs_enabled: visualizer.inputs.iter().all(|name| {
                    inputs
                        .iter()
                        .any(|input| input.name == *name && input.enabled)
                }),
            })
            .collect();
        let scheduled = schedule_visualizers(&candidates);
        if scheduled.active.is_empty() {
            continue;
        }

        active_outputs.push(output.name);
        for name in scheduled.active {
            if !active_visualizers.contains(&name) {
                active_visualizers.push(name);
            }
        }
    }

    let mut active_inputs = Vec::new();
    for input in inputs {
        if input.enabled && input.always_active {
            active_inputs.push(input.name);
            continue;
        }

        let needed = outputs
            .iter()
            .flat_map(|output| output.visualizers.iter())
            .filter(|visualizer| active_visualizers.contains(&visualizer.name))
            .any(|visualizer| visualizer.inputs.contains(&input.name));

        if input.enabled && needed {
            active_inputs.push(input.name);
        }
    }

    OperatorFrame {
        active_inputs,
        active_visualizers,
        active_outputs,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        schedule_operator_frame, schedule_visualizers, FullVisualizerSpec, InputSpec, OutputSpec,
        VisualizerSpec,
    };

    fn v(name: &'static str, priority: i32) -> VisualizerSpec {
        VisualizerSpec {
            name,
            priority,
            inputs_enabled: true,
        }
    }

    #[test]
    fn priority_zero_is_never_selected() {
        let frame = schedule_visualizers(&[v("dead-midi-test", 0)]);
        assert!(frame.active.is_empty());
    }

    #[test]
    fn priority_two_ties_run_together_for_flash_overlay() {
        let frame = schedule_visualizers(&[v("volume", 2), v("flash", 2), v("tv-static", 1)]);
        assert_eq!(frame.active, vec!["volume", "flash"]);
    }

    #[test]
    fn diagnostics_override_normal_modes() {
        let frame = schedule_visualizers(&[v("volume", 2), v("flash-colors", 1000)]);
        assert_eq!(frame.active, vec!["flash-colors"]);
    }

    #[test]
    fn always_run_priority_is_supported() {
        let frame = schedule_visualizers(&[v("volume", 2), v("future-overlay", -1)]);
        assert_eq!(frame.active, vec!["volume", "future-overlay"]);
    }

    #[test]
    fn disabled_inputs_block_visualizer() {
        let frame = schedule_visualizers(&[
            VisualizerSpec {
                name: "volume",
                priority: 2,
                inputs_enabled: false,
            },
            v("tv-static", 1),
        ]);
        assert_eq!(frame.active, vec!["tv-static"]);
    }

    #[test]
    fn schedules_inputs_visualizers_and_outputs_in_operator_order() {
        let inputs = [
            InputSpec {
                name: "audio",
                enabled: true,
                always_active: true,
            },
            InputSpec {
                name: "midi",
                enabled: true,
                always_active: false,
            },
        ];
        let outputs = [OutputSpec {
            name: "dome",
            enabled: true,
            visualizers: vec![
                FullVisualizerSpec {
                    name: "volume",
                    priority: 2,
                    inputs: &["audio"],
                },
                FullVisualizerSpec {
                    name: "flash",
                    priority: 2,
                    inputs: &["midi"],
                },
                FullVisualizerSpec {
                    name: "tv-static",
                    priority: 1,
                    inputs: &[],
                },
            ],
        }];

        let frame = schedule_operator_frame(&inputs, &outputs);

        assert_eq!(frame.active_inputs, vec!["audio", "midi"]);
        assert_eq!(frame.active_visualizers, vec!["volume", "flash"]);
        assert_eq!(frame.active_outputs, vec!["dome"]);
    }

    #[test]
    fn output_without_schedulable_visualizers_is_inactive() {
        let inputs = [InputSpec {
            name: "audio",
            enabled: false,
            always_active: false,
        }];
        let outputs = [OutputSpec {
            name: "dome",
            enabled: true,
            visualizers: vec![FullVisualizerSpec {
                name: "volume",
                priority: 2,
                inputs: &["audio"],
            }],
        }];

        let frame = schedule_operator_frame(&inputs, &outputs);

        assert!(frame.active_inputs.is_empty());
        assert!(frame.active_visualizers.is_empty());
        assert!(frame.active_outputs.is_empty());
    }
}
