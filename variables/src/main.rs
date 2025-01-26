fn main(){
    let x = 12;
    let x = x*1;
    {
        let x = x*2;
        println!("the inner value of x: {x}");
    }
	println!("the varaible: {x}");
}
