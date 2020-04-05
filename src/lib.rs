#![no_std]
#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

pub const MAGIC_NUMBER: &'static [u8] = &[0, 97, 115, 109];
pub const VERSION_1: &'static [u8] = &[1, 0, 0, 0];
pub const I32: u8 = 127;
pub const I64: u8 = 126;
pub const F32: u8 = 125;
pub const F64: u8 = 124;
pub const ANYFUNC: u8 = 112;
pub const FUNC: u8 = 96;
pub const EMPTY: u8 = 64;
pub const SECTION_CUSTOM: u8 = 0;
pub const SECTION_TYPE: u8 = 1;
pub const SECTION_IMPORT: u8 = 2;
pub const SECTION_FUNCTION: u8 = 3;
pub const SECTION_TABLE: u8 = 4;
pub const SECTION_MEMORY: u8 = 5;
pub const SECTION_GLOBAL: u8 = 6;
pub const SECTION_EXPORT: u8 = 7;
pub const SECTION_START: u8 = 8;
pub const SECTION_ELEMENT: u8 = 9;
pub const SECTION_CODE: u8 = 10;
pub const SECTION_DATA: u8 = 11;
pub const UNREACHABLE: u8 = 0;
pub const NOP: u8 = 1;
pub const BLOCK: u8 = 2;
pub const LOOP: u8 = 3;
pub const IF: u8 = 4;
pub const ELSE: u8 = 5;
pub const END: u8 = 11;
pub const BR: u8 = 12;
pub const BR_IF: u8 = 13;
pub const BR_TABLE: u8 = 14;
pub const RETURN: u8 = 15;
pub const CALL: u8 = 16;
pub const CALL_INDIRECT: u8 = 17;
pub const DROP: u8 = 26;
pub const SELECT: u8 = 27;
pub const LOCAL_GET: u8 = 32;
pub const LOCAL_SET: u8 = 33;
pub const LOCAL_TEE: u8 = 34;
pub const GLOBAL_GET: u8 = 35;
pub const GLOBAL_SET: u8 = 36;
pub const I32_LOAD: u8 = 40;
pub const I64_LOAD: u8 = 41;
pub const F32_LOAD: u8 = 42;
pub const F64_LOAD: u8 = 43;
pub const I32_LOAD8_S: u8 = 44;
pub const I32_LOAD8_U: u8 = 45;
pub const I32_LOAD16_S: u8 = 46;
pub const I32_LOAD16_U: u8 = 47;
pub const I64_LOAD8_S: u8 = 48;
pub const I64_LOAD8_U: u8 = 49;
pub const I64_LOAD16_S: u8 = 50;
pub const I64_LOAD16_U: u8 = 51;
pub const I64_LOAD32_S: u8 = 52;
pub const I64_LOAD32_U: u8 = 53;
pub const I32_STORE: u8 = 54;
pub const I64_STORE: u8 = 55;
pub const F32_STORE: u8 = 56;
pub const F64_STORE: u8 = 57;
pub const I32_STORE8: u8 = 58;
pub const I32_STORE16: u8 = 59;
pub const I64_STORE8: u8 = 60;
pub const I64_STORE16: u8 = 61;
pub const I64_STORE32: u8 = 62;
pub const MEMORY_SIZE: u8 = 63;
pub const MEMORY_GROW: u8 = 64;
pub const I32_CONST: u8 = 65;
pub const I64_CONST: u8 = 66;
pub const F32_CONST: u8 = 67;
pub const F64_CONST: u8 = 68;
pub const I32_EQZ: u8 = 69;
pub const I32_EQ: u8 = 70;
pub const I32_NE: u8 = 71;
pub const I32_LT_S: u8 = 72;
pub const I32_LT_U: u8 = 73;
pub const I32_GT_S: u8 = 74;
pub const I32_GT_U: u8 = 75;
pub const I32_LE_S: u8 = 76;
pub const I32_LE_U: u8 = 77;
pub const I32_GE_S: u8 = 78;
pub const I32_GE_U: u8 = 79;
pub const I64_EQZ: u8 = 80;
pub const I64_EQ: u8 = 81;
pub const I64_NE: u8 = 82;
pub const I64_LT_S: u8 = 83;
pub const I64_LT_U: u8 = 84;
pub const I64_GT_S: u8 = 85;
pub const I64_GT_U: u8 = 86;
pub const I64_LE_S: u8 = 87;
pub const I64_LE_U: u8 = 88;
pub const I64_GE_S: u8 = 89;
pub const I64_GE_U: u8 = 90;
pub const F32_EQ: u8 = 91;
pub const F32_NE: u8 = 92;
pub const F32_LT: u8 = 93;
pub const F32_GT: u8 = 94;
pub const F32_LE: u8 = 95;
pub const F32_GE: u8 = 96;
pub const F64_EQ: u8 = 97;
pub const F64_NE: u8 = 98;
pub const F64_LT: u8 = 99;
pub const F64_GT: u8 = 100;
pub const F64_LE: u8 = 101;
pub const F64_GE: u8 = 102;
pub const I32_CLZ: u8 = 103;
pub const I32_CTZ: u8 = 104;
pub const I32_POPCNT: u8 = 105;
pub const I32_ADD: u8 = 106;
pub const I32_SUB: u8 = 107;
pub const I32_MUL: u8 = 108;
pub const I32_DIV_S: u8 = 109;
pub const I32_DIV_U: u8 = 110;
pub const I32_REM_S: u8 = 111;
pub const I32_REM_U: u8 = 112;
pub const I32_AND: u8 = 113;
pub const I32_OR: u8 = 114;
pub const I32_XOR: u8 = 115;
pub const I32_SHL: u8 = 116;
pub const I32_SHR_S: u8 = 117;
pub const I32_SHR_U: u8 = 118;
pub const I32_ROTL: u8 = 119;
pub const I32_ROTR: u8 = 120;
pub const I64_CLZ: u8 = 121;
pub const I64_CTZ: u8 = 122;
pub const I64_POPCNT: u8 = 123;
pub const I64_ADD: u8 = 124;
pub const I64_SUB: u8 = 125;
pub const I64_MUL: u8 = 126;
pub const I64_DIV_S: u8 = 127;
pub const I64_DIV_U: u8 = 128;
pub const I64_REM_S: u8 = 129;
pub const I64_REM_U: u8 = 130;
pub const I64_AND: u8 = 131;
pub const I64_OR: u8 = 132;
pub const I64_XOR: u8 = 133;
pub const I64_SHL: u8 = 134;
pub const I64_SHR_S: u8 = 135;
pub const I64_SHR_U: u8 = 136;
pub const I64_ROTL: u8 = 137;
pub const I64_ROTR: u8 = 138;
pub const F32_ABS: u8 = 139;
pub const F32_NEG: u8 = 140;
pub const F32_CEIL: u8 = 141;
pub const F32_FLOOR: u8 = 142;
pub const F32_TRUNC: u8 = 143;
pub const F32_NEAREST: u8 = 144;
pub const F32_SQRT: u8 = 145;
pub const F32_ADD: u8 = 146;
pub const F32_SUB: u8 = 147;
pub const F32_MUL: u8 = 148;
pub const F32_DIV: u8 = 149;
pub const F32_MIN: u8 = 150;
pub const F32_MAX: u8 = 151;
pub const F32_COPYSIGN: u8 = 152;
pub const F64_ABS: u8 = 153;
pub const F64_NEG: u8 = 154;
pub const F64_CEIL: u8 = 155;
pub const F64_FLOOR: u8 = 156;
pub const F64_TRUNC: u8 = 157;
pub const F64_NEAREST: u8 = 158;
pub const F64_SQRT: u8 = 159;
pub const F64_ADD: u8 = 160;
pub const F64_SUB: u8 = 161;
pub const F64_MUL: u8 = 162;
pub const F64_DIV: u8 = 163;
pub const F64_MIN: u8 = 164;
pub const F64_MAX: u8 = 165;
pub const F64_COPYSIGN: u8 = 166;
pub const I32_WRAP_F64: u8 = 167;
pub const I32_TRUNC_S_F32: u8 = 168;
pub const I32_TRUNC_U_F32: u8 = 169;
pub const I32_TRUNC_S_F64: u8 = 170;
pub const I32_TRUNC_U_F64: u8 = 171;
pub const I64_EXTEND_S_I32: u8 = 172;
pub const I64_EXTEND_U_I32: u8 = 173;
pub const I64_TRUNC_S_F32: u8 = 174;
pub const I64_TRUNC_U_F32: u8 = 175;
pub const I64_TRUNC_S_F64: u8 = 176;
pub const I64_TRUNC_U_F64: u8 = 177;
pub const F32_CONVERT_S_I32: u8 = 178;
pub const F32_CONVERT_U_I32: u8 = 179;
pub const F32_CONVERT_S_I64: u8 = 180;
pub const F32_CONVERT_U_I64: u8 = 181;
pub const F32_DEMOTE_F64: u8 = 182;
pub const F64_CONVERT_S_I32: u8 = 183;
pub const F64_CONVERT_U_I32: u8 = 184;
pub const F64_CONVERT_S_I64: u8 = 185;
pub const F64_CONVERT_U_I64: u8 = 186;
pub const F64_PROMOTE_F32: u8 = 187;
pub const I32_REINTERPRET_F32: u8 = 188;
pub const I64_REINTERPRET_F64: u8 = 189;
pub const F32_REINTERPRET_I32: u8 = 190;
pub const F64_REINTERPRET_I64: u8 = 191;
pub const DESC_FUNCTION: u8 = 0;
pub const DESC_TABLE: u8 = 1;
pub const DESC_MEMORY: u8 = 2;
pub const DESC_GLOBAL: u8 = 3;
pub const LIMIT_MIN: u8 = 0;
pub const LIMIT_MIN_MAX: u8 = 1;
pub const IMMUTABLE: u8 = 0;
pub const MUTABLE: u8 = 1;

