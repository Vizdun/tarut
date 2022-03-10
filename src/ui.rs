use druid::keyboard_types::Key;
use druid::widget::{Controller, CrossAxisAlignment, Flex, Label, List, Scroll, TextBox};
use druid::{Color, Env, Event, EventCtx, UnitPoint, Widget, WidgetExt};

use crate::data::{AppData, Message};
use crate::delegate::SEND_MESSAGE;

struct MessageTextBox;

impl<W: Widget<String>> Controller<String, W> for MessageTextBox {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut String,
        env: &Env,
    ) {
        if let Event::WindowConnected = event {
            ctx.request_focus();
        }
        if let Event::KeyDown(keyev) = event {
            if keyev.key == Key::Enter {
                ctx.submit_command(SEND_MESSAGE);
            }
        }
        child.event(ctx, event, data, env)
    }
}

pub fn ui_builder() -> impl Widget<AppData> {
    let mut root = Flex::column();

    root.add_child(
        TextBox::new()
            .with_placeholder("address")
            .padding(5.0)
            .expand_width()
            .lens(AppData::writing_address),
    );

    let mut lists = Flex::row().cross_axis_alignment(CrossAxisAlignment::Start);

    // Build a simple list
    lists.add_flex_child(
        Scroll::new(List::new(|| {
            let mut col = Flex::column();

            col.add_child({
                let mut label = Label::new(|msg: &Message, _env: &_| format!("{}", msg.author))
                    .with_line_break_mode(druid::widget::LineBreaking::Clip);
                label.set_text_size(10.0);
                label.expand_width().align_vertical(UnitPoint::LEFT)
            });

            col.add_child(
                Label::new(|msg: &Message, _env: &_| format!("{}", msg.content))
                    .expand_width()
                    .align_vertical(UnitPoint::LEFT),
            );

            col.background(Color::rgb(0.5, 0.5, 0.5))
                .padding(5.0)
                .expand_width()
        }))
        .vertical()
        .expand_height()
        .lens(AppData::messages),
        1.0,
    );

    root.add_flex_child(lists, 1.0);

    root.add_child(
        TextBox::new()
            .with_placeholder("message")
            .controller(MessageTextBox)
            .padding(5.0)
            .expand_width()
            .lens(AppData::writing_message),
    );

    root
}
