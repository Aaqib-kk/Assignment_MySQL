#[macro_use]
extern crate mysql;
use std::io;
use mysql as my;
use std::fmt;
#[derive(Debug, PartialEq, Eq)]
struct Student {
    sid: String,
    name: String,
    email: String,
    age: String,
}
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input = input.trim().parse().unwrap();
    input
}

fn main() {
    println!("what function is required in CRUD (create, read, update, delete)");
    let mut choice = read_input();
    let choice:&str = choice.trim();

    println!("Enter your sid");
    let sid = read_input();
    println!("Enter your name");
    let name = read_input();
    println!("Enter your email");
    let email = read_input();
    println!("Enter your age");
    let age = read_input();

    let mut student = Student{sid:(sid),name:(name),email:(email),age:(age)};
    
    match choice {
        "create" => insert(student),
        "read" => fetch(),
        "update" => update(student),
        "delete" => delete(student),
        _ => println!("wrong parameter is entered"),
    }

    
}

fn insert(student: Student){
    let pool = my::Pool::new("mysql://root:@localhost:3306/Mak1DB").unwrap();    
    let students = vec![
        student
    ];
    for mut stmt in pool.prepare(r"INSERT INTO tblstudent
                                       (sid, name, email, age)
                                   VALUES
                                       (:sid, :name, :email, :age)").into_iter() {
        for s in students.iter() {
            stmt.execute(params!{
                "sid" => &s.sid,
                "name" => &s.name,
                "email" => &s.email,
                "age" => &s.age,
            }).unwrap();
        }
    }
}
fn update(student:Student){
    let pool = my::Pool::new("mysql://root:@localhost:3306/Mak1DB").unwrap();    
    let students = vec![
       student
    ];
    for mut stmt in pool.prepare(r"Update tblstudent
                                    set name=:name,
                                    email=:email, age=:age
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            stmt.execute(params!{
                "sid" => &s.sid,
                "name" => &s.name,
                "email" => &s.email,
                "age" => &s.age,
            }).unwrap();
        }
    }
}
fn delete(student: Student){
    let pool = my::Pool::new("mysql://root:@localhost:3306/Mak1DB").unwrap();    
    let students = vec![
        student
    ];

    for mut stmt in pool.prepare(r"delete from tblstudent                                    
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            stmt.execute(params!{
                "sid" => &s.sid,
                // "name" => &s.name,
                // "email" => &s.email,
                // "age" => &s.age,
            }).unwrap();
        }
    }
}

fn fetch(){
    let pool = my::Pool::new("mysql://root:@localhost:3306/Mak1DB").unwrap();
    let selected_students: Vec<Student> =
    pool.prep_exec("SELECT sid, name, email, age from tblstudent", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        result.map(|x| x.unwrap()).map(|row| {
            let (sid, name, email, age) = my::from_row(row);
            Student {
                sid: sid,
                name: name,
                email: email,
                age: age,                
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    for s in 0..selected_students.len(){
        println!("ID: {:?} Name: {:?} Email: {:?} Age: {:?}",selected_students[s].sid,selected_students[s].name,
                        selected_students[s].email,selected_students[s].age);

    }    
}