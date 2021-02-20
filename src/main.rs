use std::ops::{Deref, DerefMut};

type IOE = fn(i32) -> i32;
fn inc_by_one(x : i32) -> i32{
    x + 1
}
fn dec_by_one(x : i32) -> i32 {
    x - 1
}
fn heiger_func_1(x : i32, f : IOE) -> i32{
    f(x)
}
fn heiger_func_2(x : i32, f : fn(i32) -> i32) -> i32{
    f(x)
}
fn process(x : i32) -> i32{
    switch_hof(x)(x)
}
fn switch_hof(x : i32) -> IOE {
    if x >0 {
        inc_by_one
    }else{
        dec_by_one
    }
}
enum AppState{
    Wait,
    Succ,
    Fail,
}
struct Approval {
    _id : i32,
    _cnt_day : u32,
    _state : AppState,
}
type APPROUTINE = fn(&mut Approval) -> bool;
fn level_1(item : &mut Approval) -> bool{
    match &item._state {
        Wait => {}
        _ => {
            return false;
        }
    }
    let mut flag = false;
    if item._cnt_day < 7 {
        (*item)._state = AppState::Succ;
        flag = true;
    }
    if flag {
        println!("at level 1, review this approval!");
    }
    return flag;
}
fn level_2(item : &mut Approval) -> bool{
    match &item._state {
        Wait => {}
        _ => {
            return false;
        }
    }
    let mut flag = false;
    if item._cnt_day < 15 {
        (*item)._state = AppState::Succ;
        flag = true;
    }
    if flag {
        println!("at level 2, review this approval!");
    }
    return flag;
}
fn level_3(item : &mut Approval) -> bool{
    match &item._state{
        Wait => {}
        _ => {
            return false
        }
    }
    (*item)._state = AppState::Succ;
    println!("at level 3, review this approval!");
    return true;
}
fn chain(mut item: Approval) -> Approval{
    let mut chain  :Vec<APPROUTINE> = Vec::new();
    chain.push(level_1);
    chain.push(level_2);
    chain.push(level_3);
    for one_chain in chain {
        let ret = one_chain(&mut item);
        if ret {break;}
    }
    return item;
}
// (1) define HOF
// (2) HOF as parameter
// (3) HOF as return
// (4) example : hof and cr 责任链分离模式
fn main_exe15(){
    let mut fname : IOE = inc_by_one;
    let x = 3;
    println!("result : {}",fname(x));

    fname = dec_by_one;

    println!("result {}", fname(x));


    println!("type 1 , val = {}", heiger_func_1(3,inc_by_one));
    println!("type 2 , val = {}", heiger_func_2(3, dec_by_one));

    println!("hof as return result {}", process(3));
    println!("hof as return result {}", process(-3));

    let mut item1 = Approval{
        _id : 1,
        _state : AppState::Wait,
        _cnt_day : 6,
    };
    let mut item2 = Approval{
        _id : 1,
        _state : AppState::Wait,
        _cnt_day : 15,
    };
    chain(item1);
    chain(item2);
}


struct Packing<T> {
    obj : T,
}
impl <T> Packing<T> {
    fn new(x : T) -> Packing<T>{
        Packing{obj : x}
    }
}
impl <T> Deref for Packing<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
impl<T> DerefMut for Packing<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}
// (1) deref
// (2) packing -> Deref
fn main_exe10(){
    let a : i32 = 5;
    let b :&i32 = &a;
    println!("a.val = b.val ? : {}", a == *b);

    let c1 : i64 = 3;
    let c2 : Packing<i64> = Packing::new(3i64);
    println!("c1.val = c2.val ? : {}",c1 == *c2);

    let mut d1 : Packing<i64> = Packing::new(3);
    *d1 = 4;
    let d2 : i64 = 4;
    println!("d1.val = d2.val ? : {}", *d1 == d2);
}

fn main() {


}