pub trait TypeWasmExt {
    fn into_wasm_bytes(&self) -> Vec<u8>;
}

impl TypeWasmExt for u32 {
    fn into_wasm_bytes(&self) -> Vec<u8> {
        let mut value = *self;
        let mut bytes: Vec<u8> = vec![];
        loop {
            let mut byte = (value & 0x7F) as u8;
            value = value >> 0x07;
            let is_done = value == 0;
            if !is_done {
                // add more flag to byte
                byte = byte | 0x80;
            }
            bytes.push(byte);
            if is_done {
                break;
            }
        }
        bytes
    }
}

impl TypeWasmExt for usize {
    fn into_wasm_bytes(&self) -> Vec<u8> {
        let mut value = *self;
        let mut bytes: Vec<u8> = vec![];
        loop {
            let mut byte = (value & 0x7F) as u8;
            value = value >> 0x07;
            let is_done = value == 0;
            if !is_done {
                // add more flag to byte
                byte = byte | 0x80;
            }
            bytes.push(byte);
            if is_done {
                break;
            }
        }
        bytes
    }
}

impl TypeWasmExt for i32 {
    fn into_wasm_bytes(&self) -> Vec<u8> {
        let mut value = *self;
        let mut bytes: Vec<u8> = vec![];
        loop {
            let mut byte = value as u8;
            value >>= 6;
            let is_done = value == 0 || value == -1;
            if is_done {
                byte &= 0x7F;
                bytes.push(byte);
                break;
            } else {
                value >>= 1;
                byte |= 0x80;
                bytes.push(byte);
            }
        }
        bytes
    }
}

