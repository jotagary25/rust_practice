## Sentencias y expresiones
En Rust manejamos ambos conceptos a la hora de construir funciones
- Las sentencias son instrucciones que realizan un acción y no devuelven un valor
  - declarar una variable es una sentencia.
  - se podría decir que las sentencias terminan con un `;` en Rust.
  - una función sin expresión final, es también un sentencia.
- Las expresiones evaluan y producen un valor
  - Una expresión no lleva `;` al final.
  - Llamar a una función es una expresión.

## Condicionales
- En rust se usan los tipicos `if`, `else` y `else if`.
- No existe conversión, por tanto un 'integer' no será 'true' en una evaluación.
- Usar If en una sentencia: `let x = if condicion { valor_si } else { valor_si_no }`
- En una Expresión if, else, el resultado debe ser del mismo tipo.

## Bucles
- loop
  - Usamos `break` para salir de un bucle y `continue` para saltar a la siguiente iteración.
  - Si tenemos un bucle dentro de otro, podemos etiquer un loop con `'etiqueta: loop {}`.
  - Podemos usar la etiquesta para indicar que bucle romper o saltar `break 'etiqueta;`.
- for - while
  - bucle while: `while var < 10 { cuerpo }`
  - bucle for: `for element in array { cuerpo }`
