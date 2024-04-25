use bignum::BigInt;
use std::time::Instant;


fn main(){
     
    let time1 = Instant::now();
    let mut tmp = BigInt::from(1);
    for i in 2..100001{
        tmp = &tmp * i;
    }
    let time_passed = time1.elapsed();
    
    let mut tmp2 = BigInt::from(1);
    for i in 2..101{
        tmp2 = &tmp2 * i;
    }

    for i in tmp2.data.iter().rev(){
        print!("{:016x}",i);
    }
    println!();
    println!("{}",tmp.data.len());
    println!("{}s",time_passed.as_micros() as f64/1000000.0);

}
