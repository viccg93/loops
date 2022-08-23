//en Rust existen 3 tipos de loop: loop, while y for
fn main() {

    //estructura de loop (se comenta por que solo se ejecutaria ese bloque hasta que se termine el proceso)
    //loop {
        //println!("again");
    //}

    //para que un loop no se ejecute sin fin, tenemos la instruccion break
    //en caso de no querer ejecutar el codigo restate de la iteracion y pasar a la siguiente se usa continue
    let limit = 3;
    let mut counter = 0;
    loop {
        if counter < limit {
            println!("Again");
            counter = counter + 1;
        }else{
            break;
        }
    }

    //los loop tambien pueden retornar valores, para ello se indica la expresion despues de break

    let mut counter = 0;

    let result = loop {
        counter += 1; //abreviacion
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("el valor de result es {result}");

    //los loop se pueden etiquetar usando 'etiqueta, esto es funcional cuando
    //se usan loops anidados y existen multiples break continue, estos tambien se etiquetan de acuerdo
    //al loop que estan impactando.

    let mut count = 0;
    //este loop se marca como counting_up
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        // este loop no tiene etiqueta
        //cuando se use break sin etiqueta afectara a este loop
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; //sale del loop interior
            }
            if count == 2 {
                break 'counting_up; //sale del loop exterior
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while

    //Rust tiene un constructo while que permite acortar lo que se lograria con una mezcla de loops, if, breaks

    let mut number = 0;

    while number < 3 {
        number+=1;
        println!("el valor de number es {number}");
    }

    //una tarea que se puede realizar con while es recorrer los elementos de un array o tuple

    let a: [u8;5] = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("el valor del elemento es {}", a[index]);
        index+=1;
    }

    //aunque recorrer una coleccion con while es efectivo, no es del todo eficiente
    //ya que es propenso a errores en caso de que index tenga un valor mayor a los indices
    //posibles en la coleccion, algo que se puede dar con facilidad si se cambia la definicion de la coleccion

    //es recomendable usar en estos casos el constructo for, que itera sobre los elementos de manera implicita
    //evitando un panic index out of bounds, ademas de que es una implementacion mas concisa.

    let a: [u8;5] = [10,20,30,40,50];
    for elemento in a {
        println!("el valor del elemento es {}", elemento);
    }

    //for es el constructo mas usado en Rust, incluso para tareas que no involucran arrays
    //la libreria estandar nos facilita los rangos que nos brindan todos los numeros entre 2 limites
    //el metodo rev() invierte el rango.
    for elemento in (1..4).rev() {
        println!("{elemento}!")
    }
    println!("LIFTOFF!");
}
