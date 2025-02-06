// Defining a struct for an IPv4 header
//

//defining enum 
enum PacketType{
    TCP,
    UDP,
    ICMP,
}



//defining stuct
struct Packet{
    source_ip: u32,   //u32 can store 4 bytes data(like 192.168.1.1)
    dest_ip: u32, 
    packet_type:PacketType ,    
    payload: String,
}


fn main(){
    let tcp_packet = Packet{
        source_ip: 0xC0A80101, //192.168.1.1 in hex
        dest_ip: 0x08080808, // 8.8.8.8 in hex
        packet_type: PacketType::TCP,
        payload: String::from("GET / HTTP/1.1"),
    };

    println!("TCP packets created");
}
