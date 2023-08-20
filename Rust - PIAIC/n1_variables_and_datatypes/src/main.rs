// cargo new <projectName>  => to create a Rust project
// cargo check  => it do not run program but only check program has errors or not
// cargo build  => makes an executable file
// cargo run  => to run program

// We can also define constant variables globally
const PI:f64 = 3.164;

fn main() {
    println!("Hello, world!");

    // In Rust variables are by default immutable (cannot be modified)
    // I have intentionally putted this variable unused so I have to put _ before variable name or else it will give warning
    let _temperature = 35;
    // temperature = 12; we cannot do this

    // If we want to make a variable which is mutable or can be changed, we have to use 'mut' keyword
    let mut weather = "Good";
    println!("Weather is: {}", weather);
    weather = "Bad";
    println!("Weather is: {}", weather);

    // With const we have type annotation is essential (We have to tell the data type)
    const MAX_VALUE:u32 = 34;
    println!("MAX_VALUE is: {}", MAX_VALUE);
    println!("PI is: {}", PI);

    //Shadowing means re-initializing a variable with a different value For example we can change datatype of variable suc as variable in integer can be shadowed to variable of string
    let x = 8;
    let x = x*2;  // Here first variable is shadowed by second variable
    println!("x is: {}", x);
    let x:f64 = 6.1;
    println!("x is: {}", x);

    // In primitive Datatype Their is a Scalar Datatype and Compound Datatype
    // Scalar Datatype can only store one type of value, Rust can understand them without explicitly telling Rust
        //  Length            Signed              Unsigned
        //  ------            ------              --------
        //  8-bit             i8                  u8
        //  16-bit            i16                 u16
        //  32-bit            i32                 u32
        //  64-bit            i64                 u64
        //  128-bit           i128                u128
        //  arch              isize               usize
    // Compound Datatype can store compound type of value

    let age = 19;
    let percentage = 83;
    let grade = 'A';
    let pass = true;
    println!("Age is: {}, Percentage is: {}, Grade is: {}, Pass is: {}", age, percentage, grade, pass);

    let secret_number : i32 = -80;
    println!("secret_number is: {}", secret_number);

    // These codes are to tell compiler that which number system is this
    let decimal_number = 100;
    let hexadecimal_number = 0xff;  // 0x is used to tell compiler that it is hexadecimal
    let octal_number = 0o77;  // 0o is used to tell compiler that it is octal
    let binary_number = 0b1010101;  // 0b is used to tell compiler that it is octal
    let character = b'A';  // b'' is used to tell compiler that it is octal
    println!("decimal_number is: {}", decimal_number);
    println!("hexadecimal_number is: {}", hexadecimal_number);
    println!("octal_number is: {}", octal_number);
    println!("binary_number is: {}", binary_number);
    println!("character is: {}", character);

    // In compound Datatype there are Array and tuple where array is homogenous while tuple is heterogeneous

    let student : (u32, char, f64) = (25, 'A', 83.5);  // It is a tuple. Annotation is not necessary here
    println!("student details are: {}, {}, {}", student.0, student.1, student.2);
    let (x,y,z) = student;
    println!("student details with destructuring are: {}, {}, {}", x, y, z);

    let lottery_number : [u32;6] = [1,2,3,87,45,76];  // This is array
    println!("lottery_number is: {:#?}", lottery_number);
    let rupees_five = [5;10];  // It means there are 10 element and each element is 5
    println!("rupees_five is: {:#?}", rupees_five);
    let days = ['M', 'T', 'W'];
    println!("days is: {:#?}", days[1]);
}
