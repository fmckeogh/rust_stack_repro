#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use R_set::*;
use BitReverse::*;
use u__id::*;
use R_read::*;
use Zeros::*;
use integer_subrange::*;
use Poly32Mod2::*;
use common::*;
pub fn execute_aarch32_instrs_CRC32_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    crc32c: bool,
    d: i64,
    m: i64,
    n: i64,
    size: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        acc: u32,
        sizeshadow_7908: i64,
        ga_364589: i128,
        crc32c: bool,
        d: i64,
        m: i64,
        n: i64,
        size: i64,
    }
    let fn_state = FunctionState {
        crc32c,
        d,
        m,
        n,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var size:i64
        let s_0_0: i64 = fn_state.size;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var sizeshadow#7908 <= s_0_2
        fn_state.sizeshadow_7908 = s_0_2;
        // D s_0_4: read-var n:i64
        let s_0_4: i64 = fn_state.n;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call R_read(s_0_5)
        let s_0_6: u32 = R_read(state, tracer, s_0_5);
        // D s_0_7: write-var acc <= s_0_6
        fn_state.acc = s_0_6;
        // D s_0_8: read-var sizeshadow#7908:i64
        let s_0_8: i64 = fn_state.sizeshadow_7908;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: call __id(s_0_9)
        let s_0_10: i128 = u__id(state, tracer, s_0_9);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: sub s_0_13 s_0_12
        let s_0_14: i128 = ((s_0_13) - (s_0_12));
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // C s_0_16: const #32s : i
        let s_0_16: i128 = 32;
        // D s_0_17: cast zx s_0_15 -> i
        let s_0_17: i128 = (i128::try_from(s_0_15).unwrap());
        // D s_0_18: cmp-lt s_0_17 s_0_16
        let s_0_18: bool = ((s_0_17) < (s_0_16));
        // N s_0_19: assert s_0_18
        let s_0_19: () = assert!(s_0_18);
        // D s_0_20: read-var crc32c:u8
        let s_0_20: bool = fn_state.crc32c;
        // N s_0_21: branch s_0_20 b3 b1
        if s_0_20 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #79764919u : u32
        let s_1_0: u32 = 79764919;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 32u16);
        // C s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: write-var ga#364589 <= s_1_2
        fn_state.ga_364589 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // C s_2_1: const #0s : i
        let s_2_1: i128 = 0;
        // D s_2_2: read-var ga#364589:i
        let s_2_2: i128 = fn_state.ga_364589;
        // D s_2_3: call integer_subrange(s_2_2, s_2_0, s_2_1)
        let s_2_3: Bits = integer_subrange(state, tracer, s_2_2, s_2_0, s_2_1);
        // D s_2_4: cast reint s_2_3 -> u32
        let s_2_4: u32 = (s_2_3.value() as u32);
        // D s_2_5: read-var acc:u32
        let s_2_5: u32 = fn_state.acc;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 32u16);
        // D s_2_7: call BitReverse(s_2_6)
        let s_2_7: Bits = BitReverse(state, tracer, s_2_6);
        // D s_2_8: cast reint s_2_7 -> u32
        let s_2_8: u32 = (s_2_7.value() as u32);
        // D s_2_9: read-var sizeshadow#7908:i64
        let s_2_9: i64 = fn_state.sizeshadow_7908;
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: call Zeros(s_2_10)
        let s_2_11: Bits = Zeros(state, tracer, s_2_10);
        // D s_2_12: cast zx s_2_8 -> bv
        let s_2_12: Bits = Bits::new(s_2_8 as u128, 32u16);
        // D s_2_13: cast reint s_2_12 -> u128
        let s_2_13: u128 = (s_2_12.value() as u128);
        // D s_2_14: size-of s_2_12
        let s_2_14: u16 = s_2_12.length();
        // D s_2_15: cast reint s_2_11 -> u128
        let s_2_15: u128 = (s_2_11.value() as u128);
        // D s_2_16: size-of s_2_11
        let s_2_16: u16 = s_2_11.length();
        // D s_2_17: lsl s_2_13 s_2_16
        let s_2_17: u128 = s_2_13 << s_2_16;
        // D s_2_18: or s_2_17 s_2_15
        let s_2_18: u128 = ((s_2_17) | (s_2_15));
        // D s_2_19: add s_2_14 s_2_16
        let s_2_19: u16 = (s_2_14 + s_2_16);
        // D s_2_20: create-bits s_2_18 s_2_19
        let s_2_20: Bits = Bits::new(s_2_18, s_2_19);
        // D s_2_21: read-var m:i64
        let s_2_21: i64 = fn_state.m;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: call R_read(s_2_22)
        let s_2_23: u32 = R_read(state, tracer, s_2_22);
        // C s_2_24: const #1s : i
        let s_2_24: i128 = 1;
        // D s_2_25: read-var sizeshadow#7908:i64
        let s_2_25: i64 = fn_state.sizeshadow_7908;
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_27: sub s_2_26 s_2_24
        let s_2_27: i128 = ((s_2_26) - (s_2_24));
        // D s_2_28: cast reint s_2_27 -> i64
        let s_2_28: i64 = (s_2_27 as i64);
        // C s_2_29: const #0s : i
        let s_2_29: i128 = 0;
        // D s_2_30: cast zx s_2_23 -> bv
        let s_2_30: Bits = Bits::new(s_2_23 as u128, 32u16);
        // D s_2_31: cast zx s_2_28 -> i
        let s_2_31: i128 = (i128::try_from(s_2_28).unwrap());
        // C s_2_32: const #1s : i64
        let s_2_32: i64 = 1;
        // C s_2_33: cast zx s_2_32 -> i
        let s_2_33: i128 = (i128::try_from(s_2_32).unwrap());
        // D s_2_34: sub s_2_31 s_2_29
        let s_2_34: i128 = ((s_2_31) - (s_2_29));
        // D s_2_35: add s_2_34 s_2_33
        let s_2_35: i128 = (s_2_34 + s_2_33);
        // D s_2_36: bit-extract s_2_30 s_2_29 s_2_35
        let s_2_36: Bits = (Bits::new(
            ((s_2_30) >> (s_2_29)).value(),
            u16::try_from(s_2_35).unwrap(),
        ));
        // D s_2_37: call BitReverse(s_2_36)
        let s_2_37: Bits = BitReverse(state, tracer, s_2_36);
        // C s_2_38: const #32s : i
        let s_2_38: i128 = 32;
        // S s_2_39: call Zeros(s_2_38)
        let s_2_39: Bits = Zeros(state, tracer, s_2_38);
        // S s_2_40: cast reint s_2_39 -> u32
        let s_2_40: u32 = (s_2_39.value() as u32);
        // S s_2_41: cast zx s_2_40 -> bv
        let s_2_41: Bits = Bits::new(s_2_40 as u128, 32u16);
        // D s_2_42: cast reint s_2_37 -> u128
        let s_2_42: u128 = (s_2_37.value() as u128);
        // D s_2_43: size-of s_2_37
        let s_2_43: u16 = s_2_37.length();
        // S s_2_44: cast reint s_2_41 -> u128
        let s_2_44: u128 = (s_2_41.value() as u128);
        // D s_2_45: size-of s_2_41
        let s_2_45: u16 = s_2_41.length();
        // D s_2_46: lsl s_2_42 s_2_45
        let s_2_46: u128 = s_2_42 << s_2_45;
        // D s_2_47: or s_2_46 s_2_44
        let s_2_47: u128 = ((s_2_46) | (s_2_44));
        // D s_2_48: add s_2_43 s_2_45
        let s_2_48: u16 = (s_2_43 + s_2_45);
        // D s_2_49: create-bits s_2_47 s_2_48
        let s_2_49: Bits = Bits::new(s_2_47, s_2_48);
        // D s_2_50: xor s_2_20 s_2_49
        let s_2_50: Bits = ((s_2_20) ^ (s_2_49));
        // D s_2_51: call Poly32Mod2(s_2_50, s_2_4)
        let s_2_51: u32 = Poly32Mod2(state, tracer, s_2_50, s_2_4);
        // D s_2_52: cast zx s_2_51 -> bv
        let s_2_52: Bits = Bits::new(s_2_51 as u128, 32u16);
        // D s_2_53: call BitReverse(s_2_52)
        let s_2_53: Bits = BitReverse(state, tracer, s_2_52);
        // D s_2_54: cast reint s_2_53 -> u32
        let s_2_54: u32 = (s_2_53.value() as u32);
        // D s_2_55: read-var d:i64
        let s_2_55: i64 = fn_state.d;
        // D s_2_56: cast zx s_2_55 -> i
        let s_2_56: i128 = (i128::try_from(s_2_55).unwrap());
        // D s_2_57: call R_set(s_2_56, s_2_54)
        let s_2_57: () = R_set(state, tracer, s_2_56, s_2_54);
        // N s_2_58: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #517762881u : u32
        let s_3_0: u32 = 517762881;
        // C s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 32u16);
        // C s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: write-var ga#364589 <= s_3_2
        fn_state.ga_364589 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
