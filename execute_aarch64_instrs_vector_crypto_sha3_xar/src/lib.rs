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
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use ROR::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha3_xar<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    imm6: u8,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        imm6: u8,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        imm6,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var m:i64
        let s_1_1: i64 = fn_state.m;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // C s_1_5: const #128s : i64
        let s_1_5: i64 = 128;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call V_read(s_1_7, s_1_5)
        let s_1_8: Bits = V_read(state, tracer, s_1_7, s_1_5);
        // D s_1_9: cast reint s_1_8 -> u128
        let s_1_9: u128 = (s_1_8.value() as u128);
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_11: cast zx s_1_4 -> bv
        let s_1_11: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_12: xor s_1_10 s_1_11
        let s_1_12: Bits = ((s_1_10) ^ (s_1_11));
        // D s_1_13: cast reint s_1_12 -> u128
        let s_1_13: u128 = (s_1_12.value() as u128);
        // C s_1_14: const #128s : i64
        let s_1_14: i64 = 128;
        // C s_1_15: const #64s : i
        let s_1_15: i128 = 64;
        // D s_1_16: cast zx s_1_13 -> bv
        let s_1_16: Bits = Bits::new(s_1_13 as u128, 128u16);
        // C s_1_17: const #1s : i64
        let s_1_17: i64 = 1;
        // C s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // C s_1_19: const #63s : i
        let s_1_19: i128 = 63;
        // C s_1_20: add s_1_19 s_1_18
        let s_1_20: i128 = (s_1_19 + s_1_18);
        // D s_1_21: bit-extract s_1_16 s_1_15 s_1_20
        let s_1_21: Bits = (Bits::new(
            ((s_1_16) >> (s_1_15)).value(),
            u16::try_from(s_1_20).unwrap(),
        ));
        // D s_1_22: cast reint s_1_21 -> u64
        let s_1_22: u64 = (s_1_21.value() as u64);
        // D s_1_23: read-var imm6:u8
        let s_1_23: u8 = fn_state.imm6;
        // D s_1_24: cast zx s_1_23 -> bv
        let s_1_24: Bits = Bits::new(s_1_23 as u128, 6u16);
        // D s_1_25: cast zx s_1_24 -> i
        let s_1_25: i128 = (s_1_24.value() as i128);
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: cast zx s_1_22 -> bv
        let s_1_27: Bits = Bits::new(s_1_22 as u128, 64u16);
        // D s_1_28: cast zx s_1_26 -> i
        let s_1_28: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_29: call ROR(s_1_27, s_1_28)
        let s_1_29: Bits = ROR(state, tracer, s_1_27, s_1_28);
        // D s_1_30: cast reint s_1_29 -> u64
        let s_1_30: u64 = (s_1_29.value() as u64);
        // C s_1_31: const #0s : i
        let s_1_31: i128 = 0;
        // D s_1_32: cast zx s_1_13 -> bv
        let s_1_32: Bits = Bits::new(s_1_13 as u128, 128u16);
        // C s_1_33: const #1s : i64
        let s_1_33: i64 = 1;
        // C s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // C s_1_35: const #63s : i
        let s_1_35: i128 = 63;
        // C s_1_36: add s_1_35 s_1_34
        let s_1_36: i128 = (s_1_35 + s_1_34);
        // D s_1_37: bit-extract s_1_32 s_1_31 s_1_36
        let s_1_37: Bits = (Bits::new(
            ((s_1_32) >> (s_1_31)).value(),
            u16::try_from(s_1_36).unwrap(),
        ));
        // D s_1_38: cast reint s_1_37 -> u64
        let s_1_38: u64 = (s_1_37.value() as u64);
        // D s_1_39: read-var imm6:u8
        let s_1_39: u8 = fn_state.imm6;
        // D s_1_40: cast zx s_1_39 -> bv
        let s_1_40: Bits = Bits::new(s_1_39 as u128, 6u16);
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (s_1_40.value() as i128);
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: cast zx s_1_38 -> bv
        let s_1_43: Bits = Bits::new(s_1_38 as u128, 64u16);
        // D s_1_44: cast zx s_1_42 -> i
        let s_1_44: i128 = (i128::try_from(s_1_42).unwrap());
        // D s_1_45: call ROR(s_1_43, s_1_44)
        let s_1_45: Bits = ROR(state, tracer, s_1_43, s_1_44);
        // D s_1_46: cast reint s_1_45 -> u64
        let s_1_46: u64 = (s_1_45.value() as u64);
        // D s_1_47: cast zx s_1_30 -> bv
        let s_1_47: Bits = Bits::new(s_1_30 as u128, 64u16);
        // D s_1_48: cast zx s_1_46 -> bv
        let s_1_48: Bits = Bits::new(s_1_46 as u128, 64u16);
        // D s_1_49: cast reint s_1_47 -> u128
        let s_1_49: u128 = (s_1_47.value() as u128);
        // D s_1_50: size-of s_1_47
        let s_1_50: u16 = s_1_47.length();
        // D s_1_51: cast reint s_1_48 -> u128
        let s_1_51: u128 = (s_1_48.value() as u128);
        // D s_1_52: size-of s_1_48
        let s_1_52: u16 = s_1_48.length();
        // D s_1_53: lsl s_1_49 s_1_52
        let s_1_53: u128 = s_1_49 << s_1_52;
        // D s_1_54: or s_1_53 s_1_51
        let s_1_54: u128 = ((s_1_53) | (s_1_51));
        // D s_1_55: add s_1_50 s_1_52
        let s_1_55: u16 = (s_1_50 + s_1_52);
        // D s_1_56: create-bits s_1_54 s_1_55
        let s_1_56: Bits = Bits::new(s_1_54, s_1_55);
        // D s_1_57: cast reint s_1_56 -> u128
        let s_1_57: u128 = (s_1_56.value() as u128);
        // D s_1_58: read-var d:i64
        let s_1_58: i64 = fn_state.d;
        // D s_1_59: cast zx s_1_58 -> i
        let s_1_59: i128 = (i128::try_from(s_1_58).unwrap());
        // D s_1_60: cast zx s_1_57 -> bv
        let s_1_60: Bits = Bits::new(s_1_57 as u128, 128u16);
        // D s_1_61: call V_set(s_1_59, s_1_14, s_1_60)
        let s_1_61: () = V_set(state, tracer, s_1_59, s_1_14, s_1_60);
        // N s_1_62: return
        return;
    }
}
