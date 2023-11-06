use std::sync::{Arc, Mutex};

use iced_glow::{Alignment, Length, Renderer};
use iced_glutin::{
    widget::{checkbox, radio, Row, Text},
    widget::{Column, Slider},
    Element,
};

use crate::{
    controller::Controller,
    treeparameter::{DistributionMode, GeneticParameter, SpaceDividingMode, TreeParameter},
};

use super::Message;

pub struct GeneticsView {
    controller: Arc<Mutex<Controller>>,
}

impl GeneticsView {
    pub fn new(controller: Arc<Mutex<Controller>>) -> Self {
        Self { controller }
    }

    pub fn get_ui(&self) -> Element<Message, Renderer> {
        let controller = self.controller.lock().unwrap();
        let treedata = controller.treedata().lock().unwrap();
        let genetics = treedata.plant_genetics().lock().unwrap();

        let distribution_mode = match treedata.get_tree_param(
            TreeParameter::ResourceDistributionMode(DistributionMode::None),
        ) {
            TreeParameter::ResourceDistributionMode(mode) => mode,
            _ => DistributionMode::None,
        };

        let space_mode = match treedata
            .get_tree_param(TreeParameter::SpaceDividingMode(SpaceDividingMode::None))
        {
            TreeParameter::SpaceDividingMode(mode) => mode,
            _ => SpaceDividingMode::None,
        };

        let prune_mod = match treedata.get_tree_param(TreeParameter::PruneModOn(false)) {
            TreeParameter::PruneModOn(mode) => mode,
            _ => false,
        };

        return Column::new()
            .spacing(5)
            .padding(10)
            .width(Length::Fill)
            .push(Text::new(
                format! {"Borchert honda lambda: {:.2}",genetics.borchert_honda_lambda()},
            ))
            .push(
                Slider::new(0.4..=0.6, genetics.borchert_honda_lambda(), move |value| {
                    Message::ParamUpdate(TreeParameter::Genetic(
                        GeneticParameter::BorchertHondaLambda(value),
                    ))
                })
                .step(0.01),
            )
            .push(Text::new(
                format! {"Borchert honda alpha: {:.2}",genetics.borchert_honda_alpha()},
            ))
            .push(
                Slider::new(0.5..=3.0, genetics.borchert_honda_alpha(), move |value| {
                    Message::ParamUpdate(TreeParameter::Genetic(
                        GeneticParameter::BorchertHondaAlpha(value),
                    ))
                })
                .step(0.1),
            )
            .push(Text::new(
                format! {"Pole length: {:.2}",genetics.base_pole_lenght()},
            ))
            .push(
                Slider::new(0.0..=3.0, genetics.base_pole_lenght(), move |value| {
                    Message::ParamUpdate(TreeParameter::Genetic(GeneticParameter::PoleLength(
                        value,
                    )))
                })
                .step(0.1),
            )
            .push(Text::new(
                format! {"Auxillary shoot req: {:.2}",genetics.aux_shoot_requirement(None)},
            ))
            .push(
                Slider::new(
                    0.5..=3.0,
                    genetics.aux_shoot_requirement(None),
                    move |value| {
                        Message::ParamUpdate(TreeParameter::Genetic(GeneticParameter::AuxShootReq(
                            value,
                        )))
                    },
                )
                .step(0.1),
            )
            .push(
                Row::new()
                    .push(radio(
                        DistributionMode::BorchertHonda.to_string(),
                        DistributionMode::BorchertHonda,
                        Some(distribution_mode),
                        |v| Message::ParamUpdate(TreeParameter::ResourceDistributionMode(v)),
                    ))
                    .push(radio(
                        DistributionMode::PriorityList.to_string(),
                        DistributionMode::PriorityList,
                        Some(distribution_mode),
                        |v| Message::ParamUpdate(TreeParameter::ResourceDistributionMode(v)),
                    )),
            )
            .push(
                Row::new()
                    .push(radio(
                        SpaceDividingMode::ShadowVoxels.to_string(),
                        SpaceDividingMode::ShadowVoxels,
                        Some(space_mode),
                        |v| Message::ParamUpdate(TreeParameter::SpaceDividingMode(v)),
                    ))
                    .push(radio(
                        SpaceDividingMode::Markers.to_string(),
                        SpaceDividingMode::Markers,
                        Some(space_mode),
                        |v| Message::ParamUpdate(TreeParameter::SpaceDividingMode(v)),
                    )),
            )
            .push(Row::new().push(checkbox("Spalier", prune_mod, |v| {
                Message::ParamUpdate(TreeParameter::PruneModOn(v))
            })))
            .align_items(Alignment::Center)
            .into();
    }
}
