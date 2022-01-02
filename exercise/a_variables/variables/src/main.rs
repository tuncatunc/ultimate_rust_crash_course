const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    println!("Hello, world!");
    // let mut missiles: i32 = STARTING_MISSLES;
    // let ready: i32 = READY_AMOUNT;
    let _:i32;
    // READY_AMOUNT = 1;
    let (missiles, ready) = (STARTING_MISSLES, READY_AMOUNT);
    print!("Firing {} of my {} missiles...\n", ready, missiles - ready);
    // missiles = missiles - ready;
}
