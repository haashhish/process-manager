use gtk4 as gtk;
use gtk::{prelude::*, Orientation, ScrolledWindow, TreeStore, SearchEntry, ToggleButton};
use gtk::{Application, ApplicationWindow, CellRendererText, TreeView, TreeViewColumn};
use sysinfo::{ProcessExt, System, SystemExt, ProcessStatus};
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

fn build_ui(app: &Application) {
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    let running_filter = gtk::ToggleButton::builder()
    .label("Running Processes")
    .build();

    let Sleeping_filter = gtk::ToggleButton::builder()
    .label("Sleeping Processes")
    .build();

    static mut flag:i32=0;

    fn SleepingFlag()
    {
        if(unsafe{flag == 2})
        {
            unsafe{flag=0;} 
        }
        else
        {
            unsafe{flag=2;}
        }
        // println!("{}",unsafe {
        //     flag
        // });
    }

    fn RunningFlag()
    {
        if(unsafe{flag == 1})
        {
            unsafe{flag=0;} 
        }
        else
        {
            unsafe{flag=1;}
        }
        // println!("{}",unsafe {
        //     flag
        // });
    }

     running_filter.connect_clicked(move |_| {
        unsafe{RunningFlag();}
    });

    Sleeping_filter.connect_clicked(move |_| {
        unsafe{SleepingFlag();}
    });

    let mut button2 = gtk::Button::builder()
        .label("Search")
        .build();

    let mut button = gtk::Button::builder()
        .label("Refresh processes")
        .build();

    let treeview = TreeView::new();
    treeview.set_activate_on_single_click(true);

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

   //column 7
   let parent_column = TreeViewColumn::new();
   parent_column.set_title("Parent ID");
   let parent_cell = CellRendererText::new();
   parent_column.pack_start(&parent_cell, true);
   parent_column.add_attribute(&parent_cell, "text", 6);
   parent_column.set_sort_column_id(6);

    // append columns to the table
    treeview.append_column(&id_column);
    treeview.append_column(&name_column);
    treeview.append_column(&uid_column);
    treeview.append_column(&parent_column);
    treeview.append_column(&mem_column);
    treeview.append_column(&cpu_column);
    treeview.append_column(&uname_column);

    // treeStore that stores the data to be added later in the table
    let mut store = gtk::TreeStore::new(&[String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type(),
        String::static_type()]);

    scrolled_window.set_child(Some(&treeview));
    treeview.set_model(Some(&store));

    //search bar
    //let search_entry = SearchEntry::new();
    scrolled_window.set_child(Some(&treeview));
    // scrolled_window.set_margin_bottom(0);
    let searchProcessField = gtk::SearchEntry::builder()
    // .margin_end(700)
    .build();

    let searchButton = gtk::Button::builder()
    .label("Search")
    .build();

    searchButton.connect_clicked(clone!(@weak store => move |_|{
        // let tree_iter = store.append(None);
        // let allData : Vec<Vec<String>> = fetch_process_data(f)
    }));

    //let grid = gtk::Grid::new();
    let searchBox = gtk::Box::builder()
    .orientation(Orientation::Horizontal)
    .build();
    searchBox.append(&searchProcessField);
    searchBox.append(&searchButton);


    let settingsBox = gtk::Box::builder()
    .orientation(Orientation::Horizontal)
    .build();

    // settingsBox.append(&button);
    settingsBox.append(&running_filter);
    settingsBox.append(&Sleeping_filter);

    let mainBox = gtk::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    mainBox.append(&settingsBox);
    mainBox.append(&scrolled_window);
    mainBox.append(&searchBox);
    // mainBox.append(&textEntry);
    // mainBox.append(&button);
    // mainBox.append(&button2);
    // grid.attach(&scrolled_window, 0, 0, 1, 1);
    // grid.attach(&textEntry, 1, 1, 0, 0);
    // grid.attach(&button, 0, 1, 1, 1);
    // button.activate();

    /////////////////////////
    /// 
    ///


    fn fillTable(store:TreeStore, all:Vec<Vec<String>>) -> TreeStore
    {
        store.clear();
        for row in &all 
        {
            let tree_iter = store.append(None);
            store.set_value(&tree_iter, 0, &row[0].to_value());
            store.set_value(&tree_iter, 1, &row[1].to_value());
            store.set_value(&tree_iter, 2, &row[2].to_value());
            store.set_value(&tree_iter, 3, &row[3].to_value());
            store.set_value(&tree_iter, 4, &row[4].to_value());
            store.set_value(&tree_iter, 5, &row[5].to_value());
            store.set_value(&tree_iter, 6, &row[6].to_value());
        }
        return store;
    }

    fn fetch_process_data(f:i32) -> Vec<Vec<String>> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut data = vec![];
        // println!("{}",f);
        // let num_cpus = num_cpus::get() as f32;
        for (pid, process) in sys.processes() {
            if (f==1)
            {
                if (process.status() == ProcessStatus::Run) 
                {
                    let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
                    let mut parentID:String = " ".to_string();
                    if(process.parent() != None)
                    {
                        // println!("{}",process.parent().unwrap().to_string());
                        parentID = process.parent().unwrap().to_string();
                    }
                    let row = vec![pid.to_string(), 
                    process.name().to_string(), 
                    process.user_id().unwrap().to_string(), 
                    process.memory().to_string(), 
                    process.cpu_usage().to_string(),
                    uName.unwrap().name().to_string_lossy().to_string(),
                    parentID.to_string()];
                    data.push(row);
                }
            }
            else if (f==2)
            {
                if (process.status() == ProcessStatus::Sleep) 
                {
                    let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
                    let mut parentID:String = " ".to_string();
                    if(process.parent() != None)
                    {
                        // println!("{}",process.parent().unwrap().to_string());
                        parentID = process.parent().unwrap().to_string();
                    }
                    let row = vec![pid.to_string(), 
                    process.name().to_string(), 
                    process.user_id().unwrap().to_string(), 
                    process.memory().to_string(), 
                    process.cpu_usage().to_string(),
                    uName.unwrap().name().to_string_lossy().to_string(),
                    parentID.to_string()];
                    data.push(row);
                }
            }
            else 
            {
                    let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
                    let mut parentID:String = " ".to_string();
                    if(process.parent() != None)
                    {
                        // println!("{}",process.parent().unwrap().to_string());
                        parentID = process.parent().unwrap().to_string();
                    }
                    let row = vec![pid.to_string(), 
                    process.name().to_string(), 
                    process.user_id().unwrap().to_string(), 
                    process.memory().to_string(), 
                    process.cpu_usage().to_string(),
                    uName.unwrap().name().to_string_lossy().to_string(),
                    parentID.to_string()];
                    data.push(row);
            }
        }
        return data;
    }

    //to change refresh rate
    // Spawn a new thread to fetch the process data and update the UI when it's available
    // button.connect_clicked(clone!(@weak store, @weak treeview => move |_| {
        let mut store = store.clone();
        // let treeview = treeview.clone();
        let handle = Arc::new(Mutex::new(Some(thread::spawn(move || fetch_process_data(unsafe{flag})))));
        let handle_clone = handle.clone();


        glib::timeout_add_local(Duration::from_millis(1000), move || {
            let mut handle = handle_clone.lock().unwrap();
            if let Some(handle_inner) = handle.take() {
                match handle_inner.join() {
                    Ok(result) => {
                        let mut temp = store.clone();
                        temp = fillTable(temp, result);
                        store = temp;
                    }
                    Err(_) => {
                        println!("Failed to fetch process data");
                    }
                }
                *handle = Some(thread::spawn(move || fetch_process_data(unsafe{flag})));
            }
            glib::Continue(true)
        });
    // }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("PCM")
        .default_width(900)
        .default_height(900)
        .child(&mainBox)
        .build();

    window.show();
}