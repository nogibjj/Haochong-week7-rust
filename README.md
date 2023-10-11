# Haochong-week6-mini-repo
[![Tests](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/tests.yml)

[![Clippy](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/Haochong-week7-rust-data-engineering/actions/workflows/lint.yml)

This is a repo template for course 706_Data_Engineering Week 6 Mini Project. Since we need to perform join this week, I need to find two new datasets that fit this project. In this case, I found two datsets about the prediction of the world cup. First of all, I define functions called `extract` to get data from url. Then, use `load` to connect Databricks database. After that, I create afunction called `complex_query` to fulfill our requirement. Consequently, I use `main.py` to use my function in `lib.py`, and use the output of `main.py` and `test_main.py` to test my `main.py`. And also, I create a function called `query_record` to save my query history. Finally, I use Action to run `Makefile` and got a 100% pass. 

Important file:
* `Makefile`
* `cicd.yml`
* `lib.py`
* `main.py`
* `.env`(which is hidden)
* `query_record.md`
* `dataset`
* `test_main.py`

# Purpose
- Connect to a SQL Databricks database
- Design a complex SQL query involving joins, aggregation, and sorting
- Provide an explanation for what the query is doing and the expected results

## Preparation 
1. Open codespaces and vscode
2. Wait for container to be built with requiremnts.txt installed

## Set up for Databricks
After logging in Azure Education, I create a Azure Databricks. After launching the 
workspace, I got a database called "default". Then I found the information I need: server hostname, http path and my token. I put them in `.env` and use those three variables in my `load` function to build connection with Databricks. In addition, I also put those three variables in the `Action` under the `Secrets and variables` in the `Setting` of the repo. I create two table in the database. 

![Alt text](<截屏2023-10-08 上午12.33.57.png>)

## Check format and test errors
1. Format code with Python black by using `make format`

2. Lint code with Ruff by using `make lint`. 

3. Test code by using `make test`

In this project, I got the first failure while import my csv file. Panda can't read it. After checking the error message and my csv file, I found out that it's not raw. After putting `?raw=True` after the url, I fix this problem. 

![Alt text](<截屏2023-10-07 下午3.06.14.png>)

I didn't meet too many trouble for other parts. I write test in `test_main.py` for three functions, and I got them all pass:.

![Alt text](<截屏2023-10-07 下午3.23.45.png>)

### Query and result(you can also see in query_record):

### Lab:  Modifying a Rust Command-Line Tool

In this lab you will gain experience extending an existing Rust project by forking and modifying a simple command-line tool.

**Steps**

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


**Deliverable**

Submit a link to your forked repository showing the code changes.


**Goals**

This hands-on lab provides experience with:

- Forking and cloning a Rust project

- Modifying existing Rust code 

- Running `cargo build` and `cargo run`

- Version control with git

- Making a pull request (optional)


### Technical Notes

## Makefile

Each subdirectory project uses this style to make it easy to test and run

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
