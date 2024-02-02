mod about;
mod email;
mod recipe;
mod traceroute;
mod settings;
mod notifications;
mod dashboard;

pub use dashboard::DashboardTab;
pub use about::AboutTab;
pub use email::EmailTab;
pub use recipe::RecipeTab;
pub use traceroute::TracerouteTab;
pub use settings::SettingsTab;
pub use notifications::NotificationsTab;