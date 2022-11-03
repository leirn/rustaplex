pub struct MurphyAnimationDescriptor {
    // TODO: migrate offset to a (X, Y) coordinate system or to at least something that takes 320 pixel width screen
    // into account instead of the 122 positions of the original game
    animation_coordinates_offset: i16, // word_510F0 -> 0x0DE0 -> seems like an offset from the destination position in dimensions of the original game screen (meaning, you have to divide by 122 to get the Y coordinate, and get module 122 and multiply by 8 to get the X coordinate)
    animation_coordinates_offset_increment: i16, // word_510F2 -> this increases the offset above frame by frame
    width: u16,                                  // word_510F4
    height: u16,                                 // word_510F6
    animation_index: u16, // word_510F8; -> memory address in someBinaryData_5142E, looks like a list of coordinates of frames in MOVING.DAT
    speed_x: i16,         // word_510FA; -> applied to Murphy X position... speedX?
    speed_y: i16,         // word_510FC; -> applied to Murphy Y position... speedY?
    current_frame: u16, // Not used in the original code, I will use it to keep track of the current animation frame
}

pub struct AnimationFrameCoordinates {
    coordinates: [Point; 105],
    number_of_coordinates: u8,
}

pub struct Point {
    x: u16,
    y: u16,
}

#[allow(overflowing_literals)]
pub const K_MURPHY_ANIMATION_DESCRIPTOR: [MurphyAnimationDescriptor; 51] = [
    MurphyAnimationDescriptor {
        // 0
        animation_coordinates_offset: 0x06ac,           // -> dfe
        animation_coordinates_offset_increment: 0xff0c, // -> e00
        width: 0x0002,                                  // -> e02
        height: 0x0012,                                 // -> e04
        animation_index: 0,                             // 0x111e, // -> e06
        speed_x: 0x0000,                                // -> e08
        speed_y: 0xfffe,                                // -> e0a
        current_frame: 0x0000,                          // -> e0c
    },
    MurphyAnimationDescriptor {
        // 0
        animation_coordinates_offset: 0x06ac,           // -> dfe
        animation_coordinates_offset_increment: 0xff0c, // -> e00
        width: 0x0002,                                  // -> e02
        height: 0x0012,                                 // -> e04
        animation_index: 0,                             // 0x111e, // -> e06
        speed_x: 0x0000,                                // -> e08
        speed_y: 0xfffe,                                // -> e0a
        current_frame: 0x0000,                          // -> e0c
    },
    MurphyAnimationDescriptor {
        // 1
        animation_coordinates_offset: 0x06ac,           // -> e0e
        animation_coordinates_offset_increment: 0xff0c, // -> e10
        width: 0x0002,                                  // -> e12
        height: 0x0012,                                 // -> e14
        animation_index: 1,                             // 0x1130, // -> e16
        speed_x: 0x0000,                                // -> e18
        speed_y: 0xfffe,                                // -> e1a
        current_frame: 0x0000,                          // -> e1c
    },
    MurphyAnimationDescriptor {
        // 2
        animation_coordinates_offset: 0x0000,           // -> e1e
        animation_coordinates_offset_increment: 0x0000, // -> e20
        width: 0x0004,                                  // -> e22
        height: 0x0010,                                 // -> e24
        animation_index: 4,                             // 0x1166, // -> e26
        speed_x: 0xfffe,                                // -> e28
        speed_y: 0x0000,                                // -> e2a
        current_frame: 0x0000,                          // -> e2c
    },
    MurphyAnimationDescriptor {
        // 3
        animation_coordinates_offset: 0xf860,           // -> e2e
        animation_coordinates_offset_increment: 0x00f4, // -> e30
        width: 0x0002,                                  // -> e32
        height: 0x0012,                                 // -> e34
        animation_index: 2,                             // 0x1142, // -> e36
        speed_x: 0x0000,                                // -> e38
        speed_y: 0x0002,                                // -> e3a
        current_frame: 0x0000,                          // -> e3c
    },
    MurphyAnimationDescriptor {
        // 4
        animation_coordinates_offset: 0xf860,           // -> e3e
        animation_coordinates_offset_increment: 0x00f4, // -> e40
        width: 0x0002,                                  // -> e42
        height: 0x0012,                                 // -> e44
        animation_index: 3,                             // 0x1154 // -> e46
        speed_x: 0x0000,                                // -> e48
        speed_y: 0x0002,                                // -> e4a
        current_frame: 0x0000,                          // -> e4c
    },
    MurphyAnimationDescriptor {
        // 5
        animation_coordinates_offset: 0xfffe,           // -> e4e
        animation_coordinates_offset_increment: 0x0000, // -> e50
        width: 0x0004,                                  // -> e52
        height: 0x0010,                                 // -> e54
        animation_index: 5,                             // 0x1178, // -> e56
        speed_x: 0x0002,                                // -> e58
        speed_y: 0x0000,                                // -> e5a
        current_frame: 0x0000,                          // -> e5c
    },
    MurphyAnimationDescriptor {
        // 6
        animation_coordinates_offset: 0x0000,           // -> e5e
        animation_coordinates_offset_increment: 0x0000, // -> e60
        width: 0x0002,                                  // -> e62
        height: 0x0010,                                 // -> e64
        animation_index: 6,                             // 0x118a, // -> e66
        speed_x: 0x0000,                                // -> e68
        speed_y: 0x0000,                                // -> e6a
        current_frame: 0x0000,                          // -> e6c
    },
    MurphyAnimationDescriptor {
        // 7
        animation_coordinates_offset: 0x06ac,           // -> e6e
        animation_coordinates_offset_increment: 0xff0c, // -> e70
        width: 0x0002,                                  // -> e72
        height: 0x0012,                                 // -> e74
        animation_index: 0,                             // 0x111e, // -> e76
        speed_x: 0x0000,                                // -> e78
        speed_y: 0xfffe,                                // -> e7a
        current_frame: 0x0000,                          // -> e7c
    },
    MurphyAnimationDescriptor {
        // 8
        animation_coordinates_offset: 0x06ac,           // -> e7e
        animation_coordinates_offset_increment: 0xff0c, // -> e80
        width: 0x0002,                                  // -> e82
        height: 0x0012,                                 // -> e84
        animation_index: 1,                             // 0x1130, // -> e86
        speed_x: 0x0000,                                // -> e88
        speed_y: 0xfffe,                                // -> e8a
        current_frame: 0x0000,                          // -> e8c
    },
    MurphyAnimationDescriptor {
        // 9
        animation_coordinates_offset: 0x0000,           // -> e8e
        animation_coordinates_offset_increment: 0x0000, // -> e90
        width: 0x0004,                                  // -> e92
        height: 0x0010,                                 // -> e94
        animation_index: 7,                             // 0x11dc, // -> e96
        speed_x: 0xfffe,                                // -> e98
        speed_y: 0x0000,                                // -> e9a
        current_frame: 0x0000,                          // -> e9c
    },
    MurphyAnimationDescriptor {
        // 10
        animation_coordinates_offset: 0xf860,           // -> e9e
        animation_coordinates_offset_increment: 0x00f4, // -> ea0
        width: 0x0002,                                  // -> ea2
        height: 0x0012,                                 // -> ea4
        animation_index: 2,                             // 0x1142, // -> ea6
        speed_x: 0x0000,                                // -> ea8
        speed_y: 0x0002,                                // -> eaa
        current_frame: 0x0000,                          // -> eac
    },
    MurphyAnimationDescriptor {
        // 11
        animation_coordinates_offset: 0xf860,           // -> eae
        animation_coordinates_offset_increment: 0x00f4, // -> eb0
        width: 0x0002,                                  // -> eb2
        height: 0x0012,                                 // -> eb4
        animation_index: 3,                             // 0x1154 // -> eb6
        speed_x: 0x0000,                                // -> eb8
        speed_y: 0x0002,                                // -> eba
        current_frame: 0x0000,                          // -> ebc
    },
    MurphyAnimationDescriptor {
        // 12
        animation_coordinates_offset: 0xfffe,           // -> ebe
        animation_coordinates_offset_increment: 0x0000, // -> ec0
        width: 0x0004,                                  // -> ec2
        height: 0x0010,                                 // -> ec4
        animation_index: 8,                             // 0x11ee, // -> ec6
        speed_x: 0x0002,                                // -> ec8
        speed_y: 0x0000,                                // -> eca
        current_frame: 0x0000,                          // -> ecc
    },
    MurphyAnimationDescriptor {
        // 13
        animation_coordinates_offset: 0xf860,           // -> ece
        animation_coordinates_offset_increment: 0x0000, // -> ed0
        width: 0x0002,                                  // -> ed2
        height: 0x0010,                                 // -> ed4
        animation_index: 9,                             // 0x1200, // -> ed6
        speed_x: 0x0000,                                // -> ed8
        speed_y: 0x0000,                                // -> eda
        current_frame: 0x0000,                          // -> edc
    },
    MurphyAnimationDescriptor {
        // 14
        animation_coordinates_offset: 0xfffe,           // -> ede
        animation_coordinates_offset_increment: 0x0000, // -> ee0
        width: 0x0002,                                  // -> ee2
        height: 0x0010,                                 // -> ee4
        animation_index: 9,                             // 0x1200, // -> ee6
        speed_x: 0x0000,                                // -> ee8
        speed_y: 0x0000,                                // -> eea
        current_frame: 0x0000,                          // -> eec
    },
    MurphyAnimationDescriptor {
        // 15
        animation_coordinates_offset: 0x07a0,           // -> eee
        animation_coordinates_offset_increment: 0x0000, // -> ef0
        width: 0x0002,                                  // -> ef2
        height: 0x0010,                                 // -> ef4
        animation_index: 9,                             // 0x1200, // -> ef6
        speed_x: 0x0000,                                // -> ef8
        speed_y: 0x0000,                                // -> efa
        current_frame: 0x0000,                          // -> efc
    },
    MurphyAnimationDescriptor {
        // 16
        animation_coordinates_offset: 0x0002,           // -> efe
        animation_coordinates_offset_increment: 0x0000, // -> f00
        width: 0x0002,                                  // -> f02
        height: 0x0010,                                 // -> f04
        animation_index: 9,                             // 0x1200, // -> f06
        speed_x: 0x0000,                                // -> f08
        speed_y: 0x0000,                                // -> f0a
        current_frame: 0x0000,                          // -> f0c
    },
    MurphyAnimationDescriptor {
        // 17
        animation_coordinates_offset: 0x06ac,           // -> f0e
        animation_coordinates_offset_increment: 0xff0c, // -> f10
        width: 0x0002,                                  // -> f12
        height: 0x0012,                                 // -> f14
        animation_index: 0,                             // 0x111e, // -> f16
        speed_x: 0x0000,                                // -> f18
        speed_y: 0xfffe,                                // -> f1a
        current_frame: 0x0000,                          // -> f1c
    },
    MurphyAnimationDescriptor {
        // 18
        animation_coordinates_offset: 0x06ac,           // -> f1e
        animation_coordinates_offset_increment: 0xff0c, // -> f20
        width: 0x0002,                                  // -> f22
        height: 0x0012,                                 // -> f24
        animation_index: 1,                             // 0x1130, // -> f26
        speed_x: 0x0000,                                // -> f28
        speed_y: 0xfffe,                                // -> f2a
        current_frame: 0x0000,                          // -> f2c
    },
    MurphyAnimationDescriptor {
        // 19
        animation_coordinates_offset: 0x0000,           // -> f2e
        animation_coordinates_offset_increment: 0x0000, // -> f30
        width: 0x0004,                                  // -> f32
        height: 0x0010,                                 // -> f34
        animation_index: 10,                            // 0x1212, // -> f36
        speed_x: 0xfffe,                                // -> f38
        speed_y: 0x0000,                                // -> f3a
        current_frame: 0x0000,                          // -> f3c
    },
    MurphyAnimationDescriptor {
        // 20
        animation_coordinates_offset: 0xf860,           // -> f3e
        animation_coordinates_offset_increment: 0x00f4, // -> f40
        width: 0x0002,                                  // -> f42
        height: 0x0012,                                 // -> f44
        animation_index: 2,                             // 0x1142, // -> f46
        speed_x: 0x0000,                                // -> f48
        speed_y: 0x0002,                                // -> f4a
        current_frame: 0x0000,                          // -> f4c
    },
    MurphyAnimationDescriptor {
        // 21
        animation_coordinates_offset: 0xf860,           // -> f4e
        animation_coordinates_offset_increment: 0x00f4, // -> f50
        width: 0x0002,                                  // -> f52
        height: 0x0012,                                 // -> f54
        animation_index: 3,                             // 0x1154 // -> f56
        speed_x: 0x0000,                                // -> f58
        speed_y: 0x0002,                                // -> f5a
        current_frame: 0x0000,                          // -> f5c
    },
    MurphyAnimationDescriptor {
        // 22
        animation_coordinates_offset: 0xfffe,           // -> f5e
        animation_coordinates_offset_increment: 0x0000, // -> f60
        width: 0x0004,                                  // -> f62
        height: 0x0010,                                 // -> f64
        animation_index: 11,                            // 0x1224, // -> f66
        speed_x: 0x0002,                                // -> f68
        speed_y: 0x0000,                                // -> f6a
        current_frame: 0x0000,                          // -> f6c
    },
    MurphyAnimationDescriptor {
        // 23
        animation_coordinates_offset: 0xf860,           // -> f6e
        animation_coordinates_offset_increment: 0x0000, // -> f70
        width: 0x0002,                                  // -> f72
        height: 0x0010,                                 // -> f74
        animation_index: 12,                            // 0x1236, // -> f76
        speed_x: 0x0000,                                // -> f78
        speed_y: 0x0000,                                // -> f7a
        current_frame: 0x0000,                          // -> f7c
    },
    MurphyAnimationDescriptor {
        // 24
        animation_coordinates_offset: 0xfffe,           // -> f7e
        animation_coordinates_offset_increment: 0x0000, // -> f80
        width: 0x0002,                                  // -> f82
        height: 0x0010,                                 // -> f84
        animation_index: 12,                            // 0x1236, // -> f86
        speed_x: 0x0000,                                // -> f88
        speed_y: 0x0000,                                // -> f8a
        current_frame: 0x0000,                          // -> f8c
    },
    MurphyAnimationDescriptor {
        // 25
        animation_coordinates_offset: 0x07a0,           // -> f8e
        animation_coordinates_offset_increment: 0x0000, // -> f90
        width: 0x0002,                                  // -> f92
        height: 0x0010,                                 // -> f94
        animation_index: 12,                            // 0x1236, // -> f96
        speed_x: 0x0000,                                // -> f98
        speed_y: 0x0000,                                // -> f9a
        current_frame: 0x0000,                          // -> f9c
    },
    MurphyAnimationDescriptor {
        // 26
        animation_coordinates_offset: 0x0002,           // -> f9e
        animation_coordinates_offset_increment: 0x0000, // -> fa0
        width: 0x0002,                                  // -> fa2
        height: 0x0010,                                 // -> fa4
        animation_index: 12,                            // 0x1236, // -> fa6
        speed_x: 0x0000,                                // -> fa8
        speed_y: 0x0000,                                // -> faa
        current_frame: 0x0000,                          // -> fac
    },
    MurphyAnimationDescriptor {
        // 27
        animation_coordinates_offset: 0xfffc,           // -> fae
        animation_coordinates_offset_increment: 0x0000, // -> fb0
        width: 0x0006,                                  // -> fb2
        height: 0x0010,                                 // -> fb4
        animation_index: 13,                            // 0x1246, // -> fb6
        speed_x: 0xfffe,                                // -> fb8
        speed_y: 0x0000,                                // -> fba
        current_frame: 0x0000,                          // -> fbc
    },
    MurphyAnimationDescriptor {
        // 28
        animation_coordinates_offset: 0x0000,           // -> fbe
        animation_coordinates_offset_increment: 0x0000, // -> fc0
        width: 0x0006,                                  // -> fc2
        height: 0x0010,                                 // -> fc4
        animation_index: 14,                            // 0x1258, // -> fc6
        speed_x: 0x0002,                                // -> fc8
        speed_y: 0x0000,                                // -> fca
        current_frame: 0x0000,                          // -> fcc
    },
    MurphyAnimationDescriptor {
        // 29
        animation_coordinates_offset: 0x0000,           // -> fce
        animation_coordinates_offset_increment: 0xf0c0, // -> fd0
        width: 0x0002,                                  // -> fd2
        height: 0x0010,                                 // -> fd4
        animation_index: 19,                            // 0x1340, // -> fd6
        speed_x: 0x0000,                                // -> fd8
        speed_y: 0xfffc,                                // -> fda
        current_frame: 0x0000,                          // -> fdc
    },
    MurphyAnimationDescriptor {
        // 30
        animation_coordinates_offset: 0x0000,           // -> fde
        animation_coordinates_offset_increment: 0xfffc, // -> fe0
        width: 0x0002,                                  // -> fe2
        height: 0x0010,                                 // -> fe4
        animation_index: 15,                            // 0x12f8, // -> fe6
        speed_x: 0xfffc,                                // -> fe8
        speed_y: 0x0000,                                // -> fea
        current_frame: 0x0000,                          // -> fec
    },
    MurphyAnimationDescriptor {
        // 31
        animation_coordinates_offset: 0x0000,           // -> fee
        animation_coordinates_offset_increment: 0x0f40, // -> ff0
        width: 0x0002,                                  // -> ff2
        height: 0x0010,                                 // -> ff4
        animation_index: 21,                            // 0x1364, // -> ff6
        speed_x: 0x0000,                                // -> ff8
        speed_y: 0x0004,                                // -> ffa
        current_frame: 0x0000,                          // -> ffc
    },
    MurphyAnimationDescriptor {
        // 32
        animation_coordinates_offset: 0x0000,           // -> ffe
        animation_coordinates_offset_increment: 0x0004, // -> 1000
        width: 0x0002,                                  // -> 1002
        height: 0x0010,                                 // -> 1004
        animation_index: 17,                            // 0x131c, // -> 1006
        speed_x: 0x0004,                                // -> 1008
        speed_y: 0x0000,                                // -> 100a
        current_frame: 0x0000,                          // -> 100c
    },
    MurphyAnimationDescriptor {
        // 33
        animation_coordinates_offset: 0xff0c, // -> 100e
        animation_coordinates_offset_increment: 0xff0c, // -> 1010
        width: 0x0002,                        // -> 1012
        height: 0x0012,                       // -> 1014
        animation_index: 0,                   // 0x111e, // -> 1016
        speed_x: 0x0000,                      // -> 1018
        speed_y: 0xfffe,                      // -> 101a
        current_frame: 0x0000,                // -> 101c
    },
    MurphyAnimationDescriptor {
        // 34
        animation_coordinates_offset: 0xff0c, // -> 101e
        animation_coordinates_offset_increment: 0xff0c, // -> 1020
        width: 0x0002,                        // -> 1022
        height: 0x0012,                       // -> 1024
        animation_index: 1,                   // 0x1130, // -> 1026
        speed_x: 0x0000,                      // -> 1028
        speed_y: 0xfffe,                      // -> 102a
        current_frame: 0x0000,                // -> 102c
    },
    MurphyAnimationDescriptor {
        // 35
        animation_coordinates_offset: 0x0000, // -> 102e
        animation_coordinates_offset_increment: 0x0000, // -> 1030
        width: 0x0004,                        // -> 1032
        height: 0x0010,                       // -> 1034
        animation_index: 23,                  // 0x1448, // -> 1036
        speed_x: 0xfffe,                      // -> 1038
        speed_y: 0x0000,                      // -> 103a
        current_frame: 0x0000,                // -> 103c
    },
    MurphyAnimationDescriptor {
        // 36
        animation_coordinates_offset: 0x0000, // -> 103e
        animation_coordinates_offset_increment: 0x00f4, // -> 1040
        width: 0x0002,                        // -> 1042
        height: 0x0012,                       // -> 1044
        animation_index: 2,                   // 0x1142, // -> 1046
        speed_x: 0x0000,                      // -> 1048
        speed_y: 0x0002,                      // -> 104a
        current_frame: 0x0000,                // -> 104c
    },
    MurphyAnimationDescriptor {
        // 37
        animation_coordinates_offset: 0x0000, // -> 104e
        animation_coordinates_offset_increment: 0x00f4, // -> 1050
        width: 0x0002,                        // -> 1052
        height: 0x0012,                       // -> 1054
        animation_index: 3,                   // 0x1154 // -> 1056
        speed_x: 0x0000,                      // -> 1058
        speed_y: 0x0002,                      // -> 105a
        current_frame: 0x0000,                // -> 105c
    },
    MurphyAnimationDescriptor {
        // 38
        animation_coordinates_offset: 0xfffe, // -> 105e
        animation_coordinates_offset_increment: 0x0000, // -> 1060
        width: 0x0004,                        // -> 1062
        height: 0x0010,                       // -> 1064
        animation_index: 24,                  // 0x145a, // -> 1066
        speed_x: 0x0002,                      // -> 1068
        speed_y: 0x0000,                      // -> 106a
        current_frame: 0x0000,                // -> 106c
    },
    MurphyAnimationDescriptor {
        // 39
        animation_coordinates_offset: 0xf860, // -> 106e
        animation_coordinates_offset_increment: 0x0000, // -> 1070
        width: 0x0002,                        // -> 1072
        height: 0x0010,                       // -> 1074
        animation_index: 25,                  // 0x146e, // -> 1076
        speed_x: 0x0000,                      // -> 1078
        speed_y: 0x0000,                      // -> 107a
        current_frame: 0x0000,                // -> 107c
    },
    MurphyAnimationDescriptor {
        // 40
        animation_coordinates_offset: 0xfffe, // -> 107e
        animation_coordinates_offset_increment: 0x0000, // -> 1080
        width: 0x0002,                        // -> 1082
        height: 0x0010,                       // -> 1084
        animation_index: 25,                  // 0x146e, // -> 1086
        speed_x: 0x0000,                      // -> 1088
        speed_y: 0x0000,                      // -> 108a
        current_frame: 0x0000,                // -> 108c
    },
    MurphyAnimationDescriptor {
        // 41
        animation_coordinates_offset: 0x07a0, // -> 108e
        animation_coordinates_offset_increment: 0x0000, // -> 1090
        width: 0x0002,                        // -> 1092
        height: 0x0010,                       // -> 1094
        animation_index: 25,                  // 0x146e, // -> 1096
        speed_x: 0x0000,                      // -> 1098
        speed_y: 0x0000,                      // -> 109a
        current_frame: 0x0000,                // -> 109c
    },
    MurphyAnimationDescriptor {
        // 42
        animation_coordinates_offset: 0x0002, // -> 109e
        animation_coordinates_offset_increment: 0x0000, // -> 10a0
        width: 0x0002,                        // -> 10a2
        height: 0x0010,                       // -> 10a4
        animation_index: 25,                  // 0x146e, // -> 10a6
        speed_x: 0x0000,                      // -> 10a8
        speed_y: 0x0000,                      // -> 10aa
        current_frame: 0x0000,                // -> 10ac
    },
    MurphyAnimationDescriptor {
        // 43
        animation_coordinates_offset: 0xf76c, // -> 10ae
        animation_coordinates_offset_increment: 0xff0c, // -> 10b0
        width: 0x0002,                        // -> 10b2
        height: 0x0022,                       // -> 10b4
        animation_index: 28,                  // 0x1488, // -> 10b6
        speed_x: 0x0000,                      // -> 10b8
        speed_y: 0xfffe,                      // -> 10ba
        current_frame: 0x0000,                // -> 10bc
    },
    MurphyAnimationDescriptor {
        // 44
        animation_coordinates_offset: 0xfffc, // -> 10be
        animation_coordinates_offset_increment: 0x0000, // -> 10c0
        width: 0x0006,                        // -> 10c2
        height: 0x0010,                       // -> 10c4
        animation_index: 29,                  // 0x149a, // -> 10c6
        speed_x: 0xfffe,                      // -> 10c8
        speed_y: 0x0000,                      // -> 10ca
        current_frame: 0x0000,                // -> 10cc
    },
    MurphyAnimationDescriptor {
        // 45
        animation_coordinates_offset: 0x0000, // -> 10ce
        animation_coordinates_offset_increment: 0x00f4, // -> 10d0
        width: 0x0002,                        // -> 10d2
        height: 0x0022,                       // -> 10d4
        animation_index: 30,                  // 0x14ac, // -> 10d6
        speed_x: 0x0000,                      // -> 10d8
        speed_y: 0x0002,                      // -> 10da
        current_frame: 0x0000,                // -> 10dc
    },
    MurphyAnimationDescriptor {
        // 46
        animation_coordinates_offset: 0x0000, // -> 10de
        animation_coordinates_offset_increment: 0x0000, // -> 10e0
        width: 0x0006,                        // -> 10e2
        height: 0x0010,                       // -> 10e4
        animation_index: 31,                  // 0x14be, // -> 10e6
        speed_x: 0x0002,                      // -> 10e8
        speed_y: 0x0000,                      // -> 10ea
        current_frame: 0x0000,                // -> 10ec
    },
    MurphyAnimationDescriptor {
        // 47
        animation_coordinates_offset: 0xfffc, // -> 10ee
        animation_coordinates_offset_increment: 0x0000, // -> 10f0
        width: 0x0006,                        // -> 10f2
        height: 0x0010,                       // -> 10f4
        animation_index: 32,                  // 0x14d0, // -> 10f6
        speed_x: 0xfffe,                      // -> 10f8
        speed_y: 0x0000,                      // -> 10fa
        current_frame: 0x0000,                // -> 10fc
    },
    MurphyAnimationDescriptor {
        // 48
        animation_coordinates_offset: 0x0000, // -> 10fe
        animation_coordinates_offset_increment: 0x0000, // -> 1100
        width: 0x0006,                        // -> 1102
        height: 0x0010,                       // -> 1104
        animation_index: 33,                  // 0x14e2, // -> 1106
        speed_x: 0x0002,                      // -> 1108
        speed_y: 0x0000,                      // -> 110a
        current_frame: 0x0000,                // -> 110c
    },
    MurphyAnimationDescriptor {
        // 49
        animation_coordinates_offset: 0x0000, // -> 110e
        animation_coordinates_offset_increment: 0x0000, // -> 1110
        width: 0x0002,                        // -> 1112
        height: 0x0010,                       // -> 1114
        animation_index: 27,                  // 0x1484, // -> 1116
        speed_x: 0x0000,                      // -> 1118
        speed_y: 0x0000,                      // -> 111a
        current_frame: 0x0000,                // -> 111c
    },
];

