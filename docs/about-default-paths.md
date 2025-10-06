# About default paths

### XDG Base Directory Specification

I use `dirs::data_local_dir()` and `dirs::config_dir()` to determine the default paths for storing data and configuration files. These functions follow the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/) on Linux and use appropriate directories on other operating systems.

### Environment variable

You can override the default configuration file path by setting the `WHARF_CONFIG_PATH` environment variable to point to your desired configuration file location.

### On Linux

```bash
# config paths

$WHARF_CONFIG_PATH
$XDG_CONFIG_HOME/wharf/config.toml
$HOME/.config/wharf/config.toml
```

```bash
# storage paths

$XDG_DATA_HOME/wharf/wharf.json
$HOME/.local/share/wharf/wharf.json
```

```bash
# backup path

$XDG_DATA_HOME/wharf/backups/
$HOME/.local/share/wharf/backups/
```
