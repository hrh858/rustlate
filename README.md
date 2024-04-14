trustlate
----------
**trustlate** is a command-line tool written in Rust to help you intrgrate translations and internationalization (i18n) you can trust
into your projects.  
It offers the following features:
- [Initialize a translations structure for your project.](#initialization)
- [Help you write literal and parametrized translations using JSON files.](#translations)
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
 If you're using another shell please search on how to add a new binary to the path for such shell.  

8. **(OPTIONAL)** Ideally you will integrate trustlate commands into your current workflow so you won't be interacting with trustulate 
directly most of the time. However, if that's the case, typing `trustlate` all of the time can start to feel a little longer than desirable. 
In that case crating an alias may help.  
This is how you would create a `trlt` alias for trustlate in bash:
```
echo "alias trustlate='trlt'" >> ~/.bashrc
```  
If you're using another shell please search on how to add an alias.

### Initialization
You can run the initialization command to create a default configuration file and a set of dummy translations that you 
can start working from.

### Configuration
The configuration file will be created for you if you initialized trustlate following [the previous section](#initialization).  
It consist on a JSON file called `.trsutlaterc.json` that contains the following fields:  
| Field | Type | Default Value | Description |
|-------|------|---------------|-------------|
| base_lang | string | es | TODO |
| target_langs | string[] | ["kr", "es"] | TODO |
| codegen | string | "Typescript" | TODO |
| source_dir | string | "./trustlate/trasnalations/" | TODO |
| target_dir | string | "./trustlate/codegens/" | TODO |

### Translation files

### Commands
