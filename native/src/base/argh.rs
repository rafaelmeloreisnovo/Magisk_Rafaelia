/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

use std::str::FromStr;

pub use derive::FromArgs;

/// Information about a particular command used for output.
pub type CommandInfo = argh_shared::CommandInfo<'static>;

/// Information about the command including the options and arguments.
pub type CommandInfoWithArgs = argh_shared::CommandInfoWithArgs<'static>;

/// Information about a subcommand.
pub type SubCommandInfo = argh_shared::SubCommandInfo<'static>;

pub use argh_shared::{ErrorCodeInfo, FlagInfo, FlagInfoKind, Optionality, PositionalInfo};

/// Structured information about the command line arguments.
pub trait ArgsInfo {
    /// Returns the argument info.
    fn get_args_info() -> CommandInfoWithArgs;

    /// Returns the list of subcommands
    fn get_subcommands() -> Vec<SubCommandInfo> {
        Self::get_args_info().commands
    }
}

/// Types which can be constructed from a set of commandline arguments.
pub trait FromArgs: Sized {
    /// Construct the type from an input set of arguments.
    ///
    /// The first argument `command_name` is the identifier for the current command. In most cases,
    /// users should only pass in a single item for the command name, which typically comes from
    /// the first item from `std::env::args()`. Implementations however should append the
    /// subcommand name in when recursively calling [FromArgs::from_args] for subcommands. This
    /// allows `argh` to generate correct subcommand help strings.
    ///
    /// The second argument `args` is the rest of the command line arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use argh::FromArgs;
    ///
    /// /// Command to manage a classroom.
    /// #[derive(Debug, PartialEq, FromArgs)]
    /// struct ClassroomCmd {
    ///     #[argh(subcommand)]
    ///     subcommands: Subcommands,
    /// }
    ///
    /// #[derive(Debug, PartialEq, FromArgs)]
    /// #[argh(subcommand)]
    /// enum Subcommands {
    ///     List(ListCmd),
    ///     Add(AddCmd),
    /// }
    ///
    /// /// list all the classes.
    /// #[derive(Debug, PartialEq, FromArgs)]
    /// #[argh(subcommand, name = "list")]
    /// struct ListCmd {
    ///     /// list classes for only this teacher.
    ///     #[argh(option)]
    ///     teacher_name: Option<String>,
    /// }
    ///
    /// /// add students to a class.
    /// #[derive(Debug, PartialEq, FromArgs)]
    /// #[argh(subcommand, name = "add")]
    /// struct AddCmd {
    ///     /// the name of the class's teacher.
    ///     #[argh(option)]
    ///     teacher_name: String,
    ///
    ///     /// the name of the class.
    ///     #[argh(positional)]
    ///     class_name: String,
    /// }
    ///
    /// let args = ClassroomCmd::from_args(
    ///     &["classroom"],
    ///     &["list", "--teacher-name", "Smith"],
    /// ).unwrap();
    /// assert_eq!(
    ///    args,
    ///     ClassroomCmd {
    ///         subcommands: Subcommands::List(ListCmd {
    ///             teacher_name: Some("Smith".to_string()),
    ///         })
    ///     },
    /// );
    ///
    /// // Help returns an error, but internally returns an `Ok` status.
    /// let early_exit = ClassroomCmd::from_args(
    ///     &["classroom"],
    ///     &["help"],
    /// ).unwrap_err();
    /// assert_eq!(
    ///     early_exit,
    ///     argh::EarlyExit {
    ///        output: r#"Usage: classroom <command> [<args>]
    ///
    /// Command to manage a classroom.
    ///
    /// Options:
    ///   --help, help      display usage information
    ///
    /// Commands:
    ///   list              list all the classes.
    ///   add               add students to a class.
    /// "#.to_string(),
    ///        status: Ok(()),
    ///     },
    /// );
    ///
    /// // Help works with subcommands.
    /// let early_exit = ClassroomCmd::from_args(
    ///     &["classroom"],
    ///     &["list", "help"],
    /// ).unwrap_err();
    /// assert_eq!(
    ///     early_exit,
    ///     argh::EarlyExit {
    ///        output: r#"Usage: classroom list [--teacher-name <teacher-name>]
    ///
    /// list all the classes.
    ///
    /// Options:
    ///   --teacher-name    list classes for only this teacher.
    ///   --help, help      display usage information
    /// "#.to_string(),
    ///        status: Ok(()),
    ///     },
    /// );
    ///
    /// // Incorrect arguments will error out.
    /// let err = ClassroomCmd::from_args(
    ///     &["classroom"],
    ///     &["lisp"],
    /// ).unwrap_err();
    /// assert_eq!(
    ///    err,
    ///    argh::EarlyExit {
    ///        output: "Unrecognized argument: lisp\n".to_string(),
    ///        status: Err(()),
    ///     },
    /// );
    /// ```
    fn from_args(command_name: &[&str], args: &[&str]) -> Result<Self, EarlyExit>;
}

