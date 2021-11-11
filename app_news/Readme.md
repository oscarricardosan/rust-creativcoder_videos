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

##Crear instalador

Para crear instalador para debian primero ejecutar usar https://lib.rs/crates/cargo-deb:
1. Agregar los campos license y description en Cargo.toml
2. `cargo install cargo-deb` instala paquete para generar .deb
3. `cargo-deb` genera el binario .deb
4. Luego de correr el comando podemos ver que ya se genero el .rpm en target/debian/cli_news_v2_0.1.0_amd64.deb


Para fedora/rpm https://lib.rs/crates/cargo-rpm
1. `cargo install cargo-rpm` instala paquete para generar .rpm
2. `cargo-rpm init` configura el crate para hace los rpm
3. `cargo-rpm build -v` genera el .rpm
4. Luego de correr el comando podemos ver que ya se genero el .rpm en ./target/release/rpmbuild/RPMS/x86_64
5. Se instala con el comando `sudo rpm -i cli_news_v2-0.1.0-1.fc34.x86_64.rpm`
6. Luego de ello la aplicación es accesible por consola ejecutando `cli_news_v2`

Si en la consola esta hubicado en el archivo .env el app tomara estos valores de lo contrario no encontrara las variables por ello 
para **cargar las variables de entorno** debemos ejecutar en consola:

export API_URL="https://newsapi.org/v2/top-headlines?country=co&apiKey="
export API_KEY=e161062d10fc4e3f927763a5f743d7e9


