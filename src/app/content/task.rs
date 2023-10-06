use super::ContentInput;

use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct TaskRow {
    pub(crate) task: Task,
}

#[derive(Debug)]
pub(crate) struct Task {
    pub(crate) description: String,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for TaskRow {
    type Init = Task;

    type Input = ();
    type Output = ();

    type CommandOutput = ();
    type ParentInput = ContentInput;
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::CheckButton {
            set_label: Some(self.task.description.as_str()),
            set_halign: gtk::Align::Start,
            set_margin_all: 8,
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }

    fn init_model(
        task: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { task }
    }

    fn update(&mut self, _input: Self::Input, _sender: FactorySender<Self>) {}
}