pub const K_MURPHY_ANIMATION_FRAME_COORDINATES: [&[Point]; 37] = [
    &[
        // 0
        Point { x: 0, y: 66 },  // -> 0x2ae6 -> 111e
        Point { x: 0, y: 66 },  // -> 0x2ae6 -> 1120
        Point { x: 16, y: 66 }, // -> 0x2ae8 -> 1122
        Point { x: 16, y: 66 }, // -> 0x2ae8 -> 1124
        Point { x: 32, y: 66 }, // -> 0x2aea -> 1126
        Point { x: 32, y: 66 }, // -> 0x2aea -> 1128
        Point { x: 16, y: 66 }, // -> 0x2ae8 -> 112a
        Point { x: 16, y: 66 }, // -> 0x2ae8 -> 112c
    ],
    &[
        // 1
        Point { x: 48, y: 66 }, // -> 0x2aec -> 1130
        Point { x: 48, y: 66 }, // -> 0x2aec -> 1132
        Point { x: 64, y: 66 }, // -> 0x2aee -> 1134
        Point { x: 64, y: 66 }, // -> 0x2aee -> 1136
        Point { x: 80, y: 66 }, // -> 0x2af0 -> 1138
        Point { x: 80, y: 66 }, // -> 0x2af0 -> 113a
        Point { x: 64, y: 66 }, // -> 0x2aee -> 113c
        Point { x: 64, y: 66 }, // -> 0x2aee -> 113e
    ],
    &[
        // 2
        Point { x: 0, y: 64 },  // -> 0x29f2 -> 1142
        Point { x: 0, y: 64 },  // -> 0x29f2 -> 1144
        Point { x: 16, y: 64 }, // -> 0x29f4 -> 1146
        Point { x: 16, y: 64 }, // -> 0x29f4 -> 1148
        Point { x: 32, y: 64 }, // -> 0x29f6 -> 114a
        Point { x: 32, y: 64 }, // -> 0x29f6 -> 114c
        Point { x: 16, y: 64 }, // -> 0x29f4 -> 114e
        Point { x: 16, y: 64 }, // -> 0x29f4 -> 1150
    ],
    &[
        // 3
        Point { x: 48, y: 64 }, // -> 0x29f8 -> 1154
        Point { x: 48, y: 64 }, // -> 0x29f8 -> 1156
        Point { x: 64, y: 64 }, // -> 0x29fa -> 1158
        Point { x: 64, y: 64 }, // -> 0x29fa -> 115a
        Point { x: 80, y: 64 }, // -> 0x29fc -> 115c
        Point { x: 80, y: 64 }, // -> 0x29fc -> 115e
        Point { x: 64, y: 64 }, // -> 0x29fa -> 1160
        Point { x: 64, y: 64 }, // -> 0x29fa -> 1162
    ],
    &[
        // 4
        Point { x: 32, y: 32 },  // -> 0x1ab6 -> 1166
        Point { x: 64, y: 32 },  // -> 0x1aba -> 1168
        Point { x: 96, y: 32 },  // -> 0x1abe -> 116a
        Point { x: 128, y: 32 }, // -> 0x1ac2 -> 116c
        Point { x: 160, y: 32 }, // -> 0x1ac6 -> 116e
        Point { x: 192, y: 32 }, // -> 0x1aca -> 1170
        Point { x: 224, y: 32 }, // -> 0x1ace -> 1172
        Point { x: 256, y: 32 }, // -> 0x1ad2 -> 1174
    ],
    &[
        // 5
        Point { x: 288, y: 32 }, // -> 0x1ad6 -> 1178
        Point { x: 0, y: 48 },   // -> 0x2252 -> 117a
        Point { x: 32, y: 48 },  // -> 0x2256 -> 117c
        Point { x: 64, y: 48 },  // -> 0x225a -> 117e
        Point { x: 96, y: 48 },  // -> 0x225e -> 1180
        Point { x: 128, y: 48 }, // -> 0x2262 -> 1182
        Point { x: 160, y: 48 }, // -> 0x2266 -> 1184
        Point { x: 192, y: 48 }, // -> 0x226a -> 1186
    ],
    &[
        // 6
        Point { x: 160, y: 64 }, // -> 0x2a06 -> 118a
        Point { x: 160, y: 64 }, // -> 0x2a06 -> 118c
        Point { x: 160, y: 64 }, // -> 0x2a06 -> 118e
        Point { x: 160, y: 64 }, // -> 0x2a06 -> 1190
        Point { x: 176, y: 64 }, // -> 0x2a08 -> 1192
        Point { x: 176, y: 64 }, // -> 0x2a08 -> 1194
        Point { x: 176, y: 64 }, // -> 0x2a08 -> 1196
        Point { x: 176, y: 64 }, // -> 0x2a08 -> 1198
        Point { x: 192, y: 64 }, // -> 0x2a0a -> 119a
        Point { x: 192, y: 64 }, // -> 0x2a0a -> 119c
        Point { x: 192, y: 64 }, // -> 0x2a0a -> 119e
        Point { x: 192, y: 64 }, // -> 0x2a0a -> 11a0
        Point { x: 208, y: 64 }, // -> 0x2a0c -> 11a2
        Point { x: 208, y: 64 }, // -> 0x2a0c -> 11a4
        Point { x: 208, y: 64 }, // -> 0x2a0c -> 11a6
        Point { x: 208, y: 64 }, // -> 0x2a0c -> 11a8
        Point { x: 224, y: 64 }, // -> 0x2a0e -> 11aa
        Point { x: 224, y: 64 }, // -> 0x2a0e -> 11ac
        Point { x: 224, y: 64 }, // -> 0x2a0e -> 11ae
        Point { x: 224, y: 64 }, // -> 0x2a0e -> 11b0
        Point { x: 240, y: 64 }, // -> 0x2a10 -> 11b2
        Point { x: 240, y: 64 }, // -> 0x2a10 -> 11b4
        Point { x: 240, y: 64 }, // -> 0x2a10 -> 11b6
        Point { x: 240, y: 64 }, // -> 0x2a10 -> 11b8
        Point { x: 256, y: 64 }, // -> 0x2a12 -> 11ba
        Point { x: 256, y: 64 }, // -> 0x2a12 -> 11bc
        Point { x: 256, y: 64 }, // -> 0x2a12 -> 11be
        Point { x: 256, y: 64 }, // -> 0x2a12 -> 11c0
        Point { x: 272, y: 64 }, // -> 0x2a14 -> 11c2
        Point { x: 272, y: 64 }, // -> 0x2a14 -> 11c4
        Point { x: 272, y: 64 }, // -> 0x2a14 -> 11c6
        Point { x: 272, y: 64 }, // -> 0x2a14 -> 11c8
        Point { x: 288, y: 64 }, // -> 0x2a16 -> 11ca
        Point { x: 288, y: 64 }, // -> 0x2a16 -> 11cc
        Point { x: 288, y: 64 }, // -> 0x2a16 -> 11ce
        Point { x: 288, y: 64 }, // -> 0x2a16 -> 11d0
        Point { x: 240, y: 0 },  // -> 0x0b90 -> 11d2
        Point { x: 240, y: 0 },  // -> 0x0b90 -> 11d4
        Point { x: 240, y: 0 },  // -> 0x0b90 -> 11d6
        Point { x: 240, y: 0 },  // -> 0x0b90 -> 11d8
    ],
    &[
        // 7
        Point { x: 0, y: 0 },   // -> 0x0b72 -> 11dc
        Point { x: 32, y: 0 },  // -> 0x0b76 -> 11de
        Point { x: 64, y: 0 },  // -> 0x0b7a -> 11e0
        Point { x: 96, y: 0 },  // -> 0x0b7e -> 11e2
        Point { x: 128, y: 0 }, // -> 0x0b82 -> 11e4
        Point { x: 160, y: 0 }, // -> 0x0b86 -> 11e6
        Point { x: 192, y: 0 }, // -> 0x0b8a -> 11e8
        Point { x: 224, y: 0 }, // -> 0x0b8e -> 11ea
    ],
    &[
        // 8
        Point { x: 256, y: 0 },  // -> 0x0b92 -> 11ee
        Point { x: 288, y: 0 },  // -> 0x0b96 -> 11f0
        Point { x: 0, y: 16 },   // -> 0x1312 -> 11f2
        Point { x: 32, y: 16 },  // -> 0x1316 -> 11f4
        Point { x: 64, y: 16 },  // -> 0x131a -> 11f6
        Point { x: 96, y: 16 },  // -> 0x131e -> 11f8
        Point { x: 128, y: 16 }, // -> 0x1322 -> 11fa
        Point { x: 160, y: 16 }, // -> 0x1326 -> 11fc
    ],
    &[
        // 9
        Point { x: 256, y: 84 },  // -> 0x339a -> 1200
        Point { x: 272, y: 84 },  // -> 0x339c -> 1202
        Point { x: 288, y: 84 },  // -> 0x339e -> 1204
        Point { x: 304, y: 84 },  // -> 0x33a0 -> 1206
        Point { x: 256, y: 100 }, // -> 0x3b3a -> 1208
        Point { x: 272, y: 100 }, // -> 0x3b3c -> 120a
        Point { x: 288, y: 100 }, // -> 0x3b3e -> 120c
        Point { x: 304, y: 148 }, // -> 0x0514 -> 120e
    ],
    &[
        // 10
        Point { x: 0, y: 212 },   // -> 0x236e -> 1212
        Point { x: 32, y: 212 },  // -> 0x2372 -> 1214
        Point { x: 64, y: 212 },  // -> 0x2376 -> 1216
        Point { x: 96, y: 212 },  // -> 0x237a -> 1218
        Point { x: 128, y: 212 }, // -> 0x237e -> 121a
        Point { x: 160, y: 212 }, // -> 0x2382 -> 121c
        Point { x: 192, y: 212 }, // -> 0x2386 -> 121e
        Point { x: 224, y: 212 }, // -> 0x238a -> 1220
    ],
    &[
        // 11
        Point { x: 256, y: 212 }, // -> 0x238e -> 1224
        Point { x: 288, y: 212 }, // -> 0x2392 -> 1226
        Point { x: 0, y: 228 },   // -> 0x2b0e -> 1228
        Point { x: 32, y: 228 },  // -> 0x2b12 -> 122a
        Point { x: 64, y: 228 },  // -> 0x2b16 -> 122c
        Point { x: 96, y: 228 },  // -> 0x2b1a -> 122e
        Point { x: 128, y: 228 }, // -> 0x2b1e -> 1230
        Point { x: 160, y: 228 }, // -> 0x2b22 -> 1232
    ],
    &[
        // 12
        Point { x: 192, y: 148 }, // -> 0x0506 -> 1236
        Point { x: 208, y: 148 }, // -> 0x0508 -> 1238
        Point { x: 224, y: 148 }, // -> 0x050a -> 123a
        Point { x: 256, y: 148 }, // -> 0x050e -> 123c
        Point { x: 272, y: 148 }, // -> 0x0510 -> 123e
        Point { x: 288, y: 148 }, // -> 0x0512 -> 1240
        Point { x: 304, y: 148 }, // -> 0x0514 -> 1242
    ],
    &[
        // 13
        Point { x: 0, y: 116 },   // -> 0x42ba -> 1246
        Point { x: 48, y: 116 },  // -> 0x42c0 -> 1248
        Point { x: 96, y: 116 },  // -> 0x42c6 -> 124a
        Point { x: 144, y: 116 }, // -> 0x42cc -> 124c
        Point { x: 192, y: 116 }, // -> 0x42d2 -> 124e
        Point { x: 240, y: 116 }, // -> 0x42d8 -> 1250
        Point { x: 0, y: 132 },   // -> 0x4a5a -> 1252
        Point { x: 48, y: 132 },  // -> 0x4a60 -> 1254
    ],
    &[
        // 14
        Point { x: 96, y: 132 },  // -> 0x4a66 -> 1258
        Point { x: 144, y: 132 }, // -> 0x4a6c -> 125a
        Point { x: 192, y: 132 }, // -> 0x4a72 -> 125c
        Point { x: 240, y: 132 }, // -> 0x4a78 -> 125e
        Point { x: 0, y: 148 },   // -> 0x04ee -> 1260
        Point { x: 48, y: 148 },  // -> 0x04f4 -> 1262
        Point { x: 96, y: 148 },  // -> 0x04fa -> 1264
        Point { x: 144, y: 148 }, // -> 0x0500 -> 1266
    ],
    &[
        // 15
        Point { x: 48, y: 32 },  // -> 0x1ab8 -> 12f8
        Point { x: 80, y: 32 },  // -> 0x1abc -> 12fa
        Point { x: 112, y: 32 }, // -> 0x1ac0 -> 12fc
        Point { x: 144, y: 32 }, // -> 0x1ac4 -> 12fe
        Point { x: 176, y: 32 }, // -> 0x1ac8 -> 1300
        Point { x: 208, y: 32 }, // -> 0x1acc -> 1302
        Point { x: 240, y: 32 }, // -> 0x1ad0 -> 1304
        Point { x: 272, y: 32 }, // -> 0x1ad4 -> 1306
    ],
    &[
        // 16
        Point { x: 32, y: 32 },  // -> 0x1ab6 -> 130a
        Point { x: 64, y: 32 },  // -> 0x1aba -> 130c
        Point { x: 96, y: 32 },  // -> 0x1abe -> 130e
        Point { x: 128, y: 32 }, // -> 0x1ac2 -> 1310
        Point { x: 160, y: 32 }, // -> 0x1ac6 -> 1312
        Point { x: 192, y: 32 }, // -> 0x1aca -> 1314
        Point { x: 224, y: 32 }, // -> 0x1ace -> 1316
        Point { x: 256, y: 32 }, // -> 0x1ad2 -> 1318
    ],
    &[
        // 17
        Point { x: 288, y: 32 }, // -> 0x1ad6 -> 131c
        Point { x: 0, y: 48 },   // -> 0x2252 -> 131e
        Point { x: 32, y: 48 },  // -> 0x2256 -> 1320
        Point { x: 64, y: 48 },  // -> 0x225a -> 1322
        Point { x: 96, y: 48 },  // -> 0x225e -> 1324
        Point { x: 128, y: 48 }, // -> 0x2262 -> 1326
        Point { x: 160, y: 48 }, // -> 0x2266 -> 1328
        Point { x: 192, y: 48 }, // -> 0x226a -> 132a
    ],
    &[
        // 18
        Point { x: 304, y: 32 }, // -> 0x1ad8 -> 132e
        Point { x: 16, y: 48 },  // -> 0x2254 -> 1330
        Point { x: 48, y: 48 },  // -> 0x2258 -> 1332
        Point { x: 80, y: 48 },  // -> 0x225c -> 1334
        Point { x: 112, y: 48 }, // -> 0x2260 -> 1336
        Point { x: 144, y: 48 }, // -> 0x2264 -> 1338
        Point { x: 176, y: 48 }, // -> 0x2268 -> 133a
        Point { x: 208, y: 48 }, // -> 0x226c -> 133c
    ],
    &[
        // 19
        Point { x: 304, y: 134 }, // -> 0x4b74 -> 1340
        Point { x: 304, y: 136 }, // -> 0x4c68 -> 1342
        Point { x: 304, y: 138 }, // -> 0x0050 -> 1344
        Point { x: 304, y: 140 }, // -> 0x0144 -> 1346
        Point { x: 304, y: 142 }, // -> 0x0238 -> 1348
        Point { x: 304, y: 144 }, // -> 0x032c -> 134a
        Point { x: 304, y: 146 }, // -> 0x0420 -> 134c
        Point { x: 304, y: 148 }, // -> 0x0514 -> 134e
    ],
    &[
        // 20
        Point { x: 304, y: 118 }, // -> 0x43d4 -> 1352
        Point { x: 304, y: 120 }, // -> 0x44c8 -> 1354
        Point { x: 304, y: 122 }, // -> 0x45bc -> 1356
        Point { x: 304, y: 124 }, // -> 0x46b0 -> 1358
        Point { x: 304, y: 126 }, // -> 0x47a4 -> 135a
        Point { x: 304, y: 128 }, // -> 0x4898 -> 135c
        Point { x: 304, y: 130 }, // -> 0x498c -> 135e
        Point { x: 304, y: 132 }, // -> 0x4a80 -> 1360
    ],
    &[
        // 21
        Point { x: 304, y: 130 }, // -> 0x498c -> 1364
        Point { x: 304, y: 128 }, // -> 0x4898 -> 1366
        Point { x: 304, y: 126 }, // -> 0x47a4 -> 1368
        Point { x: 304, y: 124 }, // -> 0x46b0 -> 136a
        Point { x: 304, y: 122 }, // -> 0x45bc -> 136c
        Point { x: 304, y: 120 }, // -> 0x44c8 -> 136e
        Point { x: 304, y: 118 }, // -> 0x43d4 -> 1370
        Point { x: 304, y: 116 }, // -> 0x42e0 -> 1372
    ],
    &[
        // 22
        Point { x: 304, y: 146 }, // -> 0x0420 -> 1376
        Point { x: 304, y: 144 }, // -> 0x032c -> 1378
        Point { x: 304, y: 142 }, // -> 0x0238 -> 137a
        Point { x: 304, y: 140 }, // -> 0x0144 -> 137c
        Point { x: 304, y: 138 }, // -> 0x0050 -> 137e
        Point { x: 304, y: 136 }, // -> 0x4c68 -> 1380
        Point { x: 304, y: 134 }, // -> 0x4b74 -> 1382
        Point { x: 304, y: 132 }, // -> 0x4a80 -> 1384
    ],
    &[
        // 23

        // Murphy eating red disk left
        Point { x: 128, y: 260 }, // -> 0x3a5e -> 1448
        Point { x: 160, y: 260 }, // -> 0x3a62 -> 144a
        Point { x: 192, y: 260 }, // -> 0x3a66 -> 144c
        Point { x: 224, y: 260 }, // -> 0x3a6a -> 144e
        Point { x: 256, y: 260 }, // -> 0x3a6e -> 1450
        Point { x: 288, y: 260 }, // -> 0x3a72 -> 1452
        Point { x: 288, y: 276 }, // -> 0x4212 -> 1454
        Point { x: 288, y: 292 }, // -> 0x49b2 -> 1456
    ],
    &[
        // 24

        // Murphy eating red disk right
        Point { x: 192, y: 308 }, // -> 0x043a -> 145a
        Point { x: 224, y: 308 }, // -> 0x043e -> 145c
        Point { x: 256, y: 308 }, // -> 0x0442 -> 145e
        Point { x: 288, y: 308 }, // -> 0x0446 -> 1460
        Point { x: 288, y: 308 }, // -> 0x0446 -> 1462 // this is probably duplicated by mistake
        Point { x: 288, y: 324 }, // -> 0x0be6 -> 1464
        Point { x: 288, y: 340 }, // -> 0x1386 -> 1466
        Point { x: 192, y: 356 }, // -> 0x1b1a -> 1468
        Point { x: 224, y: 356 }, // -> 0x1b1e -> 146a
    ],
    &[
        // 25
        Point { x: 256, y: 164 }, // 0x0CAE -> 146e
        Point { x: 272, y: 164 }, // 0x0CB0 -> 1470
        Point { x: 288, y: 164 }, // 0x0CB2 -> 1472
        Point { x: 304, y: 164 }, // 0x0CB4 -> 1474
        Point { x: 256, y: 180 }, // 0x144E -> 1476
        Point { x: 272, y: 180 }, // 0x1450 -> 1478
        Point { x: 288, y: 180 }, // 0x1452 -> 147A
        Point { x: 304, y: 180 }, // 0x1454 -> 147C
    ],
    &[
        // 26
        Point { x: 288, y: 132 }, // 0x4A7E -> 1480
    ],
    &[
        // 27
        Point { x: 256, y: 164 }, // 0x0CAE -> 1484
    ],
    &[
        // 28
        Point { x: 304, y: 406 }, // 0x32FC -> 1488
        Point { x: 304, y: 406 }, // 0x32FC -> 148A
        Point { x: 304, y: 406 }, // 0x32FC -> 148C
        Point { x: 304, y: 406 }, // 0x32FC -> 148E
        Point { x: 304, y: 406 }, // 0x32FC -> 1490
        Point { x: 304, y: 406 }, // 0x32FC -> 1492
        Point { x: 304, y: 406 }, // 0x32FC -> 1494
        Point { x: 304, y: 406 }, // 0x32FC -> 1496
    ],
    &[
        // 29
        Point { x: 0, y: 324 },   // 0x0BC2 -> 149A
        Point { x: 48, y: 324 },  // 0x0BC8 -> 149C
        Point { x: 96, y: 324 },  // 0x0BCE -> 149E
        Point { x: 144, y: 324 }, // 0x0BD4 -> 14A0
        Point { x: 192, y: 324 }, // 0x0BDA -> 14A2
        Point { x: 240, y: 324 }, // 0x0BE0 -> 14A4
        Point { x: 0, y: 340 },   // 0x1362 -> 14A6
        Point { x: 48, y: 340 },  // 0x1368 -> 14A8
    ],
    &[
        // 30
        Point { x: 288, y: 406 }, // 0x32FA -> 14AC
        Point { x: 288, y: 406 }, // 0x32FA -> 14AE
        Point { x: 288, y: 406 }, // 0x32FA -> 14B0
        Point { x: 288, y: 406 }, // 0x32FA -> 14B2
        Point { x: 288, y: 406 }, // 0x32FA -> 14B4
        Point { x: 288, y: 406 }, // 0x32FA -> 14B6
        Point { x: 288, y: 406 }, // 0x32FA -> 14B8
        Point { x: 288, y: 406 }, // 0x32FA -> 14BA
    ],
    &[
        // 31
        Point { x: 96, y: 340 },  // 0x136E -> 14BE
        Point { x: 144, y: 340 }, // 0x1374 -> 14C0
        Point { x: 192, y: 340 }, // 0x137A -> 14C2
        Point { x: 240, y: 340 }, // 0x1380 -> 14C4
        Point { x: 0, y: 356 },   // 0x1B02 -> 14C6
        Point { x: 48, y: 356 },  // 0x1B08 -> 14C8
        Point { x: 96, y: 356 },  // 0x1B0E -> 14CA
        Point { x: 144, y: 356 }, // 0x1B14 -> 14CC
    ],
    &[
        // 32
        Point { x: 0, y: 276 },   // 0x41EE -> 14D0
        Point { x: 48, y: 276 },  // 0x41F4 -> 14D2
        Point { x: 96, y: 276 },  // 0x41FA -> 14D4
        Point { x: 144, y: 276 }, // 0x4200 -> 14D6
        Point { x: 192, y: 276 }, // 0x4206 -> 14D8
        Point { x: 240, y: 276 }, // 0x420C -> 14DA
        Point { x: 0, y: 292 },   // 0x498E -> 14DC
        Point { x: 48, y: 292 },  // 0x4994 -> 14DE
    ],
    &[
        // 33
        Point { x: 96, y: 292 },  // 0x499A -> 14E2
        Point { x: 144, y: 292 }, // 0x49A0 -> 14E4
        Point { x: 192, y: 292 }, // 0x49A6 -> 14E6
        Point { x: 240, y: 292 }, // 0x49AC -> 14E8
        Point { x: 0, y: 308 },   // 0x0422 -> 14EA
        Point { x: 48, y: 308 },  // 0x0428 -> 14EC
        Point { x: 96, y: 308 },  // 0x042E -> 14EE
        Point { x: 144, y: 308 }, // 0x0434 -> 14F0
    ],
    &[
        // 34
        Point { x: 32, y: 446 },  // 0x45EA -> 14F4
        Point { x: 48, y: 446 },  // 0x45EC -> 14F6
        Point { x: 64, y: 446 },  // 0x45EE -> 14F8
        Point { x: 80, y: 446 },  // 0x45F0 -> 14FA
        Point { x: 96, y: 446 },  // 0x45F2 -> 14FC
        Point { x: 112, y: 446 }, // 0x45F4 -> 14FE
        Point { x: 128, y: 446 }, // 0x45F6 -> 1500
        Point { x: 144, y: 446 }, // 0x45F8 -> 1502
        Point { x: 160, y: 446 }, // 0x45FA -> 1504
        Point { x: 176, y: 446 }, // 0x45FC -> 1506
        Point { x: 192, y: 446 }, // 0x45FE -> 1508
        Point { x: 208, y: 446 }, // 0x4600 -> 150A
    ],
    &[
        // 35
        Point { x: 304, y: 446 }, // 0x460C -> 150E
        Point { x: 288, y: 446 }, // 0x460A -> 1510
        Point { x: 272, y: 446 }, // 0x4608 -> 1512
    ],
    &[
        // 36
        Point { x: 224, y: 446 }, // 0x4602 -> 1516
        Point { x: 240, y: 446 }, // 0x4604 -> 1518
        Point { x: 256, y: 446 }, // 0x4606 -> 151A
    ],
];