/// A top-level `FromArgs` implementation that is not a subcommand.
pub trait TopLevelCommand: FromArgs {}

/// A `FromArgs` implementation that can parse into one or more subcommands.
pub trait SubCommands: FromArgs {
    /// Info for the commands.
    const COMMANDS: &'static [&'static CommandInfo];

    /// Get a list of commands that are discovered at runtime.
    fn dynamic_commands() -> &'static [&'static CommandInfo] {
        &[]
    }
}

/// A `FromArgs` implementation that represents a single subcommand.
pub trait SubCommand: FromArgs {
    /// Information about the subcommand.
    const COMMAND: &'static CommandInfo;
}

impl<T: SubCommand> SubCommands for T {
    const COMMANDS: &'static [&'static CommandInfo] = &[T::COMMAND];
}

/// Trait implemented by values returned from a dynamic subcommand handler.
pub trait DynamicSubCommand: Sized {
    /// Info about supported subcommands.
    fn commands() -> &'static [&'static CommandInfo];

    /// Perform the function of `FromArgs::redact_arg_values` for this dynamic
    /// command.
    ///
    /// The full list of subcommands, ending with the subcommand that should be
    /// dynamically recognized, is passed in `command_name`. If the command
    /// passed is not recognized, this function should return `None`. Otherwise
    /// it should return `Some`, and the value within the `Some` has the same
    /// semantics as the return of `FromArgs::redact_arg_values`.
    fn try_redact_arg_values(
        command_name: &[&str],
        args: &[&str],
    ) -> Option<Result<Vec<String>, EarlyExit>>;

    /// Perform the function of `FromArgs::from_args` for this dynamic command.
    ///
    /// The full list of subcommands, ending with the subcommand that should be
    /// dynamically recognized, is passed in `command_name`. If the command
    /// passed is not recognized, this function should return `None`. Otherwise
    /// it should return `Some`, and the value within the `Some` has the same
    /// semantics as the return of `FromArgs::from_args`.
    fn try_from_args(command_name: &[&str], args: &[&str]) -> Option<Result<Self, EarlyExit>>;
}

/// Information to display to the user about why a `FromArgs` construction exited early.
///
/// This can occur due to either failed parsing or a flag like `--help`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EarlyExit {
    /// The output to display to the user of the commandline tool.
    pub output: String,
    /// If the early exit is caused by help triggers.
    pub is_help: bool,
}

impl From<String> for EarlyExit {
    fn from(err_msg: String) -> Self {
        Self {
            output: err_msg,
            is_help: false,
        }
    }
}

/// Types which can be constructed from a single commandline value.
///
/// Any field type declared in a struct that derives `FromArgs` must implement
/// this trait. A blanket implementation exists for types implementing
/// `FromStr<Error: Display>`. Custom types can implement this trait
/// directly.
pub trait FromArgValue: Sized {
    /// Construct the type from a commandline value, returning an error string
    /// on failure.
    fn from_arg_value(value: &str) -> Result<Self, String>;
}

