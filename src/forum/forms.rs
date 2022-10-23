use crate::prelude::*;
use anansi::records::{DateTime, ForeignKey};
use anansi::forms::{VarChar, ToRecord};
use super::records::Topic;
use anansi::{GetData, ToEdit};

#[form(Topic)]
#[derive(GetData, ToEdit)]
pub struct TopicForm {
    pub title: VarChar<200>,
    pub content: VarChar<40000>,
}

#[async_trait]
impl<R: Request> ToRecord<R> for TopicForm {
    async fn on_post(&mut self, data: TopicFormData, req: &R) -> Result<Topic> {
        let now = DateTime::now();
        let user_fk = ForeignKey::from_data(req.user().pk())?;
        Topic::new(data.title, user_fk, data.content, now).save(req).await
            .or(form_error!("Problem adding topic"))
    }
}
