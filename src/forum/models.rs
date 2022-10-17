use anansi::{model, Relate, FromParams};
use anansi::models::{VarChar, DateTime, ForeignKey};
use anansi::util::auth::models::User;
use anansi::db::OrderBy;
use anansi::ToUrl;

#[model]
#[derive(Relate, FromParams, ToUrl)]
pub struct Topic {
    pub title: VarChar<200>,
    #[field(app = "auth")]
    pub user: ForeignKey<User>,
    pub content: VarChar<40000>,
    pub date: DateTime,
}

impl Topic {
    pub fn recent_comments(&self) -> OrderBy<Comment> {
        Comment::by_topic(self).order_by(comment::date().desc())
    }
}

#[model]
#[derive(Relate, FromParams)]
pub struct Comment {
    pub topic: ForeignKey<Topic>,
    #[field(app = "auth")]
    pub user: ForeignKey<User>,
    pub content: VarChar<40000>,
    pub date: DateTime,
}
