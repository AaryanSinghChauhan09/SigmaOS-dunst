//! dunst model — notification daemon (mako/dunst-style).

#[derive(Debug, Clone)]
pub struct Notification {
    pub app: String,
    pub summary: String,
    pub body: String,
    pub urgency: String,
    pub timeout: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub notifications: Vec<Notification>,
}

impl Default for Model {
    fn default() -> Self { Self { notifications: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn notify(&mut self, app: &str, summary: &str, body: &str, urgency: &str) -> bool {
        self.notifications.push(Notification {
            app: app.into(), summary: summary.into(), body: body.into(),
            urgency: urgency.into(), timeout: if urgency == "critical" { 0 } else { 5000 },
        });
        true
    }

    pub fn list(&self) -> &[Notification] { &self.notifications }

    pub fn dismiss(&mut self, index: usize) -> bool {
        if index < self.notifications.len() { self.notifications.remove(index); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn notify_dismiss() {
        let mut d = Model::new();
        assert!(d.notify("mail", "New mail", "You have mail", "normal"));
        assert_eq!(d.list().len(), 1);
        assert!(d.dismiss(0));
        assert!(d.list().is_empty());
    }
}
