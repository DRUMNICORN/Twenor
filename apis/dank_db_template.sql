-- Active: 1690204399232@@127.0.0.1@3306@dank

USE dank;

DROP TABLE IF EXISTS `TRACK_STATE`;
DROP TABLE IF EXISTS `TRACK_FEATURES`;
DROP TABLE IF EXISTS `TRACK_CORRELATION`;
DROP TABLE IF EXISTS `TRACK_METADATA`;
DROP TABLE IF EXISTS `SCENE_LIST`;
DROP TABLE IF EXISTS `SCRIPT_LIST`;
DROP TABLE IF EXISTS `FEATURE_LIST`;
DROP TABLE IF EXISTS `TRACK_CHUNK`;
DROP TABLE IF EXISTS `TRACK_LIST`;
DROP TABLE IF EXISTS `USER_LIST`;

CREATE TABLE `USER_LIST` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  
  `user_name` varchar(100) NOT NULL,
  `user_password` varchar(100) NOT NULL,
  `user_token` varchar(255) DEFAULT NULL,
  `user_email` varchar(100) DEFAULT NULL,

  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


CREATE TABLE `TRACK_LIST` (
  `track_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) DEFAULT NULL,

  PRIMARY KEY (`track_id`),
  KEY `user_id` (`user_id`),

  CONSTRAINT `TRACK_LIST_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE `TRACK_METADATA` (
  `metadata_id` int(11) NOT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,
  `user_id` int(11) DEFAULT NULL,

  `uuid` varchar(255) DEFAULT NULL,
  `title` varchar(255) DEFAULT NULL,
  `artist` varchar(255) DEFAULT NULL,
  `genre` varchar(100) DEFAULT NULL,
  `bpm` varchar(10) DEFAULT NULL,
  `offset` varchar(10) DEFAULT NULL,
  `artstyle` varchar(255) DEFAULT 'NULL',
  `scale` varchar(50) DEFAULT 'NULL',
  `lyrics` mediumtext DEFAULT 'NULL',

  PRIMARY KEY (`metadata_id`),
  KEY `track_id` (`track_id`),
  KEY `user_id` (`user_id`),

  CONSTRAINT `TRACK_METADATA_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `TRACK_METADATA_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;



DROP TABLE IF EXISTS `TRACK_STATE`;
CREATE TABLE `TRACK_STATE` (
  `state_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) DEFAULT NULL,
  `track_id` int(11) DEFAULT NULL,

  `state` mediumtext DEFAULT NULL,
  
  PRIMARY KEY (`state_id`),
  KEY `user_id` (`user_id`),
  KEY `track_id` (`track_id`),

  CONSTRAINT `TRACK_STATE_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `TRACK_STATE_ibfk_2` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


DROP TABLE IF EXISTS `TRACK_CHUNK`;
CREATE TABLE `TRACK_CHUNK` (
  `chunk_id` int(11) NOT NULL AUTO_INCREMENT,
  
  `track_id` int(11) DEFAULT NULL,
  `user_id` int(11) DEFAULT NULL,

  `chunk_index` int(11) DEFAULT NULL,

  `chunk_start` float DEFAULT NULL,
  `chunk_end` float DEFAULT NULL,

  `chunk_values` mediumtext DEFAULT NULL,

  PRIMARY KEY (`chunk_id`),
  KEY `track_id` (`track_id`),
  KEY `user_id` (`user_id`),

  CONSTRAINT `TRACK_CHUNK_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `TRACK_CHUNK_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `TRACK_FEATURES`;
CREATE TABLE `TRACK_FEATURES` (
  `feature_id` int(11) DEFAULT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,
  `chunk_id` int(11) DEFAULT NULL,

  `danceability` float DEFAULT NULL,
  `valence` float DEFAULT NULL,
  `energy` float DEFAULT NULL,
  `tempo` float DEFAULT NULL,
  `loudness` float DEFAULT NULL,
  `speechiness` float DEFAULT NULL,
  `instrumentalness` float DEFAULT NULL,
  `liveness` float DEFAULT NULL,
  `acousticness` float DEFAULT NULL,
  `key` float DEFAULT NULL,
  `mode` float DEFAULT NULL,
  `duration` float DEFAULT NULL,
  `time_signature` float DEFAULT NULL,

  PRIMARY KEY (`feature_id`),
  KEY `track_id` (`track_id`),
  key `chunk_id` (`chunk_id`),

  CONSTRAINT `TRACK_FEATURES_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `TRACK_FEATURES_ibfk_2` FOREIGN KEY (`chunk_id`) REFERENCES `TRACK_CHUNK` (`chunk_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `TRACK_CORRELATION`;

CREATE TABLE `TRACK_CORRELATION` (
  `correlation_id` int(11) NOT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,

  `correlation_values` mediumtext DEFAULT NULL,

  PRIMARY KEY (`correlation_id`),
  KEY `track_id` (`track_id`),

  CONSTRAINT `TRACK_CORRELATION_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;



DROP TABLE IF EXISTS `SCENE_LIST`;
CREATE TABLE `SCENE_LIST`
(
  `scene_id` int(11) NOT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,

  `scene_title` varchar(255) DEFAULT NULL,
  `scene_description` mediumtext DEFAULT NULL,
  `scene_tags` varchar(255) DEFAULT NULL,
  `scene_color` varchar(255) DEFAULT NULL,
  `scene_chunks` mediumtext DEFAULT NULL,
  `scene_start` float DEFAULT NULL,
  `scene_end` float DEFAULT NULL,

  PRIMARY KEY (`scene_id`),
  KEY `track_id` (`track_id`),

  CONSTRAINT `SCENE_LIST_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


SELECT * FROM TRACK_CORRELATION;


CREATE TABLE `TRACK_CORRELATED`(
  `correlated_id` int(11) NOT NULL AUTO_INCREMENT,
  `chunk_id` int(11) DEFAULT NULL,
  `scene_id` int(11) DEFAULT NULL

  PRIMARY KEY (`correlated_id`),
  KEY `chunk_id` (`chunk_id`),
  KEY `scene_id` (`scene_id`),

  CONSTRAINT `TRACK_CORRELATED_ibfk_1` FOREIGN KEY (`chunk_id`) REFERENCES `TRACK_CHUNK` (`chunk_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `TRACK_CORRELATED_ibfk_2` FOREIGN KEY (`scene_id`) REFERENCES `SCENE_LIST` (`scene_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


DROP TABLE IF EXISTS `SCRIPT_LIST`;
CREATE TABLE `SCRIPT_LIST`
(
  `script_id` int(11) NOT NULL AUTO_INCREMENT,
  `track_id` int(11) DEFAULT NULL,

  `script_prompt` mediumtext DEFAULT NULL,
  `script_response` mediumtext DEFAULT NULL,

  PRIMARY KEY (`script_id`),
  KEY `track_id` (`track_id`),

  CONSTRAINT `SCRIPT_LIST_ibfk_1` FOREIGN KEY (`track_id`) REFERENCES `TRACK_LIST` (`track_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
