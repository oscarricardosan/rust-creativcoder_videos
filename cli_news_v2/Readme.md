#Tips

Librerías usadas en el proyecto:
* Ureq: para realizar peticiones http https://docs.rs/ureq/0.4.0/ureq
* Colour: para imprimir texto de diferentes colores en laconsola https://docs.rs/colour/0.6.0/colour/
* Api de prueba: https://newsapi.org/docs/endpoints/top-headlines
* Serde: framework para serialización y deserialiación de estructuras de datos https://serde.rs/
* thiserror: Creación de typos de error personalizados de forma rápida y corta https://crates.io/crates/thiserror
* dotenv: Manejo de variables de entorno https://crates.io/crates/dotenv
* termimad: Uso de markdown para imprimir en consola https://crates.io/crates/termimad




##Uso de Cargo add

Para instalar paquetes de forma rápida desde consola se puede usar `cargo add` 

Pero para ello se debe primero instalar cargo-edit desde https://crates.io/crates/cargo-edit#cargo-add 

Con ello ya puedes instalar los paquetes así:

## Uso de deseriualización

Para poder extender una estructura que implemente deserialización debemos poner en Cargo.toml:

~~~ 
serde = {version = "1.0.130", features = ["derive"] }
~~~

