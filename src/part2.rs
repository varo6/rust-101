struct MyNumber {
    par: bool,
    value: i32,
}

pub fn main() {
    struct Vec2 {
        x: f64,
        y: f64,
    }

    let v2 = Vec2 { x: 2.0, y: 3.0 };

    // PODEMOS decirle a rust que use los DEMAS valores de v2 que no se han declarado en el vector
    let v3: Vec2 = Vec2 { x: 5.0, ..v2 };
    let n: MyNumber = MyNumber {
        par: true,
        value: 2,
    };
    patterns_destructuring(n);
}

fn patterns_destructuring(n: MyNumber) {
    // Podemos usar patrones para usar let como condicionales
    if let MyNumber { par: true, value } = n {
        println!("Es par el numero: {}", value);
    } else if let MyNumber { par: false, value } = n {
        println!("Es impar el numer: {}", value);
    }
}

// Se puede hacer condicionales con match para estructuras
fn match_structs(n: MyNumber) {
    match n {
        //DIFERENCIA ENTRE EL .. y el par: en el .. ignora lo que haya, por lo que no puedo
        //imprimir n.par, en el caso dos si que haría par: n.par osea que si puedo imprimirlo
        MyNumber { value: 1, .. } => println!("Es un uno"),
        MyNumber { value: 2, par } => println!("Es un dos"),
        MyNumber { value, par } => println!("No es ninguno de los dos"),
    }
}

//Otra forma más rapida
fn match_2(n: MyNumber) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}

// Metodos en Estructuras. Si le pasas self es metodo de instancia. Si no es una función
impl MyNumber {
    fn is_strictly_positive(&self) -> bool {
        self.value > 0
    }
}
