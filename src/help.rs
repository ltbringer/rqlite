pub fn help_message() {
    let message = "
Sqlite.rs
=========
Welcome to a sqlite implementation in pure rust.

Assumptions
-----------
- A table can have max 255 Columns.
- A column can have max 20MB of data.

Support
-------
This implementation supports only an in memory table named `table`.

Meta Commands 
~~~~~~~~~~~~~
1. .exit

Queries
~~~~~~~
1. SELECT
2. INSERT
";
    print!("{}[2J", 27 as char);
    println!("{}", message);
}