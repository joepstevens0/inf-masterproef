mod geneticsview;
mod message;
mod metamerinfo;
mod prunebuttons;

use std::sync::{Arc, Mutex};

use crate::controller::Controller;
use iced_glow::Renderer;
use iced_glutin::widget::{Button, Checkbox, Slider};
use iced_glutin::widget::{Column, Row, Text};
use iced_glutin::{Alignment, Color, Command, Element, Length, Program};

use self::geneticsview::GeneticsView;
use self::message::Message;
use self::metamerinfo::MetamerInfo;
use self::prunebuttons::PruneButtons;

pub struct Controls {
    controller: Arc<Mutex<Controller>>,
    index: u32,
    show_markers: bool,

    metamer_info_element: MetamerInfo,
    prune_buttons_element: PruneButtons,
    genetics_view: GeneticsView,
}

impl Controls {
    pub fn new(controller: Arc<Mutex<Controller>>) -> Controls {
        Controls {
            index: 0,
            show_markers: false,

            metamer_info_element: MetamerInfo::new(controller.clone()),
            prune_buttons_element: PruneButtons::new(),
            genetics_view: GeneticsView::new(controller.clone()),

            controller,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    fn get_ui(&self) -> Element<Message, Renderer> {
        let element_column = Column::new()
            .spacing(10)
            // .push(self.mode_select())
            .push(self.reset_button())
            .push(self.genetics_view.get_ui())
            .push(self.prune_buttons_element.get_ui())
            .push(self.metamer_info_element.get_ui())
            .push(self.marker_check())
            .push(self.slider());

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .push(
                Row::new()
                    .width(Length::Units(390))
                    .height(Length::Fill)
                    .align_items(Alignment::End)
                    .push(element_column),
            )
            .into()
    }

    fn reset_button(&self) -> Element<Message, Renderer> {
        return Column::new()
            .spacing(5)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(Button::new("Reset").on_press(Message::Reset))
            .push(Button::new("Recalculate").on_press(Message::Recalculate))
            .into();
    }

    // fn mode_select(&self) -> Element<Message, Renderer> {
    //     return Row::new()
    //     .spacing(10)
    //     .push(radio(
    //         SpaceDividingMode::Markers.to_string(),
    //         SpaceDividingMode::Markers,
    //         Some(self.mode),
    //         Message::ModeChanged,))
    //     .push(radio(
    //         SpaceDividingMode::ShadowVoxels.to_string(),
    //         SpaceDividingMode::ShadowVoxels,
    //         Some(self.mode),
    //         Message::ModeChanged,))
    //     .into();
    // }

    fn marker_check(&self) -> Element<Message, Renderer> {
        return Checkbox::new(self.show_markers, "Show markers", Message::CheckboxToggled).into();
    }

    fn slider(&self) -> Element<Message, Renderer> {
        let index = self.index;

        let slider = Slider::new(0..=200, index, move |index| Message::IndexChanged(index)).step(1);

        Column::new()
            .padding(10)
            .spacing(10)
            .push(Text::new("Debug layer").style(Color::WHITE))
            .push(slider)
            .push(
                Text::new(format!("{:?}", index))
                    .size(14)
                    .style(Color::WHITE),
            )
            .into()
    }

    pub fn show_markers(&self) -> bool {
        self.show_markers
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IndexChanged(index) => {
                self.index = index;
            }
            Message::CheckboxToggled(toggle) => {
                self.show_markers = toggle;
            }
            Message::PruneButton(op) => {
                self.controller.lock().unwrap().perform_prune(op);
            }
            Message::Reset => {
                self.controller.lock().unwrap().reset_plants();
            }
            Message::Recalculate => {
                self.controller.lock().unwrap().recalculate_plants();
            }
            Message::ParamUpdate(param) =>{
                self.controller.lock().unwrap().update_tree_param(param);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer> {
        return self.get_ui();
    }
}
