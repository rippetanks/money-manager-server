-- MySQL Script generated by MySQL Workbench
-- Fri Jul 19 14:30:54 2019
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema Gestione_Entrate_Uscite
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema Gestione_Entrate_Uscite
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `Gestione_Entrate_Uscite` DEFAULT CHARACTER SET utf8 ;
USE `Gestione_Entrate_Uscite` ;

-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`AccountType`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`AccountType` (
  `id` SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `tipo` VARCHAR(32) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`User`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`User` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(32) NOT NULL,
  `surname` VARCHAR(32) NOT NULL,
  `phone` VARCHAR(16) NULL,
  `country` VARCHAR(64) NULL,
  `address` VARCHAR(128) NULL,
  `birthdate` DATE NULL,
  `note` VARCHAR(256) NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Account`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Account` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(64) NOT NULL,
  `status` TINYINT(1) NOT NULL,
  `note` VARCHAR(256) NULL,
  `current_balance` DOUBLE NOT NULL,
  `initial_balance` DOUBLE NOT NULL,
  `creation_date` DATETIME NOT NULL,
  `max_balance` DOUBLE NOT NULL,
  `min_balance` DOUBLE NOT NULL,
  `avg_balance` DOUBLE NOT NULL,
  `id_account_type` INT UNSIGNED NOT NULL,
  `id_currency` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `account_account_type_idx` (`id_account_type` ASC) VISIBLE,
  CONSTRAINT `account_type_fk`
    FOREIGN KEY (`id_account_type`)
    REFERENCES `Gestione_Entrate_Uscite`.`AccountType` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`AccountUser`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`AccountUser` (
  `id_conto` BIGINT UNSIGNED NOT NULL,
  `id_utente` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id_conto`, `id_utente`),
  INDEX `account_user_fk_idx` (`id_utente` ASC) VISIBLE,
  CONSTRAINT `account_user_fk`
    FOREIGN KEY (`id_utente`)
    REFERENCES `Gestione_Entrate_Uscite`.`User` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `account_account_fk`
    FOREIGN KEY (`id_conto`)
    REFERENCES `Gestione_Entrate_Uscite`.`Account` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Causal`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Causal` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `description` VARCHAR(255) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Place`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Place` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `place` VARCHAR(64) NOT NULL,
  `address` VARCHAR(128) NULL,
  `country` VARCHAR(64) NULL,
  `email` VARCHAR(255) NULL,
  `website` VARCHAR(128) NULL,
  `phone` VARCHAR(16) NULL,
  `note` VARCHAR(256) NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Currency`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Currency` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `currency` VARCHAR(12) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`TransactionType`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`TransactionType` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `type` VARCHAR(32) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Transaction`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Transaction` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `id_account` BIGINT UNSIGNED NOT NULL,
  `id_transaction_type` INT UNSIGNED NOT NULL,
  `id_place` BIGINT UNSIGNED NULL,
  `id_beneficiary` BIGINT UNSIGNED NULL,
  `note` VARCHAR(255) NULL,
  `amount` DOUBLE NOT NULL,
  `data` DATETIME NOT NULL,
  `id_currency` INT UNSIGNED NOT NULL,
  `expense` DOUBLE NOT NULL DEFAULT 0,
  `id_causal` INT UNSIGNED NOT NULL,
  `id_transaction_type` INT UNSIGNED NULL,
  PRIMARY KEY (`id`),
  INDEX `transaction_causal_idx` (`id_causal` ASC) VISIBLE,
  INDEX `transaction_place_idx` (`id_place` ASC) VISIBLE,
  INDEX `transaction_currency_idx` (`id_currency` ASC) VISIBLE,
  INDEX `transaction_transaction_type_fk_idx` (`id_transaction_type` ASC) VISIBLE,
  CONSTRAINT `transaction_causal_fk`
    FOREIGN KEY (`id_causal`)
    REFERENCES `Gestione_Entrate_Uscite`.`Causal` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `transaction_place_fk`
    FOREIGN KEY (`id_place`)
    REFERENCES `Gestione_Entrate_Uscite`.`Place` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `transaction_currency_fk`
    FOREIGN KEY (`id_currency`)
    REFERENCES `Gestione_Entrate_Uscite`.`Currency` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `transaction_transaction_type_fk`
    FOREIGN KEY (`id_transaction_type`)
    REFERENCES `Gestione_Entrate_Uscite`.`TransactionType` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Giro`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Giro` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `id_source_account` BIGINT UNSIGNED NOT NULL,
  `id_destination_account` BIGINT UNSIGNED NOT NULL,
  `data` DATETIME NOT NULL,
  `note` VARCHAR(255) NULL,
  `amount` DOUBLE NOT NULL,
  `expense` DOUBLE NOT NULL,
  `id_currency` INT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `giro_currency_fk_idx` (`id_currency` ASC) VISIBLE,
  INDEX `giro_account_fk_idx` (`id_source_account` ASC, `id_destination_account` ASC) VISIBLE,
  CONSTRAINT `giro_currency_fk`
    FOREIGN KEY (`id_currency`)
    REFERENCES `Gestione_Entrate_Uscite`.`Currency` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `giro_account_fk`
    FOREIGN KEY (`id_source_account` , `id_destination_account`)
    REFERENCES `Gestione_Entrate_Uscite`.`Account` (`id` , `id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Detail`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Detail` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `description` VARCHAR(32) NOT NULL,
  `id_user` BIGINT UNSIGNED NULL,
  PRIMARY KEY (`id`),
  INDEX `details_user_fk_idx` (`id_user` ASC) VISIBLE,
  CONSTRAINT `details_user_fk`
    FOREIGN KEY (`id_user`)
    REFERENCES `Gestione_Entrate_Uscite`.`User` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`Auth`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`Auth` (
  `id` BIGINT UNSIGNED NOT NULL,
  `email` VARCHAR(255) NULL,
  `iteration` SMALLINT UNSIGNED NOT NULL,
  `salt` CHAR(32) NOT NULL,
  `stored_key` CHAR(32) NOT NULL,
  `server_key` CHAR(32) NOT NULL,
  `last_login` DATETIME NULL DEFAULT NULL,
  `id_user` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`, `id_user`),
  UNIQUE INDEX `email_ix` (`email` ASC) VISIBLE,
  INDEX `fk_Auth_User1_idx` (`id_user` ASC) VISIBLE,
  CONSTRAINT `fk_Auth_User1`
    FOREIGN KEY (`id_user`)
    REFERENCES `Gestione_Entrate_Uscite`.`User` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `Gestione_Entrate_Uscite`.`TransactionDetail`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Gestione_Entrate_Uscite`.`TransactionDetail` (
  `id_detail` BIGINT UNSIGNED NOT NULL,
  `id_transaction` BIGINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id_detail`, `id_transaction`),
  INDEX `transaction_detail_transaction_fk_idx` (`id_transaction` ASC) VISIBLE,
  CONSTRAINT `transaction_detail_transaction_fk`
    FOREIGN KEY (`id_transaction`)
    REFERENCES `Gestione_Entrate_Uscite`.`Transaction` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `transaction_detail_detail_fk`
    FOREIGN KEY (`id_detail`)
    REFERENCES `Gestione_Entrate_Uscite`.`Detail` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
