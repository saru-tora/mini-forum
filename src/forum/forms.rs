use crate::prelude::*;
use anansi::models::{DateTime, ForeignKey};
use anansi::forms::{VarChar, ToModel};
use super::models::Topic;
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
        let user_fk = ForeignKey::from_data(req.user().pk())?;
        Topic::new(data.title, user_fk, data.content, now).save(req).await
            .or(form_error!("Problem adding topic"))
    }
}

#[async_trait]
impl<R: Request> GetData<R> for TopicForm {
    fn from_map(form_map: FormMap) -> Result<TopicFormData> {
        let title = form_map.get("title")?.parse()?;
        let content = form_map.get("content")?.parse()?;
        Ok(TopicFormData::new(title, content))
    }
    async fn from_model(topic: Topic, _req: &R) -> Result<TopicFormData> {
        Ok(TopicFormData::new(topic.title, topic.content))
    }
}

#[async_trait]
impl<R: Request> ToEdit<R> for TopicForm {
    async fn on_post(&mut self, data: TopicFormData, req: &R) -> Result<Topic> {
	let mut topic: Topic = req.get_model().await?;
        topic.title = data.title;
        topic.content = data.content;
        topic.update(req).await?;
        Ok(topic)
    }
}
