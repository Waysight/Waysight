use std::{collections::HashMap, process::Output};

use crate::handlers::wsoutput::WaysightOutput;
use smithay::backend::{
    allocator::{dmabuf::Dmabuf, Fourcc},
    renderer::{
        damage::OutputDamageTracker,
        element::{RenderElement, RenderElementStates},
        Bind, Renderer,
    },
};

pub struct WaysightRenderer<T: Renderer + Bind<Dmabuf>> {
    renderer: T,
    format: Fourcc,
}

impl<T: Renderer + Bind<Dmabuf>> WaysightRenderer<T> {
    // Creates a new WaysightRenderer.
    // **The renderer MUST have support for binding of dmabufs**
    // The function will take ownership of the given renderer and then give it to the struct, so
    // the way to handle the renderer is from this struct.
    fn new(renderer: T, preferred_format: Fourcc) -> Self {
        WaysightRenderer {
            renderer,
            format: preferred_format,
        }
    }

    fn render_from_damage_tracker<E: RenderElement<T>>(
        &mut self,
        damage_tracker: &mut OutputDamageTracker,
        buffer: Dmabuf,
        age: u8,
        elements: &[E],
        clear_color: [f32; 4],
    ) -> (bool, RenderElementStates) {
        match self.renderer.bind(buffer) {
            Ok(_) => {}
            Err(err) => {
                tracing::error!("Failed to bind dmabuf to renderer: {}", err);
            }
        };

        match damage_tracker.render_output(&mut self.renderer, age as usize, elements, clear_color)
        {
            Ok((contents, states)) => {
                let rendered = match contents {
                    Some(_) => true,
                    None => false,
                };
                (rendered, states)
            }
            Err(err) => {
                tracing::error!("Failed to render output: {}", err);
                (
                    false,
                    RenderElementStates {
                        states: HashMap::new(),
                    },
                )
            }
        }
    }
}
