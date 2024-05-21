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
#[test]

fn char_type(){
    let char1:char = 'a';
    let char2:char = 'b';
    println! ( "{}, {}",char1,char2)
}
#[test]

fn tuple_type(){
    let data:(i32,f64,bool)=(10,10.5,true);
    println!("{:?}",data);

}

#[test]

fn tuple(){
    let data:(i32,f64,bool)=(10,10.5,true);
    println!("{:?}",data);

    let a = data.0; //mengakses data di tuple
    let b = data.1;
    let c = data.2;
    println!("{},{},{}",a,b,c);

}
//destrukturing Tuple
#[test]

fn destrukturing_tuple(){
    let data:(i32,f64,bool)=(10,10.5,true);
    let (a,b,c) = data;
    println!("{},{},{}",a,b,c);

}

//data mutable dituple
#[test]

fn mut_tuple(){
    let  mut data :(i32,f64,bool)=(10,10.5,true);
    let (a,b,c) = data;
    println!("{},{},{}",a,b,c);

    data.0 = 20;
    println!("{},{},{}",data.0,data.1,data.2);

}

fn unit(){
    println!("Hello");
}
//tuple kosong
#[test]
fn test_unit(){
    let result= unit();
    println!("{:?}",result);

    let test=();
    println!("{:?}",test);

}
//ARRAY

#[test]

fn array(){
    let array:[i32;5]=[10,20,30,40,50];
    println!("{:?}",array);

    let a = array[0]; // mengakses array
    let b = array[3];
    println!("{},{}",a,b);
}

#[test]

fn mut_array(){  //mutable array
    let mut array:[i32;5]=[10,20,30,40,50];
    println!("{:?}",array);

    let a = array[0]; // mengakses array
    let b = array[3];
    println!("{},{}",a,b);

    array[0] = 2;
    array[2] = 5;
    println!("{:?}",array );

    let length = array.len(); //PANJANG DATA
    println!("{}",length);
}

//tWO DIMENSIONAL ARRAY

