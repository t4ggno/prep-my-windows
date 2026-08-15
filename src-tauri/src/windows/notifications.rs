use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::TypedEventHandler;
use windows::UI::Notifications::{
    ToastActivatedEventArgs, ToastNotification, ToastNotificationManager,
};
use windows::core::{HSTRING, IInspectable, Interface, Ref};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessNotificationAction {
    AllowOnce,
    AlwaysAllow,
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn process_notification_xml(app_name: &str) -> String {
    format!(
        r#"<toast duration="long"><visual><binding template="ToastGeneric"><text>Stopped {}</text></binding></visual><actions><action content="Allow Once" arguments="allow-once" activationType="foreground"/><action content="Always Allow" arguments="always-allow" activationType="foreground"/></actions></toast>"#,
        escape_text(app_name)
    )
}

pub fn show_process_stopped(
    app_id: &str,
    app_name: &str,
    on_action: impl Fn(ProcessNotificationAction) + Send + 'static,
) -> Result<(), String> {
    let document = XmlDocument::new().map_err(notification_error)?;
    let xml = process_notification_xml(app_name);
    document
        .LoadXml(&HSTRING::from(xml))
        .map_err(notification_error)?;
    let toast =
        ToastNotification::CreateToastNotification(&document).map_err(notification_error)?;
    let handler = TypedEventHandler::<ToastNotification, IInspectable>::new(
        move |_toast: Ref<'_, ToastNotification>, args: Ref<'_, IInspectable>| {
            let Some(arguments) = args
                .as_ref()
                .and_then(|args| args.cast::<ToastActivatedEventArgs>().ok())
                .and_then(|args| args.Arguments().ok())
            else {
                return Ok(());
            };
            match arguments.to_string().as_str() {
                "allow-once" => on_action(ProcessNotificationAction::AllowOnce),
                "always-allow" => on_action(ProcessNotificationAction::AlwaysAllow),
                _ => {}
            }
            Ok(())
        },
    );
    toast.Activated(&handler).map_err(notification_error)?;
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(app_id))
        .map_err(notification_error)?;
    notifier.Show(&toast).map_err(notification_error)
}

fn notification_error(error: windows::core::Error) -> String {
    format!("Could not show process notification: {error}")
}

#[cfg(test)]
mod tests {
    use windows::Data::Xml::Dom::XmlDocument;
    use windows::core::HSTRING;

    use super::{ProcessNotificationAction, escape_text, process_notification_xml};

    #[test]
    fn process_notification_actions_are_distinct() {
        assert_ne!(
            ProcessNotificationAction::AllowOnce,
            ProcessNotificationAction::AlwaysAllow
        );
    }

    #[test]
    fn notification_text_is_xml_safe() {
        assert_eq!(escape_text("A & B <C>"), "A &amp; B &lt;C&gt;");
    }

    #[test]
    fn process_notification_xml_contains_both_actions() {
        let xml = process_notification_xml("A & B");
        let document = XmlDocument::new().unwrap();

        document.LoadXml(&HSTRING::from(&xml)).unwrap();

        assert!(xml.contains("content=\"Allow Once\""));
        assert!(xml.contains("content=\"Always Allow\""));
    }
}