pub const K_BUG_FRAME_COORDINATES: [Point; 16] = [
    // binaryData_51582
    Point { x: 304, y: 100 }, // 0x3B40 -> 0x1272
    Point { x: 256, y: 196 }, // 0x1BEE -> 0x1274
    Point { x: 272, y: 196 }, // 0x1BF0 -> 0x1276
    Point { x: 288, y: 196 }, // 0x1BF2 -> 0x1278
    Point { x: 304, y: 196 }, // 0x1BF4 -> 0x127A
    Point { x: 288, y: 196 }, // 0x1BF2 -> 0x127C
    Point { x: 272, y: 196 }, // 0x1BF0 -> 0x127E
    Point { x: 288, y: 196 }, // 0x1BF2 -> 0x1280
    Point { x: 304, y: 196 }, // 0x1BF4 -> 0x1282
    Point { x: 288, y: 196 }, // 0x1BF2 -> 0x1284
    Point { x: 272, y: 196 }, // 0x1BF0 -> 0x1286
    Point { x: 256, y: 196 }, // 0x1BEE -> 0x1288
    Point { x: 304, y: 100 }, // 0x3B40 -> 0x128A
    Point { x: 304, y: 64 },  // 0x2A18 -> 0x128C
    Point { x: 304, y: 64 }, // 0x2A18 -> 0x128E -> this one probably doesn't belong to this array (there are only 14 bug frames according to updateBugTiles)
    Point { x: 224, y: 84 }, // 0x3396 -> 0x1290 -> this is a zonk, probably doesn't belong to this array
];