#[test]
fn two_dimensional_array(){
    let matrix: [[i32;2];2]=[[1,2],[3,4]];
    println!("{:?}",matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

//Constant
const MAXIMUM : i32 =100;

#[test]
fn constant(){
    const MINIMUM: i32 = 0;
    println!("{}, {}",MINIMUM, MAXIMUM);
}
//VARIABLE SCOPE

#[test]
fn variable_scope(){
    let a = 10;//variable_scope
    {// inner scope
        let b = 20;
        println!("{}, {}",a,b);
    }
    println!("{}",a);
}

#[test]
fn var_scope(){
    let eko = 1;//variable_scope
    {// inner scope
        println!("inner eko : {}",eko);
        let kurniawan = 2;
        println!("inner kurniawan: {}",kurniawan);
    }
    // println!("inner Kurniawan:{}",Kurniawan );//error diluar scope variable yg bisa diakses
}

//manajemen memory

#[test]
fn stack_heap(){
    function_a();
    function_b();

}

fn function_a(){
    let a = 10;
    let b = String:: from("Surachman");
    println!("{},{}",a,b);
}

fn function_b(){
    let a = 10;
    let b = String:: from("Bagas Aditia");
    println!("{},{}",a,b);
}
//&str (string slice) yg fix dan String yg berkembang


#[test]
fn string_slice(){
    let name: &str = " Bagas Aditia Rahman ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn string_type() {
    let mut name: String = String:: from("Bagas Aditia");
   println!("{}", name);

   name.push_str("Rahman");
   println!("{}",name);


}

/*OWNERSHIP RULS
SETIAP VALUE DI RUST HANRUS PUNYA OWNER /VARIABEL PEMILIK VALUE
DALAM SATU WAKTU HANYA BOLEH ADA 1OWNER
KETIKA OWNER KELUAR SCOPE ,VALUE AKAN DIHAPUS*/

#[test]
fn ownership(){
    //variabel tidak bisa diakses blm dideklarsikan
    let name = String::from("Bagas Aditia");// name diakses mulai disini
    println!("{}",name);
    
    // variabel alamat tidak bisa diakses blm dideklarasikan
   {let alamat = String:: from ("jakarta");// bisa diakses disini
   println!("{}", alamat)
}// scope variabel alamat selesai,alamat dihapus,variabel alamat tidak bisa diakses lagi
   println!("{}",name)

   // scope variabel name dihapus, dan tidak bisa diakses lagi
}

#[test]
// data copy tesimpan di stack memory
fn data_copy(){
    let a = 10;
    let mut b = a;  //copy data dari a ke b isinya sama

    b = 20 ;
    println!("{},{}",a,b);
}

/*Ownership Movement tipe data di heap memory
ketika suatu waktu membuat variabel baru bukan mengcopy melainkan transfer 
ownership owner lama ke owne baru maka setelah selsai transfer owner lama tidak valid lagi*/

#[test]
fn ownership_movement(){
   let name1 = String:: from("Bagas ");

//ownership dari nama1 dipindahkan ke ownership nama2
   let name2: String = name1;
   // name1 tidak bisa diakses disini

   println!("{}",name2);
}

//clone membuat data tiruan di heap memory ygmempunya methode clone

#[test]
fn clone (){
    let name1 = String:: from("Bagas ");
    let name2: String = name1.clone();

    println!("{},{}",name1,name2);
}
//IF EXPRESSEN percabangan code 

#[test]
fn in_expression(){
    let value = 9;

    if value >= 8 {
        println!("Good");
    }else if value >= 6 {
        println!(" Not Bad");
    }else if value >= 3 {
        println!("Bad");
    }else {
        println!("Very Bad");
    }
}

#[test]
fn let_statement_expression(){
    let value = 9;
    let result: &str;

    if value >= 8 {
     result = "good";
    }else if value >= 6 {
      result = "Not Bad";
    }else if value >= 3 {
     result= "Bad";
    }else {
       result = "very Bad";
    }

    println!("{}",result);
}
#[test]
fn if_statement_expression(){
    let value = 2;
    let result= if value >=8 {
        "good"
    }else if value >=6 {
        "Not Bad"
    }else if value >= 3 {
        "Bad"
    }else {
        "very Bad"
    };
    println!("{}",result);
}

//Loop Break and Continue

#[test]
fn loop_expression (){
    let mut counter = 0;
    loop{
        counter += 1;
        if counter == 10 {
            break;
        }else if counter % 2 == 0 {
            continue;
        }
        println!("{}",counter);
    }
}

#[test]

fn loop_return_value(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter > 10 {
          break counter * 2;
        }
     
    };   println!("Result : {}", result);
}


// loop label 
#[test]
fn loop_label(){
    let mut number = 1;
    'outer: loop{
      let mut i = 1;
      loop{
        if number >10{
            break 'outer;
        }
        println!("{} X {} = {}", number, i, number * i);
        i += 1;
        if i > 10 {
            break;
        }
      }
      number += 1;
        }
    }

    // while loop perulangan yg mempunyai kondisi

#[test]
fn while_loop (){
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!(" Counter :{}",counter);
        }
        counter += 1;
      
    }
}

//Foor Loop mengambil data di array

#[test]

fn array_iterasi (){
    let array:[&str; 5]= ["A","B","C","D","E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value :{}",array[index]);
        index += 1;
    }
}
#[test]
fn for_loop(){
    let array:[&str; 5]= ["A","B","C","D","E"];
    for value in array{
        println!("Value :{}",value);
    }
}

   
// Tipe data Range atau jarak antara start dan end

