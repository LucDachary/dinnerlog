default: up

up:
	docker-compose up --force-recreate -d

shell:
	docker-compose exec db bash

dbshell:
	docker-compose exec db mariadb -u dbuser -pdbpassword dinnerlog

clean:
	docker-compose down -v --remove-orphans
