#[test]

/*

// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
    }

    println!("{:?}", numbers);
}
*/


// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for _name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for _n in numbers {
        // Do something with n...
    }

    println!("{:?}", numbers);
}

/*
В коде есть ошибка, связанная с перемещением данных.
Массив names состоит из объектов типа String, которые не реализуют трейт Copy, поэтому происходит
перемещение значений в цикле for. Это приведёт к тому, что после итерации переменные станут недоступны,
и при попытке доступа к массиву произойдет ошибка компиляции.
*/