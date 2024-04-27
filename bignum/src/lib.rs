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

fn carry_add(a:u64,b:u64,carry:bool)->(u64,bool){
    let (add1,carry1) = a.overflowing_add(if carry{1}else{0});
    let (add2,carry2) = add1.overflowing_add(b);
    return (add2,carry1||carry2);
}

fn internal_add(left:&mut Vec<u64>,right:&[u64]){
    let left_len = left.len();
    let right_len = right.len();
    let mut carry = false;
    for (left_val,right_val) in left.iter_mut().zip(right.iter()){
        (*left_val,carry) = carry_add(*left_val, *right_val, carry);
    }
    if left_len > right_len{
        let mut it = left.iter_mut().skip(right_len);
        while carry{
            match it.next(){
                Some(c) => {(*c,carry) = c.overflowing_add(1)}
                None => {break;}
            }
        }
        if carry{
            left.push(1);
        }
    }else{
        let mut it = right.iter().skip(left_len);
        while carry{
            match it.next(){
                Some(c) => {
                    let (tmp_val,tmp_carry) = c.overflowing_add(1);
                    left.push(tmp_val);
                    carry = tmp_carry; 
                }
                None => {break;}
            }
        }
        if carry{
            left.push(1);
        }else{
            left.extend(it);
        }
    }
}


impl Add<&BigInt> for &BigInt{
    type Output = BigInt;
    fn add(self,rhs:&BigInt) -> BigInt{
        let mut return_value = self.clone();
        internal_add(&mut return_value.data, &rhs.data); 
        return return_value;
    }
}

impl AddAssign<&BigInt> for BigInt{
    fn add_assign(&mut self, rhs: &BigInt) {
        internal_add(&mut self.data, &rhs.data)
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