// Zonk frame coordinates
pub const K_ZONK_SLIDE_LEFT_ANIMATION_FRAME_COORDINATES: [Point; 8] = [
    Point { x: 0, y: 84 },   // 0x337A -> 0x1294
    Point { x: 32, y: 84 },  // 0x337E -> 0x1296
    Point { x: 64, y: 84 },  // 0x3382 -> 0x1298
    Point { x: 96, y: 84 },  // 0x3386 -> 0x129A
    Point { x: 128, y: 84 }, // 0x338A -> 0x129C
    Point { x: 160, y: 84 }, // 0x338E -> 0x129E
    Point { x: 192, y: 84 }, // 0x3392 -> 0x12A0
    Point { x: 224, y: 84 }, // 0x3396 -> 0x12A2
];

pub const K_ZONK_SLIDE_RIGHT_ANIMATION_FRAME_COORDINATES: [Point; 8] = [
    Point { x: 0, y: 100 },   // 0x3B1A -> 0x12A4
    Point { x: 32, y: 100 },  // 0x3B1E -> 0x12A6
    Point { x: 64, y: 100 },  // 0x3B22 -> 0x12A8
    Point { x: 96, y: 100 },  // 0x3B26 -> 0x12AA
    Point { x: 128, y: 100 }, // 0x3B2A -> 0x12AC
    Point { x: 160, y: 100 }, // 0x3B2E -> 0x12AE
    Point { x: 192, y: 100 }, // 0x3B32 -> 0x12B0
    Point { x: 224, y: 100 }, // 0x3B36 -> 0x12B2
];

