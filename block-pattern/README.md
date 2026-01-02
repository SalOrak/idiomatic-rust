# Block-Pattern

Source: [Block Pattern](https://notgull.net/block-pattern/)

The rust block pattern is an idiomatic rust pattern. As the article mentions

> The pattern comes from blocks in Rust being valid expressions.

The example is quite obvious too:
```rust
// Both are equivalents
let foo = { 1 + 2 };
let foo = 1 + 2;
```

The example above is indeed simple. But there are times where an object is created in multiple steps.

For example, let's say we have a configuration file in TOML defined as:
```rust
#[derive(Debug, Deserialize)]
struct Config {
    title: String,
    width: i32,
    height: i32,
    should_open: Option<bool>,
    tail: bool
}
```

And let's say we have a function that does something on a Config object, such as:

```rust
fn do_some_work(config: Config) {
}
    println!("I'm doing some work on {:?}", config);
    println!("Work on -> {}", config.title);
    println!("Total space: {}", config.width * config.height);
    if config.should_open.is_some() {
        println!("Does it open by default? {}", config.should_open.unwrap());
    }
}
```

Let's now try to deserialize such configuration file in rust.
The way in most programming languages is done looks something like this:
```rust
fn normal_do_work(path: &str) -> anyhow::Result<()> {
    /// First we read the file in bytes
    let raw_config = std::fs::read(path)?;

    /// Then we parse it as utf8
    let config_str = String::from_utf8(raw_config)?;

    // Then we deserialize it
    let config: Config = toml::from_str(&config_str)?;

    // Then we do some work on it
    do_some_work(config);

    Ok(())
}
```

As you can see, the above code works. But the goal of the function is not crystal clear. We end up with 3 different variables: `raw_config`, `config_str` and `config`, which are all dropped once the function returns.

Let's use the block pattern to improve it:
```rust
fn idiomatic_do_work(path: &str) -> anyhow::Result<()> {
    /// Everything related to the `config` part is now abstracted into a single statement.
    /// All the other variables are dropped once we have the final Config object.
    let config = {
        let raw_config = std::fs::read(path)?;
        let config_str = String::from_utf8(raw_config)?;
        toml::from_str(&config_str)?
    };

    /// Now it is impossible to provide the incorrect value here, as there is only a single 
    /// variable in the function stack: the config: Config one. 
    do_some_work(config);

    Ok(())
}
```

With the above implementation everything related to generating the config object is encapsulated in a block, hence the name. Now, the function only has a single variable: `config`. The `raw_config` and `config_str` values are dropped the moment the block `{..}` ends.

Even though the `block pattern` implementation does not seem to gain much but in fact we: 
1. Freed unused variables after a certain point.
2. Clarified the objective of the lines `read` and `from_utf8`. They are part of creating the configuration file. 
3. Finally, when calling `do_some_work` we only have one option: the `config` object.


For me the biggest benefit is `2` and `3`. The code describes itself. The block pattern can be seen as a powerful one-liner. Everything can be read in a *single line* (here a block) and no unwanted variables are populating the current namespace scope.



