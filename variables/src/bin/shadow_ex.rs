fn main(){
    let raw_data = "1A"; //Hex byte
    println!("Raw data :{}", raw_data);

    //shadow decimal interpretation
    let raw_data = u8::from_str_radix(raw_data,16).unwrap();
    println!("Decimal value:{}", raw_data)
}
