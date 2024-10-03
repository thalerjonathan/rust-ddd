# Exploring DDD and Microservices with the help of LLMs in Rust 

This repo explores the use of Rust for implementing DDD and Microservices with the help of LLMs on top of a simple web UI in Rust with WASM. 

The Domain which I use as example is a simple Volleyball referee management tool, as I am very familiar with this domain. I have been Volleyball referee for many years in the past, as well as implemented a (much more complex) tool in Java (also following DDD) for the Vorarlberg Volleyball Association, which I used in production during my tenure as Volleyball referee manager of Vorarlberg. The existing tool in Java essentially implements the same Domain but is technically different (Monolith, integration with an existing system).

UI wise, I am using Leptos, due to it being rather close to React which I have some experience in.

Architectural wise, I start simple with a monolith architecture and then transition it into  Microservices. 

Note that in the implementation of this project I have used LLMs in my coding for the first time, where I focused on the use of Cursor and ChatGPT.

# Progress Reports
I have written daily markdown entries to reflect on and report the progress.

## [Day 1](reports/day1/README.md)

- Motivation, Aim & Objectives
- Unknowns, open Questions and Hypotheses
- High-Level Domain Description
- Event Storming
- User Stories
- Domain Model Diagram

## [Day 2](reports/day2/README.md)

Experimented bootstrapping the backend code base with LLMs.

## [Day 3](reports/day3/README.md)

Bootstrapped the UI with Leptos.

## [Day 4](reports/day4/README.md)

Continued and finished bootstrapping the UI with Leptos.

