default: up

up:
	docker-compose up --force-recreate -d

shell:
	docker-compose exec db bash

dbshell:
	docker-compose exec db mariadb -u dbuser -pdbpassword dinnerlog

migrations:
	# Run all the db_migrations/*.sql file sequentially.
	# For now it's designed to be run once from a pristine database.
	@for sql_file in db_migrations/*.sql; \
	do \
		echo "Running $$sql_file… "; \
		docker-compose exec -T db mariadb -udbuser -pdbpassword dinnerlog < $$sql_file; \
	done
	@echo "Done."

clean:
	docker-compose down -v --remove-orphans
