# pybites_rust
My solutions for https://rustplatform.com/

## exercises

(placeholder)

## exercises_downloader

- compile the downloader
```shell
cd exercises_downloader && \
    cargo build --release
```
- cd back to the project main directory
```shell
cd ..
```
- run the downloader from the project main directory
```shell
./exercises_downloader/target/release/exercises_downloader
```
- the downloader will create `exercises` in the current directory

Alternatively, use the Makefile
```shell
make download-exercises
```

<details><summary>(Output example ...)</summary>

```shell
pybites_rust [ main][+]
➜ make download-exercises
make build-executable && \
exercises_downloader/target/release/exercises_downloader && \
echo ... all done
make[1]: Entering directory '/my/home/github/pybites_rust'
cd exercises_downloader && \
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
