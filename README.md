# count_charactors

```test.txt
["Title1"]
"abcdefghijklmnopqrstuvwxyz"

["Title2"]
"12345
67890"


["Title3"]
""
```

```bash
$ cargo run --bin main test.txt

Title1: 26 abcdefghijklmnopqrstuvwxyz
Title2: 10 1234567890
Title3: 0
```