#[test]
fn range(){
    let array:[&str;5]= ["A","B","C","D","E"];
    let range = 0..5;
    println!("start: {}", range.start);
    println!("end: {}", range.end);

    for i in range {
        println!("Value :{}",array[i]);
    }

}
 
 // range Inclusive

 #[test]
 fn range_inclusive(){
    let array: [&str; 5]= ["A","B","C","D","E"];
    let range = 0..=4;
    println!("start: {}", range.start());
    println!("end: {}", range.end());

    for i in range {
        println!("Value :{}",array[i]);
    }
 }

 //FUNCTION

 fn say_hello(){
     println!("Hello World");
 }
 #[test]
 fn test_say_hello(){
     say_hello();
 }

 //parameter di function 

 fn say_goodbay(first_name:&str, last_name:&str){
 println!("Goodbay {} {}", first_name,last_name);
 }
 #[test]
 fn test_parameter(){
     say_goodbay("Eko","khanedy");
     say_goodbay("Bagas", "Aditia");
     say_goodbay("naflah", "Faulina");
 }

 //Return Value
 fn factorial_loop(n:i32) -> i32 {
    if n< 1 {
        return 0;
    }
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
 }
 #[test]
fn test_factorial_loop(){
    let result = factorial_loop(5);
    println!("Result : {}",result);

    let result = factorial_loop(-10);
    println!("Result : {}",result);
}

//Recursive function  mengambil funsinya sendiri

fn print_text (value:String, times: u32){
    if times ==0 {
        return;
    }else {
        println!("{}",value);
     
    }   print_text(value, times -1);
}

#[test]
fn test_recursive_function(){
    print_text(String::from(" Bagas"), 5);
}

fn factorial_recursive(n: u32)-> u32{
    if n < 1 {
        return 0;
    }else if n == 1 {
        return 1;
    }else {
        return n * factorial_recursive(n-1);
    }
}
#[test]
fn test_factorial_recursive(){
    let result = factorial_recursive(5);
    println!("Result : {}",result);

    
}

// Ownership Function

fn print_number ( number:i32){
    println!("{}",number);
}
fn hi(name:String){
    println!("Hi {}",name);
}

#[test]
fn test_hi(){
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name= String:: from ("Bagas");
    hi(name.clone());
    println!("{}",name);
}
//Refference 
fn full_name(first_name: &String, last_name:&String) -> String {

     format!("{} {}",first_name,last_name)


}

#[test]
fn test_full_name(){
    let first_name = String:: from ("Bagas");
    let last_name = String:: from ("Aditia Rahman");

    let name= full_name(&first_name,&last_name);
    println!("{}",name);
     println!("{}",first_name);
    println!("{}",last_name);
}


//Browing atau meminjam tapi tidak boleh memodivikasi

fn change_value(value: &mut String){
    value.push_str("Test");
}
#[test]
fn test_change_value(){
    let mut value= String::from("Bagas");
    change_value(&mut value);
    println!("{}",value);
}

//dagling pointer solution

fn get_full_name(first_name:&String, last_name:&String)->String {
    let name= format!("{} {}",first_name,last_name);
    return name;
}

#[test]
fn test_get_full_name(){
let first_name = String :: from ("Bagas");
let last_name = String :: from ("Aditia Rahman");

let name = get_full_name(&first_name,&last_name);

println!("{}",name);
}

// Slice atau reference
#[test]

fn slice_reference(){
    let array:[i32;10]=[1,2,3,4,5,6,7,8,9,10];
    let slice1:&[i32] = &array[..];
    println!("{:?}", slice1);

    let slice2:&[i32]=&array[0..5];
    println!("{:?}", slice2);

    let slice3:&[i32]=&array[5..];
    println!("{:?}", slice3);
}
#[test]
fn nama_slice(){
    let name = String::from("Bagas Aditia Rahman");
    let nama:&str = &name[0..5];
  
    println!("{}", nama);

     let last_name:&str = &name[6..];
     println!("{}",last_name);
}


// membuat Struct

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
#[test]

fn test_struct_person(){
    let first_name=String:: from ("Bagas");
    let last_name=String:: from ("Aditia Rahman");
    let person = Person {
        first_name,
        last_name,
        age: 20,
    };
    println!("{}",person.first_name);
    println!("{}",person.last_name);
    println!("{}",person.age);
}