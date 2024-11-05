// struct:
// A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented programming language, a struct is like an object’s data attributes.

// use std::fmt::Debug;

use std::f64::consts::PI;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub active: bool,
}

// impl Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "User: id: {}, username: {}, email: {}, active: {}", self.id, self.username, self.email, self.active)
//     }
// }

pub fn get_user() -> User {
    let user = User {
        id: 1,
        username: String::from("Honnur Ali"),
        email: String::from("honnurali68@gmail.com"),
        active: true,
    };
    user
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
pub struct Circle{
    pub radius: f64,
    // pub sqaure: u64,
}

impl Circle{
    pub fn area_of_circle(&self) -> f64{
        PI * self.radius * self.radius
    }
}
