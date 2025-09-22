### Tipos de datos
En Rust hay 2 tipos de datos: **Escalares** y **Compuestos**
Los tipos de datos escalares con: enteros, flotantes, booleanos y caracteres,

#### tipo de datos Enteros
- Los enteros pueden ser Isize o Usize (i8, i32, u8, u32)
- i8 -> [-128, 127] | u8 -> [0, 255]
- es posible nombrar un entero de la siguiente forma 1000 o 1_000
- la capacidad maxima de bits (i32, i64, i128) dependerá de la arquitectura del cpu (32 bits, 64bits)
- el tipo de entero determinado en Rust es u32
##### desbordamiento para enteros
Violar el limite de un tipo de dato provoca desbordamiento, podemos verificar el desbordamiento cuando compilamos con Rust en modo debug, y podemo omitir que Rust compruebe el desbordamiento en modo release, Si Rust detecta que hay desbordamiento detendra la ejeción y lanzará un `panic!`
Cuando Rust omite el desbordamiento, en realidad lo que esta haciendo es hacer una operación de wrappint, (dar la vuelta), así por ejemplo si el limite es 128, y se llega a 129, Rust asignara 0.

#### tipo de datos flotantes
En rust solo hay 2 tipos flotantes, f32 y f64, el predeterminado es f64 porque tiene la misma velocidad que f32 pero con mayor precisión.

#### tipo booleano
solo hay 2, true o false, ocupan 1 byte y usan con `bool`

#### tipo caracter
se nombran con `char`, ocupan 4 bytes y se utiliza comillas simples para indicarlos (''), Rust usa Unicode.

### Tipo Compuestos
Permiten agrupar multiples valore de distintos tipos, solo hay 2: tuplas y arrays.
#### tuplas
- las tuplas tienen un tamaño fijo y sus elementos se separan con una coma `,`
- se representan con parentsis `()`
- el indice de una tupla empieza en cero, accedemos a ellas con `.`, ejemplo: `variable.0`

#### arrays
- los array también son de longitud fija, y estos solo admiten que todos seán del mismo tipo.
- Para indicar la longitud y el tipo se usan `[tipo: longitud]`
- para inicializar un array se usa corchetes `[]`, ejemplo: `[1, 2, 3]`.
- El indice comienza en cero y para acceder usamos: `variable[indice]`
- Como el tamaño es fijo, si intentamos acceder a un indice no valido, Rust lanzará un error, algo poco común en lenguajes de bajo nivel y una caracteristica importante de Rust.
