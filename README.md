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