impl<T> FromArgValue for T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    fn from_arg_value(value: &str) -> Result<Self, String> {
        T::from_str(value).map_err(|x| x.to_string())
    }
}

// The following items are all used by the generated code, and should not be considered part
// of this library's public API surface.

#[doc(hidden)]
pub trait ParseFlag {
    fn set_flag(&mut self, arg: &str);
}

impl<T: Flag> ParseFlag for T {
    fn set_flag(&mut self, _arg: &str) {
        <T as Flag>::set_flag(self);
    }
}

// A trait for for slots that reserve space for a value and know how to parse that value
// from a command-line `&str` argument.
//
// This trait is only implemented for the type `ParseValueSlotTy`. This indirection is
// necessary to allow abstracting over `ParseValueSlotTy` instances with different
// generic parameters.
#[doc(hidden)]
pub trait ParseValueSlot {
    fn fill_slot(&mut self, arg: &str, value: &str) -> Result<(), String>;
}

// The concrete type implementing the `ParseValueSlot` trait.
//
// `T` is the type to be parsed from a single string.
// `Slot` is the type of the container that can hold a value or values of type `T`.
#[doc(hidden)]
pub struct ParseValueSlotTy<Slot, T> {
    // The slot for a parsed value.
    pub slot: Slot,
    // The function to parse the value from a string
    pub parse_func: fn(&str, &str) -> Result<T, String>,
}

// `ParseValueSlotTy<Option<T>, T>` is used as the slot for all non-repeating
// arguments, both optional and required.
impl<T> ParseValueSlot for ParseValueSlotTy<Option<T>, T> {
    fn fill_slot(&mut self, arg: &str, value: &str) -> Result<(), String> {
        if self.slot.is_some() {
            return Err("duplicate values provided".to_string());
        }
        self.slot = Some((self.parse_func)(arg, value)?);
        Ok(())
    }
}

// `ParseValueSlotTy<Vec<T>, T>` is used as the slot for repeating arguments.
impl<T> ParseValueSlot for ParseValueSlotTy<Vec<T>, T> {
    fn fill_slot(&mut self, arg: &str, value: &str) -> Result<(), String> {
        self.slot.push((self.parse_func)(arg, value)?);
        Ok(())
    }
}

// `ParseValueSlotTy<Option<Vec<T>>, T>` is used as the slot for optional repeating arguments.
impl<T> ParseValueSlot for ParseValueSlotTy<Option<Vec<T>>, T> {
    fn fill_slot(&mut self, arg: &str, value: &str) -> Result<(), String> {
        self.slot
            .get_or_insert_with(Vec::new)
            .push((self.parse_func)(arg, value)?);
        Ok(())
    }
}

/// A type which can be the receiver of a `Flag`.
pub trait Flag {
    /// Creates a default instance of the flag value;
    fn default() -> Self
    where
        Self: Sized;

    /// Sets the flag. This function is called when the flag is provided.
    fn set_flag(&mut self);
}

impl Flag for bool {
    fn default() -> Self {
        false
    }
    fn set_flag(&mut self) {
        *self = true;
    }
}

impl Flag for Option<bool> {
    fn default() -> Self {
        None
    }

    fn set_flag(&mut self) {
        *self = Some(true);
    }
}

macro_rules! impl_flag_for_integers {
    ($($ty:ty,)*) => {
        $(
            impl Flag for $ty {
                fn default() -> Self {
                    0
                }
                fn set_flag(&mut self) {
                    *self = self.saturating_add(1);
                }
            }
        )*
    }
}

impl_flag_for_integers![u8, u16, u32, u64, u128, i8, i16, i32, i64, i128,];

