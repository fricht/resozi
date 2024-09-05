use gtk::prelude::*;
use gtk4 as gtk;
mod slide_creator;

fn btn_listener_add_object(_: &gtk::Button, vbox: &gtk::Box) {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    container.append(&gtk::Label::new(Some("filename")));
    container.append(&gtk::Label::new(Some("id")));
    vbox.append(&container);
}

fn build_objects_page() -> impl IsA<gtk::Widget> {
    let overlay = gtk::Overlay::new();
    let scroll = gtk::ScrolledWindow::new();
    scroll.set_vexpand(true);
    scroll.set_hexpand(true);
    overlay.set_child(Some(&scroll));
    // mane page
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_valign(gtk::Align::Fill);
    vbox.set_halign(gtk::Align::Center);
    scroll.set_child(Some(&vbox));
    // add object btn
    let add_btn = gtk::Button::builder()
        .label("add")
        .halign(gtk::Align::End)
        .valign(gtk::Align::End)
        .margin_end(10)
        .margin_bottom(10)
        .width_request(50)
        .height_request(50)
        .build();
    add_btn.connect_clicked(move |b| btn_listener_add_object(b, &vbox));
    overlay.add_overlay(&add_btn);
    overlay
}

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Resozi")
        .build();

    // Create a vertical Box container
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Create the Stack and StackSwitcher
    let stack = gtk::Stack::new();
    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    // Add some pages to the Stack
    let button1 = gtk::Button::with_label("Page 1");
    stack.add_titled(&build_objects_page(), Some("files"), "Page 1");

    let button2 = gtk::Button::with_label("slides");
    stack.add_titled(&button2, Some("slides"), "Page 2");

    // append everything where it belongs
    vbox.append(&stack_switcher);
    vbox.append(&stack);
    window.set_child(Some(&vbox));
    // show window
    window.present();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.fricht.Resozi")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
