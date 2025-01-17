use crate::utils::normalized_args;
use crate::{
    error::ActionError, error::ActionErrorKind, Action, ActionWithResult, Command, Context, Flag,
    FlagType, Help,
};
use std::error::Error;

/// Multiple action application entry point
#[derive(Default)]
pub struct App {
    /// Application name
    pub name: String,
    /// Application author
    pub author: Option<String>,
    /// Application description
    pub description: Option<String>,
    /// Application usage
    pub usage: Option<String>,
    /// Application version
    pub version: Option<String>,
    /// Application commands
    pub commands: Option<Vec<Command>>,
    /// Application action
    pub action: Option<Action>,
    /// Alternate application action that returns a Result
    pub action_with_result: Option<ActionWithResult>,
    /// Application flags
    pub flags: Option<Vec<Flag>>,
}

impl App {
    /// Create new instance of `App`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new("cli");
    /// ```
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Set author of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new("cli")
    ///     .author(env!("CARGO_PKG_AUTHORS"));
    /// ```
    pub fn author<T: Into<String>>(mut self, author: T) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Set description of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new("cli")
    ///     .description(env!("CARGO_PKG_DESCRIPTION"));
    /// ```
    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set usage of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new("cli");
    /// app.usage("cli [command] [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }

    /// Set version of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new("cli");
    /// app.version(env!("CARGO_PKG_VERSION"));
    /// ```
    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set command of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Command};
    ///
    /// let command = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let app = App::new("cli")
    ///     .command(command);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set a command named as same as registered ones.
    ///
    /// ```should_panic
    /// use seahorse::{App, Command};
    ///
    /// let command1 = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let command2 = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let app = App::new("cli")
    ///     .command(command1)
    ///     .command(command2);
    /// ```
    pub fn command(mut self, command: Command) -> Self {
        if let Some(ref mut commands) = self.commands {
            if commands
                .iter()
                .any(|registered| registered.name == command.name)
            {
                panic!(r#"Command name "{}" is already registered."#, command.name);
            }
            (*commands).push(command);
        } else {
            self.commands = Some(vec![command]);
        }
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Action, App, Context};
    ///
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let app = App::new("cli")
    ///     .action(action);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set both action and action_with_result.
    ///
    /// ```should_panic
    /// use seahorse::{Action, ActionWithResult, App, Context};
    ///
    /// let action_with_result: ActionWithResult = |c: &Context| {println!("{:?}", c.args); Ok(())};
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let app = App::new("cli")
    ///     .action_with_result(action_with_result)
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        if self.action_with_result.is_some() {
            panic!(r#"only one of action and action_with_result can be set."#);
        }
        self.action = Some(action);
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{ActionWithResult, App, Context};
    ///
    /// let action_with_result: ActionWithResult = |c: &Context| {println!("{:?}", c.args); Ok(())};
    /// let app = App::new("cli")
    ///     .action_with_result(action_with_result);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set both action and action_with_result.
    ///
    /// ```should_panic
    /// use seahorse::{Action, ActionWithResult, App, Context};
    ///
    /// let action_with_result: ActionWithResult = |c: &Context| {println!("{:?}", c.args); Ok(())};
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let app = App::new("cli")
    ///     .action(action)
    ///     .action_with_result(action_with_result);
    /// ```
    pub fn action_with_result(mut self, action_with_result: ActionWithResult) -> Self {
        if self.action.is_some() {
            panic!(r#"only one of action and action_with_result can be set."#);
        }
        self.action_with_result = Some(action_with_result);
        self
    }

    /// Set flag of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Flag, FlagType};
    ///
    /// let app = App::new("cli")
    ///     .flag(Flag::new("bool", FlagType::Bool))
    ///     .flag(Flag::new("int", FlagType::Int));
    /// ```
    pub fn flag(mut self, flag: Flag) -> Self {
        if let Some(ref mut flags) = self.flags {
            (*flags).push(flag);
        } else {
            self.flags = Some(vec![flag]);
        }
        self
    }

    /// Run app
    ///
    /// Example
    ///
    /// ```
    /// use std::env;
    /// use seahorse::App;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = App::new("cli");
    /// app.run(args);
    /// ```
    pub fn run(&self, args: Vec<String>) {
        match self.run_with_result(args) {
            Ok(_) => return,
            Err(e) => panic!("{}", e),
        }
    }

    /// Run app, returning a result
    ///
    /// Example
    ///
    /// ```
    /// use std::env;
    /// use seahorse::App;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = App::new("cli");
    /// let result = app.run_with_result(args);
    /// ```
    pub fn run_with_result(&self, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let args = normalized_args(args);
        let (cmd_v, args_v) = match args.len() {
            1 => args.split_at(1),
            _ => args[1..].split_at(1),
        };

        let cmd = match cmd_v.first() {
            Some(c) => c,
            None => {
                self.help();
                return Err(Box::new(ActionError {
                    kind: ActionErrorKind::NotFound,
                }));
            }
        };

        match self.select_command(cmd) {
            Some(command) => return command.run_with_result(args_v.to_vec()),
            None => match self.action {
                Some(action) => {
                    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
                        self.help();
                        return Ok(());
                    }
                    action(&Context::new(
                        args[1..].to_vec(),
                        self.flags.clone(),
                        self.help_text(),
                    ));
                    return Ok(());
                }
                None => match self.action_with_result {
                    Some(action_with_result) => {
                        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string())
                        {
                            self.help();
                            return Ok(());
                        }
                        return action_with_result(&Context::new(
                            args[1..].to_vec(),
                            self.flags.clone(),
                            self.help_text(),
                        ));
                    }
                    None => {
                        self.help();
                        return Ok(());
                    }
                },
            },
        }
    }

    /// Select command
    /// Gets the Command that matches the string passed in the argument
    fn select_command(&self, cmd: &str) -> Option<&Command> {
        match &self.commands {
            Some(commands) => commands.iter().find(|command| match &command.alias {
                Some(alias) => command.name == cmd || alias.iter().any(|a| a == cmd),
                None => command.name == cmd,
            }),
            None => None,
        }
    }

    fn flag_help_text(&self) -> String {
        let mut text = String::new();
        text += "Flags:\n";
        let help_flag = "-h, --help";

        if let Some(flags) = &self.flags {
            let int_val = "<int>";
            let float_val = "<float>";
            let string_val = "<string>";

            let flag_helps = &flags.iter().map(|f| {
                let alias = match &f.alias {
                    Some(alias) => alias
                        .iter()
                        .filter(|&a| a.len() == 1)
                        .map(|a| format!("-{}", a))
                        .collect::<Vec<String>>()
                        .join(", "),
                    None => String::new(),
                };

                let long_alias = match &f.alias {
                    Some(alias) => alias
                        .iter()
                        .filter(|a| a.len() > 1)
                        .map(|a| format!("--{}", a))
                        .collect::<Vec<String>>()
                        .join(", "),
                    None => String::new(),
                };

                let val = match f.flag_type {
                    FlagType::Int => int_val,
                    FlagType::Float => float_val,
                    FlagType::String => string_val,
                    _ => "",
                };

                let help = if alias.is_empty() {
                    if long_alias.is_empty() {
                        format!("--{} {}", f.name, val)
                    } else {
                        format!("{}, --{}, {}", long_alias, f.name, val)
                    }
                } else {
                    if long_alias.is_empty() {
                        format!("{}, --{} {}", alias, f.name, val)
                    } else {
                        format!("{}, {}, --{} {}", alias, long_alias, f.name, val)
                    }
                };

                (help, f.description.clone())
            });

            let flag_name_max_len = flag_helps
                .clone()
                .map(|h| h.0.len())
                .chain(vec![help_flag.len()].into_iter())
                .max()
                .unwrap();

            for flag_help in flag_helps.clone() {
                text += &format!("\t{}", flag_help.0);

                if let Some(usage) = &flag_help.1 {
                    let flag_name_len = flag_help.0.len();
                    text += &format!(
                        "{} : {}\n",
                        " ".repeat(flag_name_max_len - flag_name_len),
                        usage
                    );
                } else {
                    text += "\n";
                }
            }

            text += &format!(
                "\t{}{} : Show help\n",
                help_flag,
                " ".repeat(flag_name_max_len - help_flag.len())
            );
        } else {
            text += &format!("\t{} : Show help\n", help_flag);
        }

        text
    }

    fn command_help_text(&self) -> String {
        let mut text = String::new();

        if let Some(commands) = &self.commands {
            text += "\nCommands:\n";

            let name_max_len = &commands
                .iter()
                .map(|c| {
                    if let Some(alias) = &c.alias {
                        format!("{}, {}", alias.join(", "), c.name).len()
                    } else {
                        c.name.len()
                    }
                })
                .max()
                .unwrap();

            for c in commands {
                let command_name = if let Some(alias) = &c.alias {
                    format!("{}, {}", alias.join(", "), c.name)
                } else {
                    c.name.clone()
                };

                let description = match &c.description {
                    Some(description) => description,
                    None => "",
                };

                text += &format!(
                    "\t{} {}: {}\n",
                    command_name,
                    " ".repeat(name_max_len - command_name.len()),
                    description
                );
            }
        }

        text
    }
}

