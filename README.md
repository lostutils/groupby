# groupby

Group lines base on a regular expression

```
groupby (lostutils) 
Group lines based on a given regex.

USAGE:
    groupby [OPTIONS] <regex>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g <group-id>        The group-id to group by.

ARGS:
    <regex>    The regex to group by.
```

## Usage

```
// chat.txt
Message from Alice: Hello!
Message from Bob: Hi!
Message from Alice: Did you try groupby?
Message from Bob: Yes. It is really cool! 
```

```bash
$ cat chat.txt | groupby "Message from (\w+):" -g 1
***NO-MATCH***
    // chat.txt
Alice
    Message from Alice: Hello!
    Message from Alice: Did you try groupby?
Bob
    Message from Bob: Hi!
    Message from Bob: Yes. It is really cool! 

```