/// This function implements argument parsing for structs.
///
/// `cmd_name`: The identifier for the current command.
/// `args`: The command line arguments.
/// `parse_options`: Helper to parse optional arguments.
/// `parse_positionals`: Helper to parse positional arguments.
/// `parse_subcommand`: Helper to parse a subcommand.
/// `help_func`: Generate a help message.
#[doc(hidden)]
pub fn parse_struct_args(
    cmd_name: &[&str],
    args: &[&str],
    mut parse_options: ParseStructOptions<'_>,
    mut parse_positionals: ParseStructPositionals<'_>,
    mut parse_subcommand: Option<ParseStructSubCommand<'_>>,
) -> Result<(), EarlyExit> {
    let mut help = false;
    let mut remaining_args = args;
    let mut positional_index = 0;
    let mut options_ended = false;

    'parse_args: while let Some(&next_arg) = remaining_args.first() {
        remaining_args = &remaining_args[1..];
        if (parse_options.help_triggers().contains(&next_arg)) && !options_ended {
            help = true;
            continue;
        }

        if next_arg.starts_with('-') && !options_ended {
            if next_arg == "--" {
                options_ended = true;
                continue;
            }

            if help {
                return Err("Trailing arguments are not allowed after `help`."
                    .to_string()
                    .into());
            }

            parse_options.parse(next_arg, &mut remaining_args)?;
            continue;
        }

        if let Some(ref mut parse_subcommand) = parse_subcommand
            && parse_subcommand.parse(help, cmd_name, next_arg, remaining_args)?
        {
            // Unset `help`, since we handled it in the subcommand
            help = false;
            break 'parse_args;
        }

        options_ended |= parse_positionals.parse(&mut positional_index, next_arg)?;
    }

    if help {
        Err(EarlyExit {
            output: String::new(),
            is_help: true,
        })
    } else {
        Ok(())
    }
}

#[doc(hidden)]
pub struct ParseStructOptions<'a> {
    /// A mapping from option string literals to the entry
    /// in the output table. This may contain multiple entries mapping to
    /// the same location in the table if both a short and long version
    /// of the option exist (`-z` and `--zoo`).
    pub arg_to_slot: &'static [(&'static str, usize)],

    /// The storage for argument output data.
    pub slots: &'a mut [ParseStructOption<'a>],

    /// help triggers is a list of strings that trigger printing of help
    pub help_triggers: &'a [&'a str],
}

impl<'a> ParseStructOptions<'a> {
    /// Parse a commandline option.
    ///
    /// `arg`: the current option argument being parsed (e.g. `--foo`).
    /// `remaining_args`: the remaining command line arguments. This slice
    /// will be advanced forwards if the option takes a value argument.
    fn parse(&mut self, arg: &str, remaining_args: &mut &[&str]) -> Result<(), String> {
        let pos = self
            .arg_to_slot
            .iter()
            .find_map(|&(name, pos)| if name == arg { Some(pos) } else { None })
            .ok_or_else(|| unrecognized_argument(arg, self.arg_to_slot, self.help_triggers()))?;

        match self.slots[pos] {
            ParseStructOption::Flag(ref mut b) => b.set_flag(arg),
            ParseStructOption::Value(ref mut pvs) => {
                let value = remaining_args
                    .first()
                    .ok_or_else(|| ["No value provided for option '", arg, "'.\n"].concat())?;
                *remaining_args = &remaining_args[1..];
                pvs.fill_slot(arg, value).map_err(|s| {
                    [
                        "Error parsing option '",
                        arg,
                        "' with value '",
                        value,
                        "': ",
                        &s,
                        "\n",
                    ]
                    .concat()
                })?;
            }
        }

        Ok(())
    }

    fn help_triggers(&self) -> &'a [&'a str] {
        if self.help_triggers.is_empty() {
            &["-h", "--help"]
        } else {
            self.help_triggers
        }
    }
}

fn unrecognized_argument(
    given: &str,
    arg_to_slot: &[(&str, usize)],
    extra_suggestions: &[&str],
) -> String {
    // get the list of available arguments
    let available = arg_to_slot
        .iter()
        .map(|(name, _pos)| *name)
        .chain(extra_suggestions.iter().copied())
        .collect::<Vec<&str>>();

    if available.is_empty() {
        return format!("Unrecognized argument: \"{}\"\n", given);
    }

    ["Unrecognized argument: ", given, "\n"].concat()
}

