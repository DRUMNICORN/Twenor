-- Active: 1693410255453@@lin-28080-15076-mysql-primary.servers.linodedb.net@3306@dank


CREATE DATABASE IF NOT EXISTS dank;

USE dank;

DROP TABLE IF EXISTS `AUDIO_STATE`;
DROP TABLE IF EXISTS `AUDIO_FEATURES`;
DROP TABLE IF EXISTS `AUDIO_CORRELATION`;
DROP TABLE IF EXISTS `AUDIO_METADATA`;
DROP TABLE IF EXISTS `SCENE_LIST`;
DROP TABLE IF EXISTS `SCRIPT_LIST`;
DROP TABLE IF EXISTS `FEATURE_LIST`;
DROP TABLE IF EXISTS `AUDIO_CHUNK`;
DROP TABLE IF EXISTS `AUDIO_LIST`;
DROP TABLE IF EXISTS `USER_LIST`;

CREATE TABLE `USER_LIST` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_name` varchar(100) NOT NULL,
  `user_access_token` varchar(255) DEFAULT NULL,
  `user_token` varchar(255) DEFAULT NULL,
  `user_email` varchar(100) DEFAULT NULL,
  `user_terms_accepted` BOOLEAN DEFAULT FALSE,
  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `AUDIO_LIST`;
CREATE TABLE `AUDIO_LIST` (
  `audio_id` int(11) NOT NULL AUTO_INCREMENT,
  `user_id` int(11) DEFAULT NULL,
  `audio_size` int(11) DEFAULT NULL,
  `audio_loaded` tinyint(1) DEFAULT NULL,

  PRIMARY KEY (`audio_id`),
  KEY `user_id` (`user_id`),

  CONSTRAINT `AUDIO_LIST_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `SCRIPT_LIST`;
CREATE TABLE `SCRIPT_LIST`
(
  `script_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
  `script_title` mediumtext DEFAULT NULL,
  `script_description` mediumtext DEFAULT NULL,
  `script_prompt` mediumtext DEFAULT NULL,
  `script_instructions` mediumtext DEFAULT NULL,
  PRIMARY KEY (`script_id`),
  KEY `audio_id` (`audio_id`),
  CONSTRAINT `SCRIPT_LIST_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `SCENE_LIST`;
CREATE TABLE `SCENE_LIST`
(
  `scene_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
  `scene_title` varchar(255) DEFAULT NULL,
  `scene_description` mediumtext DEFAULT NULL,
  `scene_tags` varchar(255) DEFAULT NULL,
  `scene_color` varchar(255) DEFAULT NULL,
  `scene_chunks` mediumtext DEFAULT NULL,
  `scene_start` float DEFAULT NULL,
  `scene_end` float DEFAULT NULL,
  PRIMARY KEY (`scene_id`),
  KEY `audio_id` (`audio_id`),
  CONSTRAINT `SCENE_LIST_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


DROP TABLE IF EXISTS `AUDIO_CORRELATION`;

CREATE TABLE `AUDIO_CORRELATION` (
  `correlation_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
  `correlation_values` mediumtext DEFAULT NULL,
  PRIMARY KEY (`correlation_id`),
  KEY `audio_id` (`audio_id`),
  CONSTRAINT `AUDIO_CORRELATION_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


CREATE TABLE `AUDIO_METADATA` (
  `metadata_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
  `user_id` int(11) DEFAULT NULL,
  `title` varchar(255) DEFAULT NULL,
  `artist` varchar(255) DEFAULT NULL,
  `genre` varchar(100) DEFAULT NULL,
  `bpm` varchar(10) DEFAULT NULL,
  `offset` varchar(10) DEFAULT NULL,
  `artstyle` varchar(255) DEFAULT NULL,
  `scale` varchar(50) DEFAULT NULL,
  `lyrics` mediumtext DEFAULT NULL,
  PRIMARY KEY (`metadata_id`),
  KEY `audio_id` (`audio_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `AUDIO_METADATA_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `AUDIO_METADATA_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


DROP TABLE IF EXISTS `AUDIO_CHUNK`;
CREATE TABLE `AUDIO_CHUNK` (
  `chunk_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
  `user_id` int(11) DEFAULT NULL,
  `chunk_index` int(11) DEFAULT NULL,
  `chunk_start` float DEFAULT NULL,
  `chunk_end` float DEFAULT NULL,
  `chunk_values` mediumtext DEFAULT NULL,
  PRIMARY KEY (`chunk_id`),
  KEY `audio_id` (`audio_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `AUDIO_CHUNK_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `AUDIO_CHUNK_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


DROP TABLE IF EXISTS `AUDIO_FEATURES`;
CREATE TABLE `AUDIO_FEATURES` (
  `feature_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
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
  KEY `audio_id` (`audio_id`),
  key `chunk_id` (`chunk_id`),
  CONSTRAINT `AUDIO_FEATURES_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `AUDIO_FEATURES_ibfk_2` FOREIGN KEY (`chunk_id`) REFERENCES `AUDIO_CHUNK` (`chunk_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

DROP TABLE IF EXISTS `AUDIO_STATE`;
CREATE TABLE `AUDIO_STATE` (
  `state_id` int(11) NOT NULL AUTO_INCREMENT,
  `audio_id` int(11) DEFAULT NULL,
  `user_id` int(11) DEFAULT NULL,
  `state` mediumtext DEFAULT NULL,
  `progress` float DEFAULT NULL,
  `message` mediumtext DEFAULT NULL,
  PRIMARY KEY (`state_id`),
  KEY `audio_id` (`audio_id`),
  KEY `user_id` (`user_id`),
  CONSTRAINT `AUDIO_STATE_ibfk_1` FOREIGN KEY (`audio_id`) REFERENCES `AUDIO_LIST` (`audio_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `AUDIO_STATE_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `USER_LIST` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--- write update for `AUDIO_STATE` to insert progress and message to all rows NOW WRITE SQL TO UPDATE ALL ROWS WITH PROGRESS AND MESSAGE
