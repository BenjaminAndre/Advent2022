# Advent2022
Advent of code 2022, used as a learning tool for Rust

## Objectives

My only aim with this Advent of Code 2022 is to familiarize myself with Rust. Having started in November 2023, I have no aim for the leaderboard. However, since I'm intersted in the performances of Rust, I'll try to get code to run blazingly fast. Therefore, I'll benchmark my results.

## Gear

My setup is a Ubuntu 22.04 WSL under Windows 11. Here's the specs :

* **CPU** Intel(R) Core(TM) i7-10700 CPU @ 2.90GHz   2.90 GHz
* **RAM** 32,0 Go DDR4
* **SSD** 1To

GPU isn't relevant... yet.

I benchmark against https://github.com/timvisee/advent-of-code-2022, which seems to know what he's doing.

Time is measured after loading input.txt into memory, which might be a bad measure since it doesn't reward optimizing loading time by loading as byte array instad of string.

The two being run one after the other, differences of a few % are not significant, since it can be due to other OS operations. The idea is to have an order of magnetude.

Sometimes, his implementation failed, and I didn't bnother to fix it, so I don't have his result for comparison.

## Benchmarks

|   |Timvisee|Mine|
|---|---|---|
|Day 1a|387.8us|728.7us|
|Day 1b|351.2us|357.9us|
|Day 2a|/|400.2us|
|Day 2b|/|404.9us|
|Day 3a|/|220.6us|
|Day 3b|/|210.5us|
|Day 4a|/|377.8us|
|Day 4b|/|373.6us|
|Day 5a|/|209.0us|
|Day 5b|/|349.0us|
|Day 6a|138.6us|18.0us|
|Day 6b|120.5us|71.5us|

## Further works

I'm disgusted by day 5. It shows a lack of understanding, and gives me a way to continue my Rust journey.

I'm relieved by day 6. It was fun optimizing those bit manipulations, but it probably can be pushed even more. On 6b, Timvisee scripts gives me the wrong answer (14 too low), which is relevant enough to be useful. Might be an error on my end. I'm also starting to think crustaceally : using Option and Result for error management.
