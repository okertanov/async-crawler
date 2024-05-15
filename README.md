AOX Trade test task: async crawler
==================================

TLTR: How to run
----------------

    make build
    make ckeck
    make test
    make run
    make clean

    make docker-build
    make docker-test
    make docker-run
    make docker-down
    make docker-clean

    make dist-clean


Task Description:
-----------------
> Your task is to create a Rust application that fulfills the following requirements:
> 
> 1. Utilize a multi-threaded approach to continuously retrieve real-time data from an external API. External API can be of your choice that doesn't require apikey [1]. 
>    If real-time data is not available, you may simulate and process past data as if it were current.
> 2. Perform meaningful metrics calculations to display output in terminal, updating every second.
> 3. Implement an asynchronous task scheduler using tokio runtime that periodically stores your calculated data after specific time-intervals.
> 4. Store data in a SQL or NoSQL like database or optionally file.
> 5. Include a README file with description about your project.

Optional Tasks:
---------------
> 1. Dockerize your project for easy setup and execution.
> 2. Write unit tests to ensure the correctness of your code.

Instructions:
-------------
> 1. Fork this repository into your GitHub account.
> 2. Implement the required functionality as described above.
> 3. Ensure your code follows best practices.
>
> [1] - https://github.com/public-apis/public-apis

High-level Design & Arch overview
---------------------------------

TBD

Progress
--------
[+] GH Repository  
[+] README & Makefile  
[+] TO choose online API service to integrate with  
[+] Docker & Compose infrastructure  
[+] Crawler base code:
    [+] D/I
    [+] Logger
    [+] CLI
    [+] Config
    [+] HTTP client
    [+] Periodic tasks
    [-] SQLITE Database
    [-] Redis  
[+] Crawler scrapping module   
[+] Crawler processing module   
[-] Crawler metrics calculation & realtime output  
[+] Crawler DB layer  
[+] Crawler tests:
    [+] ut
    [-] integration  

External deps & Links
---------------------

https://github.com/public-apis/public-apis  
https://uselessfacts.jsph.pl/
https://cat-fact.herokuapp.com/facts/random  
