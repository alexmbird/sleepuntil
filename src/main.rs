
extern crate regex;


extern crate getopts;
use getopts::Options;

extern crate chrono;
use chrono::*;
use chrono::Duration as ChronoDuration;

use std::env;
use std::process::exit;

use std::thread::sleep;
use std::time::Duration as StdDuration;

mod parser;

use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);


fn print_usage(program: &str, opts: Options) {
    let brief = format!("\nUsage: {prog} [options] <timespec>
    
    Sleep until the specified time is reached.", prog=program);
    print!("{}\n", opts.usage(&brief));
    print!("Examples:
    {prog} 22:19                - block until 19 mins past 10pm
    {prog} 22:19:30             - block until 19 mins, 30 sec past 10pm
    {prog} 2016-11-28T12:00:00Z - block until midday on 28th November\n", 
        prog=program);
    print!("\n");
    print!("Licensed under the GPL v3; see https://github.com/alexmbird/sleepuntil for details.\n");
    print!("\n");
}


// Return a Duration representing how long we need to wait until the specified
// time.  This will always be a slight overestimate since a few CPU cycles go
// by until sleep() is called.
fn parse_timespec(timespec: Vec<String>) -> StdDuration {
    // println!("Parsing {:?}", timespec);
    let timespec_concat = timespec.join(" ");
    let parsed_dt = match timespec_concat.parse::<DateTime<Local>>() {
        Ok(dt)   => dt,
        Err(err) => panic!("error: cannot parse timespec '{}' - {}", 
                            timespec_concat, err.to_string()),
    };
    let sleep_duration_chrono: Duration = parsed_dt - Local::now();
    let sleep_duration_std = match sleep_duration_chrono.to_std() {
        Ok(dur)  => dur,
        _ => StdDuration::new(0,0),
    };

    return sleep_duration_std;
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "print verbose output");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(f) => { println_stderr!("{}", f.to_string()); exit(1); }
    };
    if matches.opt_present("h") || matches.free.len() == 0 {
        print_usage(&program, opts);
        return;
    }
    let verbose = matches.opt_present("v");
    
    let sleep_duration: StdDuration = parse_timespec(matches.free);
    if verbose {
        println!("{}: sleeping for {:?}", program, sleep_duration);
    }
    sleep(sleep_duration);
    
    if verbose {
        println!("{}: sleep finished; exiting", program);
    }
}