pub const K_INFOTRON_SLIDE_LEFT_ANIMATION_FRAME_COORDINATES: [Point; 8] = [
    Point { x: 0, y: 164 },   // -> 0x0c8e -> 12b6
    Point { x: 32, y: 164 },  // -> 0x0c92 -> 12b8
    Point { x: 64, y: 164 },  // -> 0x0c96 -> 12ba
    Point { x: 96, y: 164 },  // -> 0x0c9a -> 12bc
    Point { x: 8, y: 164 },   // -> 0x0c8f -> 12be
    Point { x: 160, y: 164 }, // -> 0x0ca2 -> 12c0
    Point { x: 192, y: 164 }, // -> 0x0ca6 -> 12c2
    Point { x: 224, y: 164 }, // -> 0x0caa -> 12c4
];

pub const K_INFOTRON_SLIDE_RIGHT_ANIMATION_FRAME_COORDINATES: [Point; 8] = [
    Point { x: 0, y: 180 },   // -> 0x142e -> 12c6
    Point { x: 32, y: 180 },  // -> 0x1432 -> 12c8
    Point { x: 64, y: 180 },  // -> 0x1436 -> 12ca
    Point { x: 96, y: 180 },  // -> 0x143a -> 12cc
    Point { x: 128, y: 180 }, // -> 0x143e -> 12ce
    Point { x: 160, y: 180 }, // -> 0x1442 -> 12d0
    Point { x: 192, y: 180 }, // -> 0x1446 -> 12d2
    Point { x: 224, y: 180 }, // -> 0x144a -> 12d4
];

