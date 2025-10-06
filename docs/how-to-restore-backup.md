# Restore Backup

Read more: [About default paths](about-default-paths.md)

Backup is automatically created when you run destructive command. The backup files are stored in the `backups` directory under the default storage path.

```bash
>>> wharf import ~/.local/share/wharf/backups/wharf.bak.<timestamp>.json
```

Where `<timestamp>` is the timestamp of the backup file you want to restore.

For examples:
```bash
>>> pwd
# output: /home/<username>/.local/share/wharf

>>> ls backups
# output: wharf.bak.20250101-110000.json

>>> wharf import ./backups/wharf.bak.20250101-110000.json
# output:
# WARNING: this will overwrite ALL current descriptions
# continue? [y/N]:
```

> Note: If you want pernament backup, pick the desired backup file and copy it to another location.
