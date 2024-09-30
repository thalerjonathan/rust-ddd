# Exploring DDD and Microservices in Rust

This repo explores the use of Rust for implementing DDD and Microservices on top of a simple web UI in Rust with WASM. Also with this project I am  exploring the use of LLMs in my coding for the first time.

The Domain which I use as example is a simple Volleyball referee management tool, as I am very familiar with this domain. I have been Volleyball referee for many years in the past, as well as implemented a (much more complex) tool in Java (also following DDD) for the Vorarlberg Volleyball Association, which I used in production during my tenure as Volleyball referee manager of Vorarlberg.

UI wise, I am using Leptos, due to it being rather close to React which I have some experience in. Also, I was quite impressed by it when I worked through the documentation. 

Architectural wise, I start simple with a monolith architecture and then transition it into  Microservices. 

The LLMs used were Copilot in VSCode and Cursor.

# Progress Reports
I have written daily markdown entries in this repo which reflect on and report the progress.

## [Day 1](reports/day1/README.md)
Covers the Domain and requirements:
- Motivation, Aim & Objectives
- Unknowns, Open questions and Hypotheses
- High Level Vision
- Event Storming
- Use Cases
- UML Diagram
