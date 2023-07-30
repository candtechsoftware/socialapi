use notify_rust::Notification;

pub struct StoryNoticationService;

impl StoryNoticationService {
    pub async fn notify(user_id: i32) {
        match Notification::new()
            .summary(format!("User {:?} has created a new story", user_id).as_str())
            .show()
        {
            Ok(res) => {
                println!("{:?}", res)
            }
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
}
