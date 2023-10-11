use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::{prelude::*, PlayerWithCallStatus};
use crate::{
    theme, Avatar, Button, Icon, IconButton, IconColor, PlayerStack, ToolDivider, TrafficLights,
};

#[derive(Clone)]
pub struct Livestream {
    pub players: Vec<PlayerWithCallStatus>,
    pub channel: Option<String>, // projects
                                 // windows
}

#[derive(Element)]
pub struct TitleBar<V: 'static> {
    view_type: PhantomData<V>,
    /// If the window is active from the OS's perspective.
    is_active: Arc<AtomicBool>,
    livestream: Option<Livestream>,
}

impl<V: 'static> TitleBar<V> {
    pub fn new(cx: &mut ViewContext<V>) -> Self {
        let is_active = Arc::new(AtomicBool::new(true));
        let active = is_active.clone();

        cx.observe_window_activation(move |_, is_active, cx| {
            active.store(is_active, std::sync::atomic::Ordering::SeqCst);
            cx.notify();
        })
        .detach();

        Self {
            view_type: PhantomData,
            is_active,
            livestream: None,
        }
    }

    pub fn set_livestream(mut self, livestream: Option<Livestream>) -> Self {
        self.livestream = livestream;
        self
    }

    fn render(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);
        let has_focus = cx.window_is_active();

        let player_list = if let Some(livestream) = &self.livestream {
            livestream.players.clone().into_iter()
        } else {
            vec![].into_iter()
        };

        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h_8()
            .fill(theme.lowest.base.default.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_full()
                    .gap_4()
                    .px_2()
                    .child(TrafficLights::new().window_has_focus(has_focus))
                    // === Project Info === //
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(Button::new("zed"))
                            .child(Button::new("nate/gpui2-ui-components")),
                    )
                    .children(player_list.map(|p| PlayerStack::new(p)))
                    .child(IconButton::new(Icon::Plus)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(IconButton::new(Icon::FolderX))
                            .child(IconButton::new(Icon::Close)),
                    )
                    .child(ToolDivider::new())
                    .child(
                        div()
                            .px_2()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(IconButton::new(Icon::Mic))
                            .child(IconButton::new(Icon::AudioOn))
                            .child(IconButton::new(Icon::Screen).color(IconColor::Accent)),
                    )
                    .child(
                        div().px_2().flex().items_center().child(
                            Avatar::new("https://avatars.githubusercontent.com/u/1714999?v=4")
                                .shape(Shape::RoundedRectangle),
                        ),
                    ),
            )
    }
}