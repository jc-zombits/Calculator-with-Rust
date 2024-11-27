# Calculator-with-Rust

![alt text](<Captura desde 2024-11-26 22-15-03.png>)

- Este es el módulo que permite leer entradas desde la consola usando `io::stdin()`.

![alt text](<Captura desde 2024-11-26 22-19-23.png>)

- Pedimos al usuario que seleccione la operación que desea realizar.

![alt text](<Captura desde 2024-11-26 22-21-02.png>)

- **let mut operation = String::new();** crea una cadena mutable donde almacenaremos la entrada del usuario.
- **io::stdin().read_line(&mut operation);** lee la entrada desde la consola y la guarda en **operation**.



