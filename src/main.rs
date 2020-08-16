#[macro_use]
extern crate mysql;
use std::io;
use mysql as my;
use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Student {
    pub sid: String,
    pub name: String,
    pub email: String,
    pub age: String,
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
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.    
    let students = vec![
        student
    ];
    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO tblstudent
                                       (sid, name, email, age)
                                   VALUES
                                       (:sid, :name, :email, :age)").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
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
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.    
    let students = vec![
       student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"Update tblstudent
                                    set name=:name,
                                    email=:email, age=:age
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
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
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.    
    let students = vec![
        student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"delete from tblstudent                                    
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
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
    // Let's select payments from database
    let selected_students: Vec<Student> =
    pool.prep_exec("SELECT sid, name, email, age from tblstudent", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
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