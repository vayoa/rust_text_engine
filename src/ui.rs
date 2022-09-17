use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Instant;

use cursive::event::{Event, EventResult, EventTrigger, Key};
use cursive::theme::{BaseColor, BorderStyle, Color, Effect, Palette, Style, Theme};
use cursive::traits::{Nameable, Resizable};
use cursive::utils::markup::StyledString;
use cursive::utils::span::IndexedSpan;
use cursive::view::{Margins, ScrollStrategy, SizeConstraint};
use cursive::views::{
    DummyView, LinearLayout, OnEventView, PaddedView, Panel, ResizedView, ScrollView, StackView,
    TextArea, TextContent, TextView,
};
use cursive::{CbSink, Cursive, CursiveRunnable, With};

use crate::compiled::CompileError;
use crate::text_input::TitleInput;
use crate::{FileFormat, Initializer};

pub struct UI {
    siv: CursiveRunnable,
    text_content: TextContent,
    draw_content: TextContent,
}

impl Default for UI {
    fn default() -> Self {
        Self::new().0
    }
}

impl UI {
    pub fn new() -> (Self, Receiver<String>) {
        let mut siv = Self::root();

        let text_content = TextContent::new("");
        let draw_content = TextContent::new("");

        let (tx, rx) = mpsc::channel();

        siv.add_fullscreen_layer(
            StackView::new()
                .fullscreen_layer(TextView::new_with_content(draw_content.clone()).no_wrap())
                .transparent_layer(Self::textview(&text_content, tx)),
        );

        (
            UI {
                siv,
                text_content,
                draw_content,
            },
            rx,
        )
    }

    pub fn cb_sink(&self) -> &CbSink {
        self.siv.cb_sink()
    }

    pub fn run(&mut self, root: &str, rx: Receiver<String>) {
        let cb_sink = self.cb_sink().clone();

        let content = self.text_content.clone();
        let draw_content = self.draw_content.clone();
        let root = root.to_owned();

        self.siv
            .add_global_callback(Event::Key(Key::Esc), |s| s.quit());

        // Generate data in a separate thread.
        thread::spawn(move || {
            Self::execute_sections(cb_sink, &root, content, draw_content, rx);
        });

        self.siv.run();
    }

    // We will only simulate log generation here.
    // In real life, this may come from a running task, a separate process, ...
    fn execute_sections(
        cb_sink: CbSink,
        root: &str,
        content: TextContent,
        frame_content: TextContent,
        input_receiver: Receiver<String>,
    ) {
        let mut m = UIMessenger {
            text_content: content,
            cb_sink,
            input_receiver,
            frame_content,
        };

        let initializer = Initializer::new(root.to_owned(), FileFormat::Yaml);
        if let Ok(mut initializer) = initializer {
            initializer.execute(m);
        } else {
            m.err(&initializer.unwrap_err());
        }
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

    fn textview(text_content: &TextContent, tx: Sender<String>) -> LinearLayout {
        LinearLayout::vertical()
            .child(DummyView.full_height())
            .child(PaddedView::new(
                Margins::lr(2, 2),
                Panel::new(ResizedView::new(
                    SizeConstraint::Full,
                    SizeConstraint::Fixed(7),
                    ScrollView::new(
                        LinearLayout::vertical()
                            .child(DummyView.full_height())
                            .child(
                                TextView::new_with_content(text_content.clone())
                                    .with_name("text-output"),
                            )
                            .child(
                                OnEventView::new(
                                    TextArea::new().disabled().with_name("text-input"),
                                )
                                .on_pre_event_inner(
                                    EventTrigger::from(Key::Enter),
                                    move |v, _e| {
                                        let mut v = v.get_mut();
                                        let text = v.get_content();
                                        tx.send(text.to_string());
                                        v.set_content("");
                                        Some(EventResult::consumed())
                                    },
                                ),
                            ),
                    )
                    .scroll_strategy(ScrollStrategy::StickToBottom),
                )),
            ))
    }

    const fn get_str_ascii(intent: u8) -> &'static str {
        let index = intent / 32;
        const ASCII: [&str; 8] = [" ", ".", ",", "-", "~", "+", "=", "@"];
        ASCII[index as usize]
    }

    pub fn get_image<P>(dir: P, scale: u32) -> String
    where
        P: AsRef<Path>,
    {
        use image::GenericImageView;

        let mut output = String::from("");
        let img = image::open(dir).unwrap();
        let (width, height) = img.dimensions();
        for y in 0..height {
            for x in 0..width {
                if y % (scale * 2) == 0 && x % scale == 0 {
                    let pix = img.get_pixel(x, y);
                    let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                    if pix[3] == 0 {
                        intent = 0;
                    }
                    output += Self::get_str_ascii(intent);
                }
            }
            if y % (scale * 2) == 0 {
                output += "\n";
            }
        }
        output
    }
}

pub struct UIMessenger {
    text_content: TextContent,
    frame_content: TextContent,
    cb_sink: CbSink,
    input_receiver: Receiver<String>,
}

impl UIMessenger {
    pub fn update_ui(&self) {
        self.cb_sink.send(Box::new(Cursive::noop)).unwrap();
    }

    pub fn clear(&mut self) {
        self.text_content.set_content("");
        self.update_ui();
    }

    pub fn clear_frame(&mut self) {
        self.frame_content.set_content("");
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

    fn append_titled_err(&mut self, t: &str, s: &str) {
        let style = Style::from(Color::Light(BaseColor::Red));
        let s = StyledString::single_span(s.to_string() + "\n", style);
        let t = StyledString::single_span(
            t.to_string() + "\n",
            style.combine(Effect::Reverse).combine(Effect::Bold),
        );
        self.text_content.append(t);
        self.text_content.append(s);
        self.update_ui();
    }

    #[inline]
    pub fn append_err(&mut self, s: &str) {
        self.append_titled_err("Error", s);
    }

    #[inline]
    pub fn err(&mut self, e: &CompileError) {
        self.append_titled_err(&(e.name() + "Error"), &e.to_string());
        self.title(&TitleInput {
            text: "ERROR".to_string(),
            wait: 2,
        });
    }

    pub fn set_frame<S>(&mut self, s: S)
    where
        S: Into<StyledString>,
    {
        self.frame_content.set_content(s);
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

    pub fn update_text_input(&self, disable: bool) {
        self.cb_sink
            .send(Box::new(move |s| {
                s.call_on_name("text-input", move |v: &mut TextArea| {
                    if disable {
                        v.disable();
                    } else {
                        v.enable();
                    }
                })
                .unwrap();
                if !disable {
                    s.focus_name("text-input").unwrap();
                }
            }))
            .unwrap();
    }

    pub fn get_input(&self) -> String {
        self.update_text_input(false);
        let input = self.input_receiver.recv().unwrap();
        self.update_text_input(true);
        input
    }

    pub fn get_append_input(&mut self) -> String {
        let input = self.get_input();
        self.append(&input);
        input
    }
}
