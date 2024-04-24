use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Mul;
use core::iter;

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

impl BigInt{
    fn double(&mut self){
        let mut carry = 0;
        for i in self.data.iter_mut(){
            let (add1,carry1) = i.overflowing_add(*i);
            let (add2,carry2) = add1.overflowing_add(carry);
            carry= if carry1 || carry2 {1}else{0};
            *i = add2;
        }
        if carry > 0{
            self.data.push(carry);
        }
    }
}

fn internal_add<'a,T>(it:T)->u64
where T:Iterator<Item=(&'a mut u64,&'a u64)>{
    let negative = false;
    let mut carry:u64 = 0;
    //let mut data = Vec::with_capacity(it.size_hint().0);
    for (left,right) in it{
        let (add1,carry1) = left.overflowing_add(*right);
        let (add2,carry2) = add1.overflowing_add(carry);
        carry = if carry1||carry2{1}else{0};
        *left = add2;
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

        if carry>0{
            return_value.data.push(carry);
        }
        return return_value;
    }
}

impl AddAssign<&BigInt> for BigInt{
    fn add_assign(&mut self, rhs: &BigInt) {
        let expand = (rhs.data.len()-self.data.len()).max(0);
        self.data.extend(iter::repeat(0).take(expand));
        let carry = internal_add(self.data.iter_mut().zip(rhs.data.iter().chain(iter::repeat(&0))));
        if carry > 0{
            self.data.push(carry);
        } 
    }
}

impl Mul<u64> for &BigInt{
    type Output = BigInt;
    fn mul(self,mut right:u64) -> BigInt{
        let mut return_value = BigInt::from(0);
        let mut cumulative :BigInt= self.clone();
        while right>0{
            if right & 1 == 1{
                return_value += &cumulative;
            }
            cumulative.double();
            right >>= 1;
        }
        return return_value;
    }
}

