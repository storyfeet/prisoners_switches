extern crate rand;
use rand::{Rng,thread_rng};

pub trait Prisoner{
    fn visit(&mut self,_:&mut u8)->bool;
}

pub struct Plan1{
    visits:u8,
}

impl Plan1{
    pub fn new()->Self{
        Plan1{
            visits:0,
        }
    }
}

impl Prisoner for Plan1{
    fn visit(&mut self,sw:&mut u8)->bool{
        if *sw == 3 && self.visits > 3 {
            return true;
        }

        self.visits += 1;

        let v = if self.visits < 6 {0} else {self.visits -6};

        *sw +=1;

        *sw = (*sw).min(v).min(3);

        return false;
    }
}



pub struct Plan2{
    visits:u8,
    highest:u8,
}

impl Plan2{
    pub fn new()->Self{
        Plan2{
            visits:0,
            highest:0,
        }
    }
}

impl Prisoner for Plan2{
    fn visit(&mut self,sw:&mut u8)->bool{
        if *sw == 3 && self.visits > 3 &&self.highest == 3 {
            return true;
        }

        self.visits += 1;
        if *sw > self.highest {
            self.highest = *sw;
        }

        let v = if self.visits < 8 {0} else {self.visits -8};

        *sw +=1;

        *sw = (*sw).min(v).min(3).min(self.highest+1);

        return false;
    }
}


pub struct Plan3{
    visits:u8,
    others:u8,
    last:u8,
}

impl Plan3{
    pub fn new()->Self{
        Plan3{
            visits:0,
            others:0,
            last:0,
        }
    }
}

impl Prisoner for Plan3{
    fn visit(&mut self,sw:&mut u8)->bool{
        self.others += *sw;
        if self.last > *sw {
            self.others = *sw
        }
        if self.others >= 20 {
            return true;
        }

        self.visits += 1;
        if self.visits == 1 {
            *sw = 0;
            return false;

        }

        *sw = (*sw+1).min(3);
        return false;
    }
}


pub fn test<P:Prisoner>(ps:&mut [P])->Result<(usize,usize),usize>{
    let mut sw = 0;

    let mut complete = None;

    let mut visits = Vec::with_capacity(ps.len());
    for _ in 0..ps.len(){
        visits.push(0);
    }

    let mut i = 0;
    for _ in 0..1000000 {
        i +=1;

        let n = thread_rng().gen_range(0,ps.len());
        visits[n] += 1;

        if ps[n].visit(&mut sw){
            break;
        }
        if sw > 3 {
            return Err(i);
        }

        if let None = complete {
            if let None = (&visits).into_iter().find(|x| **x == 0) {
                complete = Some(i);
            }
        }
    }

    for v in visits {
        if v == 0 {
            return Err(i);
        }
    }

    Ok((i,complete.unwrap()))

}


fn main() {
    let mut fails = 0;
    let mut succ = 0;
    let mut tot = 0;

    for _ in 0..100{
        let mut v = Vec::new();
        for _ in 0..10 {
            v.push(Plan3::new());
        }

        let r = test(&mut v);
        
        match r {
            Ok((v,t))=>{
                tot += v;
                succ += 1;
                println!("Success {}. (complete at {})",v,t);
            },
            Err(v)=>{
                tot += v;
                fails +=1;
                println!("Fail {}",v);
            }
        }

    }
    println!("Done Success : {} , Fail {}, Avg {}",succ,fails,tot/100);
}


