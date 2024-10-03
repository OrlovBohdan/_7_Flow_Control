#[test]

/*
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}
*/

fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() {
        println!("The {}th element is {}",i+1,v);
    }
}

/*
В Rust для итерации по индексам и значениям массива можно использовать метод iter().enumerate().
Он возвращает кортежи, где первый элемент — это индекс, а второй — значение.
*/