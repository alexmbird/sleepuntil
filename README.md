sleepuntil - CLI Tool for Traveling (Forwards) in Time
------------------------------------------------------

You can use the UNIX `sleep` command to block for a specified duration.  That's fine if you know precisely how long you want to wait, but if you want to pause the execution of your script _until a specific time_?

In bash you can do something unwieldy (as suggested [here](http://stackoverflow.com/questions/645992/bash-sleep-until-a-specific-time-date)) like:

```bash
current_epoch=$(date +%s)
target_epoch=$(date -d '01/01/2010 12:00' +%s)
sleep_seconds=$(( $target_epoch - $current_epoch ))
sleep $sleep_seconds
```

But wow, just yuk.  I don't trust myself to get that right whenever I write a script.  In any case it's a royal PITA when I only want to introduce a quick 'n dirty pause on the command line.

Enter the __sleepuntil__ utility.

```bash
$ echo "hello"; sleepuntil 18:00; date; echo "eat your greens"
hello
   [waits until 18:00]
Mon May  9 18:00:00 BST 2016
eat your greens
```


__WARNING__ work-in-progress; only the basics work yet.


## Time Specification

_sleepuntil_ tries to parse your timespec in each of the following formats, in order.  The first to work is used.

1. `HH:MM`
2. `HH:MM:SS`
3. ISO 8601, timezone-naive, like `2014-11-28T21:00:09`
4. ISO 8601, timezone-aware, like `2014-11-28T21:00:09+09:00`
5. RFC 2822, like `Fri, 28 Nov 2014 21:00:09 +0900`


#### Examples:

All of the following are acceptable...

* `sleepuntil 18:00` - sleep until the next 6pm rolls around
* `sleepuntil 2017-11-28T21:00:09+09:00` - ISO 8601 with offset specified.  Sleep from now (like, the actual now-that-is-now) until people living in a +9h offset reach 21:00:09 on November 28th 2017.
 

## Serving Suggestions

How to use...

* Within the shell: use bash's `;` to specify multiple commands on a single line.  E.g. `$ sleepuntil 02:00; echo "bet that woke you up - love me" | mail me@myaddress.com`


## Questions...

#### Why?

A few common scenarios:

* Kick off an upgrade in the wee small hours while you're asleep (without having to setup atd)
* Delay sending that expletive-ridden resignation email until you've sobered up


#### But Doesn't `at` Do It Already?

Not really.  [at](http://linux.about.com/library/cmd/blcmdl1_at.htm) passes your command to a long-running daemon (which isn't installed by default on a lot of unixes) which queues it to run at a later time.  The `at` command then returns immediately.  If you're lucky you'll get your job's output via email later.

Trouble is, `at` returns immediately so it's no use for pausing a script or introducing a delay on the command line.


#### What About Cron?

Nope.  Cron is for repeating tasks and it's no use for pausing your bash script.


#### I Wish the `sleep` in [GNU Coreutils](http://www.gnu.org/software/coreutils/coreutils.html) Had a Switch for That

Me too.  Maybe I should submit a patch for it but this was more fun and waiting for that to reach the Linux distros would take, mmm, about a million years.


#### Why Rust?

You want OS utilities to be written in something fairly low-level and with good cross-platform compatibility, and Rust's safety features make it very attractive.

Also because I'd never written anything in [Rust](https://www.rust-lang.org/) before.  It probably shows.


## Building

```bash
$ git clone git@github.com:alexmbird/sleepuntil.git
$ cd sleepuntil
$ cargo build --release                             # dammit Rust, 350k binary
$ sudo cp target/release/sleepuntil /usr/local/bin  # yes yes I need to package it
```


## Platform Support

Should work anywhere Rust does.

I've run it on Ubuntu (AMD64 & ARMv7) and OSX.


## Limitations / TODO

* Backwards time travel not yet supported.  Instead it exits immediately if the target timespec has already passed.
* Time to wait is calculated at the moment you start the program.  If you subsequently change your computer's clock (or timezone) it'll have no effect.
* Behaviour for sleep encompassing a change in your local daylight savings time is undefined.
* Accuracy is good but not perfect.  An indeterminate number of CPU cycles will pass between calculating the time delta and passing it up to the POSIX sleep() function.  Unless you need microsecond accuracy this probably doesn't matter.
* No tests LOL


## License

GPLv3.

