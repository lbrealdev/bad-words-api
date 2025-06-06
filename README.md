# bad-words-api

This repository is a simple example of how to use Reqwest to interact with the **Bad Words API** through **API Layer**.

To run this example, first create an account on [API Layer](https://apilayer.com/) for a free subscription and generate your API Key.

> [!TIP]
> You can also use an alias email service like [SimpleLogin](https://simplelogin.io/) or [Addy](https://addy.io/) to create your account.

## Usage

**NOTE:** This repository uses [just](https://github.com/casey/just) as a command task runner.

To make a request, run the following commands below:
```shell
just run
```

It's also possible pass a new sentence as argument:
```shell
just run "That guy is such a complete ass, I wish he would just leave."
```

## Sources

- https://github.com/apilayer/
- https://apilayer.com/marketplace/bad_words-api