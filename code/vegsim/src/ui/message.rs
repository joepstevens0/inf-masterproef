use crate::{treeparameter::TreeParameter, tree::pruning_module::PruneOperation};

#[derive(Debug, Clone)]
pub enum Message {
    IndexChanged(u32),
    CheckboxToggled(bool),
    PruneButton(PruneOperation),
    Reset,
    Recalculate,
    ParamUpdate(TreeParameter),
}
