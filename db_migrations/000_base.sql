-- The database must exist here. It's normally created by the MariaDB Docker container.

--
-- Table structure for table `happening`
--
DROP TABLE IF EXISTS `happening`;
CREATE TABLE `happening` (
  `id` char(32) NOT NULL,
  `date` datetime(6) NOT NULL,
  `name` varchar(100) NOT NULL,
  `comment` varchar(200) DEFAULT NULL,
  `created_on` datetime(6) NOT NULL,
  `last_modified_on` datetime(6) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
