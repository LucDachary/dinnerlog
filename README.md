Dinner Log
===
The logbook of the meals you prepared, when, and for who!

__Features__
(none yet)

__TODO__
* latest dinners entries as the homepage;
* happenings CRUD;
* guests CRUD;
* search (course, guest, date, etc.).

# Database
## Backup
Obviously the password is in the commandline here as an example. If your database remains local it's
not a big deal, otherwise it should be typed afterwards.
```shell
docker-compose exec db mariadb-dump -u dbuser -pdbpassword dinnerlog > backups/mybackup.sql
```
The stdout redirection is made back on the host, so `backups/mybackup.sql` refers to this project's
folder.

## Restore
```shell
docker-compose exec -T db mariadb -udbuser -pdbpassword dinnerlog < backups/000_initial.sql
```
