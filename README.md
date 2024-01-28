# Backup Rotator

Usage:
```
# this can be put on a cron to regularly clean up old backups
b-roll --path /path/to/backups/ --glob "backup.*" --days 7

# run without actually deleting any files
b-roll --path /path/to/backups/ --glob "backup.*" --days 7 --test
```

Backup files can be named anything, but currently must end with "-[ unix time ].7z" such as "backup_configs-1000000000.7z"

This program will parse the timestamp out of the file name to get the backup time, but for most situations the file's modified time can also be an adequate timestamp to use.

To use the modified time instead of parsing out of the file name, we could always just put this on a cron:
```
find /path/to/backups -name "backup*.7z" -mtime +7 -exec rm {} \;
```
