use relm4::{prelude::*, actions::{RelmAction, RelmActionGroup}};

use super::TaskRow;

relm4::new_action_group!(pub(super) RowActions, "row");

relm4::new_stateless_action!(pub(super) MoveRowUp, RowActions, "move-up");
relm4::new_stateless_action!(pub(super) MoveRowDown, RowActions, "move-down");
relm4::new_stateless_action!(pub(super) RemoveRow, RowActions, "remove");

impl TaskRow {
    pub(super) fn create_actions(
        index: &DynamicIndex,
        widgets: &<Self as FactoryComponent>::Widgets,
        sender: &FactorySender<Self>,
    ) {
        let mut row_actions = RelmActionGroup::<RowActions>::new();

		let move_row_up = {
			let sender = sender.clone();
			let index = index.clone();
			RelmAction::<MoveRowUp>::new_stateless(move |_| {
				sender.output(<Self as FactoryComponent>::Output::MoveUp(index.clone()));
			})
		};
		row_actions.add_action(move_row_up);

		let move_row_down = {
			let sender = sender.clone();
			let index = index.clone();
			RelmAction::<MoveRowDown>::new_stateless(move |_| {
				sender.output(<Self as FactoryComponent>::Output::MoveDown(index.clone()));
			})
		};
		row_actions.add_action(move_row_down);

        let remove_row = {
            let sender = sender.clone();
            let index = index.clone();
            RelmAction::<RemoveRow>::new_stateless(move |_| {
                sender.output(<Self as FactoryComponent>::Output::Remove(index.clone()));
            })
        };
        row_actions.add_action(remove_row);

        row_actions.register_for_widget(&widgets.menu);
    }
}
