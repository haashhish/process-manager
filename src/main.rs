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
use glib::source::Continue;
use std::borrow::Borrow;
use users::{get_current_uid, get_user_by_uid};
use std::sync::mpsc;
const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

// fn build_ui(app: &Application) {
//     let scrolled_window = ScrolledWindow::new();
//     scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
//     scrolled_window.set_vexpand(true);
//     scrolled_window.set_hexpand(true);

//     let button = gtk::Button::builder()
//         .label("Refresh processes")
//         .build();

//     let treeview = TreeView::new();

//     // create column 2
//     let id_column = TreeViewColumn::new();
//     id_column.set_title("Process ID");
//     let id_cell = CellRendererText::new();
//     id_column.pack_start(&id_cell, true);
//     id_column.add_attribute(&id_cell, "text", 0);

//     // create column 1
//     let name_column = TreeViewColumn::new();
//     name_column.set_title("Process Name");
//     let name_cell = CellRendererText::new();
//     name_column.pack_start(&name_cell, true);
//     name_column.add_attribute(&name_cell, "text", 1);

//     // // create column 3
//     // let uid = get_current_uid();
//     // let uid_column = TreeViewColumn::new();
//     // uid_column.set_title("UID");
//     // let uid_cell = CellRendererText::new();
//     // uid_column.pack_start(&uid_cell, true);
//     // uid_column.add_attribute(&uid_cell, "text", 1);

//     // append columns to the table
//     treeview.append_column(&id_column);
//     treeview.append_column(&name_column);
//     //treeview.append_column(&uid_column);

//     // treeStore that stores the data to be added later in the table
//     let mut store = gtk::TreeStore::new(&[String::static_type(), String::static_type()]);

//     scrolled_window.set_child(Some(&treeview));
//     treeview.set_model(Some(&store));

//     let grid = gtk::Grid::new();
//     grid.attach(&scrolled_window, 0, 0, 1, 1);
//     grid.attach(&button, 0, 1, 1, 1);

//     let mut gtk_box = gtk::Box::builder()
//         .orientation(Orientation::Vertical)
//         .build();
//     gtk_box.append(&scrolled_window);
//     gtk_box.append(&button);

//     button.connect_clicked(clone!(@weak store, @weak treeview => move |_| {
//         let store = store.clone();
//         let treeview = treeview.clone();
//         let mut handle = Some(thread::spawn(move || {
//             let mut sys = System::new();
//             sys.refresh_all();
//             let mut data = vec![];
//             for (pid, process) in sys.processes() {
//                 let row = vec![pid.to_string(), process.name().to_string()];
//                 data.push(row);
//             }
//             data
//         }));
//         let closure: Box<dyn FnMut() -> glib::Continue + 'static> = Box::new(move || {
//             //let handle = handle.clone();
//             let handle = handle.take().unwrap();
//             let result = handle.join().unwrap();
//             store.clear();
//             for row in result {
//                 let row = store.append(None);
//                 store.set_value(&row, 0, &store.get_value(&row, 0).get::<String>().expect("REASON").to_value());
//                 store.set_value(&row, 1, &store.get_value(&row, 1).get::<String>().expect("REASON").to_value());
//             }
//             treeview.show();
//             glib::Continue(true)
//         });
//         glib::timeout_add_local(Duration::from_millis(100), closure);
//     }));

