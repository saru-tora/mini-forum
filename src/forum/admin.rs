use anansi::{init_admin, register, model_admin};
use super::models::Topic;

init_admin! {
    register!(Topic),
}

model_admin!(Topic {});
