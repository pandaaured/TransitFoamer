pub fn state(args: Vec<String>) -> i32 {
    let mut argiter = args.iter();
    if argiter.position(|x| *x == "-s".to_string()) != None {
        return 1;
    } else if argiter.position(|x| *x == "-d".to_string()) != None {
        return 2;
    }

    return 0;
}