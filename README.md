# pybites_rust - Exercise downloader
Exercise downloader for https://rustplatform.com/

## quickstart

- install the exercise downloader directly from GitHub, open a terminal and run:
    ```shell
    cargo install --git https://github.com/markgreene74/pybites_rust.git
    ```
- cd to the directory where you want to save the exercises
- run the downloader
    ```shell
    pybites-rust-download
    ```

## compile it manually

Maybe you want to have a look at the code, make some changes, try something new.

- clone the repo and cd to the directory project
- compile the downloader
    ```shell
    cd exercise_downloader && \
        cargo build --release
    ```
- cd back to the project main directory
    ```shell
    cd ..
    ```
- run the downloader from the project main directory
    ```shell
    ./exercise_downloader/target/release/pybites-rust-download
    ```
- the downloader will create `exercises` in the current directory

Alternatively, use the Makefile
```shell
make download-exercises
```

<details><summary>(Output example ...)</summary>

Using `cargo` to install it from GitHub.

```shell
➜ cargo install --git https://github.com/markgreene74/pybites_rust.git
    Updating git repository `https://github.com/markgreene74/pybites_rust.git`
  Installing pybites-rust-download v0.1.2 (https://github.com/markgreene74/pybites_rust.git#d1afb2ec)

(...)


   Compiling pybites-rust-download v0.1.2 (/my/home/.cargo/git/checkouts/pybites_rust-b497f94da89af8aa/d1afb2e/excercise_downloader)
    Finished `release` profile [optimized] target(s) in 15.61s
  Installing /my/home/.cargo/bin/pybites-rust-download
   Installed package `pybites-rust-download v0.1.0 (https://github.com/markgreene74/pybites_rust.git#d1afb2ec)` (executable `pybites-rust-download`)                                                                                                /15.8s

➜
```

Using `make` to compile and execute the exercise downloader.

```shell
➜ make download-exercises
make build-executable && \
exercise_downloader/target/release/pybites-rust-download && \
echo ... all done
make[1]: Entering directory '/my/home/github/pybites_rust'
cd exercise_downloader && \
cargo build --release
    Finished `release` profile [optimized] target(s) in 0.06s
make[1]: Leaving directory '/my/home/github/pybites_rust'
Downloading the exercises from Pybites Rust (rustplatform.com) ✅
'exercises' will be created in the current directory (/my/home/github/pybites_rust/exercises)
21 exercises found!

"Strings and Slices" ✅
"URL Query Parameter Parser" ✅
"Hello Rustacean" ✅
"Vectors and Vec" ✅
"Variables and Mutability" ✅
"Json Serialization" ✅
"Simple Calculations" ✅
"Working with Enums" ✅
"Vowel Counter" ✅
"Using Structs in Rust" ✅
"Fibonacci Sequence" ✅
"Primitive Types" ✅
"Basic Tokenizer" ✅
"Reverse a String" ✅
"Variable Assigment and Mutability" ✅
"Ownership and Borrowing" ✅
"Scopes and Shadowing" ✅
"Function Return Values" ✅
"Result Handling" ✅
"Basic Struct" ✅
"Control Flow" ✅
... all done
```

</details>
