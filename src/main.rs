use std::{env,path::PathBuf};

fn main() {
    let arg:Vec<String> = env::args().collect();
    // 命令参数的限定 如果大于2的参数 那就直接退出
    let argc = arg.len(); //参数计数
    match argc {
        1 => { current_file(); },
        2 => { path_dir(&arg[1]); },
        _ => {
            println!("必须只能最少一个参数");
            std::process::exit(1);
        },
    }
    
    
}

fn current_file(){
    let path = env::current_dir().unwrap();
    let dir  = path.as_path().read_dir().unwrap();
    for x in dir{
        if let Ok(path) = x{
            println!("{:?}",path.file_name());
        }
    }
}

fn path_dir(path:&str){
    let mut dir_path = PathBuf::new();
    dir_path.push(path);
    let dir = dir_path.as_path().read_dir().unwrap();
    for x in dir{
        if let Ok(path) = x {
            println!("{:?}",path.file_name())
        }
    }

}
