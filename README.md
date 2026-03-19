# ConfigTemplate
A config build in a weird flow but show a flexible way on handling config

### Context
When we works with Rust, it will easy to put Option in a config file, and AI 
also love to do this way, however this will make the code verbose, and also AI
will write down a lot of test code on the redundant branch.

Once the business grows, it is easy to put more feature, more config on it,
and then docker comes some parameters will from environment and more, such that
we rely on crates to handle these configs.  However, some crates include the 
deserialized method.  Such that the it will be hard to handle in a well 
maintainace in the feautre.  So the `sturct-patch` is provided to make "patch"
and "deserialize" of the action of configure decoupled, and help we can have a
much easier maintainace Rust code.


### Patch Flow of the Config
The config start from default values and then path with following:
yaml, json, toml, environment;
