# Odetto

A graphql-like syntax to declare types to generate anything.

My original intent behind this project was to write one file and generate your database types, ORM objects, graphql types, and client libraries to consume the graphql. You could write your own plugins to do even more with it. In the end it was an exercise in writing a parser. Looking back I should have just used graphql and decorators much like AWS Amplify.

Feel free to poke around!

## Roadmap

https://www.notion.so/tadscritch/Roadmap-fa93914c2b6f438db3e10877a378849f

### Building

Run `cargo build`

#### Lib Running/Testing

Run `cargo test`

If you desire to view the outputs of every step add the `-- --nocapture` flag.
