use dces::prelude::*;

use crate::{
    theming::{Selector, Theme},
    tree::Tree,
    widget_base::Context,
};

/// This system is used to initializes the widgets.
pub struct InitSystem;

impl InitSystem {
    /// Creates a new init system.
    pub fn new() -> Self {
        InitSystem
    }
}

impl System<Tree> for InitSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>, res: &mut Resources) {
        let root = ecm.entity_store().root();

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        if debug {
            // crate::shell::CONSOLE.log("\n------ Widget tree ------\n".to_string());

            print_tree(root, 0, ecm);

            // crate::shell::CONSOLE.log("\n------ Widget tree ------\n".to_string());
        }

        // init css ids
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let mut current_node = root;

        loop {
            {
                let mut ctx = Context::new((current_node, ecm), &theme);

                if let Some(state) = self
                    .context_provider
                    .states
                    .borrow_mut()
                    .get_mut(&current_node)
                {
                    state.init(&mut ctx, res);
                }

                drop(ctx);
            }

            let mut it = ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }
    }
}

pub fn print_tree(entity: Entity, depth: usize, ecm: &mut EntityComponentManager<Tree>) {
    let name = ecm.component_store().get::<String>("name", entity).unwrap();

    let selector = if let Ok(selector) = ecm.component_store().get::<Selector>("selector", entity) {
        selector.clone()
    } else {
        Selector::default()
    };

    // crate::shell::CONSOLE.log(format!(
    //     "{}{} (entity: {}{})",
    //     "| ".repeat(depth),
    //     name,
    //     entity.0,
    //     selector
    // ));

    for child in ecm.entity_store().clone().children.get(&entity).unwrap() {
        print_tree(*child, depth + 1, ecm);
    }
}
