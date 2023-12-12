#![no_std]

#[cfg(debug_assertions)]
pub mod CodeHash {

    pub const CLUSTER_CODE_HASHES: [[u8; 32]; 1] = [
        [
            0x25, 0xb3, 0xc2, 0xde, 0x0c, 0xcc, 0x3e, 0x5f,
            0x3a, 0x3a, 0x38, 0x97, 0x40, 0xd1, 0xb9, 0x48,
            0xa0, 0x7f, 0x82, 0xa7, 0x44, 0x0e, 0xcb, 0xc6,
            0xb4, 0x6e, 0x00, 0x54, 0xf6, 0xd8, 0x3b, 0x93
        ]
    ];

    pub const CLUSTER_PROXY_CODE_HASHES: [[u8; 32]; 1] = [
        [
            0x99, 0x40, 0x1e, 0xe5, 0x5b, 0xcd, 0x7c, 0x09,
            0xe3, 0x7f, 0xc9, 0xa4, 0x8f, 0x4f, 0x21, 0x56,
            0x15, 0x3c, 0x54, 0xe5, 0x14, 0x39, 0x54, 0x81,
            0x11, 0xdd, 0xa0, 0x5e, 0x33, 0x83, 0x08, 0xdc
        ]
    ];

    pub const CLUSTER_AGENT_CODE_HASHES: [[u8; 32]; 1] = [
        [
            0xab, 0xd3, 0xe0, 0x32, 0x53, 0x8b, 0x39, 0xe9,
            0x55, 0x5d, 0x96, 0xc9, 0xc3, 0x2f, 0x4b, 0x55,
            0xf7, 0x8d, 0x11, 0x9e, 0x9b, 0x26, 0x70, 0x22,
            0xd9, 0x20, 0xdb, 0xb2, 0xa4, 0x47, 0x33, 0x4a
        ]
    ];
    pub const CKB_LUA_LIB_CODE_HASH: [u8; 32] = [237, 8, 250, 238, 140, 41, 183, 167, 194, 155, 217, 212, 149, 180, 185, 60, 194, 7, 189, 112, 202, 147, 247, 179, 86, 243, 156, 103, 126, 122, 176, 252];

    pub const SPORE_EXTENSION_LUA: [[u8; 32]; 1] = [
        [
            0xed, 0x3c, 0xe7, 0xc7, 0xe0, 0xbc, 0xc1, 0x48,
            0xc4, 0x0a, 0x0c, 0x5f, 0x16, 0x66, 0xbf, 0xa0,
            0x4f, 0x72, 0x84, 0xbb, 0x05, 0x87, 0xbb, 0xd5,
            0x04, 0xad, 0x43, 0xcf, 0x7b, 0x64, 0xa4, 0x0e
        ]
    ];
}

#[cfg(not(debug_assertions))]
pub mod CodeHash {
    pub const CLUSTER_CODE_HASHES: [[u8; 32]; 2] = [
        [
            89, 141, 121, 61, 239, 239, 54, 226,
            238, 186, 84, 169, 180, 81, 48, 228,
            202, 146, 130, 46, 29, 25, 54, 113,
            244, 144, 149, 12, 59, 133, 96, 128
        ],
        [
            0x15, 0xf8, 0x35, 0xc4, 0xca, 0x0b, 0x86, 0x1d,
            0xf3, 0x8f, 0x10, 0xd4, 0xe9, 0x5c, 0x51, 0xba,
            0x9c, 0xee, 0x3c, 0x89, 0xf1, 0x78, 0xb2, 0x1e,
            0x2e, 0x28, 0xba, 0xa6, 0x7e, 0xbd, 0x8b, 0x42
        ]
    ];

    pub const CLUSTER_PROXY_CODE_HASHES: [[u8; 32]; 1] = [
        [
            0x42, 0x84, 0x57, 0xc4, 0x47, 0xf0, 0x20, 0x0e,
            0x30, 0x2c, 0x3b, 0x64, 0xf0, 0xee, 0x0c, 0x16,
            0x5b, 0x75, 0x9e, 0x9d, 0x3b, 0x98, 0x11, 0x8c,
            0x55, 0x71, 0x0b, 0xf2, 0xf2, 0x94, 0xa7, 0xc2
        ]
    ];

    pub const CLUSTER_AGENT_CODE_HASHES: [[u8; 32]; 1] = [
        [
            0xe3, 0x68, 0x25, 0x66, 0x55, 0x6f, 0xa3, 0x43,
            0x9c, 0x24, 0xf7, 0xf9, 0xf9, 0x30, 0x6a, 0xbc,
            0x29, 0x2c, 0x15, 0x01, 0x09, 0x43, 0x49, 0x41,
            0x33, 0xd8, 0x94, 0x0d, 0x0e, 0x05, 0xba, 0x32
        ]
    ];

    pub const CKB_LUA_LIB_CODE_HASH: [u8; 32] = [237, 8, 250, 238, 140, 41, 183, 167, 194, 155, 217, 212, 149, 180, 185, 60, 194, 7, 189, 112, 202, 147, 247, 179, 86, 243, 156, 103, 126, 122, 176, 252];

    pub const SPORE_EXTENSION_LUA: [[u8; 32]; 1] = [
        [
            0xb4, 0xd3, 0xf2, 0x07, 0x83, 0x1e, 0x27, 0x74,
            0xd3, 0x10, 0xa8, 0x75, 0x71, 0xfb, 0x00, 0x95,
            0xf5, 0xb4, 0xaf, 0x4f, 0xa1, 0x76, 0xd8, 0xbf,
            0xaa, 0xe0, 0x19, 0x1a, 0x4d, 0x69, 0x89, 0xc8
        ]
    ];
}
