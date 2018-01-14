## Getting Started

In order to work on this project, you'll need to install a few things:

1. A recent version of [Rust](https://www.rustup.rs/) (CI runs against the latest stable, beta, and nightly).

Unit tests are in a local `mod test` in each file. If an integration test is warranted, please add a new test to the
_tests_ directory.

Be sure to only test the public interface. (Not trying to start a war, just picked a side already and don't want to hash
it out again).

To run your tests, just run `cargo test --all`.

## Submitting a PR

Here are some general guidelines for making PRs for this repo.

1. [Fork this repo](https://github.com/pseudomuto/json-ref/fork)
1. Make a branch off of master (`git checkout -b <your_branch_name>`)
1. Make focused commits with descriptive messages
1. Add tests that fail without your code, and pass with it
1. RustFmt your code! (I've got vim setup to `cargo fmt` on save)
1. **Ping someone on the PR** (Lots of people, including myself, won't get a notification unless pinged directly)

Every PR should have a well detailed summary of the changes being made and the reasoning behind them. Make sure to add
at least three sections.

### What is Changing?

Make sure you spell out in as much detail as necessary what will happen to which systems when your PR is merged, 
what are the expected changes.

### How is it Changing?

Include any relevant implementation details, mimize surprises for the reviewers in this section, if you had to take some 
unorthodox approaches (read hacks), explain why here.

### What Could Go Wrong?

How has this change been tested? In your opinion what is the risk, if any, of merging these changes.

#### Reviewers should:

1. Identify anything that the PR author may have missed from above.
2. Test the PR through whatever means necessary, including manually, to verify it is safe to be deployed.
3. Question everything. Never assume that something was tested or fully understood, always question and ask if there is
any uncertainty.

