use gtk4 as gtk;
use gtk::{prelude::*, Orientation, ScrolledWindow, TreeStore};
use gtk::{Application, ApplicationWindow, CellRendererText, TreeView, TreeViewColumn};
use sysinfo::{ProcessExt, System, SystemExt};
use std::thread::sleep;
use std::time::{Duration, Instant};
use glib::source::timeout_add_local;
use glib::clone;
use std::sync::{Arc, Mutex};
use std::thread;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";


fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let button = gtk::Button::builder()
    .label("Refresh processes")
    // .margin_start(5)
    .build();
    let editT = gtk::Entry::builder()
    .build();

    let treeview = TreeView::new();

    // create column 2
    let id_column = TreeViewColumn::new();
    id_column.set_title("Process ID");
    let id_cell = CellRendererText::new();
    id_column.pack_start(&id_cell, true);
    id_column.add_attribute(&id_cell, "text", 0);
    
    // create column 1
    let name_column = TreeViewColumn::new();
    name_column.set_title("Process Name");
    let name_cell = CellRendererText::new();
    name_column.pack_start(&name_cell, true);
    name_column.add_attribute(&name_cell, "text", 1);


    // append columns to the table
    treeview.append_column(&id_column);
    treeview.append_column(&name_column);
    
    
    // treeStore that stores the data to be added later in the table
    let mut store = gtk::TreeStore::new(&[String::static_type(), String::static_type()]);

    // add data
    let mut sys = System::new();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let row = store.append(None);
        let t = pid.to_string();
        store.set_value(&row, 0, &t.to_value());
        store.set_value(&row, 1, &process.name().to_value());
    }
    
    
    scrolled_window.set_child(Some(&treeview));
    treeview.set_model(Some(&store));

    let mut gtk_box = gtk::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    gtk_box.append(&scrolled_window);
    // gtk_box.append(&editT);
    gtk_box.append(&button);


    button.connect_clicked(move |_|{
        store.clear();
        let mut sys = System::new();
        sys.refresh_all();
        for (pid, process) in sys.processes()
        {
            let row = store.append(None);
            let t = pid.to_string();
            store.set_value(&row, 0, &t.to_value());
            store.set_value(&row, 1, &process.name().to_value());
        }
        treeview.show();
    }
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PCM")
        .default_width(500)
        .default_height(500)
        // .resizable(true)
        .child(&gtk_box)
        .build();
    window.present();
}