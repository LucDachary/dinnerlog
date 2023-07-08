default: up

up:
	docker-compose up --force-recreate -d

dbshell:
	docker-compose exec db mariadb -u dbuser -pdbpassword dinnerlog
