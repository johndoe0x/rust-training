use std::io;

fn main() {
    output();
}

const FOMULA: f64 = 0.09290304;
fn output() {
    
    let mut leng = String::new();
    
    println!(" what is the length of the room in feet ? {} ",  io::stdin().read_line(&mut leng).expect("failed to readline") );
    let mut width = String::new();
    println!(" waht is the width of the room in feet?  {} ", io::stdin().read_line(&mut width).expect("failed to readline"));
    print!(" you entered dimensions of {} feet by {} feet", &leng, &width);
    println!(" The area is"); 
    let area = compute(leng.split("\n"), width);
    println!(" {} square feet", area.width);
    println!(" {} squaure meters" ,area.squarefeet);
}

fn compute(length_str :String, width_str : String)-> Result  {
    let length = length_str.parse::<i32>().unwrap();
    let width = width_str.parse::<i32>().unwrap();
    let multi = length * width;
    let squreft = FOMULA * ((multi * multi) as f64) ;
    let res = Result{
        width: multi,
        squarefeet: squreft
    };
    return res;
}

struct Result {
    width: i32,
    squarefeet: f64
}