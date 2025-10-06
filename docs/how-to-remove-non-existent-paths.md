# How to remove non-existent paths

I added `wharf autoremove` in `v1.3.0` to help you clean up descriptions for files or directories that no longer exist.

Examples:

```bash
pwd
# output: /home/<username>/dev/project

mkdir temp
# output: mkdir: created directory 'temp'

wharf add temp "this will be remove"
# output: description added for: dev/project/temp

what temp
# output: dev/project/temp: this will be remove

rmdir temp
# output: rmdir: removing directory, 'temp'

# now, temp no longer exists
wharf autoremove
# output:
# backup created: /home/<username>/.local/share/wharf/backups/wharf.bak.<timestamp>.json
# removed 1 descriptions for non-existent paths

what temp
# output: dev/project/temp: no description found
```
