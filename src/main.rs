// Struct containing user information
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Struct containing coordinates in a three-dimensional space
struct Point(i32, i32, i32);

// Method implementation for the Point struct
impl Point {
    // Compute the length of the vector as the distance from the origin
    fn length(&self) -> f64 {
        let ss = (self.0.pow(2) + self.1.pow(2) + self.2.pow(2)) as f64;
        ss.sqrt()
    }
}

fn main() {
    // Variables and Mutability
    {
        // Standard
        let x = 5;
        println!("The value of x is: {}", x);

        // Shadowing
        let y = 5;
        println!("The value of y is: {}", y);
        let y = 2 * y;
        println!("The value of y is: {}", y);

        // Mutability
        let mut z = 5;
        println!("The value of z is: {}", z);
        z = 10;
        println!("The value of z is: {}", z);
    }

    // Functions
    {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = square(x);
        println!("The value of x is: {}", x);
    }

    // Control Flow
    {
        // if-else
        let x = 5;
        if x < 5 {
            println!("Less than 5");
        } else if x == 5 {
            println!("Equal to 5");
        } else {
            println!("Greater than 5");
        }

        // let-if
        let y = if x < 5 { 10 } else { 0 };
        println!("The value of y is: {}", y);

        // while
        let mut count = 3;
        while count > 0 {
            println!("{}!", count);
            count -= 1;
        }
        println!("LIFTOFF!");

        // for
        let a = [10, 20, 30, 40, 50];
        for e in a {
            println!("Value: {}", e);
        }
    }

    // Ownership, References, and Borrowing
    {
        // Ownership
        let s1 = String::from("hello");
        let s2 = s1;
        let mut s3 = s2.clone();
        s3.push_str(", world!");
        println!("{}", s2);
        print_str(s3);

        // References and Borrowing
        let s4 = String::from("hello world");
        let r4 = &s4;
        let l4 = calculate_length(r4);
        let s4f = first_word(&s4);
        let l4f = calculate_length(s4f);
        println!("The length of \"{}\" is: {}", s4, l4);
        println!("The length of \"{}\" is: {}", s4f, l4f);
    }

    // Structs
    {
        let u1 = User {
            username: String::from("Hillard28"),
            email: String::from("hillard28@example.com"),
            sign_in_count: 1,
            active: true,
        };
        println!(
            "Username: {}\nEmail: {}\nSign-in Count: {}\nActive: {}",
            u1.username, u1.email, u1.sign_in_count, u1.active
        );

        let u2 = build_user(
            String::from("Hillard28"),
            String::from("hillard28@example.com"),
        );
        println!(
            "Username: {}\nEmail: {}\nSign-in Count: {}\nActive: {}",
            u2.username, u2.email, u2.sign_in_count, u2.active
        );

        let o = Point(1, 2, 3);
        println!("Point: ({}, {}, {})", o.0, o.1, o.2);

        let ol = o.length();
        println!("Vector length: {}", ol);
    }
}

// Basic function with a parameter and return value
fn square(x: i64) -> i64 {
    x * x
}

// Takes ownership of a string and prints it
fn print_str(s: String) {
    println!("{}", s);
}

// Takes a reference to a string slice and returns its length
fn calculate_length(s: &str) -> usize {
    s.len()
}

// Takes a reference to a string and returns the first word as a string slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Builds an initial user from input username and email
fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}
