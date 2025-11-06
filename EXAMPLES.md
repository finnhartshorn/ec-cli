# EC-CLI Usage Examples

## Setup

First, set your Everybody Codes cookie:

```bash
export EC_COOKIE="your-cookie-value-here"
```

Or save it to a file:

```bash
echo "your-cookie-value-here" > ~/.everybodycodes.cookie
```

## Example Workflows

### Workflow 1: Fetch and Read a Puzzle

```bash
# Fetch day 5, part 1 puzzle and input
$ ec-cli fetch -d 5 -p 1
[INFO] Fetching user seed...
[INFO] Fetching quest keys for 2024/5...
[INFO] Downloading encrypted description for 2024/5...
[INFO] Decrypting description...
[INFO] Description saved to "2024/descriptions/5.html"
[INFO] Found 3 sample(s)
[INFO] Sample 1 saved to "2024/samples/5-1.txt"
[INFO] Sample 2 saved to "2024/samples/5-2.txt"
[INFO] Sample 3 saved to "2024/samples/5-3.txt"
[INFO] Downloading encrypted input for 2024/5 part 1...
[INFO] Decrypting input...
[INFO] Input saved to "2024/inputs/5-1.txt"

# Read the puzzle description
$ ec-cli read -d 5
[INFO] Reading description from local storage...

=== THE KINGDOM OF ALGORITHMIA ===
Quest 5: [Puzzle Title]

[Formatted puzzle description here...]

# Work on your solution...
$ cat 2024/inputs/5-1.txt
[Your puzzle input]

$ cat 2024/samples/5-1.txt
[Example input from puzzle description]
```

### Workflow 2: Fetch Only Description

```bash
# Just get the puzzle description, no input yet
$ ec-cli fetch -d 10 -p 1 --description-only
[INFO] Fetching quest keys for 2024/10...
[INFO] Downloading encrypted description for 2024/10...
[INFO] Description saved to "2024/descriptions/10.html"
```

### Workflow 3: Fetch Only Input

```bash
# Already read the puzzle online, just need the input
$ ec-cli fetch -d 7 -p 2 --input-only
[INFO] Fetching user seed...
[INFO] Fetching quest keys for 2024/7...
[INFO] Downloading encrypted input for 2024/7 part 2...
[INFO] Input saved to "2024/inputs/7-2.txt"
```

### Workflow 4: Submit Answer

```bash
# After solving the puzzle
$ ec-cli submit -d 5 -p 1 "42"
[INFO] Submitting answer for 2024/5 part 1...
✓ Correct!
  Global place: 123
  Global score: 1000
  Time: 1234ms
```

### Workflow 5: Submit Wrong Answer

```bash
$ ec-cli submit -d 5 -p 1 "wrong"
[INFO] Submitting answer for 2024/5 part 1...
✗ Incorrect
  (Answer length is correct)
```

### Workflow 6: Working Through All Parts

```bash
# Day 8, starting from scratch
$ ec-cli fetch -d 8 -p 1
$ ec-cli read -d 8
# [solve part 1]
$ ec-cli submit -d 8 -p 1 "answer1"
✓ Correct!

# Move to part 2
$ ec-cli fetch -d 8 -p 2
$ ec-cli read -d 8  # description already cached
# [solve part 2]
$ ec-cli submit -d 8 -p 2 "answer2"
✓ Correct!

# Move to part 3
$ ec-cli fetch -d 8 -p 3
# [solve part 3]
$ ec-cli submit -d 8 -p 3 "answer3"
✓ Correct!
```

### Workflow 7: Different Years

```bash
# Work on 2025 puzzles
$ ec-cli fetch -y 2025 -d 1 -p 1
$ ec-cli read -y 2025 -d 1
$ ec-cli submit -y 2025 -d 1 -p 1 "answer"
```

### Workflow 8: Custom Terminal Width

```bash
# For wide terminals
$ ec-cli read -d 5 -w 120

# For narrow terminals
$ ec-cli read -d 5 -w 60
```

### Workflow 9: Debug Mode

```bash
# See detailed API calls and decryption process
$ ec-cli --debug fetch -d 5 -p 1
[DEBUG] Loaded cookie from EC_COOKIE environment variable
[DEBUG] User seed: 12345
[DEBUG] Fetched quest keys for 2024/5
[DEBUG] Decryption successful, length: 1024
...
```

### Workflow 10: Quiet Mode

```bash
# Only show errors (good for scripting)
$ ec-cli --quiet fetch -d 5 -p 1
$ echo $?
0
```

## Error Handling

### Missing Cookie

```bash
$ ec-cli fetch -d 1 -p 1
[ERROR] Authentication failed: missing cookie
```

### Invalid Day

```bash
$ ec-cli fetch -d 99 -p 1
[ERROR] Invalid day: 99 (must be 1-20)
```

### Invalid Part

```bash
$ ec-cli fetch -d 5 -p 5
[ERROR] Invalid part: 5 (must be 1-3)
```

### Already Submitted

```bash
$ ec-cli submit -d 5 -p 1 "42"
[ERROR] Answer already submitted
```

### Network Error

```bash
$ ec-cli fetch -d 5 -p 1
[ERROR] API request failed: connection timeout
```

## Tips and Tricks

### Batch Processing with Shell Scripts

```bash
#!/bin/bash
# Fetch all parts for a day
for part in 1 2 3; do
    ec-cli fetch -d $1 -p $part
done
```

Usage:
```bash
$ chmod +x fetch_all.sh
$ ./fetch_all.sh 5  # Fetches day 5, all parts
```

### Integration with Your Solver

```bash
#!/bin/bash
# Run solver and auto-submit
DAY=$1
PART=$2

# Ensure input is downloaded
ec-cli fetch -d $DAY -p $PART --input-only

# Run your solver (assumes it prints answer to stdout)
ANSWER=$(python3 solve.py 2024/inputs/$DAY-$PART.txt)

# Submit answer
ec-cli submit -d $DAY -p $PART "$ANSWER"
```

### Quick Alias Setup

Add to your `.bashrc` or `.zshrc`:

```bash
alias ec='ec-cli'
alias ecf='ec-cli fetch'
alias ecr='ec-cli read'
alias ecs='ec-cli submit'
```

Then:
```bash
$ ecf -d 5 -p 1
$ ecr -d 5
$ ecs -d 5 -p 1 "answer"
```

## File Organization

After running the CLI, your directory will look like:

```
.
├── 2024/
│   ├── inputs/
│   │   ├── 1-1.txt
│   │   ├── 1-2.txt
│   │   ├── 1-3.txt
│   │   ├── 2-1.txt
│   │   └── ...
│   ├── descriptions/
│   │   ├── 1.html
│   │   ├── 2.html
│   │   └── ...
│   └── samples/
│       ├── 1-1.txt
│       ├── 1-2.txt
│       ├── 1-3.txt
│       └── ...
└── my-solutions/
    ├── day1.py
    ├── day2.py
    └── ...
```

This structure makes it easy to:
- Keep inputs and descriptions organized by year
- Test solutions against samples before submitting
- Version control your solutions without committing puzzle data
