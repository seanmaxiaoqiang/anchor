### database
CREATE DATABASE IF NOT EXISTS tdx DEFAULT CHARACTER SET utf8mb4;

CREATE USER 'user'@'%' IDENTIFIED BY 'Password';
grant all privileges on *.* to 'user'@'%';
create database tdx character set utf8mb4;
USE tdx;

### tables

### 
### 999999 上证指数 日线 前复权
###      日期	    开盘	    最高	    最低	    收盘	    成交量	    成交额
### 19910102,127.61,128.84,127.61,128.84,91,59000.00

### 类型
CREATE TABLE `trade_data_type` (
  `type_code` varchar(15) NOT NULL DEFAULT '',
  `stock_code` varchar(6) DEFAULT NULL,
  `title` varchar(63) DEFAULT NULL,
  `trading_cycle` varchar(16) DEFAULT NULL,
  `restoration` varchar(16) DEFAULT NULL,
  `precision` int DEFAULT '-2' COMMENT '数据精度\n-2：十的负2次方，表示2为小数\n-3：十的负3次方，表示3为小数',
  PRIMARY KEY (`type_code`),
  UNIQUE KEY `idtrade_data_type_UNIQUE` (`type_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='交易数据类型\n### 999999 上证指数 日线 前复权';


### 交易数据
CREATE TABLE `trade_data` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `trade_code` varchar(15) DEFAULT '',
  `trade_date` datetime DEFAULT NULL,
  `open` bigint DEFAULT NULL,
  `high` bigint DEFAULT NULL,
  `low` bigint DEFAULT NULL,
  `close` bigint DEFAULT NULL,
  `volume` bigint DEFAULT NULL COMMENT '单位(手)',
  `amount` double DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id_UNIQUE` (`id`),
  UNIQUE KEY `unq_trade_code_date` (`trade_code`,`trade_date`),
  KEY `idx_trade_code` (`trade_code`),
  KEY `idx_trade_code_date` (`trade_code`,`trade_date`),
  CONSTRAINT `fk_trade_code` FOREIGN KEY (`trade_code`) REFERENCES `trade_data_type` (`type_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='###      日期	    开盘	    最高	    最低	    收盘	    成交量	    成交额\n### 19910102,127.61,128.84,127.61,128.84,91,59000.00\n';
