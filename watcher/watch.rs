use std::fs;
// use std::path::Path;
use std::time::SystemTime;
use std::collections::HashMap;
use std::env;
use std::{thread, time};


fn trasverse(path: &String, _manager: &mut HashMap<String, SystemTime>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        let metadata = fs::metadata(dir.path())?;
        let is_dir = metadata.is_dir();

        
        if let Ok(modified) = metadata.modified() {
            let path = dir.path();
            let current_path = path.into_os_string().into_string().unwrap();

            if is_dir {
                trasverse(&current_path,  _manager);
            } else {
                _manager.insert(current_path, modified);
            }

        } else {
            println!("Not supported on this platform");
        }
       
    }
    Ok(())
}


fn collect(last_hash_map: &mut HashMap<String, SystemTime>, new_hash_map: &mut HashMap<String, SystemTime>) {


    for (path, system_time) in last_hash_map {
        // caso 1: presente in newHashMap non presente in lastHashMap (file creato)
        let o: Option<&SystemTime> = new_hash_map.get(path);
        if o.is_none() {
            println!("{path} has been created");
            new_hash_map.insert(String::from(path), *system_time);
        } else {
            let value: SystemTime = *(o.expect("something went wrong"));
            // let a = system_time.duration_since(*value);
            match system_time.duration_since(value) {
                Ok(n) => {
                    let elapsed = n.as_secs();
                    if elapsed != 0 {
                        println!("{} has been modified", path);
                        new_hash_map.insert(String::from(path), value);
                        trigger_event();
                    }
                },
                Err(_) => panic!("negative"),
            }
        }
    }
}

// do something like here
fn trigger_event() {
    println!("run task");
}


fn main()  {
    let args: Vec<String> = env::args().collect();
    
    let mut last_hash_map: HashMap<String, SystemTime> = HashMap::new();
    let mut new_hash_map: HashMap<String, SystemTime> = HashMap::new();


    let ten_millis = time::Duration::from_millis(300);
    // how to use
    println!("Usage: ./watch <entrypoint>");
    println!("<entrypoint> should be a file or directory");


    let base_name = &args[1];

    println!("watcher on: {base_name}");

    loop {
        trasverse(&base_name, &mut last_hash_map); 
        collect(&mut last_hash_map, &mut new_hash_map);
        thread::sleep(ten_millis);
    }
}