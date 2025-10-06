# wharf

Because sometimes I forgot what things are for.

Remember that time you created a directory named `important` and six months later had no idea what it was?

`wharf` is a simple tool that lets you attach descriptions to files and directories.

### Commands

```text
>>> wharf --help
A simple file and directory description tool

Usage: wharf <COMMAND>

Commands:
  show        Show description
  add         Add or update description
  edit        Edit description interactively
  list        List all descriptions
  search      Search descriptions
  remove      Remove description
  autoremove  Autoremove descriptions for non-existent paths
  export      Export descriptions to file
  import      Import descriptions from file
  generate    Generate shell completion scripts
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Example

```bash
# highly recommeded
alias what="wharf show"
```

```bash
cd ~/dev/project
mkdir temp
wharf add temp "this is safe to delete"
# output: dscription added for: dev/project/temp

# later...
what temp
# output: dev/project/temp: this is safe to delete

# or
wharf search "delete"
# output: dev/project/temp: this is safe to delete
```

### Installation

```bash
curl -fsSL https://raw.githubusercontent.com/haolamnm/wharf/main/install.sh | sh
```
