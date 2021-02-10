use dces::prelude::*;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::*};

/// The `LayoutSystem` builds per iteration the layout of the current ui. The layout parts are calculated by the layout objects of layout widgets.
#[derive(Constructor)]
pub struct LayoutSystem {
    context_provider: ContextProvider,
}

impl System<Tree> for LayoutSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>, res: &mut Resources) {
        let root = ecm.entity_store().root();

        if ecm
            .component_store()
            .get::<Vec<Entity>>("dirty_widgets", root)
            .unwrap()
            .is_empty()
            && !self.context_provider.first_run.get()
        {
            return;
        }

        let mut window_size = (0.0, 0.0);
        let root = ecm.entity_store().root();

        if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", root) {
            window_size.0 = bounds.width();
            window_size.1 = bounds.height();
        };

        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let rtx = res.get_mut::<RenderContext2D>();

        self.context_provider.layouts.borrow()[&root].measure(
            rtx,
            root,
            ecm,
            &self.context_provider.layouts.borrow(),
            &theme,
        );

        self.context_provider.layouts.borrow()[&root].arrange(
            rtx,
            window_size,
            root,
            ecm,
            &self.context_provider.layouts.borrow(),
            &theme,
        );

        // if self.debug_flag.get() {
        //     println!("\n------ End layout update   ------\n");
        // }
    }
}
