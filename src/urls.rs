use anansi::web::prelude::*;
use crate::forum::{self, topic::views::TopicView};

routes! {
    path("/", TopicView::index),
    import!("/topic", forum),
    path("/login", TopicView::login),
}
