fn main(){
    let bytes_transferred = 1_000_000.0; //1MB
    let time_seconds = 1.0;

    //convert bytes to megabits: 1byte = 8bits, 1 megabit = 1,000,000 bits
    let mbps = (bytes_transferred*8.0) / (1_000_000.0*time_seconds);
    println!("the speed: {:.2}", mbps);
}
