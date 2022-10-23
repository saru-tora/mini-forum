use crate::prelude::*;
use super::super::records::{Topic, topic::date};
use anansi::get_or_404;
use anansi::humanize::ago;
use anansi::handle;
use anansi::forms::ToRecord;
use anansi::util::auth::forms::UserLogin;
use anansi::{check, render};
use crate::forum::forms::TopicForm;
use anansi::handle_or_404;
use anansi::forms::ToEdit;

#[base_view]
fn base<R: Request>(_req: R) -> Result<Response> {}

#[record_view]
impl<R: Request> TopicView<R> {
    #[view(Group::is_visitor)]
    pub async fn index(req: R) -> Result<Response> {
        let title = "Latest Topics";
        let topics = Topic::order_by(date().desc())
            .limit(25).query(&req).await?;
    }
    #[view(Group::is_visitor)]
    pub async fn show(req: R) -> Result<Response> {
        let topic = get_or_404!(Topic, req);
        let title = &topic.title;
        let poster = topic.user.get(&req).await?.username;
        let comments = topic.recent_comments().limit(25).query(&req).await?;
        let users = comments.parents(&req, |c| &c.user).await?;
    }
    #[view(Group::is_visitor)]
    pub async fn login(mut req: R) -> Result<Response> {
        let title = "Log in";
        let button = "Log in";
        let form = handle!(UserLogin, ToRecord<R>, req, user, {
            req.auth(&user).await?;
    	    req.session().set_and_redirect(&req, Self::index)
        })?;
    }
    #[check(Group::is_auth)]
    pub async fn new(mut req: R) -> Result<Response> {
        let title = "New Topic";
        let button = "Create";
        let form = handle!(TopicForm, ToRecord<R>, req, |topic| {
    	    Ok(redirect!(req, Self::show, topic))
        })?;
        render!("login")
    }
    #[check(Topic::owner)]
    pub async fn edit(mut req: R) -> Result<Response> {
        let title = "Update Topic";
        let button = "Update";
        let form = handle_or_404!(TopicForm, ToEdit<R>, req, |topic| {
    	    Ok(redirect!(req, Self::show, topic))
        })?;
        render!("login")
    }
    #[view(Topic::owner)]
    pub async fn destroy(mut req: R) -> Result<Response> {
        let title = "Delete topic";
        let topic = get_or_404!(Topic, req);
        let form = handle!(req, R, {
            topic.delete(&req).await?;
            Ok(redirect!(req, Self::index))
        })?;
    }
}
