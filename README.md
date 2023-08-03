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
## Migrations
This project supports a basic migration system. Pile up SQL files in `db_migrations/` and they will
be run sequentially by running `make migrations`.
This is designed to run fine on a pristine database. If you face any difficulty, wipe the database
and run the command again.

## Fixtures
On the same model as migrations, you can run `make fixtures` to populate the database with fixtures
defines in `db_fixtures/`.

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