pub const K_REGULAR_EXPLOSION_ANIMATION_FRAME_COORDINATES: [Point; 8] = [
    Point { x: 0, y: 196 },   // -> 0x1bce -> 12d6
    Point { x: 16, y: 196 },  // -> 0x1bd0 -> 12d8
    Point { x: 32, y: 196 },  // -> 0x1bd2 -> 12da
    Point { x: 48, y: 196 },  // -> 0x1bd4 -> 12dc
    Point { x: 64, y: 196 },  // -> 0x1bd6 -> 12de
    Point { x: 80, y: 196 },  // -> 0x1bd8 -> 12e0
    Point { x: 96, y: 196 },  // -> 0x1bda -> 12e2
    Point { x: 112, y: 196 }, // -> 0x1bdc -> 12e4
];

pub const K_INFOTRON_EXPLOSION_ANIMATION_FRAME_COORDINATES: [Point; 8] = [
    Point { x: 128, y: 196 }, // -> 0x1bde -> 12e6
    Point { x: 144, y: 196 }, // -> 0x1be0 -> 12e8
    Point { x: 160, y: 196 }, // -> 0x1be2 -> 12ea
    Point { x: 176, y: 196 }, // -> 0x1be4 -> 12ec
    Point { x: 192, y: 196 }, // -> 0x1be6 -> 12ee
    Point { x: 208, y: 196 }, // -> 0x1be8 -> 12f0
    Point { x: 224, y: 196 }, // -> 0x1bea -> 12f2
    Point { x: 240, y: 196 }, // -> 0x1bec -> 12f4
];

