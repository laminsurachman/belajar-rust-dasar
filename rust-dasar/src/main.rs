fn main() {
    println!("Hello, world!");
    println!("Hello, Surachman Belajar bahasa Rust");
}

#[test]

fn hello_test(){
    println!("Hello,Test");
    println!("Hello, Surachman Belajar bahasa Rust");
}

#[test]

fn test_variabel(){
    let name = "Bagas Aditia Rahman";
    println!("nama saya {} ",name)
}


#[test]
fn shadowing(){
    let name = "Bagas Aditia Rahman";
    println!("nama saya {} ",name);

    let name = 20;
    println!("nama saya {} ",name);
}
/*
Ini adalah komentar
ini adalah komentar
ini adalah komentar */

#[test]
fn komentar(){
    let name = "Bagas Aditia Rahman";//ini adalah komentar
    println!("nama saya {} ",name);

    
}

#[test]
fn implicit(){
   let age = 23;
    println!(" Umur saya {}",age)
}

#[test]

fn number(){
    let a = 10;
    let b = 20.5;
    println!("{} ,{}",a,b);
//     println!("{}",b)
//
 }
#[test]
fn number_convertion(){
    let a:i8 = 10;
    let b : i16 = a as i16;
    let c: i32 = a as i32;

    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
}
#[test]
fn numeric_operator(){
    let a = 10;
    let b = 10;
    let c = a* b;
    let d = a+ b;
    println!("{},{}",c,d)
}

#[test]
fn augmented_assigment(){
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]

fn boolean(){
    let a = true;
    let b:bool = false;
    println!("{} {}",a,b);
}
#[test]
fn comparison(){
    let hasil:bool = 10 > 20;
    println!("{}", hasil)
}