impl TypeWasmExt for i64 {
    fn into_wasm_bytes(&self) -> Vec<u8> {
        let mut value = *self;
        let mut bytes: Vec<u8> = vec![];
        loop {
            let mut byte = value as u8;
            value >>= 6;
            let is_done = value == 0 || value == -1;
            if is_done {
                byte &= 0x7F;
                bytes.push(byte);
                break;
            } else {
                value >>= 1;
                byte |= 0x80;
                bytes.push(byte);
            }
        }
        bytes
    }
}

impl TypeWasmExt for f64 {
    fn into_wasm_bytes(&self) -> Vec<u8> {
        let raw_bytes: [u8; 8] = unsafe { core::mem::transmute(*self) };
        let bytes: Vec<u8> = raw_bytes.to_vec();
        bytes
    }
}

impl TypeWasmExt for f32 {
    fn into_wasm_bytes(&self) -> Vec<u8> {
        let raw_bytes: [u8; 4] = unsafe { core::mem::transmute(*self) };
        let bytes: Vec<u8> = raw_bytes.to_vec();
        bytes
    }
}

pub trait BytesWasmExt {
    fn try_extract_f32(self, start: usize) -> Result<(f32, usize), &'static str>;
    fn try_extract_f64(self, start: usize) -> Result<(f64, usize), &'static str>;
    fn try_extract_u32(self, start: usize) -> Result<(u32, usize), &'static str>;
    fn try_extract_i32(self, start: usize) -> Result<(i32, usize), &'static str>;
    fn try_extract_i64(self, start: usize) -> Result<(i64, usize), &'static str>;
}

