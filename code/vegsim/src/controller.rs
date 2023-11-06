use std::sync::{Arc, Mutex};

use crate::{
    tree::{metamer::Metamer, TreeApp, pruning_module::PruneOperation},
    treeparameter::TreeParameter,
};

pub struct Controller {
    treedata: Arc<Mutex<TreeApp>>,
    selected_metamer: Arc<Mutex<Option<Metamer>>>,
}

impl Controller {
    pub fn new(treedata: Arc<Mutex<TreeApp>>) -> Self {
        Self {
            treedata,
            selected_metamer: Arc::new(Mutex::new(None)),
        }
    }

    pub fn perform_prune(&mut self, prune_index: PruneOperation) {
        self.treedata.lock().unwrap().prune_by_rule(prune_index);
    }
    pub fn reset_plants(&mut self) {
        self.treedata.lock().unwrap().reset_plants();
    }
    pub fn recalculate_plants(&mut self) {
        self.treedata.lock().unwrap().recalculate_plants();
    }

    pub fn selected_metamer(&self) -> &Mutex<Option<Metamer>> {
        self.selected_metamer.as_ref()
    }

    pub fn prune_id(&mut self, id: u32) {
        self.treedata.lock().unwrap().prune_id(id);
    }

    pub fn perform_growth_iteration(&self) {
        self.treedata.lock().unwrap().perform_growth_iteration();
    }

    pub fn update_selected(&mut self, id: Option<u32>) {
        let mut treedata = self.treedata.lock().unwrap();
        treedata.set_selected_id(id);
        if let Some(id) = id {
            let metamer = treedata.get_metamer_by_id(id);
            *self.selected_metamer.lock().unwrap() = metamer;
        }
    }

    pub fn treedata(&self) -> &Mutex<TreeApp> {
        self.treedata.as_ref()
    }

    pub fn update_tree_param(&self, param: TreeParameter) {
        self.treedata.lock().unwrap().update_tree_param(param);
    }

    pub fn get_tree_param(&self, param: TreeParameter) -> TreeParameter{
        self.treedata.lock().unwrap().get_tree_param(param)
    }
}
