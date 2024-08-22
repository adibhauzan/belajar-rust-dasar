mod first;
mod second;
mod model;
mod third;

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::ops::Add;
use plotters::prelude::*;
use crate::model::User;


#[test]
fn test_module() {
    let user: model::User = User::new(String::from("adib"),
                                String::from("hauzan"),
                                String::from("sofyan"),
                                String::from("adibhauzan48@gmail.com"),
                            22,
    );

    user.say_hello("bujang inam")
}





use first::say_hello;
use second::say_hello as say_hello_second;
#[test]
fn test_use() {
    say_hello();
    say_hello_second();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Buat file output dalam format PNG
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Buat grafik dengan koordinat X dan Y
    let mut chart = ChartBuilder::on(&root)
        .caption("Sederhana Line Plot", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..10, 0..100)?;

    // Menambahkan sumbu X dan Y
    chart.configure_mesh().draw()?;

    // Menambahkan data ke grafik
    chart.draw_series(LineSeries::new(
        (0..10).map(|x| (x, x*x)),
        &BLUE,
    ))?.label("y = x^2");

    // Menampilkan legenda
    chart.configure_series_labels().draw()?;

    Ok(())
}



#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable(){
    let name = "adib hauzan";
    println!("Hello, {}!", name);
}

#[test]
fn test_variable_mutable(){
    let mut name = "adib hauzan";
    println!("Hello, {}!", name);

    name = "adib hauzan sofyan";
    println!("Hello, {}!", name);
}

#[test]
fn test_static_typing(){
    let mut name = "adib hauzan";
    println!("Hello, {}!", name);

    name = "bujang inam";
    println!("Hello, {}!", name);

    let name2 : &str;
    name2 = "bujang inam";
    println!("Hello, {}!", name2);
}

#[test]
fn test_explicit(){
    let age : i8 = 22;
    println!("My age is {}", age);
}

#[test]
fn test_number (){
    let a: u8 = 255;
    println!("a is {}", a);
}

#[test]
fn test_conversion_number (){
    let a: i8 = 127;
    let b: i16 = a as i16;
    let c: i32 = a as i32;
    let mut d: i64 = a as i64;
    println!("a is {}, b is {}, c is {}, d is {}", a, b, c, d);

    d = 100000000000;
    let e: i8 = d as i8;
    println!("d is {}", d);
    println!("e is {}", e);
}

#[test]
fn test_numeric_operator(){
    let a = 10;
    let b = 20;
    let c = a + b;
    let d = a - b;
    let e = a * b;
    let f = a / b;
    let g = a % b;
    println!("c is {}, d is {}, e is {}, f is {}, g is {}", c, d, e, f, g);
}

#[test]
fn test_augmented_assignment(){
    let mut a = 10;
    println!("{}",a);

    a += 10;
    println!("{}",a);
}

#[test]
fn test_boolean(){
    let mut a: bool = true;
    println!("nilai awal a adalah {}",a);
    a = false;
    let b: bool = false;

    println!("a is {}, b is {}", a, b);
}

#[test]
fn test_comparison(){
    let a = 10;
    let b = 20;
   
    let c: bool = a < b;
    println!("{}", c);
    let f: bool = a > b;
    println!("{}", f);
    let d: bool = a == b;
    println!("{}", d);
    let e: bool = a != b;
    println!("{}", e);
}

#[test]
fn test_boolean_operator(){
    let absen :i8 = 75;
    let nilai_akhir : i8 = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai = nilai_akhir >= 75;

    let lulus_akhir = lulus_absen && lulus_nilai;

    if lulus_akhir == false{ 
        println!("Maaf Anda belum lulus yahahaha kasian"); 
    }else {
        println!("Selamat Anda lulus");
    }
}

#[test]
fn test_char_type(){
    let char1 = 'a';
    let char2 = 'b';

    println!("char1 is {}, char2 is {}", char1, char2);
}

