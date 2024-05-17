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
use Q_read::*;
use CheckCryptoEnabled32::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_SHA1SU0_Op_A_txt<T: Tracer>(
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
        // S s_0_1: call CheckCryptoEnabled32(s_0_0)
        let s_0_1: () = CheckCryptoEnabled32(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i64
        let s_1_0: i64 = 1;
        // D s_1_1: read-var d:i64
        let s_1_1: i64 = fn_state.d;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // D s_1_6: read-var n:i64
        let s_1_6: i64 = fn_state.n;
        // D s_1_7: lsr s_1_6 s_1_5
        let s_1_7: i64 = s_1_6 >> s_1_5;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call Q_read(s_1_8)
        let s_1_9: u128 = Q_read(state, tracer, s_1_8);
        // C s_1_10: const #1s : i64
        let s_1_10: i64 = 1;
        // D s_1_11: read-var m:i64
        let s_1_11: i64 = fn_state.m;
        // D s_1_12: lsr s_1_11 s_1_10
        let s_1_12: i64 = s_1_11 >> s_1_10;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: call Q_read(s_1_13)
        let s_1_14: u128 = Q_read(state, tracer, s_1_13);
        // C s_1_15: const #0s : i
        let s_1_15: i128 = 0;
        // D s_1_16: cast zx s_1_9 -> bv
        let s_1_16: Bits = Bits::new(s_1_9 as u128, 128u16);
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
        // C s_1_23: const #64s : i
        let s_1_23: i128 = 64;
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
        // D s_1_31: cast zx s_1_22 -> bv
        let s_1_31: Bits = Bits::new(s_1_22 as u128, 64u16);
        // D s_1_32: cast zx s_1_30 -> bv
        let s_1_32: Bits = Bits::new(s_1_30 as u128, 64u16);
        // D s_1_33: cast reint s_1_31 -> u128
        let s_1_33: u128 = (s_1_31.value() as u128);
        // D s_1_34: size-of s_1_31
        let s_1_34: u16 = s_1_31.length();
        // D s_1_35: cast reint s_1_32 -> u128
        let s_1_35: u128 = (s_1_32.value() as u128);
        // D s_1_36: size-of s_1_32
        let s_1_36: u16 = s_1_32.length();
        // D s_1_37: lsl s_1_33 s_1_36
        let s_1_37: u128 = s_1_33 << s_1_36;
        // D s_1_38: or s_1_37 s_1_35
        let s_1_38: u128 = ((s_1_37) | (s_1_35));
        // D s_1_39: add s_1_34 s_1_36
        let s_1_39: u16 = (s_1_34 + s_1_36);
        // D s_1_40: create-bits s_1_38 s_1_39
        let s_1_40: Bits = Bits::new(s_1_38, s_1_39);
        // D s_1_41: cast reint s_1_40 -> u128
        let s_1_41: u128 = (s_1_40.value() as u128);
        // C s_1_42: const #1s : i64
        let s_1_42: i64 = 1;
        // D s_1_43: read-var d:i64
        let s_1_43: i64 = fn_state.d;
        // D s_1_44: lsr s_1_43 s_1_42
        let s_1_44: i64 = s_1_43 >> s_1_42;
        // D s_1_45: cast zx s_1_4 -> bv
        let s_1_45: Bits = Bits::new(s_1_4 as u128, 128u16);
        // D s_1_46: cast zx s_1_41 -> bv
        let s_1_46: Bits = Bits::new(s_1_41 as u128, 128u16);
        // D s_1_47: xor s_1_45 s_1_46
        let s_1_47: Bits = ((s_1_45) ^ (s_1_46));
        // D s_1_48: cast reint s_1_47 -> u128
        let s_1_48: u128 = (s_1_47.value() as u128);
        // D s_1_49: cast zx s_1_48 -> bv
        let s_1_49: Bits = Bits::new(s_1_48 as u128, 128u16);
        // D s_1_50: cast zx s_1_14 -> bv
        let s_1_50: Bits = Bits::new(s_1_14 as u128, 128u16);
        // D s_1_51: xor s_1_49 s_1_50
        let s_1_51: Bits = ((s_1_49) ^ (s_1_50));
        // D s_1_52: cast reint s_1_51 -> u128
        let s_1_52: u128 = (s_1_51.value() as u128);
        // D s_1_53: cast zx s_1_44 -> i
        let s_1_53: i128 = (i128::try_from(s_1_44).unwrap());
        // D s_1_54: call Q_set(s_1_53, s_1_52)
        let s_1_54: () = Q_set(state, tracer, s_1_53, s_1_52);
        // N s_1_55: return
        return;
    }
}
