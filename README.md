# Calculator-with-Rust

![alt text](<Captura desde 2024-11-26 22-15-03.png>)

- Este es el módulo que permite leer entradas desde la consola usando `io::stdin()`.

![alt text](<Captura desde 2024-11-26 22-19-23.png>)

- Pedimos al usuario que seleccione la operación que desea realizar.

![alt text](<Captura desde 2024-11-26 22-21-02.png>)

- `let mut operation = String::new();` crea una cadena mutable donde almacenaremos la entrada del usuario.
- `io::stdin().read_line(&mut operation);` lee la entrada desde la consola y la guarda en `operation`.

![alt text](<Captura desde 2024-11-26 22-28-39.png>)

- `.trim()` elimina espacios en blanco y saltos de línea.
- `.parse()` convierte la cadena a un entero (`i32`). Si el usuario no introduce un número válido, el programa se detendrá con el mensaje de error proporcionado en `expect()`.

![alt text](<Captura desde 2024-11-26 22-31-27.png>)

- Nos aseguramos que el usuario ha seleccionado la operación correcta (entre 1 y 4). Si no, el programa muestra un mensaje de error.

![alt text](<Captura desde 2024-11-26 22-32-50.png>)

- Se lee el primer número (`num1`) como una cadena, se limpia y se convierte a un número flotante (`f64`) usando `.parse()`.
- Este proceso se repite para el segundo número (`num2`).

![alt text](<Captura desde 2024-11-26 22-35-25.png>)

- Realizamos la operación de acuerdo a la selección del usuario.
- `match operation` evalúa el valor de `operation` y selecciona la operación correspondiente.
- Si la operación seleccionada es 4 (division), se agrega una validación para evitar realizar división por cero.

![alt text](<Captura desde 2024-11-26 22-37-53.png>)

- Imprimimos el resultado de la operación.

![alt text](<Captura desde 2024-11-26 22-39-04.png>)

- Manejamos el caso de que un usuario seleccione una operación inválida.

### Ejemplo de Ejecución:

> **Entrada válida:**
>
> **Usuario**
>
> ![alt text](<Captura desde 2024-11-26 22-41-43.png>)
>
> **Salida**
>
> ![alt text](<Captura desde 2024-11-26 22-42-25.png>)
>
> **Entrada inválida (División por cero):**
>
> **Usuario**
>
> ![alt text](<Captura desde 2024-11-26 22-43-47.png>)
>
> **Salida**
>
> ![alt text](<Captura desde 2024-11-26 22-44-37.png>)
>
> **Operación seleccionada inválida:**
>
> **Usuario**
>
> ![alt text](<Captura desde 2024-11-26 22-45-27.png>)
>
> **Salida**
>
> ![alt text](<Captura desde 2024-11-26 22-46-05.png>)
>
- Esta es una breve explicación de lo que sería una calculadora básica escrita con Rust.
