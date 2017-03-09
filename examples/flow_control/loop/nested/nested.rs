#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Вошли во внешний цикл");

        'inner: loop {
            println!("Вошли во внутренний цикл");

            // Это прервёт лишь внутренний цикл
            //break;

            // Это прервёт внешний цикл
            break 'outer;
        }

        println!("Эта точка не будет достигнута");
    }

    println!("Вышли из внешнего цикла");
}
