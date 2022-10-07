use anansi::web::prelude::*;
use super::topic::views::TopicView;

routes! {
    path("new", TopicView::new),
    path("{topic_id}", TopicView::show),
    path("{topic_id}/edit", TopicView::edit),
    path("{topic_id}/destroy", TopicView::destroy),
}
