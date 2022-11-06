use anansi::{record, Relate, FromParams};
use anansi::records::{VarChar, DateTime, ForeignKey};
use anansi::util::auth::records::User;
use serde::{Serialize, Deserialize};
use anansi::db::OrderBy;
use anansi::ToUrl;

#[record]
#[derive(Relate, FromParams, Serialize, Deserialize, ToUrl)]
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

#[record]
#[derive(Relate, FromParams)]
pub struct Comment {
    pub topic: ForeignKey<Topic>,
    #[field(app = "auth")]
    pub user: ForeignKey<User>,
    pub content: VarChar<40000>,
    pub date: DateTime,
}
