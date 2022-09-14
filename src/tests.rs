use super::*;
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor, primitives::Triangle};

fn triangle() -> impl PointsIter {
    let tri = Triangle::new(Point::new(0, 10), Point::new(0, 0), Point::new(20, 0));

    tri
}

#[test]
fn rot0() {
    let mut disp = Rotate0::new(MockDisplay::new());

    assert_eq!(
        disp.bounding_box(),
        Rectangle {
            top_left: Point::default(),
            size: Size {
                width: 64,
                height: 64
            }
        }
    );

    disp.draw_iter(triangle().points().map(|pt| Pixel(pt, BinaryColor::On)))
        .expect("draw failed");

    assert_eq!(
        disp.as_ref().affected_area(),
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: 21,
                height: 11
            }
        }
    );

    disp.as_ref().assert_pattern(&[
        "#####################",
        "###################  ",
        "#################    ",
        "###############      ",
        "#############        ",
        "###########          ",
        "#########            ",
        "#######              ",
        "#####                ",
        "###                  ",
        "#                    ",
    ]);
}

#[test]
fn flipy() {
    let mut disp = FlipY::new(MockDisplay::new());

    assert_eq!(
        disp.bounding_box(),
        Rectangle {
            top_left: Point::default(),
            size: Size {
                width: 64,
                height: 64
            }
        }
    );

    disp.draw_iter(triangle().points().map(|pt| Pixel(pt, BinaryColor::On)))
        .expect("draw failed");
    assert_eq!(
        disp.as_ref().affected_area(),
        Rectangle {
            top_left: Point { x: 0, y: 53 },
            size: Size {
                width: 21,
                height: 11
            }
        }
    );

    disp.as_ref().assert_pattern(&[
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "#                    ",
        "###                  ",
        "#####                ",
        "#######              ",
        "#########            ",
        "###########          ",
        "#############        ",
        "###############      ",
        "#################    ",
        "###################  ",
        "#####################",
    ]);
}

#[test]
fn rot90() {
    let mut disp = Rotate90::new(MockDisplay::new());

    disp.draw_iter(triangle().points().map(|pt| Pixel(pt, BinaryColor::On)))
        .expect("draw failed");

    assert_eq!(
        disp.as_ref().affected_area(),
        Rectangle {
            top_left: Point { x: 53, y: 0 },
            size: Size {
                width: 11,
                height: 21
            }
        }
    );

    disp.as_ref().assert_pattern(&[
        "                                                     ###########",
        "                                                      ##########",
        "                                                      ##########",
        "                                                       #########",
        "                                                       #########",
        "                                                        ########",
        "                                                        ########",
        "                                                         #######",
        "                                                         #######",
        "                                                          ######",
        "                                                          ######",
        "                                                           #####",
        "                                                           #####",
        "                                                            ####",
        "                                                            ####",
        "                                                             ###",
        "                                                             ###",
        "                                                              ##",
        "                                                              ##",
        "                                                               #",
        "                                                               #",
    ]);
}

#[test]
fn rot270() {
    let mut disp = Rotate270::new(MockDisplay::new());

    disp.draw_iter(triangle().points().map(|pt| Pixel(pt, BinaryColor::On)))
        .expect("draw failed");

    assert_eq!(
        disp.as_ref().affected_area(),
        Rectangle {
            top_left: Point { x: 0, y: 43 },
            size: Size {
                width: 11,
                height: 21
            }
        }
    );

    disp.as_ref().assert_pattern(&[
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "                     ",
        "#                    ",
        "#                    ",
        "##                   ",
        "##                   ",
        "###                  ",
        "###                  ",
        "####                 ",
        "####                 ",
        "#####                ",
        "#####                ",
        "######               ",
        "######               ",
        "#######              ",
        "#######              ",
        "########             ",
        "########             ",
        "#########            ",
        "#########            ",
        "##########           ",
        "##########           ",
        "###########          ",
    ]);
}