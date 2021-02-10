use std::collections::BTreeMap;

use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, tree::Tree};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
#[derive(Constructor)]
pub struct RenderSystem {
    context_provider: ContextProvider,
}

impl System<Tree> for RenderSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>, res: &mut Resources) {
        let root = ecm.entity_store().root();

        let dirty_widgets = ecm
            .component_store()
            .get::<Vec<Entity>>("dirty_widgets", root)
            .unwrap()
            .clone();

        if dirty_widgets.is_empty() && !self.context_provider.first_run.get() {
            return;
        }

        // reset the dirty flag of all dirty widgets to `false`
        for widget in dirty_widgets {
            if let Ok(dirty) = ecm.component_store_mut().get_mut::<bool>("dirty", widget) {
                *dirty = false;
            }
        }

        ecm.component_store_mut()
            .get_mut::<Vec<Entity>>("dirty_widgets", root)
            .unwrap()
            .clear();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        let root = ecm.entity_store().root();
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let mut offsets = BTreeMap::new();
        offsets.insert(root, (0.0, 0.0));

        // CONSOLE.time("render");
        let rtx = res.get_mut::<RenderContext2D>();

        rtx.start();
        rtx.begin_path();
        self.context_provider.render_objects.borrow()[&root].render(
            rtx,
            root,
            ecm,
            &self.context_provider,
            &theme,
            &mut offsets,
            debug,
        );
        rtx.finish();

        if self.context_provider.first_run.get() {
            self.context_provider.first_run.set(false);
        }
    }
}
