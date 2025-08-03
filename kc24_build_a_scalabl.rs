use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct AutomationScriptNotifier {
    scripts: HashMap<String, Vec<String>>,
    notification_channels: HashMap<String, String>,
    notification_queue: Arc<Mutex<Vec<(String, String)>>>,
}

impl AutomationScriptNotifier {
    fn new() -> Self {
        AutomationScriptNotifier {
            scripts: HashMap::new(),
            notification_channels: HashMap::new(),
            notification_queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn add_script(&mut self, script_name: String, notification_channel: String) {
        self.scripts.entry(script_name).or_insert(Vec::new()).push(notification_channel.clone());
        self.notification_channels.insert(script_name.clone(), notification_channel);
    }

    fn notify(&self, script_name: &str, message: &str) {
        let notification_queue = self.notification_queue.clone();
        thread::spawn(move || {
            let mut queue = notification_queue.lock().unwrap();
            queue.push((script_name.to_string(), message.to_string()));
        });
    }

    fn process_notifications(&self) {
        let queue = self.notification_queue.clone();
        let mut queue_lock = queue.lock().unwrap();
        while let Some((script_name, message)) = queue_lock.pop() {
            if let Some(channel) = self.notification_channels.get(&script_name) {
                // Send notification to channel
                println!("Sending notification to {} for script {}: {}", channel, script_name, message);
            }
        }
    }
}

fn main() {
    let mut notifier = AutomationScriptNotifier::new();

    notifier.add_script("script1".to_string(), "email@example.com".to_string());
    notifier.add_script("script2".to_string(), "slack_channel".to_string());

    notifier.notify("script1", "Script 1 completed successfully!");
    notifier.notify("script2", "Script 2 failed with errors.");

    notifier.process_notifications();
}