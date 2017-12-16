# What is Maid?
**Maid is a project manager for C and C++ that enables anyone to create a project and get started working, without needing to worry about IDEs, or their compiler and its hundreds of options.**

I was never fond of writing C or C++. I liked the language, but it all just felt so complicated; being required to memorize tons of compiler options, CMake commands, and trying to plan ahead for users of different development environments.

When I met [Cargo](https://github.com/rust-lang/cargo), I knew that I wanted the same for C++. I liked the simple configuration file and not having to deal with anything other than writing my code. It's just a few things that would make it difficult to create a Cargo-like software for C/C++: dependencies come in many forms, I don't have a lot of access to the compiler, and I've honestly never made anything like this before.

Instead, I tried to forget all the things that were going to hold me back, and just started making *something that could compile the code you give it, manage it, and enable you to work with a high-level interface over your compiler.* It definitely makes it easier to start projects, and I'm starting to enjoy C and C++ development more.
# FAQ
## Why so many comments in the code?
I don't usually comment my code like that, but *I'd rather someone know way more than needed, than not having a clue.*
## Does using Maid decrease compilation flexability?
Currently, it does. But in the future, when I've had a chance to work on compilation more, you'll never need to build manually. **I aspire to make operating systems, and if I thought that Maid wouldn't be able to do this for me, I'd never have started working on it.**