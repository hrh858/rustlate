trustlate (trl)
---------------
**trustlate** is a command-line tool written in Rust to help you intrgrate translations and internationalization (i18n) you can trust
into your projects.  
It offers the following features:
- Initialize a translations structure for your project.
- Help you write literal and parametrized translations using JSON files.
- Check if all of your translation files comply with the base translations.
- Auto-fill the missing fields in the non-complying translation files.
- Generate ready-to-use clients for your programming language (currently, only Typescript is supported).

### Installation
So far building from source is the only supported way of getting the binary, these are the steps to follow:
 1. Install Rust for your OS and architecture.  
 The best way is to follow [this link](https://www.rust-lang.org/learn/get-started) to get rustup and follow all the instructions until
 you get the cargo command running in your system.

 2. Check that you get output from running:

 ```
 cargo --version
 ``` 

 3. Clone this respository in any directory you want by running:

 ```
 git clone https://github.com/hrh858/trustlate
 ```

 4. Move into the root of the project with:

 ```
 cd trustlate
 ```

 5. From the root of the project to generate a release binary by running:

 ```
 cargo build --release
 ```  

 6. You should be able to get some instructions on how to use trustlate if you run:
 ```
 ./target/release/trustlate
 ```

 7. Finally you have to add the generated binary at `./target/release/trustlate` to the path.  
 This is how yuo would do it for bash:  
 ```
 PATH=$PATH:~/opt/bin
 ```  
 If you're using another shell please search on how to add a new binary to the path.  
 8. **(Optional)** TODO: Add alias.
<!-- This is a basic command-line tool written in Rust which means that as long as you can compile it you can start using it in any -->
<!-- way you want. I did not bother with creating bianries for distribution, so probably the easiest way to get started using trustlate -->
<!-- is to install [rustup](https://rustup.rs/) to get the Rust toolchain and compile the project using cargo. -->
<!---->
<!-- These are the steps to follow: -->
<!-- 1. Make sure you have rustup installed. (You can follow the instructions in [this link](https://rustup.rs/) to get the rustup command-line -->
<!-- and from there follow the rustup's instructions to get the rust toolchain). -->
<!-- 2. Once installed make sure you can run cargo. (You should get some output if you run `cargo version`). -->
<!-- 3. Clone this repository to any directory you want. -->
<!-- 4. Navigate to the root of the project once cloned and run the following command: `cargo build --release`. -->
<!-- 5. Add the genereated binary to the path. (The binary location is `<trustlate_directory>/target/release/trustlate`)   -->
<!--     If you're running bash (usually the default shell) this is how you would do it: -->
<!--     ``` -->
<!--     PATH=$PATH:~/<trustlate_directory>/target/release/trustlate -->
<!--     ``` -->
<!--     If you're running a different shell, I assume you know how to add a binary to your path or have the knowledge to search how -->
<!--     to do so. -->
<!-- <!--     Make sure to `source ~/.bashrc` (or whatever file it is for your shell) or closing and opening your terminal! --> -->
<!-- 6. (Optional) Add an alias `trl` for `trustlate` since it makes it more comfortable to use this way.   -->
<!--     Again if you're running bash this is how you would do it: -->
<!--     1. Run `gedit ~/.bashrc` to start editing your bash configuration. -->
<!--     2. At the end of the file append `alias trl='trustlate'`. -->
<!--     3. Save and close. -->
<!--     4. Run `source ~/.bashrc` or close and open your terminal. -->

### Initialization
You can run the initialization command to create a default configuration file and a set of dummy translations that you 
can start working from.

### Configuration

### Translation files

### Commands
