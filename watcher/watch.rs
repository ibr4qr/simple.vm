use std::fs;
use std::time::SystemTime;
use std::collections::HashMap;
use std::env;
use std::{thread, time};
use std::process::Command;


fn clone(source: &mut HashMap<String, SystemTime>, destination: &mut HashMap<String, SystemTime>) {
    for (path, value) in source {
        destination.insert(String::from(path), *value);
    }
}

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
    
    let mut another_one: HashMap<String, SystemTime> = HashMap::new();
    let mut temporary: HashMap<String, SystemTime> = HashMap::new();
    clone(last_hash_map, &mut another_one);
    clone(new_hash_map, &mut temporary);



    for path in another_one.keys() {

        let a: Option<&SystemTime> = last_hash_map.get(path);
        let last_system_time = *(a.expect("something went wrong"));
        // caso 1: presente in newHashMap non presente in lastHashMap (file creato)
        let o: Option<&SystemTime> = new_hash_map.get(path);
        if o.is_none() {
            println!("{path} has been created");
            new_hash_map.insert(String::from(path), last_system_time);
        } else {
            let new_system_time: SystemTime = *(o.expect("something went wrong"));
            // let a = system_time.duration_since(*value);
            match last_system_time.duration_since(new_system_time) {
                Ok(n) => {
                    let elapsed = n.as_secs();
                    if elapsed > 0 {
                        println!("{} has been modified", path);
                        new_hash_map.insert(String::from(path), last_system_time);
                        trigger_event(&path);
                    }
                },
                Err(_) => panic!("negative"),
            }
        }
    }


    for path in temporary.keys() {
        let o = last_hash_map.get(path);

        if o.is_none() {
            println!("{path} has been removed");
            new_hash_map.remove(path);
            // trigger_event();
        }
    }    

}

// do something like here
fn trigger_event(path: &String) {
    println!("compiling: {path}");
    let output = Command::new("./compiler").arg(String::from(path))
                .output()
                .expect("ls command failed to start");

    if true {
        println!("running virtual machine");
        let rawFile = String::from("./examples/add.raw");


        Command::new("./simple-vm").arg(rawFile)
                .spawn()
                .expect("ls command failed to start");
    } else {
        println!("somethign went wrong while compiling");
    }
}


fn main()  {
    let args: Vec<String> = env::args().collect();
    
    let mut last_hash_map: HashMap<String, SystemTime> = HashMap::new();
    let mut new_hash_map: HashMap<String, SystemTime> = HashMap::new();


    let ten_millis = time::Duration::from_millis(30);
    // how to use
    println!("Usage: ./watch <entrypoint>");
    println!("<entrypoint> should be a file or directory");


    let base_name = &args[1];

    println!("watcher on: {base_name}");

    loop {
        trasverse(&base_name, &mut last_hash_map); 
        collect(&mut last_hash_map, &mut new_hash_map);
        last_hash_map.clear();
        thread::sleep(ten_millis);
    }
}