// `--` or `-` options, including a mutable reference to their value.
#[doc(hidden)]
pub enum ParseStructOption<'a> {
    // A flag which is set to `true` when provided.
    Flag(&'a mut dyn ParseFlag),
    // A value which is parsed from the string following the `--` argument,
    // e.g. `--foo bar`.
    Value(&'a mut dyn ParseValueSlot),
}

#[doc(hidden)]
pub struct ParseStructPositionals<'a> {
    pub positionals: &'a mut [ParseStructPositional<'a>],
    pub last_is_repeating: bool,
    pub last_is_greedy: bool,
}

impl ParseStructPositionals<'_> {
    /// Parse the next positional argument.
    ///
    /// `arg`: the argument supplied by the user.
    ///
    /// Returns true if non-positional argument parsing should stop
    /// after this one.
    fn parse(&mut self, index: &mut usize, arg: &str) -> Result<bool, EarlyExit> {
        if *index < self.positionals.len() {
            self.positionals[*index].parse(arg)?;

            if self.last_is_repeating && *index == self.positionals.len() - 1 {
                // Don't increment position if we're at the last arg
                // *and* the last arg is repeating. If it's also remainder,
                // halt non-option processing after this.
                Ok(self.last_is_greedy)
            } else {
                // If it is repeating, though, increment the index and continue
                // processing options.
                *index += 1;
                Ok(false)
            }
        } else {
            Err(EarlyExit {
                output: unrecognized_arg(arg),
                is_help: false,
            })
        }
    }
}

#[doc(hidden)]
pub struct ParseStructPositional<'a> {
    // The positional's name
    pub name: &'static str,

    // The function to parse the positional.
    pub slot: &'a mut dyn ParseValueSlot,
}

impl ParseStructPositional<'_> {
    /// Parse a positional argument.
    ///
    /// `arg`: the argument supplied by the user.
    fn parse(&mut self, arg: &str) -> Result<(), EarlyExit> {
        self.slot.fill_slot("", arg).map_err(|s| {
            [
                "Error parsing positional argument '",
                self.name,
                "' with value '",
                arg,
                "': ",
                &s,
                "\n",
            ]
            .concat()
            .into()
        })
    }
}

// A type to simplify parsing struct subcommands.
//
// This indirection is necessary to allow abstracting over `FromArgs` instances with different
// generic parameters.
#[doc(hidden)]
pub struct ParseStructSubCommand<'a> {
    // The subcommand commands
    pub subcommands: &'static [&'static CommandInfo],

    pub dynamic_subcommands: &'a [&'static CommandInfo],

    // The function to parse the subcommand arguments.
    #[allow(clippy::type_complexity)]
    pub parse_func: &'a mut dyn FnMut(&[&str], &[&str]) -> Result<(), EarlyExit>,
}

impl ParseStructSubCommand<'_> {
    fn parse(
        &mut self,
        help: bool,
        cmd_name: &[&str],
        arg: &str,
        remaining_args: &[&str],
    ) -> Result<bool, EarlyExit> {
        for subcommand in self
            .subcommands
            .iter()
            .chain(self.dynamic_subcommands.iter())
        {
            if subcommand.name == arg {
                let mut command = cmd_name.to_owned();
                command.push(subcommand.name);
                let prepended_help;
                let remaining_args = if help {
                    prepended_help = prepend_help(remaining_args);
                    &prepended_help
                } else {
                    remaining_args
                };

                (self.parse_func)(&command, remaining_args)?;

                return Ok(true);
            }
        }

        Ok(false)
    }
}

// Prepend `help` to a list of arguments.
// This is used to pass the `help` argument on to subcommands.
fn prepend_help<'a>(args: &[&'a str]) -> Vec<&'a str> {
    [&["help"], args].concat()
}

