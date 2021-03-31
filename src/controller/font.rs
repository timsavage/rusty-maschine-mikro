pub const FONT_WIDTH: usize = 5;
pub const FONT: [u8; 480] = [
    0x00, 0x00, 0x00, 0x00, 0x00,  //
    0x00, 0x00, 0x5F, 0x00, 0x00,  // !
    0x00, 0x03, 0x00, 0x03, 0x00,  // "
    0x14, 0x7F, 0x14, 0x7F, 0x14,  // #
    0x26, 0x2A, 0x7F, 0x2A, 0x32,  // $
    0x10, 0x0A, 0x08, 0x28, 0x04,  // %
    0x36, 0x49, 0x59, 0x26, 0x50,  // &
    0x00, 0x00, 0x03, 0x00, 0x00,  // '
    0x00, 0x1C, 0x22, 0x41, 0x00,  // (
    0x00, 0x41, 0x22, 0x1C, 0x00,  // )
    0x04, 0x28, 0x1E, 0x28, 0x04,  // *
    0x08, 0x08, 0x3E, 0x08, 0x08,  // +
    0x00, 0x40, 0x20, 0x00, 0x00,  // ,
    0x08, 0x08, 0x08, 0x08, 0x08,  // -
    0x00, 0x00, 0x40, 0x00, 0x00,  // .
    0x00, 0x60, 0x1C, 0x03, 0x00,  // /
    0x3E, 0x41, 0x5D, 0x41, 0x3E,  // 0
    0x00, 0x42, 0x7F, 0x40, 0x00,  // 1
    0x42, 0x61, 0x51, 0x49, 0x46,  // 2
    0x22, 0x49, 0x49, 0x49, 0x36,  // 3
    0x18, 0x14, 0x12, 0x7F, 0x10,  // 4
    0x2F, 0x49, 0x49, 0x49, 0x31,  // 5
    0x3E, 0x49, 0x49, 0x49, 0x32,  // 6
    0x41, 0x21, 0x19, 0x05, 0x03,  // 7
    0x36, 0x49, 0x49, 0x49, 0x36,  // 8
    0x26, 0x49, 0x49, 0x49, 0x3E,  // 9
    0x00, 0x00, 0x14, 0x00, 0x00,  // :
    0x00, 0x20, 0x14, 0x00, 0x00,  // ;
    0x08, 0x14, 0x14, 0x22, 0x22,  // <
    0x14, 0x14, 0x14, 0x14, 0x14,  // =
    0x22, 0x22, 0x14, 0x14, 0x08,  // >
    0x02, 0x01, 0x59, 0x09, 0x06,  // ?
    0x3E, 0x41, 0x49, 0x55, 0x0E,  // @
    0x7E, 0x09, 0x09, 0x09, 0x7E,  // A
    0x7F, 0x49, 0x49, 0x49, 0x36,  // B
    0x3E, 0x41, 0x41, 0x41, 0x22,  // C
    0x7F, 0x41, 0x41, 0x41, 0x3E,  // D
    0x7F, 0x49, 0x49, 0x49, 0x41,  // E
    0x7F, 0x09, 0x09, 0x09, 0x01,  // F
    0x3E, 0x41, 0x41, 0x51, 0x32,  // G
    0x7F, 0x08, 0x08, 0x08, 0x7F,  // H
    0x41, 0x41, 0x7F, 0x41, 0x41,  // I
    0x21, 0x41, 0x41, 0x3F, 0x01,  // J
    0x7F, 0x08, 0x14, 0x22, 0x41,  // K
    0x7F, 0x40, 0x40, 0x40, 0x40,  // L
    0x7F, 0x04, 0x78, 0x04, 0x7F,  // M
    0x7F, 0x04, 0x08, 0x10, 0x7F,  // N
    0x3E, 0x41, 0x41, 0x41, 0x3E,  // O
    0x7F, 0x09, 0x09, 0x09, 0x06,  // P
    0x3E, 0x41, 0x51, 0x61, 0x7E,  // Q
    0x7F, 0x09, 0x19, 0x29, 0x46,  // R
    0x26, 0x49, 0x49, 0x49, 0x32,  // S
    0x01, 0x01, 0x7F, 0x01, 0x01,  // T
    0x3F, 0x40, 0x40, 0x40, 0x3F,  // U
    0x03, 0x1C, 0x60, 0x1C, 0x03,  // V
    0x1F, 0x60, 0x18, 0x60, 0x1F,  // W
    0x63, 0x14, 0x08, 0x14, 0x63,  // X
    0x63, 0x14, 0x08, 0x04, 0x03,  // Y
    0x61, 0x51, 0x49, 0x45, 0x43,  // Z
    0x00, 0x7F, 0x41, 0x41, 0x00,  // [
    0x00, 0x03, 0x1C, 0x60, 0x00,  // \
    0x00, 0x41, 0x41, 0x7F, 0x00,  // ]
    0x04, 0x02, 0x01, 0x02, 0x04,  // ^
    0x40, 0x40, 0x40, 0x40, 0x40,  // _
    0x00, 0x01, 0x02, 0x04, 0x00,  // `
    0x7E, 0x09, 0x09, 0x09, 0x7E,  // a
    0x7F, 0x49, 0x49, 0x49, 0x36,  // b
    0x3E, 0x41, 0x41, 0x41, 0x22,  // c
    0x7F, 0x41, 0x41, 0x41, 0x3E,  // d
    0x7F, 0x49, 0x49, 0x49, 0x41,  // e
    0x7F, 0x09, 0x09, 0x09, 0x01,  // f
    0x3E, 0x41, 0x41, 0x51, 0x32,  // g
    0x7F, 0x08, 0x08, 0x08, 0x7F,  // h
    0x41, 0x41, 0x7F, 0x41, 0x41,  // i
    0x21, 0x41, 0x41, 0x3F, 0x01,  // j
    0x7F, 0x08, 0x14, 0x22, 0x41,  // k
    0x7F, 0x40, 0x40, 0x40, 0x40,  // l
    0x7F, 0x04, 0x78, 0x04, 0x7F,  // m
    0x7F, 0x04, 0x08, 0x10, 0x7F,  // n
    0x3E, 0x41, 0x41, 0x41, 0x3E,  // o
    0x7F, 0x09, 0x09, 0x09, 0x06,  // p
    0x3E, 0x41, 0x51, 0x61, 0x7E,  // q
    0x7F, 0x09, 0x19, 0x29, 0x46,  // r
    0x26, 0x49, 0x49, 0x49, 0x32,  // s
    0x01, 0x01, 0x7F, 0x01, 0x01,  // t
    0x3F, 0x40, 0x40, 0x40, 0x3F,  // u
    0x03, 0x1C, 0x60, 0x1C, 0x03,  // v
    0x1F, 0x60, 0x18, 0x60, 0x1F,  // w
    0x63, 0x14, 0x08, 0x14, 0x63,  // x
    0x63, 0x14, 0x08, 0x04, 0x03,  // y
    0x61, 0x51, 0x49, 0x45, 0x43,  // z
    0x08, 0x36, 0x41, 0x41, 0x00,  // {
    0x00, 0x00, 0x7F, 0x00, 0x00,  // |
    0x00, 0x41, 0x41, 0x36, 0x08,  // }
    0x02, 0x01, 0x02, 0x04, 0x02,  // ~
    0x00, 0x00, 0x00, 0x00, 0x00,  //
];