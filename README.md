trustlate (trl)
---------------
**trustlate** is a command-line tool written in Rust to help you manage translations and internationalization (i18n) in your projects.
It offers the following features:
- Initialize a translations structure for your project.
- Helps you write literal and parametrized translations using JSON files.
- Check if all of your translation files comply with the base translations.
- Auto-fill the missing fields in the non-complying translation files.
- Generate ready-to-use clients for your programming language (currently, only Typescript is supported).

### Installation
This is a basic command-line tool written in Rust which means that as long as you can compile it you can start using it in any
way you want. I did not bother with creating bianries for distribution, so probably the easiest way to get started using trustlate
is to install [rustup](https://rustup.rs/) to get the Rust toolchain and compile the project using cargo.

These are the steps to follow:
1. Make sure you have rustup installed. (You can follow the instructions in [this link](https://rustup.rs/) to get the rustup command-line
and from there follow the rustup's instructions to get the rust toolchain).
2. Once installed make sure you can run cargo. (You should get some output if you run `cargo version`).
3. Clone this repository to any directory you want.
4. Navigate to the root of the project once cloned and run the following command: `cargo build --release`.
5. Add the genereated binary to the path. (The binary location is `<trustlate_directory>/target/release/trustlate`)  
    If you're running bash (usually the default shell) this is how you would do it:
    ```
    PATH=$PATH:~/<trustlate_directory>/target/release/trustlate
    ```
    If you're running a different shell, I assume you know how to add a binary to your path or have the knowledge to search how
    to do so.
    Make sure to `source ~/.bashrc` (or whatever file it is for your shell) or closing and opening your terminal!
6. (Optional) Add an alias `trl` for `trustlate` since it makes it more comfortable to use this way.  
    Again if you're running bash this is how you would do it:
    1. Run `gedit ~/.bashrc` to start editing your bash configuration.
    2. At the end of the file append `alias trl='trustlate'`.
    3. Save and close.
    4. Run `source ~/.bashrc` or close and open your terminal.

### Initialization
You can run the initialization command to create a default configuration file and a set of dummy translations that you 
can start working from.

### Configuration

### Translation files

### Commands
