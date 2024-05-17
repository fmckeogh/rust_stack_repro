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
use execute_aarch64_instrs_vector_shift_right_sisd::*;
use common::*;
pub fn decode_ssra_advsimd_aarch64_instrs_vector_shift_right_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o0: bool,
    o1: bool,
    immb: u8,
    immh: u8,
    U: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        o0: bool,
        o1: bool,
        immb: u8,
        immh: u8,
        U: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o0,
        o1,
        immb,
        immh,
        U,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // C s_0_10: const #3s : i
        let s_0_10: i128 = 3;
        // D s_0_11: read-var immh:u8
        let s_0_11: u8 = fn_state.immh;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 4u16);
        // C s_0_13: const #1u : u64
        let s_0_13: u64 = 1;
        // D s_0_14: bit-extract s_0_12 s_0_10 s_0_13
        let s_0_14: Bits = (Bits::new(
            ((s_0_12) >> (s_0_10)).value(),
            u16::try_from(s_0_13).unwrap(),
        ));
        // D s_0_15: cast reint s_0_14 -> u8
        let s_0_15: bool = ((s_0_14.value()) != 0);
        // C s_0_16: const #0s : i
        let s_0_16: i128 = 0;
        // C s_0_17: const #0u : u64
        let s_0_17: u64 = 0;
        // D s_0_18: cast zx s_0_15 -> u64
        let s_0_18: u64 = (s_0_15 as u64);
        // C s_0_19: const #1u : u64
        let s_0_19: u64 = 1;
        // D s_0_20: and s_0_18 s_0_19
        let s_0_20: u64 = ((s_0_18) & (s_0_19));
        // D s_0_21: cmp-eq s_0_20 s_0_19
        let s_0_21: bool = ((s_0_20) == (s_0_19));
        // D s_0_22: lsl s_0_18 s_0_16
        let s_0_22: u64 = s_0_18 << s_0_16;
        // D s_0_23: or s_0_17 s_0_22
        let s_0_23: u64 = ((s_0_17) | (s_0_22));
        // D s_0_24: cmpl s_0_22
        let s_0_24: u64 = !s_0_22;
        // D s_0_25: and s_0_17 s_0_24
        let s_0_25: u64 = ((s_0_17) & (s_0_24));
        // D s_0_26: select s_0_21 s_0_23 s_0_25
        let s_0_26: u64 = if s_0_21 { s_0_23 } else { s_0_25 };
        // D s_0_27: cast trunc s_0_26 -> u8
        let s_0_27: bool = ((s_0_26) != 0);
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 1u16);
        // C s_0_29: const #1u : u8
        let s_0_29: bool = true;
        // C s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 1u16);
        // D s_0_31: cmp-ne s_0_28 s_0_30
        let s_0_31: bool = ((s_0_28) != (s_0_30));
        // N s_0_32: branch s_0_31 b2 b1
        if s_0_31 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var immh:u8
        let s_1_0: u8 = fn_state.immh;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 4u16);
        // D s_1_2: read-var immb:u8
        let s_1_2: u8 = fn_state.immb;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 3u16);
        // D s_1_4: cast reint s_1_1 -> u128
        let s_1_4: u128 = (s_1_1.value() as u128);
        // D s_1_5: size-of s_1_1
        let s_1_5: u16 = s_1_1.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u8
        let s_1_12: u8 = (s_1_11.value() as u8);
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 7u16);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (s_1_13.value() as i128);
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // C s_1_16: const #128s : i
        let s_1_16: i128 = 128;
        // D s_1_17: cast zx s_1_15 -> i
        let s_1_17: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_18: sub s_1_16 s_1_17
        let s_1_18: i128 = ((s_1_16) - (s_1_17));
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var U:u8
        let s_1_20: bool = fn_state.U;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 1u16);
        // C s_1_22: const #1u : u8
        let s_1_22: bool = true;
        // C s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 1u16);
        // D s_1_24: cmp-eq s_1_21 s_1_23
        let s_1_24: bool = ((s_1_21) == (s_1_23));
        // D s_1_25: read-var o1:u8
        let s_1_25: bool = fn_state.o1;
        // D s_1_26: cast zx s_1_25 -> bv
        let s_1_26: Bits = Bits::new(s_1_25 as u128, 1u16);
        // C s_1_27: const #1u : u8
        let s_1_27: bool = true;
        // C s_1_28: cast zx s_1_27 -> bv
        let s_1_28: Bits = Bits::new(s_1_27 as u128, 1u16);
        // D s_1_29: cmp-eq s_1_26 s_1_28
        let s_1_29: bool = ((s_1_26) == (s_1_28));
        // D s_1_30: read-var o0:u8
        let s_1_30: bool = fn_state.o0;
        // D s_1_31: cast zx s_1_30 -> bv
        let s_1_31: Bits = Bits::new(s_1_30 as u128, 1u16);
        // C s_1_32: const #1u : u8
        let s_1_32: bool = true;
        // C s_1_33: cast zx s_1_32 -> bv
        let s_1_33: Bits = Bits::new(s_1_32 as u128, 1u16);
        // D s_1_34: cmp-eq s_1_31 s_1_33
        let s_1_34: bool = ((s_1_31) == (s_1_33));
        // C s_1_35: const #64s : i64
        let s_1_35: i64 = 64;
        // C s_1_36: const #64s : i64
        let s_1_36: i64 = 64;
        // C s_1_37: const #1s : i
        let s_1_37: i128 = 1;
        // C s_1_38: cast zx s_1_35 -> i
        let s_1_38: i128 = (i128::try_from(s_1_35).unwrap());
        // C s_1_39: cast zx s_1_36 -> i
        let s_1_39: i128 = (i128::try_from(s_1_36).unwrap());
        // D s_1_40: cast zx s_1_19 -> i
        let s_1_40: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_41: read-var d:i64
        let s_1_41: i64 = fn_state.d;
        // D s_1_42: read-var n:i64
        let s_1_42: i64 = fn_state.n;
        // D s_1_43: call execute_aarch64_instrs_vector_shift_right_sisd(s_1_34, s_1_41, s_1_38, s_1_37, s_1_39, s_1_42, s_1_29, s_1_40, s_1_24)
        let s_1_43: () = execute_aarch64_instrs_vector_shift_right_sisd(
            state,
            tracer,
            s_1_34,
            s_1_41,
            s_1_38,
            s_1_37,
            s_1_39,
            s_1_42,
            s_1_29,
            s_1_40,
            s_1_24,
        );
        // N s_1_44: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
