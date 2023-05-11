use std::mem;

pub mod channel_manager;
pub mod connection_manager;
pub mod text_message_manager;
pub mod user_manager;
pub mod voice_manager;

trait Update<New> {
    fn update_if_some<T: Default>(original: &mut T, other: &mut Option<T>) {
        if let Some(id) = other {
            *original = mem::take(id);
        }
    }

    fn update_from(&mut self, new_state: &mut New) -> &Self;
}
