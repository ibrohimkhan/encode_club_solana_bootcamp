fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 4 { 
        println!("Need more args!");
        return; 
    }

    for arg in args.iter().skip(1) {
        println!("{} ", arg);
    }
}
