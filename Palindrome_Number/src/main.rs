
fn main() {

    /*

    1.- First method using aritmetic operations, leetcode doesn't like this one but it works, I think.

    let mut string_number = String::new();

    io::stdin().read_line(&mut string_number)
        .expect("No se pudo leer la linea");

    let mut number:i32 = string_number.trim().parse()
        .expect("Como paso esto wn");

    let mut string_len:i32 = string_number.len().try_into().unwrap();

    loop {

        string_len -= 2;

        println!("stringlen = {}", string_len);
        println!("number = {}", number);
        println!("First digit = {}", number/(10_i32.pow(string_len.try_into().unwrap())));
        println!("Last digit = {}", number%10);

        if number%10 == number/(10_i32.pow(string_len.try_into().unwrap())) {
            number = number - number/(10_i32.pow(string_len.try_into().unwrap())) * 10_i32.pow(string_len.try_into().unwrap());
            number = number /10;
        }else{
            println!("False");
            break;
        }

        println!("Number after changes = {}", number);

        if string_len <= 1{
            println!("True");
            break;
        }
    }

    2.- So, I make a try with references and how I don't actually know how to use them
    I have to copy a solution, at least I understand the logic and how it works, i think.
    but because I don't commit it and don't save it, it's no longer with us :(
    and I'm to lazy to search and paste it again
    */

    // Finally I watch the official solution, It was easy and simple than I though
    // I can just say I don't know how to think.
    
    let mut x:i32 = 121;

    if x < 0 || (x % 10 == 0 && x != 0) {
        println!("False")
    }
    
    let mut rev:i32 = 0;

    while x > rev {
        rev = rev * 10 + x % 10;
        x = x / 10;
    }

    if x == rev || x == rev/10 {
        println!("True")
    }else{
        println!("False")
    }
}
