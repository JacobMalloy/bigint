use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Mul;
use core::ops::ShlAssign;
use core::iter;
//use std::collections::VecDeque;

#[derive(Clone)]
pub struct BigInt{
    negative:bool,
    pub data:Vec<u64>
}

impl From<u64> for BigInt{
    fn from(val:u64)->BigInt{
        return BigInt{negative:false,data:vec![val]}
    }
}

fn internal_add<'a,T>(it:T)->bool
where T:Iterator<Item=(&'a mut u64,&'a u64)>{
    let negative = false;
    let mut carry = false;
    //let mut data = Vec::with_capacity(it.size_hint().0);
    for (left,right) in it{
        let (add1,carry1) = left.overflowing_add(if carry{1}else{0});
        let (add2,carry2) = add1.overflowing_add(*right);
        *left = add2;
        carry = carry1||carry2;
    }
    return carry; 

    //return BigInt{negative:negative,data:data};

}


impl Add<&BigInt> for &BigInt{
    type Output = BigInt;
    fn add(self,right:&BigInt) -> BigInt{
        let mut return_value = self.clone();
        let carry = if self.data.len() > right.data.len(){
            internal_add(return_value.data.iter_mut().zip(right.data.iter().chain(iter::repeat(&0))))
        } else if self.data.len() < right.data.len() {
            let diff = right.data.len()-self.data.len(); 
            return_value.data.extend(iter::repeat(0).take(diff));
            internal_add(return_value.data.iter_mut().zip(right.data.iter()))
        }else{
            internal_add(return_value.data.iter_mut().zip(right.data.iter()))
        };

        if carry{
            return_value.data.push(1);
        }
        return return_value;
    }
}

impl AddAssign<&BigInt> for BigInt{
    fn add_assign(&mut self, rhs: &BigInt) {
        let expand = rhs.data.len().checked_sub(self.data.len()).unwrap_or(0);
        self.data.extend(iter::repeat(0).take(expand));
        let carry = internal_add(self.data.iter_mut().zip(rhs.data.iter().chain(iter::repeat(&0))));
        if carry{
            self.data.push(1);
        } 
    }
}

impl ShlAssign<u64> for BigInt{
    fn shl_assign(&mut self, rhs: u64) {
        if rhs == 0{return;}
        let zeros_to_add = rhs/64;
        let shift_value = rhs%64;
        let mut current = 0;
        for i in self.data.iter_mut(){
           let tmp = (*i) >> (64-shift_value);
           *i = current + (*i << shift_value);
           current = tmp; 
        }
        if current != 0{
            self.data.push(current);
        }
        for _ in 0..zeros_to_add{
            self.data.insert(0,0);
        }
    }
}

impl Mul<u64> for &BigInt{
    type Output = BigInt;
    fn mul(self,mut right:u64) -> BigInt{
        if right == 0{return BigInt::from(0)}
        let mut return_value = BigInt::from(0);
        let mut cumulative :BigInt= self.clone();
        while right>0{
            let shift_val = right.trailing_zeros() as u64;
            cumulative <<= shift_val;
            return_value += &cumulative;
            cumulative <<= 1;
            right >>= shift_val + 1;
        }
        return return_value;
    }
}

