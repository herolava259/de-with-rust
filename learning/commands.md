# Common + Basic + Neccessary: cargo

- new project: `cargo new <project-name>`
- build project: `cargo build`
- run (include build): `cargo run`
- check build files: `cargo check`
- create a project in current folder: `cargo init .`
    + create binary package
- create lib package: `cargo init --lib <NAME>`
- help: `cargo <specificed-cmd> --help`
- add libL `cargo add <needed-lib>`
- add lib for dev env: `cargo add <libs> --dev`
- only doc-test: `cargo test --doc`
- run doc test in parallel
    + `cargo test -- --test-threads=<num-of-thread(type:number)>`

# setup code-space:


## With Dev-container

### getting started: 
- Ctrl+shift+P => typing: `dev container` => click `Codespaces: Add Dev Container Configuration Files...`
=> Typing `Rust`

### rebuild 

- `Ctrl+Shift+P` => typing `dev containers..` => click rebuild

### add extensions 

- typing a specified ext that u want 
- right click setting icons -> choose add to dev container 


## common cmd in window 

- `cat <file_name>` : read inside the file
- `tree` : read folder structure of the current folder