impl BytesWasmExt for &[u8] {
    fn try_extract_f32(self, start: usize) -> Result<(f32, usize), &'static str> {
        if start + 4 > self.len() {
            return Err("invalid f32 representation");
        }
        let raw_bytes: [u8; 4] = [self[0], self[1], self[2], self[3]];
        let r: f32 = unsafe { core::mem::transmute(raw_bytes) };
        Ok((r, 4))
    }

    fn try_extract_f64(self, start: usize) -> Result<(f64, usize), &'static str> {
        if start + 8 > self.len() {
            return Err("invalid f64 representation");
        }
        let raw_bytes: [u8; 8] = [
            self[0], self[1], self[2], self[3], self[4], self[5], self[6], self[7],
        ];
        let r: f64 = unsafe { core::mem::transmute(raw_bytes) };
        Ok((r, 4))
    }

    fn try_extract_u32(self, start: usize) -> Result<(u32, usize), &'static str> {
        let mut i = 0;
        let mut r = 0;
        let mut ct = 0;
        loop {
            if i >= self.len() {
                return Err("invalid u32 representation");
            }
            let cur_byte = self[i + start];
            let has_more = cur_byte & 0x80 != 0;
            let cur_value = cur_byte & 0x7F;
            ct += 1;
            r += (cur_value as u32) << i * 7;
            if !has_more {
                break;
            }
            i += 1;
        }
        Ok((r, ct))
    }

    fn try_extract_i32(self, start: usize) -> Result<(i32, usize), &'static str> {
        let mut i = 0;
        let mut r = 0;
        let mut ct = 0;
        loop {
            if i >= self.len() {
                return Err("invalid i32 representation");
            }
            let cur_byte = self[i + start];
            ct += 1;
            let has_more = cur_byte & 0x80 != 0;
            let cur_value = cur_byte & 0x7F;
            r += (cur_value as i32) << i * 7;
            if !has_more {
                let is_negative = (cur_byte & 0x40) != 0;
                if is_negative {
                    if i < 4 {
                        r |= !0 << (i+1) * 7
                    };
                }
                break;
            }
            i += 1;
        }
        Ok((r, ct))
    }

    fn try_extract_i64(self, start: usize) -> Result<(i64, usize), &'static str> {
        let mut i = 0;
        let mut r = 0;
        let mut ct = 0;
        loop {
            if i >= self.len() {
                return Err("invalid i64 representation");
            }
            let cur_byte = self[i + start];
            ct += 1;
            let has_more = cur_byte & 0x80 != 0;
            let cur_value = cur_byte & 0x7F;
            r += (cur_value as i64) << i * 7;
            if !has_more {
                let is_negative = (cur_byte & 0x40) != 0;
                if is_negative {
                    if i < 8 {
                        r |= !0 << (i + 1) * 7;
                    }
                }
                break;
            }
            i += 1;
        }
        Ok((r, ct))
    }
}

#[cfg(test)]
mod tests {
    use crate::BytesWasmExt;
    use crate::TypeWasmExt;

    #[test]
    fn test_unsigned_int() {
        let n = 0 as u32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0]);
        assert_eq!(n, data.try_extract_u32(0).unwrap().0);

        let n = 42 as u32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [42]);
        assert_eq!(n, data.try_extract_u32(0).unwrap().0);

        let n = 127 as u32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [127]);
        assert_eq!(n, data.try_extract_u32(0).unwrap().0);

        let n = 128 as u32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x80, 1]);
        assert_eq!(n, data.try_extract_u32(0).unwrap().0);

        let n = 16384 as u32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x80, 0x80, 1]);
        assert_eq!(n, data.try_extract_u32(0).unwrap().0);
        
        let n = 2139062144 as u32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [128, 255, 253, 251, 7]);
        assert_eq!(n, data.try_extract_u32(0).unwrap().0);
    }

    #[test]
    fn test_unsigned_f32() {
        let n = 0.0 as f32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0, 0, 0, 0]);
        assert_eq!(n, data.try_extract_f32(0).unwrap().0);

        let n = 3.14 as f32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [195, 245, 72, 64]);
        assert_eq!(n, data.try_extract_f32(0).unwrap().0);

        let n = -1000.0 as f32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0, 0, 122, 196]);
        assert_eq!(n, data.try_extract_f32(0).unwrap().0);
    }

    #[test]
    fn test_unsigned_f64() {
        let n = 0.0 as f64;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(n, data.try_extract_f64(0).unwrap().0);

        let n = 3.14 as f64;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [31, 133, 235, 81, 184, 30, 9, 64]);
        assert_eq!(n, data.try_extract_f64(0).unwrap().0);

        let n = -1000.0 as f64;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0, 0, 0, 0, 0, 64, 143, 192]);
        assert_eq!(n, data.try_extract_f64(0).unwrap().0);
    }

    #[test]
    fn test_signed_int() {
        let n = 0 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = 1 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [1]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = 2 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [2]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = 42 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [42]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = 127 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0xFF, 0]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = 128 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x80, 1]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = 16384 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x80, 0x80, 1]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = -1 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x7F]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = -2 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x7E]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = -127 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x81, 0x7F]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = -128 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x80, 0x7F]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);

        let n = -16384 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [0x80, 0x80, 0x7F]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);


        let n = -2139062144 as i32;
        let data = n.into_wasm_bytes();
        assert_eq!(data, [128, 129, 130, 132, 120]);
        assert_eq!(n, data.try_extract_i32(0).unwrap().0);


        let n = 0 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = 1 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = 2 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = 42 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = 127 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = 128 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = 16384 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = -1 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = -2 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = -127 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = -128 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = -16384 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);

        let n = -16234324234 as i64;
        let data = n.into_wasm_bytes();
        assert_eq!(n, data.try_extract_i64(0).unwrap().0);
    }
}
