use cursive::align::VAlign;
use cursive::backends::crossterm::crossterm::event::{Event, KeyCode, KeyEvent};
use cursive::event::{EventResult, EventTrigger, Key};
use cursive::theme::{BorderStyle, Palette, Theme};
use cursive::traits::{Nameable, Resizable};
use cursive::utils::markup::StyledString;
use cursive::view::SizeConstraint;
use cursive::views::{
    BoxedView, Dialog, DummyView, EditView, LinearLayout, OnEventView, Panel, ResizedView,
    ScrollView, TextArea, TextContent, TextView,
};
use cursive::{CbSink, Cursive, CursiveRunnable, View, With};

pub struct UI {
    siv: CursiveRunnable,
    text_content: TextContent,
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    pub fn new() -> Self {
        let mut siv = Self::root();

        let text_content = TextContent::new("asd");

        siv.add_layer(Self::textview(&text_content));

        UI { siv, text_content }
    }

    pub fn cb_sink(&self) -> &CbSink { self.siv.cb_sink() }

    pub fn run(&mut self) {
        self.siv.run();
    }

    fn root() -> CursiveRunnable {
        let mut siv = cursive::default();

        siv.set_theme(Theme {
            shadow: false,
            borders: BorderStyle::Simple,
            palette: Palette::default().with(|palette| {
                use cursive::theme::BaseColor::*;
                use cursive::theme::Color::*;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Red.light();
                palette[Secondary] = Red.light();
                palette[Highlight] = Red.dark();
            }),
        });
        siv
    }

    fn textview(text_content: &TextContent) -> Panel<ResizedView<ScrollView<LinearLayout>>> {
        Panel::new(ResizedView::new(
            SizeConstraint::Full,
            SizeConstraint::Fixed(5),
            ScrollView::new(
                LinearLayout::vertical()
                    .child(DummyView.full_height())
                    .child(
                        TextView::new_with_content(text_content.clone()).with_name("text-output"),
                    )
                    .child(OnEventView::new(TextArea::new()).on_pre_event_inner(
                        EventTrigger::from(Key::Enter),
                        |v, _e| {
                            let _text = v.get_content();
                            v.set_content("");
                            Some(EventResult::consumed())
                        },
                    )),
            ),
        ))
    }

    pub fn clear_textview(&mut self) {
        self.text_content.set_content("");
    }

    pub fn append_to_textview<S>(&mut self, s: S)
    where
        S: Into<StyledString>,
    {
        self.text_content.append(s);
    }
}
