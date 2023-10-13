# Haochong-week7-mini-repo
[![Tests](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/lint.yml)

This is a repo template for course 706_Data_Engineering Week 7 Mini Project. I choose to finish the Rust lab, so my requirement is little different. First of all, I selected the folder `caesar-cipher-cli`. Then, I read and run the code in `lib.rs` and `main.rs` to learn what the code is doing and how it makes things happen. After that, I came up with an idea to create my own function, `int_to_ascii`, which can transfer a number to the correspond character according to ASCII table as one more processing logic. To fulfill the other requirement, I also add a new command line argument `int_to_ascii` to fit my new function and add function `read_input` to accept file format input with `.txt` file, and change the format of the output to make more sense. I also create `test.rs` to test the functions provided by professor and my own function. Finally, I use Action to run `Makefile` provided by professor and got a 100% pass. 

Important file:
* `Makefile`
* `lib.rs`
* `main.rs`
* `test.rs`
* `test.txt`
* `Cargo.toml`

## Purpose
1. Fork the repository at **https://github.com/nogibjj/rust-data-engineering**

2. Clone your forked repository 

3. Navigate to one of the command-line tool projects

4. Make a small modification to the tool such as:

   - Adding a new command line argument
    
   - Supporting additional input file formats
    
   - Adding more processing logic
    
   - Changing output formatting

5. Run `cargo build` to compile your changes  

6. Run `cargo run` to test your modified tool

7. Commit your changes and push to your forked repository


## Preparation 
1. Open codespaces and vscode
2. Open github and find professor's repo. Then, fork it and clone it to local

## Guide for using my tool
1. Fiset, set up for Rust. Install Rust and Cargo

Use this command to install them: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After that, I restart the shell by using 
```
source $HOME/.cargo/env
```

Then, I check the version to confirm that they are installed successfully by using
```
rustc --version
cargo --version
```

If you get a version, then you install correctly. I got this:

![Alt text](<截屏2023-10-11 下午9.25.04.png>)

2. Make sure we have `Cargo.toml` under the project folder, and run 
```
cargo build
```
```
cargo run
```

3. You can do three things with my tool: encrypt a string, decrypt a string, and transfer a int value to a correspond character in ASCII table.

* To encrypt:
```
cargo run --  --message "your string" --encrypt --shift "your shift value"
```


* To decrypt:
```
cargo run --  --message "your string" --decrypt --shift "your shift value"
```

* To transfer a int value to a correspond character in ASCII table, replace "int" with your number:
```
cargo run -- --int-to-ascii --message int
``` 

* To use file format input, change the content after message to the `.txt` file name:
```
cargo run -- --encrypt --message test.txt --shift 3
```

```
cargo run -- --int-to-ascii --message test.txt
```

* For mroe information, run:
```
cargo run -- --help
```

## Check format and test errors
1. Format code  by using `make format`

2. Lint code  by using `make lint`. 

3. Test code by using `make test`

I create two tests in the `test.rs`, one to test both `encrypt` and `decrypt`, the other one to test my `int_to_ascii`. I got them all passed.

![Alt text](<截屏2023-10-12 上午12.10.50.png>)

Other kinds of validation of my tool can be varified by output, here are some examples.

* Examples:
   - I updated the output format for those two functions that provided by professor:

   Running 
   ```
   cargo run --  --message "Off to the bunker. Every person for themselves" --encrypt --shift 10
   ```
   I got: 
   ```
   Encrypted message: Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc
   ```

   Running 
   ```
   cargo run --  --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --decrypt --shift 10
   ```
   I got: 
   ```
   Decrypted message: Off to the bunker. Every person for themselves
   ```

   - For my own function:

   Running 
   ```
   cargo run -- --int-to-ascii --message 65
   ```
   I got: 
   ```
   Integer: 65 corresponds to ASCII character: A
   ```

   Running 
   ```
   cargo run -- --int-to-ascii --message 140
   ```
      I got: 
   ```
   Error: Input not valid! Should be less than or equal to 127.
   ```

   - For file format input, with "Off to the bunker. Every person for themselves" inside the test.txt:

   Running 
   ```
   cargo run -- --encrypt --message test.txt --shift 10
   ```
   I got: 
   ```
   Encrypted message: Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc
   ```

   Running 
   ```
   cargo run -- --int-to-ascii --message test.txt
   ```
   Since the content is not number, I got: 
   ```
   Error: Invalid integer value provided.
   ```

   - If you run the code without specifing function name, you will get a reminder:

   Running 
   ```
   cargo run -- --message 65 
   ```
   I got: 
   ```
   Please specify either --encrypt, --decrypt, or --int-to-ascii
   ```

## Makefile

I have a makefile under folder `Haochong-week7-rust`. Besides, under `caesar-cipher-cli`, there is also a makefile useing this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```
## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
