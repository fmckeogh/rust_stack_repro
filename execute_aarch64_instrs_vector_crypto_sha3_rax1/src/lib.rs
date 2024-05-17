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
use ROL::*;
use V_set::*;
use V_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sha3_rax1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
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
        // C s_1_10: const #128s : i64
        let s_1_10: i64 = 128;
        // C s_1_11: const #64s : i
        let s_1_11: i128 = 64;
        // D s_1_12: cast zx s_1_4 -> bv
        let s_1_12: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_13: const #1s : i64
        let s_1_13: i64 = 1;
        // C s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // C s_1_15: const #63s : i
        let s_1_15: i128 = 63;
        // C s_1_16: add s_1_15 s_1_14
        let s_1_16: i128 = (s_1_15 + s_1_14);
        // D s_1_17: bit-extract s_1_12 s_1_11 s_1_16
        let s_1_17: Bits = (Bits::new(
            ((s_1_12) >> (s_1_11)).value(),
            u16::try_from(s_1_16).unwrap(),
        ));
        // D s_1_18: cast reint s_1_17 -> u64
        let s_1_18: u64 = (s_1_17.value() as u64);
        // C s_1_19: const #1s : i
        let s_1_19: i128 = 1;
        // D s_1_20: cast zx s_1_18 -> bv
        let s_1_20: Bits = Bits::new(s_1_18 as u128, 64u16);
        // D s_1_21: call ROL(s_1_20, s_1_19)
        let s_1_21: Bits = ROL(state, tracer, s_1_20, s_1_19);
        // D s_1_22: cast reint s_1_21 -> u64
        let s_1_22: u64 = (s_1_21.value() as u64);
        // C s_1_23: const #0s : i
        let s_1_23: i128 = 0;
        // D s_1_24: cast zx s_1_4 -> bv
        let s_1_24: Bits = Bits::new(s_1_4 as u128, 128u16);
        // C s_1_25: const #1s : i64
        let s_1_25: i64 = 1;
        // C s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // C s_1_27: const #63s : i
        let s_1_27: i128 = 63;
        // C s_1_28: add s_1_27 s_1_26
        let s_1_28: i128 = (s_1_27 + s_1_26);
        // D s_1_29: bit-extract s_1_24 s_1_23 s_1_28
        let s_1_29: Bits = (Bits::new(
            ((s_1_24) >> (s_1_23)).value(),
            u16::try_from(s_1_28).unwrap(),
        ));
        // D s_1_30: cast reint s_1_29 -> u64
        let s_1_30: u64 = (s_1_29.value() as u64);
        // C s_1_31: const #1s : i
        let s_1_31: i128 = 1;
        // D s_1_32: cast zx s_1_30 -> bv
        let s_1_32: Bits = Bits::new(s_1_30 as u128, 64u16);
        // D s_1_33: call ROL(s_1_32, s_1_31)
        let s_1_33: Bits = ROL(state, tracer, s_1_32, s_1_31);
        // D s_1_34: cast reint s_1_33 -> u64
        let s_1_34: u64 = (s_1_33.value() as u64);
        // D s_1_35: cast zx s_1_22 -> bv
        let s_1_35: Bits = Bits::new(s_1_22 as u128, 64u16);
        // D s_1_36: cast zx s_1_34 -> bv
        let s_1_36: Bits = Bits::new(s_1_34 as u128, 64u16);
        // D s_1_37: cast reint s_1_35 -> u128
        let s_1_37: u128 = (s_1_35.value() as u128);
        // D s_1_38: size-of s_1_35
        let s_1_38: u16 = s_1_35.length();
        // D s_1_39: cast reint s_1_36 -> u128
        let s_1_39: u128 = (s_1_36.value() as u128);
        // D s_1_40: size-of s_1_36
        let s_1_40: u16 = s_1_36.length();
        // D s_1_41: lsl s_1_37 s_1_40
        let s_1_41: u128 = s_1_37 << s_1_40;
        // D s_1_42: or s_1_41 s_1_39
        let s_1_42: u128 = ((s_1_41) | (s_1_39));
        // D s_1_43: add s_1_38 s_1_40
        let s_1_43: u16 = (s_1_38 + s_1_40);
        // D s_1_44: create-bits s_1_42 s_1_43
        let s_1_44: Bits = Bits::new(s_1_42, s_1_43);
        // D s_1_45: cast reint s_1_44 -> u128
        let s_1_45: u128 = (s_1_44.value() as u128);
        // D s_1_46: cast zx s_1_9 -> bv
        let s_1_46: Bits = Bits::new(s_1_9 as u128, 128u16);
        // D s_1_47: cast zx s_1_45 -> bv
        let s_1_47: Bits = Bits::new(s_1_45 as u128, 128u16);
        // D s_1_48: xor s_1_46 s_1_47
        let s_1_48: Bits = ((s_1_46) ^ (s_1_47));
        // D s_1_49: cast reint s_1_48 -> u128
        let s_1_49: u128 = (s_1_48.value() as u128);
        // D s_1_50: read-var d:i64
        let s_1_50: i64 = fn_state.d;
        // D s_1_51: cast zx s_1_50 -> i
        let s_1_51: i128 = (i128::try_from(s_1_50).unwrap());
        // D s_1_52: cast zx s_1_49 -> bv
        let s_1_52: Bits = Bits::new(s_1_49 as u128, 128u16);
        // D s_1_53: call V_set(s_1_51, s_1_10, s_1_52)
        let s_1_53: () = V_set(state, tracer, s_1_51, s_1_10, s_1_52);
        // N s_1_54: return
        return;
    }
}
