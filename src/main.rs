use gix;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");
    
    let repo = gix::open(&PathBuf::from("/home/json420/src/rust")).unwrap();
    println!("{:?}", repo.branch_names());
    
    let head = repo.head_commit().unwrap();
    println!("id: {:?}", head.id);
}
