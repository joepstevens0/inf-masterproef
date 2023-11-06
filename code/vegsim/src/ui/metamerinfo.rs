use std::sync::{Arc, Mutex};

use iced_glow::{Color, Renderer};
use iced_glutin::{widget::Column, widget::Text, Element};

use crate::controller::Controller;

use super::Message;

pub struct MetamerInfo {
    controller: Arc<Mutex<Controller>>,
}

impl MetamerInfo {
    pub fn new(controller: Arc<Mutex<Controller>>) -> Self {
        Self { controller }
    }

    pub fn get_ui(&self) -> Element<Message, Renderer> {
        let controller = self
        .controller
        .lock()
        .unwrap();
        let metamer = controller
            .selected_metamer()
            .lock()
            .unwrap();
        if let Some(metamer) = &*metamer {
            return Column::new()
                .spacing(5)
                .push(
                    Text::new(format!("Select {}", metamer.id()))
                        .size(20)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("dir {:.3?}", metamer.direction()))
                        .size(20)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("aux dir {:.3?}", metamer.auxillary_direction()))
                        .size(20)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("light gen {:.3?}", metamer.last_light_generated()))
                        .size(20)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("last aux resources {:.3?}", metamer.last_aux_resources))
                        .size(20)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("last term resources {:.3?}", metamer.last_terminal_resources))
                        .size(20)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!(
                        "Supportpole {:.2?}",
                        metamer.support_pole.as_ref().map(|pole| pole.dir())
                    ))
                    .size(17)
                    .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("Width {:.5?}", metamer.end_width()))
                        .size(17)
                        .style(Color::WHITE),
                )
                .push(
                    Text::new(format!("Endpoint {:.5?}", metamer.end_point()))
                        .size(17)
                        .style(Color::WHITE),
                )
                .into();
        }
        return Text::new("No select").size(30).style(Color::WHITE).into();
    }
}
