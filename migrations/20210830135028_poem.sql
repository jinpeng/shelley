-- Add migration script here
CREATE TABLE IF NOT EXISTS `poem`(
   `id` VARCHAR(36) NOT NULL,
   `author` VARCHAR(128) NOT NULL,
   `title` VARCHAR(128) NOT NULL,
   `type` TINYINT NOT NULL DEFAULT 100 COMMENT '0: 古体诗, 1: 乐府, 2: 五言, 3: 七言, 4: 五绝, 5: 七绝, 100: 其它',
   `dynasty` TINYINT NOT NULL DEFAULT 100 COMMENT '0: 先秦, 1: 秦, 2: 汉, 3: 三国, 4: 晋; 5:南北朝; 6: 隋, 7: 唐, 8: 五代, 9: 宋, 10: 元, 11: 明, 12: 清, 13: 民国, 14: 当代， 100: 其它',
   `paragraphs` TEXT NOT NULL,
   `create_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
   `update_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
   PRIMARY KEY ( `id` )
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;