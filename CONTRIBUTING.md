## How to contribute

We're really glad you're reading this, because we need volunteer developers to help this project come to fruition.

The following is a set of guidelines for contributing to shadow service, which are hosted in the [Darwinia Network][0] Organization on GitHub. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.


## Tests

We have test cases for Rust and Golang, you can trigger the test cases of Rust using `cargo test --all`, for Golang is `go test -v ./...`.


### Optional Tests

Here is a special cargo feature `darwinia` only for test usages, if you are contributing to the `scale codec` part of our rust code, you can run `cargo test --all-features` to trigger it.


## Branches

Here are 3 branches in this project

+ `v1`: The is the stable branch which can be importing by other go projects
+ `next`: This is the branch our prs will merge into
+ `gh-pages`: This branch serves the API docs of shadow


## Tagging

Only tagging on the `v1` branch, along with the updates of relay module of darwinia.


## Coding conventions

Start reading our code and you'll get the hang of it. We are using `clippy` (Rust), and `golangci-lint` (go) for our code style, you can check your code style by `cargo clippy && golangci-lint run`.


## Submitting Changes

The commit messages of shadow follows the standard of [Conventional Commits][1], please use a format like:

```
type(scope?): subject  #scope is optional; multiple scopes are supported (current delimiter options: "/", "\" and ",")
```

Common types according to [commitlint-config-conventional (based on the the Angular convention)][2] can be:

```
[
  'build',
  'ci',
  'chore',
  'docs',
  'feat',
  'fix',
  'perf',
  'refactor',
  'revert',
  'style',
  'test'
];
```



Thanks, 

Darwinia Team


[0]: https://github.com/darwinia-network
[1]: https://www.conventionalcommits.org/en/v1.0.0/
[2]: https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional#type-enum