#[test]
fn test_tuple_type(){
    let mut data: (&str, i8, bool) = ("adib hauzan", 22, true);
    println!("data is {}, {}, {}", data.0, data.1, data.2);

    data = ("bujang inam", 22, true);
    println!("data is {}, {}, {}", data.0, data.1, data.2);
    println!("{:?}", data)

}

fn unit(){
    println!("hello");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result);
}

#[test]
fn test_array(){
    let mut arr : [i32; 5]= [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    arr[0] = 10;
    arr[1] = 20;
    println!("{:?}", arr);

   let result = arr.len();
    println!("{}", result);
}

#[test]
fn test_two_dimensional_array(){
    let matrix :[[ i32; 3]; 3] =
    [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("{:?}", matrix);
}

fn function_a(){
    let a = 10;
    let b = String::from("adib hauzan");
    let mut name = String::from("adib hauzan");
    name.push_str("adib hauzan sofyan");
    println!("{}", name);
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("sofyan");
    println!("{} {}", a, b);
}


#[test]
fn test_stack_heap(){
    function_a();
    function_b();
}

#[test]
fn test_string(){
    let name = " adib hauzan sofyan ";
    let trim = name.trim();
    println!("{}", name);
    println!("{}", trim);
}

#[test]
fn test_string_type(){
    let mut name = String::from("adib hauzan");
    name.push_str(" sofyan");

    println!("{}", name);

    let tampan = name.replace("adib", "tampan");
    println!("{}", tampan);
}

#[test]
fn ownership_rules(){
    let a = 10;
    {
        let b = 10;
        println!("{}", b);
    }
    println!("{}", a);
    // println!("{}", b); // error karna diluar scope
}

#[test]
fn ownership_movement(){
    let name1: String = String::from("adib");
    println!("{}", name1);

    let name2 = name1;
    // println!("{}", name1);
    println!("{}", name2);
}

#[test]
fn ownership_clone(){
    let name1: String = String::from("adib");
    println!("{}", name1);

    let name2 = name1.clone();
    println!("{}", name1);
    println!("{}", name2);
}

#[test]
fn test_if_expression(){
    let value = 6;
    let result: &str;

    if value > 8 {
        result = "Very Good"
    }else if value >= 7 {
        result = " Good"
    }else if value >= 6 {
        result = "Not Bad"
    }else {
        result = "Bad"
    }

    println!("{}",result)
}

#[test]
fn test_let_if_expression(){
    let value = 6;
    let result = if value > 8 {
        "Very Good"
    }else if value >= 7 {
        " Good"
    }else if value >= 6 {
        "Not Bad"
    }else {
        "Bad"
    };
    println!("{}",result)
}

#[test]
fn test_loop(){
    let mut counter = 0;
     loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }else if counter % 2 == 0 {
            continue;
        }
         println!("Counter : {}", counter);
    };
}

#[test]
fn test_loop_return_value(){
    let mut counter = 0;
     let result = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }else if counter % 2 == 0 {
            continue;
        }
    };
    println!("{}", result);
}

#[test]
fn test_loop_label() {
    let mut number = 0;

    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10{
                break 'outer;
            }

            let result = number * i;

            println!("{} x {} = {}", number, i, result);

            if i >= 10 {
                break
            }
            i+=1;

        }
        number += 1;
    }
}

