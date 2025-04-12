# Open API Spec and code generation for Rust

## Spec-first approach
I prefer a spec-first approach to REST API development.
This streamlines doc generation, DTOs, handlers / boilerplate and testing. 

## Rust code generation
Rust doesn't yet have great native tooling for open api spec-first code gen as other languages, 
therefore I'm using the open api generator to generate a client in python, and then using that client in rust via pyo3.

For this project I'm using openapi-generator-cli which does have rust support. It needs to be installed to the OS,
and runs on top of the Java runtime (JRE)

The provided file 'install-openapi-gen.sh' will work for Ubuntu-like systems.
```shell

cd ./openapi
chmod +x install-openapi-gen.sh
sudo ./install-openapi-gen.sh
```
