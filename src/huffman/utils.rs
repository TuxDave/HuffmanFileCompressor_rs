use std::collections::{HashMap};
use std::fs::File;
use std::io::Read;

pub fn count_occurrence(f: &mut File) -> HashMap<u8, u32> {
    let mut occurrence: HashMap<u8, u32> = HashMap::new();

    let mut buff: [u8; 1] = [0];
    while f.read(&mut buff).unwrap_or(0) != 0 {
        let byte = buff[0];
        occurrence.insert(byte, if let Some(prev) = occurrence.get(&byte) { prev + 1 } else { 1 });
    }
    occurrence
    //TODO: to be tested
}

#[cfg(test)]
mod t_utils {
    use std::collections::HashMap;

    #[test]
    fn count_occurrence() {
        let compare: HashMap<u8, u32> = HashMap::from([
            (0, 389),
            (255, 415),
            (254, 393),
            (1, 390),
            (253, 409),
            (2, 354),
            (3, 423),
            (252, 356),
            (251, 382),
            (4, 386),
            (250, 397),
            (5, 397),
            (6, 396),
            (249, 428),
            (7, 405),
            (248, 409),
            (247, 372),
            (8, 419),
            (9, 392),
            (246, 373),
            (245, 416),
            (10, 389),
            (11, 362),
            (244, 375),
            (12, 365),
            (243, 396),
            (242, 359),
            (13, 386),
            (14, 382),
            (241, 384),
            (15, 432),
            (240, 411),
            (16, 362),
            (239, 383),
            (17, 426),
            (238, 373),
            (18, 418),
            (237, 414),
            (19, 372),
            (236, 383),
            (20, 389),
            (235, 377),
            (234, 406),
            (21, 398),
            (233, 408),
            (22, 421),
            (23, 407),
            (232, 429),
            (231, 389),
            (24, 408),
            (230, 431),
            (25, 435),
            (229, 414),
            (26, 384),
            (27, 370),
            (228, 423),
            (28, 422),
            (227, 402),
            (29, 414),
            (226, 432),
            (225, 390),
            (30, 377),
            (224, 405),
            (31, 403),
            (32, 402),
            (223, 390),
            (222, 383),
            (33, 390),
            (34, 384),
            (221, 418),
            (220, 368),
            (35, 390),
            (219, 408),
            (36, 385),
            (218, 392),
            (37, 411),
            (217, 401),
            (38, 415),
            (39, 387),
            (216, 416),
            (40, 420),
            (215, 377),
            (214, 407),
            (41, 395),
            (42, 435),
            (213, 368),
            (43, 433),
            (212, 421),
            (211, 412),
            (44, 433),
            (210, 381),
            (45, 398),
            (209, 404),
            (46, 411),
            (47, 423),
            (208, 426),
            (48, 384),
            (207, 405),
            (206, 416),
            (49, 417),
            (50, 386),
            (205, 379),
            (51, 436),
            (204, 395),
            (52, 379),
            (203, 394),
            (53, 384),
            (202, 407),
            (54, 432),
            (201, 399),
            (200, 419),
            (55, 396),
            (56, 381),
            (199, 378),
            (57, 403),
            (198, 384),
            (197, 394),
            (58, 427),
            (59, 425),
            (196, 410),
            (60, 407),
            (195, 411),
            (194, 419),
            (61, 386),
            (193, 391),
            (62, 392),
            (192, 411),
            (63, 378),
            (191, 410),
            (64, 426),
            (190, 418),
            (65, 386),
            (66, 412),
            (189, 410),
            (188, 401),
            (67, 411),
            (187, 351),
            (68, 410),
            (186, 408),
            (69, 381),
            (185, 413),
            (70, 427),
            (184, 424),
            (71, 402),
            (72, 366),
            (183, 418),
            (182, 396),
            (73, 423),
            (181, 376),
            (74, 344),
            (180, 398),
            (75, 424),
            (179, 435),
            (76, 382),
            (178, 386),
            (77, 418),
            (177, 398),
            (78, 397),
            (79, 392),
            (176, 406),
            (80, 406),
            (175, 397),
            (174, 406),
            (81, 412),
            (82, 372),
            (173, 419),
            (172, 402),
            (83, 395),
            (84, 425),
            (171, 404),
            (85, 380),
            (170, 388),
            (169, 424),
            (86, 422),
            (168, 396),
            (87, 375),
            (88, 359),
            (167, 397),
            (166, 439),
            (89, 403),
            (165, 416),
            (90, 451),
            (91, 393),
            (164, 416),
            (163, 389),
            (92, 409),
            (162, 417),
            (93, 409),
            (161, 433),
            (94, 408),
            (160, 434),
            (95, 389),
            (96, 409),
            (159, 369),
            (97, 408),
            (158, 413),
            (98, 416),
            (157, 414),
            (156, 403),
            (99, 410),
            (155, 385),
            (100, 394),
            (154, 417),
            (101, 366),
            (153, 396),
            (102, 390),
            (103, 424),
            (152, 417),
            (151, 365),
            (104, 437),
            (105, 390),
            (150, 418),
            (149, 415),
            (106, 414),
            (107, 402),
            (148, 371),
            (147, 370),
            (108, 432),
            (109, 373),
            (146, 405),
            (110, 366),
            (145, 396),
            (144, 387),
            (111, 410),
            (143, 379),
            (112, 385),
            (142, 408),
            (113, 407),
            (114, 440),
            (141, 372),
            (140, 388),
            (115, 385),
            (116, 379),
            (139, 385),
            (117, 392),
            (138, 374),
            (118, 353),
            (137, 371),
            (136, 410),
            (119, 381),
            (120, 378),
            (135, 394),
            (134, 426),
            (121, 406),
            (133, 424),
            (122, 421),
            (132, 367),
            (123, 385),
            (124, 379),
            (131, 384),
            (125, 385),
            (130, 450),
            (126, 434),
            (129, 429),
            (127, 431),
            (128, 366),
        ]);
      //TODO: finish the test
    }
}