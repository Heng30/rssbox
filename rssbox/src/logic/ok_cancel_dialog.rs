use crate::slint_generatedAppWindow::{AppWindow, Logic, Store};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Logic>()
        .on_handle_ok_cancel_dialog(move |handle_type, handle_uuid| {
            let ui = ui_handle.unwrap();

            if handle_type.as_str() == "rss" {
                ui.global::<Logic>().invoke_delete_rss(handle_uuid);
            // } else if handle_type.as_str() == "session-item" {
            //     ui.global::<Logic>().invoke_delete_session(handle_uuid);
            // } else if handle_type.as_str() == "session-reset" {
            //     ui.global::<Logic>().invoke_reset_current_session();
            // } else if handle_type.as_str() == "session-archive-item" {
            //     ui.global::<Logic>().invoke_delete_session_archive(
            //         ui.global::<Store>().get_current_session_uuid(),
            //         handle_uuid,
            //     );
            }
        });
}