// Snik snak animations
pub const K_SNIK_SNAK_ANIMATION_FRAME_COORDINATES: [Point; 48] = [
    Point { x: 192, y: 388 }, // -> 0x2a5a -> 1388
    Point { x: 64, y: 260 },  // -> 0x3a56 -> 138a
    Point { x: 96, y: 244 },  // -> 0x32ba -> 138c
    Point { x: 80, y: 260 },  // -> 0x3a58 -> 138e
    Point { x: 208, y: 388 }, // -> 0x2a5c -> 1390
    Point { x: 96, y: 260 },  // -> 0x3a5a -> 1392
    Point { x: 48, y: 260 },  // -> 0x3a54 -> 1394
    Point { x: 112, y: 260 }, // -> 0x3a5c -> 1396
    Point { x: 192, y: 388 }, // -> 0x2a5a -> 1398
    Point { x: 112, y: 260 }, // -> 0x3a5c -> 139a
    Point { x: 48, y: 260 },  // -> 0x3a54 -> 139c
    Point { x: 96, y: 260 },  // -> 0x3a5a -> 139e
    Point { x: 208, y: 388 }, // -> 0x2a5c -> 13a0
    Point { x: 80, y: 260 },  // -> 0x3a58 -> 13a2
    Point { x: 96, y: 244 },  // -> 0x32ba -> 13a4
    Point { x: 64, y: 260 },  // -> 0x3a56 -> 13a6
    Point { x: 0, y: 424 },   // -> 0x3b6a -> 13a8
    Point { x: 16, y: 424 },  // -> 0x3b6c -> 13aa
    Point { x: 32, y: 424 },  // -> 0x3b6e -> 13ac
    Point { x: 48, y: 424 },  // -> 0x3b70 -> 13ae
    Point { x: 64, y: 424 },  // -> 0x3b72 -> 13b0
    Point { x: 80, y: 424 },  // -> 0x3b74 -> 13b2
    Point { x: 96, y: 424 },  // -> 0x3b76 -> 13b4
    Point { x: 112, y: 424 }, // -> 0x3b78 -> 13b6
    Point { x: 192, y: 228 }, // -> 0x2b26 -> 13b8
    Point { x: 224, y: 228 }, // -> 0x2b2a -> 13ba
    Point { x: 256, y: 228 }, // -> 0x2b2e -> 13bc
    Point { x: 288, y: 228 }, // -> 0x2b32 -> 13be
    Point { x: 0, y: 244 },   // -> 0x32ae -> 13c0
    Point { x: 32, y: 244 },  // -> 0x32b2 -> 13c2
    Point { x: 64, y: 244 },  // -> 0x32b6 -> 13c4
    Point { x: 96, y: 244 },  // -> 0x32ba -> 13c6
    Point { x: 144, y: 422 }, // -> 0x3a88 -> 13c8
    Point { x: 160, y: 422 }, // -> 0x3a8a -> 13ca
    Point { x: 176, y: 422 }, // -> 0x3a8c -> 13cc
    Point { x: 192, y: 422 }, // -> 0x3a8e -> 13ce
    Point { x: 208, y: 422 }, // -> 0x3a90 -> 13d0
    Point { x: 224, y: 422 }, // -> 0x3a92 -> 13d2
    Point { x: 240, y: 422 }, // -> 0x3a94 -> 13d4
    Point { x: 256, y: 422 }, // -> 0x3a96 -> 13d6
    Point { x: 128, y: 244 }, // -> 0x32be -> 13d8
    Point { x: 160, y: 244 }, // -> 0x32c2 -> 13da
    Point { x: 192, y: 244 }, // -> 0x32c6 -> 13dc
    Point { x: 224, y: 244 }, // -> 0x32ca -> 13de
    Point { x: 256, y: 244 }, // -> 0x32ce -> 13e0
    Point { x: 288, y: 244 }, // -> 0x32d2 -> 13e2
    Point { x: 0, y: 260 },   // -> 0x3a4e -> 13e4
    Point { x: 32, y: 260 },  // -> 0x3a52 -> 13e6
];

