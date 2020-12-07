pub fn inspect(arg: &String) {
    if is_plural(arg) {
        println!("{} is plural", arg);
    } else {
        println!("{} is singular", arg);
    };
}

pub fn change(arg: &mut String) {
    if !is_plural(arg) {
        arg.push_str("s");
    };
}

pub fn eat(arg: String) -> bool {
    arg.starts_with("b") && arg.contains("a")
}

pub fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}

fn is_plural(arg: &String) -> bool {
    arg.ends_with("s")
}
