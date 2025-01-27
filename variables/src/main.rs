fn main(){

    //immutable by default for fixed header size
    //
    let header_size: usize = 20; //bytes
    println!("header size:{}bytes", header_size);

    //Mutable variable for dynamic payload size
    let payload_size: usize = 512; //bytes
    payload_size = 1024;//update later
    println!("Payload size: {}bytes", payload_size);
    
}


