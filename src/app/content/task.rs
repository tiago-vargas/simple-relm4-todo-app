use super::ContentInput;

use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) struct TaskRow(String);

#[relm4::factory(pub(crate))]
impl FactoryComponent for TaskRow {
    type Init = String;

    type Input = ();
    type Output = ();

    type CommandOutput = ();
    type ParentInput = ContentInput;
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::CheckButton {
            set_label: Some(self.0.as_str()),
            set_halign: gtk::Align::Start,
            set_margin_all: 8,
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }

    fn init_model(
        description: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self(description)
    }

    fn update(&mut self, _input: Self::Input, _sender: FactorySender<Self>) {}
}
