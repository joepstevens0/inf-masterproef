use iced_glow::{Renderer, Length, Alignment};
use iced_glutin::Element;
use iced_glutin::widget::{Button};
use iced_glutin::widget::{Column};

use crate::tree::pruning_module::PruneOperation;

use super::message::Message;

pub struct PruneButtons{

}

impl PruneButtons {
    pub fn new() -> Self { Self {  } }


    pub fn get_ui(&self) -> Element<Message, Renderer> {
        return Column::new()
            .spacing(5)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(Button::new("Prune 1").on_press(Message::PruneButton(PruneOperation::Op1)))
            .push(Button::new("Prune 2").on_press(Message::PruneButton(PruneOperation::Op2)))
            .push(Button::new("Prune 3").on_press(Message::PruneButton(PruneOperation::Op3)))
            // .push(Button::new("Prune 4").on_press(Message::PruneButton(4)))
            // .push(Button::new("Prune 5").on_press(Message::PruneButton(5)))
            .push(Button::new("Prune spil 1").on_press(Message::PruneButton(PruneOperation::Spil_1)))
            .push(Button::new("Prune spil 2").on_press(Message::PruneButton(PruneOperation::Spil_2)))
            .push(Button::new("Prune spil 3").on_press(Message::PruneButton(PruneOperation::Spil_3)))
            .into();
    }
}