#[test]
fn test_while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter %2 == 0{
            println!("{}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration_while_loop() {
    let array : [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;
    while index < array.len(){
        println!("value dari array index ke {} = {}", index, array[index]);
        index+=1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array : [&str; 5] = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("{}", value)
    }
}

#[test]
fn test_range() {
    let array : [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5;
    println!("range start = {}", range.start);
    println!("range end {}", range.end);

    for i in range{
        println!("{}", array[i])
    }
}

#[test]
fn test_range_inclusive() {

    let range = 0..=4;
    println!("range start = {}", range.start());
    println!("range end {}", range.end());
    let array : [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range{
        println!("{}", array[i])
    }
}


// fn say_hello(){
//     println!("hello");
// }

#[test]
fn function() {
    for _ in 0..4{
        say_hello()
    }
    say_hello()
}

fn say_good_bye(first_name : String, middle_name : String, last_name : String){
    let mut full_name: String = String::new();
    full_name.push_str(&first_name);
    full_name.push_str(" ");
    full_name.push_str(&middle_name);
    full_name.push_str(" ");
    full_name.push_str(&last_name);

    println!("GoodBye {}", full_name)
}

#[test]
fn test_function_with_parameter() {
    let first_name = String::from("adib");
    let middle_name = String::from("hauzan");
    let last_name = String::from("sofyan");

    say_good_bye(first_name, middle_name, last_name)
}

fn factorial_loop_1(n: i32)->i32{
    if n < 1 {
        return 0
    }

    let mut result =  1;
    for i in 1..=n{
        result *= i;
    }

    result
}

#[test]
fn test_function_return_value() {
    let result = factorial_loop_1(-4);
    println!("{}", result)
}


fn factorial_loop_recursive(n: i32)->i32{
    if n == 1 {
        return 1;
    }

    n * factorial_loop_recursive(n - 1)
}

fn test_print_text(value: String, times: i32){
    if times == 0 {
        return;
    }else {
        println!("{}", value)
    }

    test_print_text(value, times-1);
}

#[test]
fn test_function_recursive() {
    let result = factorial_loop_recursive(4);
    println!("{}", result);

    test_print_text(String::from("adib"), 5)
}

// fn print_number(n : i32){
//     println!("{}", n);
// }
//
// fn hi(name: String){
//     println!("{}", name);
// }

// fn full_name(first_name: String, middle_name: String, last_name: String) -> String{
//     format!("Hello {} {} {}", first_name, middle_name, last_name)
// }
//
// #[test]
// fn test_full_name() {
//     let first_name: String = String::from("adib");
//     let midle_name: String = String::from("hauzan");
//     let last_name: String = String::from("sofyan");
//
//     let full_name = full_name(first_name, midle_name, last_name);
//     println!("{}",full_name);
//     // println!("{}", first_name);
// }

// fn full_name(first_name: String, middle_name: String, last_name: String) -> (String, String, String){
//     (first_name, middle_name, last_name)
// }
//
// #[test]
// fn test_full_name() {
//     let first_name: String = String::from("adib");
//     let middle_name: String = String::from("hauzan");
//     let last_name: String = String::from("sofyan");
//
//     let (first_name, middle_name, last_name)= full_name(first_name, middle_name, last_name);
//     let full_name = format!("{} {} {}", first_name, middle_name, last_name);
//     println!("{}",full_name);
//     assert_eq!(full_name, "adib hauzan sofyan");
//     // println!("{}", first_name);
// }

fn full_name(first_name: &String, middle_name: &String, last_name: &String) -> String{
   format!("{} {} {}", first_name, middle_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name: String = String::from("adib");
    let middle_name: String = String::from("hauzan");
    let last_name: String = String::from("sofyan");

    let full_name = full_name(&first_name, &middle_name, &last_name);
    println!("{}",full_name);
    assert_eq!(full_name, "adib hauzan sofyan");
    println!("{}", first_name);
    println!("{}", middle_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String){
    value.push_str("Test")
}

#[test]
fn test_change_value() {
    let mut value = String::from("Adib");
    change_value(&mut value);

    println!("{}", value)
}

#[test]
fn test_string_slice() {
    let name: String = String::from("Adib Hauzan Sofyan");
    let first_name: &str = &name[0..4];
    println!("{}", first_name);

    let middle_name: &str = &name[5..11];
    println!("{}", middle_name);

    let last_name = &name[12..];
    println!("{}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

trait Human {
    fn introduce(&self);
}

impl Human for Person  {
    fn introduce(&self){
        let full_name = format!("{}, {}, {}",&self.first_name, &self.middle_name, &self.last_name);
        println!("Hi my name is {}, my friend usualy call me {}. and im {} years old", full_name, self.first_name, self.age);
    }
}

fn print_person(person: &Person){
    println!("{}\n{}\n{}\n{}\n", person.first_name, person.middle_name, person.last_name, person.age)
}

#[test]
fn test_trait() {

    // let first_name: String = String::from("Adib");
    let adib = Person{
        // first_name,
        first_name: "Adib".to_string(),
        middle_name: "Hauzan".to_string(),
        last_name: "Sofyan".to_string(),
        age: 22,
    };

    adib.introduce();

    print_person(&adib);
    // println!("{}", first_name)

    let adib2 = &Person{
        first_name: adib.first_name.clone(),
        middle_name: adib.middle_name.clone(),
        last_name: adib.last_name.clone(),
        ..adib
    };

    print_person(adib2);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing{};
}

impl Person{
    fn new(first_name: String, middle_name: String, last_name: String, age: u8) -> Person {
        Person {
            first_name,
            middle_name,
            last_name,
            age,
        }
    }

    fn match_person(&self) {
        match self {
            Person { first_name, last_name, ..} => {
                println!("firstname : {}, lastname : {}", first_name, last_name);
            }
        }
    }
    fn say_hello(&self, name: &str){
        println!("hello {}, my name is {}",name, self.first_name)
    }

}

#[test]
fn test_method() {
    let person1 = Person{
        first_name: String::from("Adib"),
        middle_name: String::from("Hauzan"),
        last_name: String::from("Sofyan"),
        age: 22,
    };

    person1.say_hello("calon pacar");

    println!("{}", person1.first_name)
}


struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(lat: f64, long: f64) -> GeoPoint {
        GeoPoint(lat, long)
    }

    fn print(&self) {
        println!("Latitude: {}, Longitude: {}", self.0, self.1);
    }

    fn match_geo_point(&self) {
        let point = GeoPoint::new(self.0, self.1);
        match point {
            GeoPoint(lat, 0.0) => {
                println!("Latitude: {}", lat);
            }
            GeoPoint(0.0, long) => {
                println!("Longitude: {}", long);
            }
            GeoPoint(lat, long) => {
                println!("Latitude: {}, Longitude: {}", lat, long);
            }
        }
    }
}

#[test]
fn test_associated_function() {
    let gp = GeoPoint::new(-2.0000123, 91.02920334);
    gp.print()
}

enum Level{
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    // let _ = Level::Regular;
    // let _ = Level::Premium;
    // let _ = Level::Platinum;

    let level: Level= Level::Premium;
    match level {
        Level::Regular => {
            println!("Regular")
        }
        Level::Premium => {
            println!("Premium")
        }
        Level::Platinum => {
            println!("Platinum")
        }
    }

}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    EWallet(String, String),
}

impl Payment{
    fn pay(&self, amount: u32){
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {}, amount {}", number, amount)
            }
            Payment::BankTransfer(bank, number) => {
                println!("Paying with Bank Transfer {} {}, amount {}",bank, number, amount)
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with EWallet {} {}, amount {}", wallet, number, amount)
            }

        }
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("12093324123"));
    _payment1.pay(123123312);
    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("123123123"));
    _payment2.pay(13242133);
    let _payment3 = Payment::EWallet(String::from("Gopay"), String::from("123123123"));
    _payment3.pay(123934213);
}


#[test]
fn test_struct_pattern() {
    let point = GeoPoint::new(0.2, 1.0);
    point.match_geo_point();
    point.print();

    let person: Person = Person::new("adib".to_string(), "hauzan".to_string(), "sofyan".to_string(), 22);
    person.match_person()
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

trait Kontol {
    fn introduce(&self);
}

impl Customer {
    fn new(id: IdentityNumber, name: String, age: Age)-> Customer{
        Customer{
            id,
            name,
            age
        }
    }

    fn introduce(&self){
        println!("hai my id is {}, my name is {}, and im {} years old", self.id, self.name, self.age)
    }
}

impl Kontol for Customer{
    fn introduce(&self) {
        println!("hai my id is {}, my name is {}, and im {} years old", self.id, self.name, self.age)

    }
}

#[test]
fn test_type_alias() {
    let mut adib = Customer::new(String::from("1203017"), String::from("Adib Hauzan Sofyan"), 22);
    adib.introduce();
    adib.name = String::from("adib hauzan");
    adib.introduce();

    let mut yudi = "yudi";
    println!("{}", yudi);
    let yudi2: String = String::from("yudi2");
    println!("{}", yudi2);

    yudi = "adib";
    println!("{}", yudi);
}



trait CanSayGoodBye {
    fn good_bye(&self)->String;
    fn good_bye_to(&self, name: &str)->String;
}

trait CanSayHello {
    fn hello(&self)->String{
        String::from("Hello")
    }
    fn say_hello(&self)->String;
    fn say_hello_to(&self, name: &str)->String;
}
struct Orang {
    first_name: String,
    middle_name: String,
    last_name: String
}

impl Orang {
    pub fn new(first_name: String, middle_name: String, last_name: String)-> Orang{
        Orang{
            first_name,
            middle_name,
            last_name
        }
    }
}

impl CanSayHello for Orang{
    fn say_hello(&self) -> String {
        format!("hello my name is {} {} {}", self.first_name,self.middle_name, self.last_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("hello {} my name is {} {} {} my friend usualy call me {}", name, self.first_name,self.middle_name, self.last_name, self.first_name)
    }
}

impl CanSayGoodBye for Orang{
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.first_name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye {}, my name is {}",name, self.first_name)
    }
}

fn say_hello_trait(value: &impl CanSayHello){
    println!("{}",value.say_hello())
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)){
    println!("{}",value.say_hello());
    println!("{}",value.good_bye());
}

#[test]
fn test_trait_lagi() {
    let mut adib = Orang::new(String::from("Adib"), String::from("Hauzan"), String::from("Sofyan"));
    println!("{}", adib.say_hello());
    println!("{}", adib.say_hello_to("Gilang"));
    println!("{}",adib.hello());
    say_hello_trait(&adib);
    println!("{}", adib.good_bye());
    println!("{}", adib.good_bye_to("Gilang"));
    hello_and_goodbye(&adib);
}

struct SimplePerson {
    name: String,
}

impl SimplePerson{
    fn new(name: String)->SimplePerson{
        SimplePerson{
            name
        }
    }
}

impl CanSayGoodBye for SimplePerson{
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye {}, my name is {}", name, self.name)
    }
}


fn new_simple_person(name: String) -> impl CanSayGoodBye {
    SimplePerson{
       name
    }
}

#[test]
fn test_function_contructor() {
    let mut adib = &SimplePerson{
        name: String::from("adib"),
    };

    let simple_constructor = new_simple_person(adib.name.clone());
    println!("{}", simple_constructor.good_bye());
    println!("{}", simple_constructor.good_bye_to("GILANG"));
}

trait CanSay: CanSayHello + CanSayGoodBye{

}

struct SimpleMan {
    name: String,
}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        format!("hello my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        let orang = format!("hello {} my name is {}", name, self.name);

        orang
    }
}

impl CanSayGoodBye for SimpleMan {
    fn good_bye(&self) -> String {
        format!("Good bye, my name is {}", self.name)
    }

    fn good_bye_to(&self, name: &str) -> String {
        format!("Good bye {}, my name is {}", name, self.name)
    }
}

impl CanSay for SimpleMan {
}

struct Point<T>{
    x: T,
    y: T,
}

impl <T> Point<T> {

    fn get_y(&self)-> &T {
        &self.y
    }

    fn get_x(&self)-> &T {
        &self.x
    }

}

#[test]
fn test_generic_struct() {
    let integer:Point<i32> = Point::<i32>{
        x: 1, y: 2,
    };

    println!("{} {}", integer.x, integer.y)
}

enum Value<T>{
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_enum() {
    let value: Value<i8> = Value::VALUE(8);

    match value {
        Value::NONE => {
            println!("NONE")
        }
        Value::VALUE(value) => {
            println!("VALUE = {}", value)
        }
    }
}

struct HI<T: CanSayGoodBye>{
    value: T
}

#[test]
fn test_generic_bound() {
    let simple_person = HI::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Adib"),
        },
    };

    println!("{}", simple_person.value.name)
}

