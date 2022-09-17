use std::sync::mpsc::Receiver;

use cursive::{CbSink, Cursive};
use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::views::{TextArea, TextContent, TextView};
use cursive_aligned_view::AlignedView;

use crate::compiled::CompileError;
use crate::show_input::Alignment;
use crate::text_input::TitleInput;

pub struct UIMessenger {
    text_content: TextContent,
    frame_content: TextContent,
    cb_sink: CbSink,
    input_receiver: Receiver<String>,
}

impl UIMessenger {
    pub fn new(
        text_content: TextContent,
        frame_content: TextContent,
        cb_sink: CbSink,
        input_receiver: Receiver<String>,
    ) -> Self {
        UIMessenger {
            text_content,
            frame_content,
            cb_sink,
            input_receiver,
        }
    }

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
        use cursive::utils::span::IndexedSpan;
        use std::thread::sleep;
        use std::time::Instant;

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

    pub fn align_frame(&mut self, alignment: Alignment) {
        let _ = self.cb_sink.send(Box::new(move |s| {
            s.call_on_name(
                "frame-view",
                |v: &mut AlignedView<TextView>| match alignment {
                    Alignment::Center => v.set_center(),
                    Alignment::TopLeft => v.set_top_left(),
                },
            );
        }));
    }
}