//     let window = ApplicationWindow::builder()
//         .application(app)
//         .title("PCM")
//         .default_width(500)
//         .default_height(500)
//         .child(&gtk_box)
//         .build();
//     window.present();
// }
fn build_ui(app: &Application) {
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let button = gtk::Button::builder()
        .label("Refresh processes")
        .build();

    let treeview = TreeView::new();

   // create column 1
   let id_column = TreeViewColumn::new();
   id_column.set_title("Process ID");
   let id_cell = CellRendererText::new();
   id_column.pack_start(&id_cell, true);
   id_column.add_attribute(&id_cell, "text", 0);
   id_column.set_sort_column_id(0);

   
   // create column 2
   let name_column = TreeViewColumn::new();
   name_column.set_title("Process Name");
   let name_cell = CellRendererText::new();
   name_column.pack_start(&name_cell, true);
   name_column.add_attribute(&name_cell, "text", 1);
   name_column.set_sort_column_id(1);

   // create column 3
   let uid_column = TreeViewColumn::new();
   uid_column.set_title("UID");
   let uid_cell = CellRendererText::new();
   uid_column.pack_start(&uid_cell, true);
   uid_column.add_attribute(&uid_cell, "text", 2);
   uid_column.set_sort_column_id(2);

   //column 4
   let mem_column = TreeViewColumn::new();
   mem_column.set_title("Memory usage (Bytes)");
   let mem_cell = CellRendererText::new();
   mem_column.pack_start(&mem_cell, true);
   mem_column.add_attribute(&mem_cell, "text", 3);
   mem_column.set_sort_column_id(3);

   //column 5
   let cpu_column = TreeViewColumn::new();
   cpu_column.set_title("CPU Usage (%)");
   let cpu_cell = CellRendererText::new();
   cpu_column.pack_start(&cpu_cell, true);
   cpu_column.add_attribute(&cpu_cell, "text", 4);
   cpu_column.set_sort_column_id(4);

   //column 6
   let uname_column = TreeViewColumn::new();
   uname_column.set_title("User");
   let uname_cell = CellRendererText::new();
   uname_column.pack_start(&uname_cell, true);
   uname_column.add_attribute(&uname_cell, "text", 5);
   uname_column.set_sort_column_id(5);

    // append columns to the table
    treeview.append_column(&id_column);
    treeview.append_column(&name_column);
    treeview.append_column(&uid_column);
    treeview.append_column(&mem_column);
    treeview.append_column(&cpu_column);
    treeview.append_column(&uname_column);

    // treeStore that stores the data to be added later in the table
    let mut store = gtk::TreeStore::new(&[String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type()]);

    scrolled_window.set_child(Some(&treeview));
    treeview.set_model(Some(&store));

    let grid = gtk::Grid::new();
    grid.attach(&scrolled_window, 0, 0, 1, 1);
    // button.activate();
    grid.attach(&button, 0, 1, 1, 1);
    // button.connect_clicked(clone!(@weak store, @weak treeview => move |_| {
    //     let store = store.clone();
    //     let treeview = treeview.clone();
    //     let mut handle = Arc::new(Mutex::new(Some(thread::spawn(move || {
    //         let mut sys = System::new();
    //         sys.refresh_all();
    //         let mut data = vec![];
    //         for (pid, process) in sys.processes() {
    //             let row = vec![process.name().to_string(), pid.to_string()];
    //             data.push(row);
    //         }
    //         data
    //     }))));
    //     let handle_clone = handle.clone();
    //     // if handle.is_none() {
    //     //     println!("Failed to start thread");
    //     // }
    //     glib::timeout_add_local(Duration::from_millis(100), move || {
    //         let result = handle_clone.lock().unwrap().take().unwrap().join().unwrap();
    //         store.clear();
    //         for row in &result {
    //             let tree_iter = store.append(None);
    //             store.set_value(&tree_iter, 0, &row[0].to_value());
    //             store.set_value(&tree_iter, 1, &row[1].to_value());
    //         }
    //         treeview.show();
    //         glib::Continue(true)
    //     });
    // }));

    /////////////////////////
    fn fetch_process_data() -> Vec<Vec<String>> {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200)); //refresh 3 times to get more accurate results
        sys.refresh_cpu();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_cpu();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_cpu();
        let mut data = vec![];
        let num_cpus = num_cpus::get() as f32;
        
        
        for (pid, process) in sys.processes() {
            let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
            let row = vec![pid.to_string(), 
            process.name().to_string(), 
            process.user_id().unwrap().to_string(), 
            process.memory().to_string(), 
            (process.cpu_usage()/num_cpus).to_string(), 
            uName.unwrap().name().to_string_lossy().to_string()];

            data.push(row);
        }
        data
    }
    // Spawn a new thread to fetch the process data and update the UI when it's available
    button.connect_clicked(clone!(@weak store, @weak treeview => move |_| {
        let store = store.clone();
        let treeview = treeview.clone();
        let handle = Arc::new(Mutex::new(Some(thread::spawn(move || fetch_process_data()))));
        let handle_clone = handle.clone();
        glib::timeout_add_local(Duration::from_millis(1000), move || {
            let mut handle = handle_clone.lock().unwrap();
            if let Some(handle_inner) = handle.take() {
                match handle_inner.join() {
                    Ok(result) => {
                        store.clear();
                        for row in &result {
                            let tree_iter = store.append(None);
                            store.set_value(&tree_iter, 0, &row[0].to_value());
                            store.set_value(&tree_iter, 1, &row[1].to_value());
                            store.set_value(&tree_iter, 2, &row[2].to_value());
                            store.set_value(&tree_iter, 3, &row[3].to_value());
                            store.set_value(&tree_iter, 4, &row[4].to_value());
                            store.set_value(&tree_iter, 5, &row[5].to_value());

                        }
                        // treeview.show();
                    }
                    Err(_) => {
                        println!("Failed to fetch process data");
                    }
                }
                *handle = Some(thread::spawn(move || fetch_process_data()));
            }
            glib::Continue(true)
        });
    }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PCM")
        .default_width(900)
        .default_height(900)
        .child(&grid)
        .build();

    window.show();
}