use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use cursive::{CbSink, CursiveRunnable, With};
use cursive::event::{Event, EventResult, EventTrigger, Key};
use cursive::theme::{BorderStyle, Palette, Theme};
use cursive::traits::{Nameable, Resizable};
use cursive::view::{Margins, ScrollStrategy, SizeConstraint};
use cursive::views::{
    DummyView, LinearLayout, OnEventView, PaddedView, Panel, ResizedView, ScrollView,
    StackView, TextArea, TextContent, TextView,
};
use cursive_aligned_view::Alignable;

use crate::{FileFormat, Initializer};
use crate::compiled::{Comp, CompileError};
use crate::ui_messenger::UIMessenger;

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
                .fullscreen_layer(
                    TextView::new_with_content(draw_content.clone())
                        .no_wrap()
                        .align_center()
                        .with_name("frame-view")
                        .full_screen(),
                )
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

    #[inline]
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
        let mut m = UIMessenger::new(content, frame_content, cb_sink, input_receiver);

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

    pub fn get_image<P>(dir: P, scale: u32, invert: bool) -> Comp<String>
        where
            P: AsRef<Path>,
    {
        use image::GenericImageView;

        let mut output = String::from("");
        let dir: &Path = dir.as_ref();
        if !dir.exists() {
            return Err(CompileError::InvalidPath(dir.to_path_buf()));
        }
        let img = image::open(dir)?;
        let (width, height) = img.dimensions();
        for y in 0..height {
            for x in 0..width {
                if y % (scale * 2) == 0 && x % scale == 0 {
                    let mut pix = img.get_pixel(x, y);
                    if invert {
                        for i in 0..3 {
                            pix[i] = 255 - pix[i];
                        }
                    }
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
        Ok(output)
    }
}