#[doc(hidden)]
pub fn print_subcommands<'a>(commands: impl Iterator<Item = &'a CommandInfo>) -> String {
    let mut out = String::new();
    for cmd in commands {
        argh_shared::write_description(&mut out, cmd);
    }
    out
}

fn unrecognized_arg(arg: &str) -> String {
    ["Unrecognized argument: ", arg, "\n"].concat()
}

// An error string builder to report missing required options and subcommands.
#[doc(hidden)]
#[derive(Default)]
pub struct MissingRequirements {
    options: Vec<&'static str>,
    subcommands: Option<Vec<&'static CommandInfo>>,
    positional_args: Vec<&'static str>,
}

const NEWLINE_INDENT: &str = "\n    ";

impl MissingRequirements {
    // Add a missing required option.
    #[doc(hidden)]
    pub fn missing_option(&mut self, name: &'static str) {
        self.options.push(name)
    }

    // Add a missing required subcommand.
    #[doc(hidden)]
    pub fn missing_subcommands(&mut self, commands: impl Iterator<Item = &'static CommandInfo>) {
        self.subcommands = Some(commands.collect());
    }

    // Add a missing positional argument.
    #[doc(hidden)]
    pub fn missing_positional_arg(&mut self, name: &'static str) {
        self.positional_args.push(name)
    }

    // If any missing options or subcommands were provided, returns an error string
    // describing the missing args.
    #[doc(hidden)]
    pub fn err_on_any(&self) -> Result<(), String> {
        if self.options.is_empty() && self.subcommands.is_none() && self.positional_args.is_empty()
        {
            return Ok(());
        }

        let mut output = String::new();

        if !self.positional_args.is_empty() {
            output.push_str("Required positional arguments not provided:");
            for arg in &self.positional_args {
                output.push_str(NEWLINE_INDENT);
                output.push_str(arg);
            }
        }

        if !self.options.is_empty() {
            if !self.positional_args.is_empty() {
                output.push('\n');
            }
            output.push_str("Required options not provided:");
            for option in &self.options {
                output.push_str(NEWLINE_INDENT);
                output.push_str(option);
            }
        }

        if let Some(missing_subcommands) = &self.subcommands {
            if !self.options.is_empty() {
                output.push('\n');
            }
            output.push_str("One of the following subcommands must be present:");
            output.push_str(NEWLINE_INDENT);
            output.push_str("help");
            for subcommand in missing_subcommands {
                output.push_str(NEWLINE_INDENT);
                output.push_str(subcommand.name);
            }
        }

        output.push('\n');

        Err(output)
    }
}

mod argh_shared {
    //! Shared functionality between argh_derive and the argh runtime.
    //!
    //! This library is intended only for internal use by these two crates.

