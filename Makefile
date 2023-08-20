default: up

up:
	docker-compose up --force-recreate -d

shell:
	docker-compose exec db bash

dbshell:
	docker-compose exec db mariadb -u dbuser -pdbpassword dinnerlog

dbshellroot:
	docker-compose exec db mariadb -u root -pdev-db-root-password dinnerlog

migrations:
	# Run the db_migrations/*.sql files sequentially.
	# For now it's designed to be run once from a pristine database.
	@for sql_file in db_migrations/*.sql; \
	do \
		echo "Running $$sql_file… "; \
		docker-compose exec -T db mariadb -udbuser -pdbpassword dinnerlog < $$sql_file; \
	done
	@echo "Done."

fixtures:
	# Run the db_fixtures/*.sql files sequentially.
	@for sql_file in db_fixtures/*.sql; \
	do \
		echo "Running fixture $$sql_file… "; \
		docker-compose exec -T db mariadb -udbuser -pdbpassword dinnerlog < $$sql_file; \
	done
	@echo "Done."

clean:
	docker-compose down -v --remove-orphans
