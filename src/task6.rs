#[test]

/*
// Fill in the blanks to make the last println! work !
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n __ 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        __;
    }

    println!("n reached {}, so loop is over",n);
}
*/


// Fill in the blanks to make the last println! work !
fn main() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }


        n+=1;
    }

    println!("n reached {}, so loop is over",n);
}

/*
В условии цикла while, чтобы цикл выполнялся, пока переменная n меньше 10, нужно использовать оператор <.
В конце каждой итерации цикла переменная n должна увеличиваться на 1, поэтому нужно использовать инкремент: n += 1;.
*/