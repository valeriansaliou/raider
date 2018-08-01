CREATE TABLE `account` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `email` varchar(191) NOT NULL DEFAULT '',
  `password` binary(32) NOT NULL DEFAULT '\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0',
  `recovery` binary(32) DEFAULT '\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0',
  `commission` decimal(3,2) NOT NULL DEFAULT '0.00',
  `full_name` varchar(191) DEFAULT NULL,
  `address` varchar(191) DEFAULT NULL,
  `country` enum('AF','AX','AL','DZ','AS','AD','AO','AI','AQ','AG','AR','AM','AW','AU','AT','AZ','BS','BH','BD','BB','BY','BE','BZ','BJ','BM','BT','BO','BQ','BA','BW','BV','BR','IO','BN','BG','BF','BI','KH','CM','CA','CV','KY','CF','TD','CL','CN','CX','CC','CO','KM','CG','CD','CK','CR','CI','HR','CU','CW','CY','CZ','DK','DJ','DM','DO','EC','EG','SV','GQ','ER','EE','ET','FK','FO','FJ','FI','FR','GF','PF','TF','GA','GM','GE','DE','GH','GI','GR','GL','GD','GP','GU','GT','GG','GN','GW','GY','HT','HM','VA','HN','HK','HU','IS','IN','ID','IR','IQ','IE','IM','IL','IT','JM','JP','JE','JO','KZ','KE','KI','KP','KR','KW','KG','LA','LV','LB','LS','LR','LY','LI','LT','LU','MO','MK','MG','MW','MY','MV','ML','MT','MH','MQ','MR','MU','YT','MX','FM','MD','MC','MN','ME','MS','MA','MZ','MM','NA','NR','NP','NL','NC','NZ','NI','NE','NG','NU','NF','MP','NO','OM','PK','PW','PS','PA','PG','PY','PE','PH','PN','PL','PT','PR','QA','RE','RO','RU','RW','BL','SH','KN','LC','MF','PM','VC','WS','SM','ST','SA','SN','RS','SC','SL','SG','SX','SK','SI','SB','SO','ZA','GS','SS','ES','LK','SD','SR','SJ','SZ','SE','CH','SY','TW','TJ','TZ','TH','TL','TG','TK','TO','TT','TN','TR','TM','TC','TV','UG','UA','AE','GB','US','UM','UY','UZ','VU','VE','VN','VG','VI','WF','EH','YE','ZM','ZW') DEFAULT NULL,
  `payout_method` enum('bank','paypal','bitcoin','other') DEFAULT NULL,
  `payout_instructions` text,
  `notify_balance` char(1) NOT NULL DEFAULT '1',
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `tracker` (
  `id` char(10) NOT NULL DEFAULT '',
  `label` varchar(191) NOT NULL,
  `statistics_signups` int(11) unsigned NOT NULL DEFAULT '0',
  `account_id` int(11) unsigned NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_tracker_account_id` (`account_id`),
  CONSTRAINT `fk_tracker_account_id` FOREIGN KEY (`account_id`) REFERENCES `account` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `balance` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `amount` decimal(9,2) NOT NULL DEFAULT '0.00',
  `currency` enum('AFN','EUR','ALL','DZD','USD','AOA','XCD','ARS','AMD','AWG','AUD','AZN','BSD','BHD','BDT','BBD','BYN','BZD','XOF','BMD','INR','BTN','BOB','BOV','BAM','BWP','NOK','BRL','BND','BGN','BIF','CVE','KHR','XAF','CAD','KYD','CLP','CLF','CNY','COP','COU','KMF','CDF','NZD','CRC','HRK','CUP','CUC','ANG','CZK','DKK','DJF','DOP','EGP','SVC','ERN','ETB','FKP','FJD','XPF','GMD','GEL','GHS','GIP','GTQ','GBP','GNF','GYD','HTG','HNL','HKD','HUF','ISK','IDR','XDR','IRR','IQD','ILS','JMD','JPY','JOD','KZT','KES','KPW','KRW','KWD','KGS','LAK','LBP','LSL','ZAR','LRD','LYD','CHF','MOP','MKD','MGA','MWK','MYR','MVR','MRO','MUR','XUA','MXN','MXV','MDL','MNT','MAD','MZN','MMK','NAD','NPR','NIO','NGN','OMR','PKR','PAB','PGK','PYG','PEN','PHP','PLN','QAR','RON','RUB','RWF','SHP','WST','STD','SAR','RSD','SCR','SLL','SGD','XSU','SBD','SOS','SSP','LKR','SDG','SRD','SZL','SEK','CHE','CHW','SYP','TWD','TJS','TZS','THB','TOP','TTD','TND','TRY','TMT','UGX','UAH','AED','USN','UYU','UYI','UZS','VUV','VEF','VND','YER','ZMW','ZWL','XBA','XBB','XBC','XBD','XTS','XXX','XAU','XPD','XPT','XAG','AFA','FIM','ALK','ADP','ESP','FRF','AOK','AON','AOR','ARA','ARP','ARY','RUR','ATS','AYM','AZM','BYR','BYB','BEC','BEF','BEL','BOP','BAD','BRB','BRC','BRE','BRN','BRR','BGJ','BGK','BGL','BUK','CNX','HRD','CYP','CSJ','CSK','ECS','ECV','GQE','EEK','XEU','GEK','DDM','DEM','GHC','GHP','GRD','GNE','GNS','GWE','GWP','ITL','ISJ','IEP','ILP','ILR','LAJ','LVL','LVR','LSM','ZAL','LTL','LTT','LUC','LUF','LUL','MGF','MVQ','MLF','MTL','MTP','MXP','MZE','MZM','NLG','NIC','PEH','PEI','PES','PLZ','PTE','ROK','ROL','CSD','SKK','SIT','RHD','ESA','ESB','SDD','SDP','SRG','CHC','TJR','TPE','TRL','TMM','UGS','UGW','UAK','SUR','USS','UYN','UYP','VEB','VNC','YDD','YUD','YUM','YUN','ZRN','ZRZ','ZMK','ZWC','ZWD','ZWN','ZWR','XFO','XRE','XFU') NOT NULL DEFAULT 'USD',
  `released` char(1) NOT NULL DEFAULT '0',
  `trace` text,
  `account_id` int(11) unsigned NOT NULL,
  `tracker_id` char(10) DEFAULT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_balance_account_id` (`account_id`),
  KEY `fk_balance_tracker_id` (`tracker_id`),
  CONSTRAINT `fk_balance_account_id` FOREIGN KEY (`account_id`) REFERENCES `account` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk_balance_tracker_id` FOREIGN KEY (`tracker_id`) REFERENCES `tracker` (`id`) ON DELETE SET NULL ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `payout` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `number` int(11) unsigned NOT NULL DEFAULT '0',
  `amount` decimal(9,2) NOT NULL DEFAULT '0.00',
  `currency` enum('AFN','EUR','ALL','DZD','USD','AOA','XCD','ARS','AMD','AWG','AUD','AZN','BSD','BHD','BDT','BBD','BYN','BZD','XOF','BMD','INR','BTN','BOB','BOV','BAM','BWP','NOK','BRL','BND','BGN','BIF','CVE','KHR','XAF','CAD','KYD','CLP','CLF','CNY','COP','COU','KMF','CDF','NZD','CRC','HRK','CUP','CUC','ANG','CZK','DKK','DJF','DOP','EGP','SVC','ERN','ETB','FKP','FJD','XPF','GMD','GEL','GHS','GIP','GTQ','GBP','GNF','GYD','HTG','HNL','HKD','HUF','ISK','IDR','XDR','IRR','IQD','ILS','JMD','JPY','JOD','KZT','KES','KPW','KRW','KWD','KGS','LAK','LBP','LSL','ZAR','LRD','LYD','CHF','MOP','MKD','MGA','MWK','MYR','MVR','MRO','MUR','XUA','MXN','MXV','MDL','MNT','MAD','MZN','MMK','NAD','NPR','NIO','NGN','OMR','PKR','PAB','PGK','PYG','PEN','PHP','PLN','QAR','RON','RUB','RWF','SHP','WST','STD','SAR','RSD','SCR','SLL','SGD','XSU','SBD','SOS','SSP','LKR','SDG','SRD','SZL','SEK','CHE','CHW','SYP','TWD','TJS','TZS','THB','TOP','TTD','TND','TRY','TMT','UGX','UAH','AED','USN','UYU','UYI','UZS','VUV','VEF','VND','YER','ZMW','ZWL','XBA','XBB','XBC','XBD','XTS','XXX','XAU','XPD','XPT','XAG','AFA','FIM','ALK','ADP','ESP','FRF','AOK','AON','AOR','ARA','ARP','ARY','RUR','ATS','AYM','AZM','BYR','BYB','BEC','BEF','BEL','BOP','BAD','BRB','BRC','BRE','BRN','BRR','BGJ','BGK','BGL','BUK','CNX','HRD','CYP','CSJ','CSK','ECS','ECV','GQE','EEK','XEU','GEK','DDM','DEM','GHC','GHP','GRD','GNE','GNS','GWE','GWP','ITL','ISJ','IEP','ILP','ILR','LAJ','LVL','LVR','LSM','ZAL','LTL','LTT','LUC','LUF','LUL','MGF','MVQ','MLF','MTL','MTP','MXP','MZE','MZM','NLG','NIC','PEH','PEI','PES','PLZ','PTE','ROK','ROL','CSD','SKK','SIT','RHD','ESA','ESB','SDD','SDP','SRG','CHC','TJR','TPE','TRL','TMM','UGS','UGW','UAK','SUR','USS','UYN','UYP','VEB','VNC','YDD','YUD','YUM','YUN','ZRN','ZRZ','ZMK','ZWC','ZWD','ZWN','ZWR','XFO','XRE','XFU') NOT NULL DEFAULT 'USD',
  `status` enum('pending','accepted','rejected','processed') NOT NULL DEFAULT 'pending',
  `account` varchar(191) DEFAULT NULL,
  `invoice_url` varchar(191) DEFAULT NULL,
  `account_id` int(11) unsigned NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_payout_account_id` (`account_id`),
  CONSTRAINT `fk_payout_account_id` FOREIGN KEY (`account_id`) REFERENCES `account` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