    /// Information about a particular command used for output.
    pub struct CommandInfo<'a> {
        /// The name of the command.
        pub name: &'a str,
        /// A short description of the command's functionality.
        pub description: &'a str,
    }

    /// Information about the command line arguments for a given command.
    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    pub struct CommandInfoWithArgs<'a> {
        /// The name of the command.
        pub name: &'a str,
        /// A short description of the command's functionality.
        pub description: &'a str,
        /// Examples of usage
        pub examples: &'a [&'a str],
        /// Flags
        pub flags: &'a [FlagInfo<'a>],
        /// Notes about usage
        pub notes: &'a [&'a str],
        /// The subcommands.
        pub commands: Vec<SubCommandInfo<'a>>,
        /// Positional args
        pub positionals: &'a [PositionalInfo<'a>],
        /// Error code information
        pub error_codes: &'a [ErrorCodeInfo<'a>],
    }

    /// Information about a documented error code.
    #[derive(Debug, PartialEq, Eq)]
    pub struct ErrorCodeInfo<'a> {
        /// The code value.
        pub code: i32,
        /// Short description about what this code indicates.
        pub description: &'a str,
    }

    /// Information about positional arguments
    #[derive(Debug, PartialEq, Eq)]
    pub struct PositionalInfo<'a> {
        /// Name of the argument.
        pub name: &'a str,
        /// Description of the argument.
        pub description: &'a str,
        /// Optionality of the argument.
        pub optionality: Optionality,
        /// Visibility in the help for this argument.
        /// `false` indicates this argument will not appear
        /// in the help message.
        pub hidden: bool,
    }

    /// Information about a subcommand.
    /// Dynamic subcommands do not implement
    /// get_args_info(), so the command field
    /// only contains the name and description.
    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    pub struct SubCommandInfo<'a> {
        /// The subcommand name.
        pub name: &'a str,
        /// The information about the subcommand.
        pub command: CommandInfoWithArgs<'a>,
    }

    /// Information about a flag or option.
    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct FlagInfo<'a> {
        /// The kind of flag.
        pub kind: FlagInfoKind<'a>,
        /// The optionality of the flag.
        pub optionality: Optionality,
        /// The long string of the flag.
        pub long: &'a str,
        /// The single character short indicator
        /// for this flag.
        pub short: Option<char>,
        /// The description of the flag.
        pub description: &'a str,
        /// Visibility in the help for this argument.
        /// `false` indicates this argument will not appear
        /// in the help message.
        pub hidden: bool,
    }

    /// The kind of flags.
    #[derive(Debug, Default, PartialEq, Eq)]
    pub enum FlagInfoKind<'a> {
        /// switch represents a boolean flag,
        #[default]
        Switch,
        /// option is a flag that also has an associated
        /// value. This value is named `arg_name`.
        Option { arg_name: &'a str },
    }

    /// The optionality defines the requirements related
    /// to the presence of the argument on the command line.
    #[derive(Debug, Default, PartialEq, Eq)]
    pub enum Optionality {
        /// Required indicates the argument is required
        /// exactly once.
        #[default]
        Required,
        /// Optional indicates the argument may or may not
        /// be present.
        Optional,
        /// Repeating indicates the argument may appear zero
        /// or more times.
        Repeating,
        /// Greedy is used for positional arguments which
        /// capture the all command line input up to the next flag or
        /// the end of the input.
        Greedy,
    }

    pub const INDENT: &str = "  ";
    const DESCRIPTION_INDENT: usize = 20;
    const WRAP_WIDTH: usize = 80;

    /// Write command names and descriptions to an output string.
    pub fn write_description(out: &mut String, cmd: &CommandInfo<'_>) {
        let mut current_line = INDENT.to_string();
        current_line.push_str(cmd.name);

        if cmd.description.is_empty() {
            new_line(&mut current_line, out);
            return;
        }

        if !indent_description(&mut current_line) {
            // Start the description on a new line if the flag names already
            // add up to more than DESCRIPTION_INDENT.
            new_line(&mut current_line, out);
        }

        let mut words = cmd.description.split(' ').peekable();
        while let Some(first_word) = words.next() {
            indent_description(&mut current_line);
            current_line.push_str(first_word);

            'inner: while let Some(&word) = words.peek() {
                if (char_len(&current_line) + char_len(word) + 1) > WRAP_WIDTH {
                    new_line(&mut current_line, out);
                    break 'inner;
                } else {
                    // advance the iterator
                    let _ = words.next();
                    current_line.push(' ');
                    current_line.push_str(word);
                }
            }
        }
        new_line(&mut current_line, out);
    }

    // Indent the current line in to DESCRIPTION_INDENT chars.
    // Returns a boolean indicating whether or not spacing was added.
    fn indent_description(line: &mut String) -> bool {
        let cur_len = char_len(line);
        if cur_len < DESCRIPTION_INDENT {
            let num_spaces = DESCRIPTION_INDENT - cur_len;
            line.extend(std::iter::repeat_n(' ', num_spaces));
            true
        } else {
            false
        }
    }

    fn char_len(s: &str) -> usize {
        s.chars().count()
    }

    // Append a newline and the current line to the output,
    // clearing the current line.
    fn new_line(current_line: &mut String, out: &mut String) {
        out.push('\n');
        out.push_str(current_line);
        current_line.truncate(0);
    }
}