fn min<T: PartialOrd>(value1: T, value2: T)->T{
    if value1 > value2{
        value1
    }else {
        value2
    }
}

#[test]
fn test_generic_function() {
   let result = min::<i32>(2, 10);
    println!("{}", result);

    let result = min(12, 6);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point: Point<i32> = Point{
        x: 2,
        y: 10,
    };

    println!("{}", point.get_y());
    println!("{}", point.get_x());
}

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Apple{
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_oveloadable_operator() {
    let apple1 = Apple{ quantity: 20 };
    let apple2 = Apple{ quantity: 20 };

    let apple3 = apple1 + apple2;
    println!("{:?}", apple3.quantity)
}

fn add(value: Option<i32>)-> Option<i32>{
    match value {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn test_option() {
    let result = add(Some(2));
    let result2 = add(None);
    let result3 = result.clone();

    println!("{:?}", result);
    println!("{:?}", result2);
    println!("{:?}", result3);
}

impl PartialEq for Apple{
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_string_manipulation() {
    let s = String::from("Adib Hauzan Sofyan");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("Adib", "Bida"));
    println!("{}", s.trim());
    println!("{}", s.starts_with("Adib"));
    println!("{}", s.ends_with("Sofyan"));
    println!("{}", s.contains("Hauzan"));
    println!("{}", &s[0..4]);
    println!("{:?}", s.get(0..=3));
}

struct Category {
    id: String,
    name: String,
}

use std::fmt::{Debug, Formatter};

impl Debug for Category{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_format() {
    let category = Category{
        id: String::from("2"),
        name: String::from("Laptop"),
    };

    println!("{:?}", category)
}

#[test]
fn test_closure() {
    let sum = |mut value1: i32, mut value2: i32|{
        value1 + value2
    };

    let result = sum(10, 20);
    println!("{}", result)
}

fn print_with_filter(value: String, filter : fn(String)->String){
    let result = filter(value);
    println!("{}", result)
}

fn to_uppercase(value: String)->String{
    value.to_uppercase()
}

#[test]
fn test_closure_as_parameter() {
    let filter = |value: String| -> String{
        value.to_uppercase()
    };

    // let adib = String::from("adib");
    print_with_filter(String::from("adib"), filter);
    print_with_filter(String::from("adib"), to_uppercase);
}

#[test]
fn test_vector() {
    let mut names: Vec<String> = Vec::<String>::new();
    names.push(String::from("Adib"));
    names.push(String::from("Hauzan"));
    names.push(String::from("Sofyan"));

    for name in &names{
        println!("{}", name)
    }
    println!("{:?}", names);

    let mut vec: Vec<String>= Vec::new();
    vec.push("2".to_string());
    vec.push("24".to_string());
}

#[test]
fn test_vector_deque() {
    let mut names: VecDeque<String> = VecDeque::<String>::new();
    names.push_back(String::from("Adib"));
    names.push_front(String::from("Hauzan"));
    names.push_front(String::from("Sofyan"));

    for name in &names{
        println!("{}", name)
    }

    println!("{:?}", names[0])
}

#[test]
fn test_hash_map() {
    let mut maps: HashMap<String, String> = HashMap::<String, String>::new();
    maps.insert(String::from("name"), String::from("Adib"));
    maps.insert(String::from("age"), String::from("22"));
    maps.insert(String::from("country"), String::from("Indonesia"));

    for map in &maps{
        println!("{} : {}", map.0, map.1)
    }

    // println!("{:?}", map[0])
}

#[test]
fn test_btree_map() {
    let mut maps: BTreeMap<String, String> = BTreeMap::<String, String>::new();
    maps.insert(String::from("name"), String::from("Adib"));
    maps.insert(String::from("age"), String::from("22"));
    maps.insert(String::from("country"), String::from("Indonesia"));

    for map in &maps{
        println!("{} : {}", map.0, map.1)
    }

    // println!("{}", map[0])
}

#[test]
fn test_iterator() {
    let array:[i32; 5] = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();
    while let Some(value) = iterator.next(){
        println!("{}", value)
    }

    for value in array{
        println!("{}", value)
    }
}

#[test]
fn test_iterator_method() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);

    let count = vector.iter().count();
    println!("{}", count);

    let sum: i32 = vector.iter().sum();
    println!("{}", sum);

    let doubled: Vec<i32> = vector.iter().map(|x| x *2).collect();
    println!("{:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("{:?}", odd);
}

fn connect_database(host: Option<String>){
    match host {
        None => {
            panic!("No database host provided");
        }

        Some(host) => {
            println!("Connecting to database {}", host)
        }
    }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
    connect_database(None);
}

fn connect_cache(host: Option<String>) -> Result<String, String>{
    match host {
        None => {
            Err("No cache host provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_email(email: Option<String>) -> Result<String, String>{
    match email {
        None => {
            Err("No cache email provided".to_string())
        }
        Some(host) => {
            Ok(host)
        }
    }
}

fn connect_application(host: Option<String>)-> Result<String, String> {
    // let connect_cache = connect_cache(host.clone());
    // match connect_cache {
    //     Ok(_) => {}
    //     Err(err)=>{
    //         return Err(err)
    //     }
    // }
    // let connect_email = connect_email(host.clone());
    // match connect_email {
    //     Ok(_) => {}
    //     Err(err) => {
    //         return Err(err)
    //     }
    // }

    connect_cache(host.clone())?;
    connect_email(host.clone())?;
    Ok("Connected to application".to_string())
}

#[test]
fn test_application_error() {
    // let result = connect_application(Some(String::from("localhost")));
    let result = connect_application(None);
    match result {
        Ok(host) => {
            println!("Success connect with message {}", host)
        }

        Err(err) =>{
            println!("Error with messages {}", err)
        }
    }
}

#[test]
fn test_recoverable_error() {
    // let cache = connect_cache(None);
    let cache = connect_cache(Some("localhost".to_string()));
    match cache {
        Ok(host) => {
            println!("Connected to cache at {}", host)
        }

        Err(err) => {
            println!("Error connecting to cache {}", err)
        }
    }
}

#[test]
fn test_dangling_references() {
    // let r: &i32;
    //
    // {
    //     let x: i32 = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r)
}

fn longest<'a>(value1: &'a str, value2: &'a str)-> &'a str{
    if value1.len() > value2.len() {
        value1
    }else { 
        value2
    }
}

#[test]
fn test_lifetime_annotation() {
    let result = longest("adib hauzan sofyan", "kajsdknas");
    println!("{}", result)
}

struct Student <'a, 'b>{
    name: &'a str,
    last_name: &'b str,
}

impl <'a, 'b> Student<'a, 'b> {
    fn longest_student_name(&self, student: &Student<'a, 'b>)-> &'a str{
        if self.name.len() > student.name.len() {
            self.name
        }else {
            student.name
        }
    }

}
fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>)-> &'a str{
    if student1.name.len() > student2.name.len() {
        student1.name
    }else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student = Student {
        name: "adib",
        last_name: "sofyan",
    };
    // println!("{}", student.name)

    let student2 = Student {
        name: "gilang",
        last_name: "saputra"
    };

    let result = longest_student_name(&student, &student2);
    println!("{}", result);

   let result2 =  student.longest_student_name(&student2);
    println!("{}", result2)
}