impl Help for App {
    fn help_text(&self) -> String {
        let mut text = String::new();

        text += &format!("Name:\n\t{}\n\n", self.name);

        if let Some(author) = &self.author {
            text += &format!("Author:\n\t{}\n\n", author);
        }

        if let Some(description) = &self.description {
            text += &format!("Description:\n\t{}\n\n", description);
        }

        if let Some(usage) = &self.usage {
            text += &format!("Usage:\n\t{}\n\n", usage);
        }

        text += &self.flag_help_text();
        text += &self.command_help_text();

        if let Some(version) = &self.version {
            text += &format!("\nVersion:\n\t{}\n", version);
        }

        text
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, ActionWithResult, App, Command, Context, Flag, FlagType};
    use std::fmt;

    #[test]
    fn app_new_only_test() {
        let app = App::new("cli");
        app.run(vec!["cli".to_string()]);

        assert_eq!(app.name, "cli".to_string());
        assert_eq!(app.usage, None);
        assert_eq!(app.author, None);
        assert_eq!(app.description, None);
        assert_eq!(app.version, None);
    }

    #[test]
    fn multiple_app_test() {
        let a: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Ok(flag) => assert_eq!("string".to_string(), flag),
                _ => assert!(false, "string test false..."),
            }
            match c.int_flag("int") {
                Ok(flag) => assert_eq!(100, flag),
                _ => assert!(false, "int test false..."),
            }
            match c.float_flag("float") {
                Ok(flag) => assert_eq!(1.23, flag),
                _ => assert!(false, "float test false..."),
            }
        };
        let c = Command::new("hello")
            .alias("h")
            .description("hello command")
            .usage("test hello(h) args")
            .action(a)
            .flag(Flag::new("bool", FlagType::Bool))
            .flag(Flag::new("string", FlagType::String))
            .flag(Flag::new("int", FlagType::Int))
            .flag(Flag::new("float", FlagType::Float));

        let app = App::new("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test [command] [arg]")
            .version("0.0.1")
            .command(c);

        app.run(vec![
            "test".to_string(),
            "hello".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        app.run(vec![
            "test".to_string(),
            "h".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, Some("test [command] [arg]".to_string()));
        assert_eq!(app.author, Some("Author <author@example.com>".to_string()));
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, Some("0.0.1".to_string()));
    }

    #[test]
    fn single_app_test() {
        let action: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Ok(flag) => assert_eq!("string".to_string(), flag),
                _ => assert!(false, "string test false..."),
            }
            match c.int_flag("int") {
                Ok(flag) => assert_eq!(100, flag),
                _ => assert!(false, "int test false..."),
            }
            match c.float_flag("float") {
                Ok(flag) => assert_eq!(1.23, flag),
                _ => assert!(false, "float test false..."),
            }
        };

        let app = App::new("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test [arg]")
            .version("0.0.1")
            .action(action)
            .flag(Flag::new("bool", FlagType::Bool))
            .flag(Flag::new("string", FlagType::String))
            .flag(Flag::new("int", FlagType::Int))
            .flag(Flag::new("float", FlagType::Float));

        app.run(vec![
            "test".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, Some("test [arg]".to_string()));
        assert_eq!(app.author, Some("Author <author@example.com>".to_string()));
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, Some("0.0.1".to_string()));
    }

    #[test]
    fn flag_only_app_test() {
        let action: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Ok(flag) => assert_eq!("string".to_string(), flag),
                _ => assert!(false, "string test false..."),
            }
            match c.int_flag("int") {
                Ok(flag) => assert_eq!(100, flag),
                _ => assert!(false, "int test false..."),
            }
            match c.float_flag("float") {
                Ok(flag) => assert_eq!(1.23, flag),
                _ => assert!(false, "float test false..."),
            }
        };

        let app = App::new("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test")
            .version("0.0.1")
            .action(action)
            .flag(Flag::new("bool", FlagType::Bool))
            .flag(Flag::new("string", FlagType::String))
            .flag(Flag::new("int", FlagType::Int))
            .flag(Flag::new("float", FlagType::Float));

        app.run(vec![
            "test".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, Some("test".to_string()));
        assert_eq!(app.author, Some("Author <author@example.com>".to_string()));
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, Some("0.0.1".to_string()));
    }

    #[test]
    fn single_app_equal_notation_test() {
        let action: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Ok(flag) => assert_eq!("str=ing".to_string(), flag),
                _ => assert!(false, "string test false..."),
            }
            match c.int_flag("int") {
                Ok(flag) => assert_eq!(100, flag),
                _ => assert!(false, "int test false..."),
            }
            match c.float_flag("float") {
                Ok(flag) => assert_eq!(1.23, flag),
                _ => assert!(false, "float test false..."),
            }
        };

        let app = App::new("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test [arg]")
            .version("0.0.1")
            .action(action)
            .flag(Flag::new("bool", FlagType::Bool))
            .flag(Flag::new("string", FlagType::String))
            .flag(Flag::new("int", FlagType::Int))
            .flag(Flag::new("float", FlagType::Float).alias("f"));

        app.run(vec![
            "test".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string=str=ing".to_string(),
            "--int=100".to_string(),
            "-f=1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, Some("test [arg]".to_string()));
        assert_eq!(app.author, Some("Author <author@example.com>".to_string()));
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, Some("0.0.1".to_string()));
    }

    #[test]
    fn app_with_ok_result_test() {
        let a: ActionWithResult = |_: &Context| {
            return Ok(());
        };
        let app = App::new("test").action_with_result(a);
        app.run(vec!["test".to_string()]);
    }

    #[test]
    #[should_panic]
    fn app_with_error_result_test() {
        let a: ActionWithResult = |_: &Context| {
            return Err(Box::new(Error));
        };
        let app = App::new("test").action_with_result(a);
        app.run(vec!["test".to_string()]);
    }

    #[test]
    fn app_with_ok_result_value_test() {
        let a: ActionWithResult = |_: &Context| {
            return Ok(());
        };
        let app = App::new("test").action_with_result(a);
        let result = app.run_with_result(vec!["test".to_string()]);
        assert!(!result.is_err());
    }

    #[test]
    fn app_with_error_result_value_test() {
        let a: ActionWithResult = |_: &Context| {
            return Err(Box::new(Error));
        };
        let app = App::new("test").action_with_result(a);
        let result = app.run_with_result(vec!["test".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn command_with_ok_result_test() {
        let a: ActionWithResult = |_: &Context| {
            return Ok(());
        };
        let command = Command::new("hello").action_with_result(a);
        let app = App::new("test").command(command);
        app.run(vec!["test".to_string(), "hello".to_string()]);
    }

    #[test]
    #[should_panic]
    fn command_with_error_result_test() {
        let a: ActionWithResult = |_: &Context| {
            return Err(Box::new(Error));
        };
        let command = Command::new("hello").action_with_result(a);
        let app = App::new("test").command(command);
        app.run(vec!["test".to_string(), "hello".to_string()]);
    }

    #[test]
    fn command_with_ok_result_value_test() {
        let a: ActionWithResult = |_: &Context| {
            return Ok(());
        };
        let command = Command::new("hello").action_with_result(a);
        let app = App::new("test").command(command);
        let result = app.run_with_result(vec!["test".to_string(), "hello".to_string()]);
        assert!(!result.is_err());
    }

    #[test]
    fn command_with_error_result_value_test() {
        let a: ActionWithResult = |_: &Context| {
            return Err(Box::new(Error));
        };
        let command = Command::new("hello").action_with_result(a);
        let app = App::new("test").command(command);
        let result = app.run_with_result(vec!["test".to_string(), "hello".to_string()]);
        assert!(result.is_err());
    }

    #[derive(Debug, Clone)]
    struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "test error")
        }
    }

    impl std::error::Error for Error {}
}
