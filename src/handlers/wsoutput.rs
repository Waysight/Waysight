use smithay::{
    backend::{
        allocator::dmabuf::Dmabuf,
        renderer::{damage::OutputDamageTracker, Bind, Renderer},
    },
    output::Output,
};

use crate::rendering::WaysightRenderer;

pub struct WaysightOutput<T: Renderer + Bind<Dmabuf>> {
    output: Output,
    renderer: WaysightRenderer<T>,
    damage_tracker: Option<OutputDamageTracker>,
}

impl<T: Renderer + Bind<Dmabuf>> WaysightOutput<T> {
    pub fn new(output: Output, renderer: WaysightRenderer<T>, create_damage_tracker: bool) -> Self {
        let damage_tracker;
        if create_damage_tracker {
            damage_tracker = Some(OutputDamageTracker::from_output(&output));
        } else {
            damage_tracker = None;
        }

        WaysightOutput {
            output,
            renderer,
            damage_tracker,
        }
    }
}
