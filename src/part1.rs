fn main() {
    // Crear variables, con let es un binding value
    let x1 = 42;

    // Se puede tipar explicitamente con i32 (tpye anottation)
    let x2: i32 = 42; // tambien i16,i32... u16,u32 para unsigned.

    // Para variables sin inicializar no me dejara usarla
    // let x;
    // foobar(x); ME DARA ERROR!!
    // x = 42;

    // Las barras bajas son para descartar valores
    let file = 39; // Abre un archivo.
    let _ = file; // ¡Peligro! Lo descarta inmediatamente, el archivo se cierra aquí.

    let _file2 = 39; // las barras bajas variables se descartan al final de la func
    //

    // TUPLA
    // la  definiremos como colección de tamaño fijado de diferentes tipos
    let pair: (i32, char) = (23, 's');
    pair.0; // 23
    pair.1; // 's'

    // Se puede separar tambien
    let (valor1, valor2) = ('s', 12);

    // el punto y coma es el final de statement, se pueden tener varias lineas sin eso:
    let x3 = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);

    //BLOQUES: SON COMO CONTENEDORES DE CODIGO
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);

    // también se puede tratar a un bloque como una expresión por lo que podemos decir que x =
    // {algo}
    // Lo que pongas sin semicolon será lo que DEUVELVA una expresión.
}

//Funciones:
fn fair_dice_roll() -> i32 {
    4 // No hay punto y coma, es el valor de retorno
}

fn fair_dice_roll2() -> i32 {
    return 4; // Se retorna con coma
}

// If y match también funcionan de esta forma:

fn fair_dice_roll3() -> i32 {
    let feeling_lucky = true;

    if feeling_lucky { 1 } else { 6 }
}

fn fair_dice_roll4() -> i32 {
    let feeling_lucky = true;

    match feeling_lucky {
        true => 6,
        false => 4,
    }
}

fn funcs() {
    // se puede usar use y coger la funcion, por ejemplo:
    let least = std::cmp::min(3, 8); // this is 3
    //Tamabien llamarla cuando se quiera si se importa:
    use std::cmp::min;
    let least = min(7, 1); // this is 1
    // this also works:
    // use std::cmp::{max, min};
    // this also works!
    //use std::{cmp::max, cmp::min};
    // and finally u can import *: use std::cmp::*;
    // Especificar el tipo completo de la variable
}

fn vectores_y_structs() {
    //str is a primitive type, but many non-primitive types are also in scope by default.
    // `Vec` is a regular struct, not a primitive type
    let v: Vec<i32> = Vec::new();

    // this is exactly the same code, but with the *full* path to `Vec`
    let v2: Vec<i32> = std::vec::Vec::new();

    struct Vec2Struct {
        x: f64,
        y: f64,
    }

    //Y ahora puedo crear objetos con esa estructura
    let v1 = Vec2Struct { x: 3.0, y: 4.0 };
}

// Todo viene de https://fasterthanli.me/articles/a-half-hour-to-learn-rust
//