// Electron animations
pub const K_ELECTRON_ANIMATION_FRAME_COORDINATES: [Point; 48] = [
    Point { x: 0, y: 404 },   // -> 0x31e2 -> 13e8
    Point { x: 16, y: 404 },  // -> 0x31e4 -> 13ea
    Point { x: 32, y: 404 },  // -> 0x31e6 -> 13ec
    Point { x: 48, y: 404 },  // -> 0x31e8 -> 13ee
    Point { x: 64, y: 404 },  // -> 0x31ea -> 13f0
    Point { x: 80, y: 404 },  // -> 0x31ec -> 13f2
    Point { x: 96, y: 404 },  // -> 0x31ee -> 13f4
    Point { x: 112, y: 404 }, // -> 0x31f0 -> 13f6
    Point { x: 0, y: 404 },   // -> 0x31e2 -> 13f8
    Point { x: 112, y: 404 }, // -> 0x31f0 -> 13fa
    Point { x: 96, y: 404 },  // -> 0x31ee -> 13fc
    Point { x: 80, y: 404 },  // -> 0x31ec -> 13fe
    Point { x: 64, y: 404 },  // -> 0x31ea -> 1400
    Point { x: 48, y: 404 },  // -> 0x31e8 -> 1402
    Point { x: 32, y: 404 },  // -> 0x31e6 -> 1404
    Point { x: 16, y: 404 },  // -> 0x31e4 -> 1406
    Point { x: 144, y: 404 }, // -> 0x31f4 -> 1408
    Point { x: 160, y: 404 }, // -> 0x31f6 -> 140a
    Point { x: 176, y: 404 }, // -> 0x31f8 -> 140c
    Point { x: 192, y: 404 }, // -> 0x31fa -> 140e
    Point { x: 208, y: 404 }, // -> 0x31fc -> 1410
    Point { x: 224, y: 404 }, // -> 0x31fe -> 1412
    Point { x: 240, y: 404 }, // -> 0x3200 -> 1414
    Point { x: 256, y: 404 }, // -> 0x3202 -> 1416
    Point { x: 0, y: 372 },   // -> 0x22a2 -> 1418
    Point { x: 32, y: 372 },  // -> 0x22a6 -> 141a
    Point { x: 64, y: 372 },  // -> 0x22aa -> 141c
    Point { x: 96, y: 372 },  // -> 0x22ae -> 141e
    Point { x: 128, y: 372 }, // -> 0x22b2 -> 1420
    Point { x: 160, y: 372 }, // -> 0x22b6 -> 1422
    Point { x: 192, y: 372 }, // -> 0x22ba -> 1424
    Point { x: 224, y: 372 }, // -> 0x22be -> 1426
    Point { x: 0, y: 402 },   // -> 0x30ee -> 1428
    Point { x: 16, y: 402 },  // -> 0x30f0 -> 142a
    Point { x: 32, y: 402 },  // -> 0x30f2 -> 142c
    Point { x: 48, y: 402 },  // -> 0x30f4 -> 142e
    Point { x: 64, y: 402 },  // -> 0x30f6 -> 1430
    Point { x: 80, y: 403 },  // -> 0x3172 -> 1432
    Point { x: 96, y: 403 },  // -> 0x3174 -> 1434
    Point { x: 112, y: 402 }, // -> 0x30fc -> 1436
    Point { x: 256, y: 372 }, // -> 0x22c2 -> 1438
    Point { x: 288, y: 372 }, // -> 0x22c6 -> 143a
    Point { x: 0, y: 388 },   // -> 0x2a42 -> 143c
    Point { x: 32, y: 388 },  // -> 0x2a46 -> 143e
    Point { x: 64, y: 388 },  // -> 0x2a4a -> 1440
    Point { x: 96, y: 388 },  // -> 0x2a4e -> 1442
    Point { x: 128, y: 388 }, // -> 0x2a52 -> 1444
    Point { x: 160, y: 388 }, // -> 0x2a56 -> 1446
];
