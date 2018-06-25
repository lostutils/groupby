# groupby

Group lines base on a regular expression

```
groupby (lostutils) 0.1.5
Group lines based on a given regex.

USAGE:
    groupby [FLAGS] [OPTIONS] <regex>

FLAGS:
        --count-only    
            Only show the count of matches per group.

    -h, --help          
            Prints help information

    -u                  
            Remove duplicate lines in the same group

    -V, --version       
            Prints version information


OPTIONS:
    -g <group-id>        
            The group-id to group by. Can be an index or a group name.


ARGS:
    <regex>    
            The regex to group by. The match will use the entire expression, unless a group-id is provided.

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
```bash
$ cat chat.txt | groupby "Message from (\w+):" -g 1 --count-only
    1 ***NO-MATCH***
    2 Alice
    2 Bob
```