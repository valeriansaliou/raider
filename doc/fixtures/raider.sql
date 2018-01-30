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
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `balance` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `amount` decimal(9,2) NOT NULL DEFAULT '0.00',
  `currency` enum('AFN','EUR','ALL','DZD','USD','EUR','AOA','XCD','XCD','ARS','AMD','AWG','AUD','EUR','AZN','BSD','BHD','BDT','BBD','BYN','EUR','BZD','XOF','BMD','INR','BTN','BOB','BOV','USD','BAM','BWP','NOK','BRL','USD','BND','BGN','XOF','BIF','CVE','KHR','XAF','CAD','KYD','XAF','XAF','CLP','CLF','CNY','AUD','AUD','COP','COU','KMF','CDF','XAF','NZD','CRC','XOF','HRK','CUP','CUC','ANG','EUR','CZK','DKK','DJF','XCD','DOP','USD','EGP','SVC','USD','XAF','ERN','EUR','ETB','EUR','FKP','DKK','FJD','EUR','EUR','EUR','XPF','EUR','XAF','GMD','GEL','EUR','GHS','GIP','EUR','DKK','XCD','EUR','USD','GTQ','GBP','GNF','XOF','GYD','HTG','USD','AUD','EUR','HNL','HKD','HUF','ISK','INR','IDR','XDR','IRR','IQD','EUR','GBP','ILS','EUR','JMD','JPY','GBP','JOD','KZT','KES','AUD','KPW','KRW','KWD','KGS','LAK','EUR','LBP','LSL','ZAR','LRD','LYD','CHF','EUR','EUR','MOP','MKD','MGA','MWK','MYR','MVR','XOF','EUR','USD','EUR','MRO','MUR','EUR','XUA','MXN','MXV','USD','MDL','EUR','MNT','EUR','XCD','MAD','MZN','MMK','NAD','ZAR','AUD','NPR','EUR','XPF','NZD','NIO','XOF','NGN','NZD','AUD','USD','NOK','OMR','PKR','USD','PAB','USD','PGK','PYG','PEN','PHP','NZD','PLN','EUR','USD','QAR','EUR','RON','RUB','RWF','EUR','SHP','XCD','XCD','EUR','EUR','XCD','WST','EUR','STD','SAR','XOF','RSD','SCR','SLL','SGD','ANG','XSU','EUR','EUR','SBD','SOS','ZAR','SSP','EUR','LKR','SDG','SRD','NOK','SZL','SEK','CHF','CHE','CHW','SYP','TWD','TJS','TZS','THB','USD','XOF','NZD','TOP','TTD','TND','TRY','TMT','USD','AUD','UGX','UAH','AED','GBP','USD','USD','USN','UYU','UYI','UZS','VUV','VEF','VND','USD','USD','XPF','MAD','YER','ZMW','ZWL','XBA','XBB','XBC','XBD','XTS','XXX','XAU','XPD','XPT','XAG','AFA','FIM','ALK','ADP','ESP','FRF','AOK','AON','AOR','ARA','ARP','ARY','RUR','ATS','AYM','AZM','RUR','BYR','BYB','RUR','BEC','BEF','BEL','BOP','BAD','BRB','BRC','BRE','BRN','BRR','BGJ','BGK','BGL','BUK','CNX','HRD','HRK','CYP','CSJ','CSK','ECS','ECV','GQE','EEK','XEU','FIM','FRF','FRF','FRF','GEK','RUR','DDM','DEM','GHC','GHP','GRD','FRF','GNE','GNS','GWE','GWP','ITL','ISJ','IEP','ILP','ILR','ITL','RUR','RUR','LAJ','LVL','LVR','LSM','ZAL','LTL','LTT','LUC','LUF','LUL','MGF','MWK','MVQ','MLF','MTL','MTP','FRF','FRF','MXP','RUR','FRF','MZE','MZM','NLG','ANG','NIC','PEN','PEH','PEI','PES','PLZ','PTE','FRF','ROK','RON','ROL','RUR','FRF','FRF','FRF','ITL','CSD','EUR','SKK','SIT','ZAL','SDG','RHD','ESA','ESB','ESP','SDD','SDP','SRG','CHC','RUR','TJR','IDR','TPE','TRL','TRY','RUR','TMM','UGS','UGW','UAK','SUR','USS','UYN','UYP','RUR','VEB','VEF','VEF','VNC','YDD','YUD','YUM','YUN','ZRN','ZRZ','ZMK','ZWC','ZWD','ZWD','ZWN','ZWR','XFO','XRE','XFU') NOT NULL DEFAULT 'USD',
  `released` bit(1) NOT NULL DEFAULT b'0',
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
  `currency` enum('AFN','EUR','ALL','DZD','USD','EUR','AOA','XCD','XCD','ARS','AMD','AWG','AUD','EUR','AZN','BSD','BHD','BDT','BBD','BYN','EUR','BZD','XOF','BMD','INR','BTN','BOB','BOV','USD','BAM','BWP','NOK','BRL','USD','BND','BGN','XOF','BIF','CVE','KHR','XAF','CAD','KYD','XAF','XAF','CLP','CLF','CNY','AUD','AUD','COP','COU','KMF','CDF','XAF','NZD','CRC','XOF','HRK','CUP','CUC','ANG','EUR','CZK','DKK','DJF','XCD','DOP','USD','EGP','SVC','USD','XAF','ERN','EUR','ETB','EUR','FKP','DKK','FJD','EUR','EUR','EUR','XPF','EUR','XAF','GMD','GEL','EUR','GHS','GIP','EUR','DKK','XCD','EUR','USD','GTQ','GBP','GNF','XOF','GYD','HTG','USD','AUD','EUR','HNL','HKD','HUF','ISK','INR','IDR','XDR','IRR','IQD','EUR','GBP','ILS','EUR','JMD','JPY','GBP','JOD','KZT','KES','AUD','KPW','KRW','KWD','KGS','LAK','EUR','LBP','LSL','ZAR','LRD','LYD','CHF','EUR','EUR','MOP','MKD','MGA','MWK','MYR','MVR','XOF','EUR','USD','EUR','MRO','MUR','EUR','XUA','MXN','MXV','USD','MDL','EUR','MNT','EUR','XCD','MAD','MZN','MMK','NAD','ZAR','AUD','NPR','EUR','XPF','NZD','NIO','XOF','NGN','NZD','AUD','USD','NOK','OMR','PKR','USD','PAB','USD','PGK','PYG','PEN','PHP','NZD','PLN','EUR','USD','QAR','EUR','RON','RUB','RWF','EUR','SHP','XCD','XCD','EUR','EUR','XCD','WST','EUR','STD','SAR','XOF','RSD','SCR','SLL','SGD','ANG','XSU','EUR','EUR','SBD','SOS','ZAR','SSP','EUR','LKR','SDG','SRD','NOK','SZL','SEK','CHF','CHE','CHW','SYP','TWD','TJS','TZS','THB','USD','XOF','NZD','TOP','TTD','TND','TRY','TMT','USD','AUD','UGX','UAH','AED','GBP','USD','USD','USN','UYU','UYI','UZS','VUV','VEF','VND','USD','USD','XPF','MAD','YER','ZMW','ZWL','XBA','XBB','XBC','XBD','XTS','XXX','XAU','XPD','XPT','XAG','AFA','FIM','ALK','ADP','ESP','FRF','AOK','AON','AOR','ARA','ARP','ARY','RUR','ATS','AYM','AZM','RUR','BYR','BYB','RUR','BEC','BEF','BEL','BOP','BAD','BRB','BRC','BRE','BRN','BRR','BGJ','BGK','BGL','BUK','CNX','HRD','HRK','CYP','CSJ','CSK','ECS','ECV','GQE','EEK','XEU','FIM','FRF','FRF','FRF','GEK','RUR','DDM','DEM','GHC','GHP','GRD','FRF','GNE','GNS','GWE','GWP','ITL','ISJ','IEP','ILP','ILR','ITL','RUR','RUR','LAJ','LVL','LVR','LSM','ZAL','LTL','LTT','LUC','LUF','LUL','MGF','MWK','MVQ','MLF','MTL','MTP','FRF','FRF','MXP','RUR','FRF','MZE','MZM','NLG','ANG','NIC','PEN','PEH','PEI','PES','PLZ','PTE','FRF','ROK','RON','ROL','RUR','FRF','FRF','FRF','ITL','CSD','EUR','SKK','SIT','ZAL','SDG','RHD','ESA','ESB','ESP','SDD','SDP','SRG','CHC','RUR','TJR','IDR','TPE','TRL','TRY','RUR','TMM','UGS','UGW','UAK','SUR','USS','UYN','UYP','RUR','VEB','VEF','VEF','VNC','YDD','YUD','YUM','YUN','ZRN','ZRZ','ZMK','ZWC','ZWD','ZWD','ZWN','ZWR','XFO','XRE','XFU') NOT NULL DEFAULT 'USD',
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

CREATE TABLE `tracker` (
  `id` char(10) NOT NULL DEFAULT '',
  `label` varchar(191) NOT NULL,
  `statistics_signups` int(11) unsigned NOT NULL DEFAULT '0',
  `statistics_paying` int(11) unsigned NOT NULL DEFAULT '0',
  `account_id` int(11) unsigned NOT NULL,
  `created_at` datetime NOT NULL,
  `updated_at` datetime NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_tracker_account_id` (`account_id`),
  CONSTRAINT `fk_tracker_account_id` FOREIGN KEY (`account_id`) REFERENCES `account` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
