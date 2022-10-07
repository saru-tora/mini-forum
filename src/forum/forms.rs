use crate::prelude::*;
use anansi::models::{DateTime, ForeignKey};
use anansi::forms::{VarChar, ToModel};
use super::models::{Topic, Comment};
use anansi::web::FormMap;
use anansi::forms::{GetData, ToEdit};

#[form(Topic)]
pub struct TopicForm {
    pub title: VarChar<200>,
    pub content: VarChar<40000>,
}

#[async_trait]
impl<R: Request> ToModel<R> for TopicForm {
    async fn on_post(&mut self, data: TopicFormData, req: &R) -> Result<Topic> {
        let now = DateTime::now();
        transact!(req, {
            let topic = Topic::new(data.title, now).save(req).await
                .or(form_error!("Problem adding topic"))?;
            let topic_fk = ForeignKey::new(&topic);
            let user_fk = ForeignKey::new(req.user());
            Comment::new(topic_fk, user_fk, data.content, now).save(req).await
                .or(form_error!("Problem adding comment"))?;
            Ok(topic)
        })
    }
}

#[async_trait]
impl<R: Request> GetData<R> for TopicForm {
    fn from_map(form_map: FormMap) -> Result<TopicFormData> {
        let title = form_map.get("title")?.parse()?;
        let content = form_map.get("content")?.parse()?;
        Ok(TopicFormData::new(title, content))
    }
    async fn from_model(topic: Topic, req: &R) -> Result<TopicFormData> {
        let comment = topic.recent_comments().get(req).await?;
        Ok(TopicFormData::new(topic.title, comment.content))
    }
}

#[async_trait]
impl<R: Request> ToEdit<R> for TopicForm {
    async fn on_post(&mut self, data: TopicFormData, req: &R) -> Result<Topic> {
        let (mut topic, mut comment) = Topic::first_post(req).await?;
        topic.title = data.title;
        comment.content = data.content;
        transact!(req, {
            topic.update(req).await?;
            comment.update(req).await?;
            Ok(topic)
        })
    }
}
