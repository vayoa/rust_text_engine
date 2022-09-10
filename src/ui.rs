use std::thread;
use std::time::Instant;

use cursive::event::{Event, EventResult, EventTrigger, Key};
use cursive::theme::{BorderStyle, Palette, Style, Theme};
use cursive::traits::{Nameable, Resizable};
use cursive::utils::markup::StyledString;
use cursive::utils::span::IndexedSpan;
use cursive::view::{ScrollStrategy, SizeConstraint};
use cursive::views::{
    DebugView, DummyView, LinearLayout, OnEventView, Panel, ResizedView, ScrollView, TextArea,
    TextContent, TextView,
};
use cursive::{CbSink, Cursive, CursiveRunnable, With};

use crate::text_input::TitleInput;
use crate::{FileFormat, Initializer};

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

        let text_content = TextContent::new("");

        siv.add_layer(Self::textview(&text_content));

        UI { siv, text_content }
    }

    pub fn cb_sink(&self) -> &CbSink {
        self.siv.cb_sink()
    }

    pub fn run(&mut self, root: &str) {
        let cb_sink = self.cb_sink().clone();

        let content = self.text_content.clone();
        let root = root.to_owned();

        self.siv
            .add_global_callback(Event::Key(Key::Esc), |s| s.quit());

        // Generate data in a separate thread.
        thread::spawn(move || {
            Self::execute_sections(cb_sink, &root, content);
        });

        self.siv.run();
    }

    // We will only simulate log generation here.
    // In real life, this may come from a running task, a separate process, ...
    fn execute_sections(cb_sink: CbSink, root: &str, content: TextContent) {
        let mut initializer = Initializer::new(root.to_owned(), FileFormat::Yaml);
        initializer.execute(UIMessenger {
            text_content: content,
            cb_sink,
        });
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

    fn textview(text_content: &TextContent) -> LinearLayout {
        LinearLayout::vertical()
            .child(DummyView.full_height())
            .child(Panel::new(ResizedView::new(
                SizeConstraint::Full,
                SizeConstraint::Fixed(5),
                ScrollView::new(
                    LinearLayout::vertical()
                        .child(DummyView.full_height())
                        .child(
                            TextView::new_with_content(text_content.clone())
                                .with_name("text-output"),
                        )
                        .child(OnEventView::new(TextArea::new()).on_pre_event_inner(
                            EventTrigger::from(Key::Enter),
                            |v, _e| {
                                let _text = v.get_content();
                                v.set_content("");
                                Some(EventResult::consumed())
                            },
                        )),
                )
                .scroll_strategy(ScrollStrategy::StickToBottom),
            )))
    }
}

pub struct UIMessenger {
    text_content: TextContent,
    cb_sink: CbSink,
}

impl UIMessenger {
    pub fn update_ui(&self) {
        self.cb_sink.send(Box::new(Cursive::noop)).unwrap();
    }

    pub fn clear(&mut self) {
        self.text_content.set_content("");
        self.update_ui();
    }

    pub fn append<S>(&mut self, s: S)
    where
        S: Into<StyledString>,
    {
        self.text_content.append(s);
        self.text_content.append("\n");
        self.update_ui();
    }

    pub fn typewrite_s<S>(&mut self, s: S, speed: f32)
    where
        S: Into<StyledString>,
    {
        let s = s.into();
        let l = s.width();
        self.typewrite(s, l as f32 / speed);
        self.text_content.append("\n");
        self.update_ui();
    }

    // Stolen from snailprint...
    pub fn typewrite<S>(&mut self, s: S, duration: f32)
    where
        S: Into<StyledString>,
    {
        use std::thread::sleep;

        let time = Instant::now();

        let s = s.into();
        let mut string = s.source().to_string();
        let fps = 60.0;
        let delta = 1.0 / fps;
        let len = string.len();
        let span: &IndexedSpan<Style> = s.spans_raw().first().unwrap();
        // TODO: Use the actual styling, per span!! (instead of just the first span...)
        let style = span.attr;

        'outer: while !s.is_empty() {
            let char_targ = (len as f32 * time.elapsed().as_secs_f32() / duration) as usize;

            while char_targ > len - string.len() {
                if !string.is_empty() {
                    let character = string.remove(0);
                    self.text_content
                        .append(StyledString::styled(character, style));
                    self.update_ui();
                } else {
                    // this is so sleep() is not called when this loop breaks
                    break 'outer;
                }
            }
            sleep(std::time::Duration::from_secs_f32(delta));
        }
    }

    pub fn title(&self, input: &TitleInput) {
        let figure = input.figure().to_string();
        self.cb_sink
            .send(Box::new(|s| s.add_layer(TextView::new(figure))))
            .unwrap();
        crate::common::sleep(input.wait);
        self.cb_sink
            .send(Box::new(|s| {
                s.pop_layer();
            }))
            .unwrap();
    }
}
