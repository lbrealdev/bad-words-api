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

Output:
```json
{
  "bad_words_total": 1,
  "input_content": "This new software is absolute bullshit.",
  "output_content": "This new software is absolute ********."
}
```

It's also possible to pass a new sentence as an argument:
```shell
just run "That guy is such a complete ass, I wish he would just leave."
```

Output:
```json
{
  "bad_words_total": 2,
  "input_content": "That guy is such a complete ass, I wish he would just leave.",
  "output_content": "That *** is such a complete ***, I wish he would just leave."
}
```

## Sources

- https://github.com/apilayer/
- https://apilayer.com/marketplace/bad_words-api