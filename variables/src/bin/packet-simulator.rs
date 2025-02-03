const MAX_PACKET_SIZE: usize = 1500;

fn main(){
    let mut packets_sent:u32 = 0;
    packets_sent+=1;//send and increment packet
    
    //Shadow to a string for logging 
    let packets_sent = format!("packets sent: {}", packets_sent);
    println!("{} | Max Size {}B ", packets_sent, MAX_PACKET_SIZE);

}
