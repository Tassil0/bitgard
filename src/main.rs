use adw::prelude::*;

use adw::{ActionRow, Application, ApplicationWindow, gio, HeaderBar, Window};
use gtk::{Box, ListBox, Orientation, SelectionMode};

const APP_ID: &str = "com.tassilobalbo.bitgard";

fn main() {
    // Register and include resources
    gio::resources_register_include!("bitgard.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    /*app.connect_activate(|app| {
        let mut rows: Vec<ActionRow> = Vec::new();

        for i in 0..10 {
            rows.push(ActionRow::builder()
                            .activatable(true)
                            .title("Click me")
                            .build());
        }

        // ActionRows are only available in Adwaita
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();

        for i in 0..10 {
            list.append(&rows[i]);
        }

        let header = HeaderBar::builder()
            .title("ahojki")
            .show_close_button(true)
            .build();

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("First App")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });*/

    app.run();
}

fn build_ui(app: &Application) {
    let window = Window::new();
    //window.show();
    window.present();
}