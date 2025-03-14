use rust_wasm::become_supporter;

fn main() {
    let supporter1 = become_supporter("John", 2018);
    // John has been a supporter of Vissel Kobe since 2018!
    println!("{}", supporter1.say());
    
    let supporter2 = become_supporter("Jane", 1998);
    // Jane has been a supporter of Vissel Kobe since 1998!
    println!("{}", supporter2.say());
}
