use anansi::model;
use anansi::models::{VarChar, DateTime, ForeignKey};
use anansi::util::auth;
use anansi::db::OrderBy;
use anansi::{ToUrl, FromParams};
use anansi::web::Result;
use crate::project::Request;

#[model]
#[derive(ToUrl, FromParams)]
pub struct Topic {
    pub title: VarChar<200>,
    pub date: DateTime,
}

impl Topic {
    pub fn recent_comments(&self) -> OrderBy<Comment> {
        Comment::by_topic(self).order_by(comment::created().desc())
    }
    pub async fn first_post<R: Request>(req: &R) -> Result<(Self, Comment)> {
        let topic: Self = req.to_model().await?;
        let comment = topic.recent_comments().get(req).await?;
        Ok((topic, comment))
    }
}

#[model]
pub struct Comment {
    pub topic: ForeignKey<Topic>,
    pub user: ForeignKey<auth::models::User>,
    pub content: VarChar<40000>,
    pub created: DateTime,
}
