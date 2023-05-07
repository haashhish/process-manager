use gio::ffi::GListModel;
use gtk4 as gtk;
use gtk::{prelude::*, Orientation, ScrolledWindow, TreeStore, SearchEntry, ToggleButton, ScrollablePolicy};
use gtk::{Application, ApplicationWindow, CellRendererText, TreeView, TreeViewColumn};
use sysinfo::{ProcessExt, System, SystemExt, ProcessStatus, CpuExt, ProcessRefreshKind, Pid};
use std::thread::sleep;
use std::time::{Duration, Instant};
use glib::source::timeout_add_local;
use glib::clone;
use std::sync::{Arc, Mutex};
use std::thread;
use glib::source::Continue;
use std::borrow::{Borrow, BorrowMut};
use users::{get_current_uid, get_user_by_uid};
use std::sync::mpsc;
use std::cell::RefCell;
use crate::{AdjustmentExt};
extern crate uptime_lib;
const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}


#[derive(Clone)]
struct proc
{
    procID : i32,
    procName : String,
    UID : i32,
    parentID : i32,
    memUsage : u64,
    cpuUsage : f64,
    user : String,
}

static mut refreshRate:u64 = 4000;


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
    let clone1:TreeView = treeview.clone();
    let clone2:TreeView = treeview.clone();
    let clone3:TreeView = treeview.clone();


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
   let mem_clone:TreeViewColumn = mem_column.clone();
   let mem_clone2:TreeViewColumn = mem_column.clone();

   //column 5
   let cpu_column = TreeViewColumn::new();
   cpu_column.set_title("CPU Usage (%)");
   let cpu_cell = CellRendererText::new();
   cpu_column.pack_start(&cpu_cell, true);
   cpu_column.add_attribute(&cpu_cell, "text", 4);
   cpu_column.set_sort_column_id(4);
   let cpu_clone : TreeViewColumn = cpu_column.clone();
   let cpu_clone2 : TreeViewColumn = cpu_column.clone();
   let cpu_clone3 : TreeViewColumn = cpu_column.clone();



   //column 6
   let uname_column = TreeViewColumn::new();
   uname_column.set_title("User");
   let uname_cell = CellRendererText::new();
   uname_column.pack_start(&uname_cell, true);
   uname_column.add_attribute(&uname_cell, "text", 5);
   uname_column.set_sort_column_id(5);
   let uname_clone:TreeViewColumn = uname_column.clone();
   let uname_clone2:TreeViewColumn = uname_column.clone();
   let uname_clone3:TreeViewColumn = uname_column.clone();



   //column 7
   let parent_column = TreeViewColumn::new();
   parent_column.set_title("Parent ID");
   let parent_cell = CellRendererText::new();
   parent_column.pack_start(&parent_cell, true);
   parent_column.add_attribute(&parent_cell, "text", 6);
   parent_column.set_sort_column_id(6);
   let parentClone:TreeViewColumn = parent_column.clone();
   

    // append columns to the table
    treeview.append_column(&id_column);
    treeview.append_column(&name_column);
    treeview.append_column(&uid_column);
    treeview.append_column(&parent_column);
    treeview.append_column(&mem_column);
    treeview.append_column(&cpu_column);
    treeview.append_column(&uname_column);

    // treeStore that stores the data to be added later in the table
    let mut store = gtk::TreeStore::new(
        &[i32::static_type(),
        String::static_type(),
        i32::static_type(),
        u64::static_type(),
        f32::static_type(),
        String::static_type(),
        i32::static_type()]);
    let v = scrolled_window.vadjustment();
    let h = scrolled_window.hadjustment();
    scrolled_window.set_vadjustment(Some(&v));
    scrolled_window.set_hadjustment(Some(&h));
    scrolled_window.set_child(Some(&treeview));
    treeview.set_model(Some(&store));

    //search bar
    //let search_entry = SearchEntry::new();
    scrolled_window.set_child(Some(&treeview));
    // scrolled_window.set_margin_bottom(0);
    let searchProcessField = gtk::SearchEntry::builder()
    // .margin_end(700)
    .build();

    let killLabel = gtk::Label::builder()
    .label("    Enter process PID:")
    .build();
    let killLabel_clone = killLabel.clone();
    
    let killField = gtk::Entry::builder()
    .build();

    let killField_clone = killField.clone();

    let killButton = gtk::Button::builder()
    .label("Kill process")
    .build();

    let killResult = gtk::Label::builder()
    .visible(false)
    .build();
    let killResult_clone = killResult.clone();

    let refreshRateField = gtk::Entry::builder()
    // .margin_start(500)
    // .margin_end(500)
    .width_request(100)
    .build();

    let searchButton = gtk::Button::builder()
    .label("Search")
    .build();
    let searchField = searchProcessField.clone();
    
    let searchResult = gtk::Label::builder()
    .visible(false)
    .build();

    let searchRes_clone = searchResult.clone();

    let setRefreshRate = gtk::Button::builder()
    .label("Update refresh rate")
    .build();

    let searchByLabel = gtk::Label::builder()
    .label("  Search by PID:")
    .build();

    let searchBox = gtk::Box::builder()
    .orientation(Orientation::Horizontal)
    .build();

    // let mut refresh_rate = RefCell::new(unsafe { refreshRate });
    let mut rValue = unsafe{refreshRate};

    fn updateRefreshRate(value:u64)
    {
        unsafe{refreshRate = value;};
    }

    setRefreshRate.connect_clicked(clone!(@weak refreshRateField => move |_|{
        let rRfield = refreshRateField.clone();
        let rR = rRfield.text().to_string();
        let value = rR.parse::<u64>().unwrap();
        updateRefreshRate(value);
        // thread::park();
        println!("{}",unsafe{refreshRate});
        println!("doneee");
        // timeout_add_local(unsafe{refreshRate}, func)
        // thread::park_timeout(Duration::from_millis(200));
    }));

    searchButton.connect_clicked(move |_|{
      let temp = unsafe {allData.to_vec()};
      let textField = searchField.clone();
      let text = textField.text().to_string();
      let finalRes = searchRes_clone.clone();
      let mut isFound:bool = false;
      for i in temp
      {
        if(i[0].procID == text.parse::<i32>().unwrap())
        {
            isFound = true;
            let stringP1 : String = "Process Info:\n".to_owned();
            // let stringP2 : String = " CPU Usage: ".to_owned();
            // let stringP3 : String = " User: ".to_owned();
            let pName : &str = i[0].procName.as_str();
            // let cpuUsage : &str = i[0].cpuUsage.to_string().as_str();
            let userName : &str = i[0].user.as_str();
            let finalR = stringP1.clone() +"Process name: "+ pName+ ", User: " + userName;
            finalRes.set_text(&finalR);
            finalRes.set_visible(true);
        }
      }
      if(!isFound)
      {
        finalRes.set_text("No process was found");
        finalRes.set_visible(true);
      }
    });

    killButton.connect_clicked(move |_|{
        let data = unsafe{allData.to_vec()};
        let mut isKilled:bool = false;
        let mut sys = System::new();
        sys.refresh_all();
        let enteredPID : i32 = killField_clone.text().to_string().parse::<i32>().unwrap();
        for i in data
        {
            if(i[0].procID == enteredPID)
            {
                if let Some(process) = sys.process(Pid::from(enteredPID as usize)) {
                    process.kill();
                    isKilled = true;
                }
            }
        }
        if(!isKilled)
        {
            killResult_clone.set_text("Process not found to be killed");
            killResult_clone.set_visible(true);
        }
        else
        {
            killResult_clone.set_text("Process killed successfully");
            killResult_clone.set_visible(true);
        }
    });

    searchBox.append(&searchByLabel);
    // searchBox.append(&choice);
    searchBox.append(&searchProcessField);
    searchBox.append(&searchButton);
    searchBox.append(&searchResult);
    searchBox.append(&killLabel);
    searchBox.append(&killField);
    searchBox.append(&killButton);
    searchBox.append(&killResult);

    let filterLabel = gtk::Label::builder()
    .label("  Filter by:    ")
    .build();

    let customizeLabel = gtk::Label::builder()
    .label("    Remove column:  ")
    .build();

    let nameCheckBox = gtk::CheckButton::builder()
    .label("Name")
    .build();

    let parentCheckBox = gtk::CheckButton::builder()
    .label("Parent ID")
    .build();

    let memCheckBox = gtk::CheckButton::builder()
    .label("Memory usage")
    .build();

    let cpuCheckBox = gtk::CheckButton::builder()
    .label("CPU usage")
    .build();

    static mut removeNameCol : bool = false;

    nameCheckBox.connect_toggled(clone!(@weak nameCheckBox => move |_| {
        if(nameCheckBox.is_active())
        {
            treeview.remove_column(&name_column);
            unsafe{removeNameCol = true};
        }
        else 
        {
            treeview.remove_column(&uid_column);
            treeview.remove_column(&parent_column);
            treeview.remove_column(&mem_column);
            treeview.remove_column(&cpu_column);
            treeview.remove_column(&uname_column);
            treeview.append_column(&name_column);
            treeview.append_column(&uid_column);
            treeview.append_column(&parent_column);
            treeview.append_column(&mem_column);
            treeview.append_column(&cpu_column);
            treeview.append_column(&uname_column);
        }
    }));

    parentCheckBox.connect_toggled(clone!(@weak parentCheckBox => move |_| {
        if(parentCheckBox.is_active())
        {
            clone1.remove_column(&parentClone);
        }
        else 
        {
            clone1.remove_column(&mem_clone);
            clone1.remove_column(&cpu_clone);
            clone1.remove_column(&uname_clone);
            clone1.append_column(&parentClone);
            clone1.append_column(&mem_clone);
            clone1.append_column(&cpu_clone);
            clone1.append_column(&uname_clone);
        }
    }));

    memCheckBox.connect_toggled(clone!(@weak memCheckBox => move |_| {
        if(memCheckBox.is_active())
        {
            clone2.remove_column(&mem_clone2);
        }
        else 
        {
            clone2.remove_column(&cpu_clone2);
            clone2.remove_column(&uname_clone2);
            clone2.append_column(&mem_clone2);
            clone2.append_column(&cpu_clone2);
            clone2.append_column(&uname_clone2);
        }
    }));

    cpuCheckBox.connect_toggled(clone!(@weak cpuCheckBox => move |_| {
        if(cpuCheckBox.is_active())
        {
            clone3.remove_column(&cpu_clone3);
        }
        else 
        {
            clone3.remove_column(&uname_clone3);
            clone3.append_column(&cpu_clone3);
            clone3.append_column(&uname_clone3);
        }
    }));


    let settingsBox = gtk::Box::builder()
    .orientation(Orientation::Horizontal)
    .build();

    // settingsBox.append(&button);
    settingsBox.append(&filterLabel);
    settingsBox.append(&running_filter);
    settingsBox.append(&Sleeping_filter);
    settingsBox.append(&customizeLabel);
    settingsBox.append(&nameCheckBox);
    settingsBox.append(&parentCheckBox);
    settingsBox.append(&memCheckBox);
    settingsBox.append(&cpuCheckBox);
    settingsBox.append(&refreshRateField);
    settingsBox.append(&setRefreshRate);

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
    /// 


    fn fillTable(store:TreeStore, all:Vec<Vec<proc>>) -> TreeStore
    {
        store.clear();
        for row in &all 
        {
            let tree_iter = store.append(None);
            store.set_value(&tree_iter, 0, &row[0].procID.to_value());
            store.set_value(&tree_iter, 1, &row[0].procName.to_value());
            store.set_value(&tree_iter, 2, &row[0].UID.to_value());
            store.set_value(&tree_iter, 3, &row[0].memUsage.to_value());
            store.set_value(&tree_iter, 4, &row[0].cpuUsage.to_value());
            store.set_value(&tree_iter, 5, &row[0].user.to_value());
            store.set_value(&tree_iter, 6, &row[0].parentID.to_value());
        }
        // editTable(&treeview);
        return store;
    }

    fn fetch_process_data(f:i32) -> Vec<Vec<proc>> {
        let mut sys = System::new_all();
        sys.refresh_all();
        // sys.refresh_cpu();
        let mut data = vec![];
        // println!("{}",f);
        // let num_cpus = num_cpus::get() as f32;
        for (pid, process) in sys.processes() {
            if (f==1)
            {
                if (process.status() == ProcessStatus::Run) 
                {
                    let mut t:f64 = 0.;
                for prc in procfs::process::all_processes().unwrap() {
                    let p = prc.unwrap();
                    let stat = p.stat().unwrap();
                    let clk_tck : f64 = 100.;
                    let utime_sec = stat.utime as f64 / clk_tck;
                    let stime_sec = stat.stime as f64 / clk_tck;
                    let starttime_sec = stat.starttime as f64 / clk_tck;
                    let elapsed_sec = sys.uptime() as f64 - starttime_sec;
                    let usage_sec = utime_sec + stime_sec;
                    let cpu_usage1 = 100.0 * usage_sec / elapsed_sec;
                    //println!("{}",cpu_usage1);
                    // println!("{}",p.pid);
                    if (p.pid == pid.to_string().parse::<i32>().unwrap())
                    {
                        t = cpu_usage1;
                    }
                }
                    let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
                    let mut parentID:i32 = 0;
                    if(process.parent() != None)
                    {
                        // println!("{}",process.parent().unwrap().to_string());
                        parentID = process.parent().unwrap().to_string().parse::<i32>().unwrap();
                    }
                    let mut allProcs : Vec<proc> = vec![];
                    allProcs.push(proc { procID: (pid.to_string().parse::<i32>().unwrap()), 
                        procName: (process.name().to_string()), 
                        UID: (process.user_id().unwrap().to_string().parse::<i32>().unwrap()), 
                        parentID: (parentID), 
                        memUsage: (process.memory()), 
                        cpuUsage: (t), 
                        user: (uName.unwrap().name().to_string_lossy().to_string()) });
                    // let row = vec![pid.to_string(), 
                    // process.name().to_string(), 
                    // process.user_id().unwrap().to_string(), 
                    // process.memory().to_string(), 
                    // process.cpu_usage().to_string(),
                    // uName.unwrap().name().to_string_lossy().to_string(),
                    // parentID.to_string()];
                    data.push(allProcs);
                }
            }
            else if (f==2)
            {
                if (process.status() == ProcessStatus::Sleep) 
                {
                    let mut t:f64 = 0.;
                for prc in procfs::process::all_processes().unwrap() {
                    let p = prc.unwrap();
                    let stat = p.stat().unwrap();
                    let clk_tck : f64 = 100.;
                    let utime_sec = stat.utime as f64 / clk_tck;
                    let stime_sec = stat.stime as f64 / clk_tck;
                    let starttime_sec = stat.starttime as f64 / clk_tck;
                    let elapsed_sec = sys.uptime() as f64 - starttime_sec;
                    let usage_sec = utime_sec + stime_sec;
                    let cpu_usage1 = 100.0 * usage_sec / elapsed_sec;
                    //println!("{}",cpu_usage1);
                    // println!("{}",p.pid);
                    if (p.pid == pid.to_string().parse::<i32>().unwrap())
                    {
                        t = cpu_usage1;
                    }
                }
                    let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
                    let mut parentID:i32 = 0;
                    if(process.parent() != None)
                    {
                        // println!("{}",process.parent().unwrap().to_string());
                        parentID = process.parent().unwrap().to_string().parse::<i32>().unwrap();
                    }
                    let mut allProcs : Vec<proc> = vec![];
                    allProcs.push(proc { procID: (pid.to_string().parse::<i32>().unwrap()), 
                        procName: (process.name().to_string()), 
                        UID: (process.user_id().unwrap().to_string().parse::<i32>().unwrap()), 
                        parentID: (parentID), 
                        memUsage: (process.memory()), 
                        cpuUsage: (t), 
                        user: (uName.unwrap().name().to_string_lossy().to_string()) });
                    data.push(allProcs);
                }
            }
            else 
            {
                let mut t:f64 = 0.;
                for prc in procfs::process::all_processes().unwrap() {
                    let p = prc.unwrap();
                    let stat = p.stat().unwrap();
                    let clk_tck : f64 = 100.;
                    let utime_sec = stat.utime as f64 / clk_tck;
                    let stime_sec = stat.stime as f64 / clk_tck;
                    let starttime_sec = stat.starttime as f64 / clk_tck;
                    let elapsed_sec = sys.uptime() as f64 - starttime_sec;
                    let usage_sec = utime_sec + stime_sec;
                    let cpu_usage1 = 100.0 * usage_sec / elapsed_sec;
                    //println!("{}",cpu_usage1);
                    // println!("{}",p.pid);
                    if (p.pid == pid.to_string().parse::<i32>().unwrap())
                    {
                        t = cpu_usage1;
                    }
                }
                let uName = users::get_user_by_uid(process.user_id().unwrap().to_string().parse().unwrap());
                let mut parentID:i32 = 0;
                if(process.parent() != None)
                {
                    // println!("{}",process.parent().unwrap().to_string());
                    parentID = process.parent().unwrap().to_string().parse::<i32>().unwrap();
                }
                let runtime = process.run_time();
                // println!("{}",runtime);
                let mut allProcs : Vec<proc> = vec![];
                allProcs.push(proc { procID: (pid.to_string().parse::<i32>().unwrap()), 
                    procName: (process.name().to_string()), 
                    UID: (process.user_id().unwrap().to_string().parse::<i32>().unwrap()), 
                    parentID: (parentID), 
                    memUsage: (process.memory()), 
                    cpuUsage: (t), 
                    user: (uName.unwrap().name().to_string_lossy().to_string()) });
                data.push(allProcs);
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

        let v = scrolled_window.vadjustment();
        let h = scrolled_window.hadjustment();
        static mut allData:Vec<Vec<proc>> = vec![];

        fn updateDataToSearch(res:Vec<Vec<proc>>)
        {
            unsafe{allData = res};
        }

        glib::timeout_add_local(Duration::from_millis(1000+unsafe{refreshRate}),move || {
            let mut handle = handle_clone.lock().unwrap();
            if let Some(handle_inner) = handle.take() {
                match handle_inner.join() {
                    Ok(result) => {
                        let mut temp = store.clone();
                        let res_clone:Vec<Vec<proc>> = result.to_vec();
                        temp = fillTable(temp, result);
                        store = temp;
                        scrolled_window.set_vadjustment(Some(&v));
                        scrolled_window.set_hadjustment(Some(&h));
                        updateDataToSearch(res_clone);
                    }
                    Err(_) => {
                        println!("Failed to fetch process data");
                    }
                }
                // println!("from thread: {}",unsafe{refreshRate});
                *handle = Some(thread::spawn(move || fetch_process_data(unsafe{flag})));
            }
            glib::Continue(true)
    });
    // }));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Process Manager")
        .default_width(900)
        .default_height(900)
        .resizable(true)
        .child(&mainBox)
        .build();

    window.show();
}