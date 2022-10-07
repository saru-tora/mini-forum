use crate::prelude::*;
use super::super::models::{Topic, topic::date};
use anansi::get_or_404;
use anansi::humanize::ago;
use anansi::handle;
use anansi::forms::ToModel;
use anansi::util::auth::forms::UserLogin;
use anansi::checker;
use crate::forum::forms::TopicForm;
use anansi::handle_or_404;
use anansi::forms::ToEdit;

checker!(if_auth<R: Request>, |req| req.check_auth(),
    redirect!(req, TopicView::login)
);

#[base_view]
fn base<R: Request>(_req: R) -> Result<Response> {}

#[viewer]
impl<R: Request> TopicView<R> {
    #[view(if_guest)]
    pub async fn index(req: R) -> Result<Response> {
        let title = "Latest Topics";
        let topics = Topic::order_by(date().desc())
            .limit(25).query(&req).await?;
    }
    #[view(if_guest)]
    pub async fn show(req: R) -> Result<Response> {
        let topic = get_or_404!(Topic, req);
        let title = &topic.title;
        let comments = topic.recent_comments().limit(25).query(&req).await?;
        let users = comments.parents(|c| &c.user).query(&req).await?;
    }
    #[view(if_guest)]
    pub async fn login(mut req: R) -> Result<Response> {
        let title = "Log in";
        let button = "Log in";
        let form = handle!(UserLogin, ToModel<R>, req, user, {
            req.auth(&user).await?;
    	    req.session().set_and_redirect(&req, Self::index)
        })?;
    }
    #[view(if_auth)]
    pub async fn new(mut req: R) -> Result<Response> {
        let title = "New Topic";
        let button = "Create";
        let form = handle!(TopicForm, ToModel<R>, req, |topic| {
    	    Ok(redirect!(req, Self::show, topic))
        })?;
    }
    #[view(if_auth)]
    pub async fn edit(mut req: R) -> Result<Response> {
        let title = "Update Topic";
        let button = "Update";
        let form = handle_or_404!(TopicForm, ToEdit<R>, req, |topic| {
    	    Ok(redirect!(req, Self::show, topic))
        })?;
    }
    #[view(if_auth)]
    pub async fn destroy(mut req: R) -> Result<Response> {
        let title = "Delete topic";
        let topic = get_or_404!(Topic, req);
        let form = handle!(req, R, {
            topic.delete(&req).await?;
            Ok(redirect!(req, Self::index))
        })?;
    }
}
