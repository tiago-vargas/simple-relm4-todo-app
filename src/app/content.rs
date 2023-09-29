use relm4::prelude::*;
use gtk::prelude::*;

pub(crate) struct ContentModel;

#[derive(Debug)]
pub(crate) enum ContentInput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ();

    type Input = ContentInput;
    type Output = ();

    view! {
        gtk::Label {
            set_label: "Hello, World!",
            set_margin_all: 4,
            set_css_classes: &["title-1"],
            set_vexpand: true,
        },
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ContentModel;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
