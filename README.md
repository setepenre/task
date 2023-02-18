# task

mini command line task list tool, add and delete tasks to a text file

## setup

```console
$ git clone https://github.com/setepenre/task.git
$ cd task
$ rustc task.rs
```

## usage

```
$ touch tasks.txt
$ ./task tasks.txt list
$ ./task tasks.txt add readme: add usage example - 1
[0] readme: add usage example - 1
$ ./task tasks.txt add readme: add another usage example - 0
[0] readme: add another usage example - 0
[1] readme: add usage example - 1
$ ./task tasks.txt add groceries: buy some coffee - 1
[0] readme: add another usage example - 0
[1] readme: add usage example - 1
[2] groceries: buy some coffee - 1
$ ./task tasks.txt
[0] readme: add another usage example - 0
[1] readme: add usage example - 1
[2] groceries: buy some coffee - 1
$ ./task tasks.txt del 1
[0] readme: add another usage example - 0
[1] groceries: buy some coffee - 1
$ ./task tasks.txt del 0
[0] groceries: buy some coffee - 1
```
