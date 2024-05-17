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
use HaveBF16Ext::*;
use execute_aarch64_instrs_vector_arithmetic_binary_element_bfdot::*;
use common::*;
pub fn decode_bfdot_advsimd_elt_aarch64_instrs_vector_arithmetic_binary_element_bfdot<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    H: bool,
    Rm: u8,
    M: bool,
    L: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_249828: i64,
        i: i64,
        n: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        H: bool,
        Rm: u8,
        M: bool,
        L: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        H,
        Rm,
        M,
        L,
        Q,
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
        // S s_0_1: call HaveBF16Ext(s_0_0)
        let s_0_1: bool = HaveBF16Ext(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rn:u8
        let s_1_0: u8 = fn_state.Rn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var n <= s_1_3
        fn_state.n = s_1_3;
        // D s_1_5: read-var M:u8
        let s_1_5: bool = fn_state.M;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 1u16);
        // D s_1_7: read-var Rm:u8
        let s_1_7: u8 = fn_state.Rm;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 4u16);
        // D s_1_9: cast reint s_1_6 -> u128
        let s_1_9: u128 = (s_1_6.value() as u128);
        // D s_1_10: size-of s_1_6
        let s_1_10: u16 = s_1_6.length();
        // D s_1_11: cast reint s_1_8 -> u128
        let s_1_11: u128 = (s_1_8.value() as u128);
        // D s_1_12: size-of s_1_8
        let s_1_12: u16 = s_1_8.length();
        // D s_1_13: lsl s_1_9 s_1_12
        let s_1_13: u128 = s_1_9 << s_1_12;
        // D s_1_14: or s_1_13 s_1_11
        let s_1_14: u128 = ((s_1_13) | (s_1_11));
        // D s_1_15: add s_1_10 s_1_12
        let s_1_15: u16 = (s_1_10 + s_1_12);
        // D s_1_16: create-bits s_1_14 s_1_15
        let s_1_16: Bits = Bits::new(s_1_14, s_1_15);
        // D s_1_17: cast reint s_1_16 -> u8
        let s_1_17: u8 = (s_1_16.value() as u8);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 5u16);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (s_1_18.value() as i128);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var m <= s_1_20
        fn_state.m = s_1_20;
        // D s_1_22: read-var Rd:u8
        let s_1_22: u8 = fn_state.Rd;
        // D s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 5u16);
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (s_1_23.value() as i128);
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var d <= s_1_25
        fn_state.d = s_1_25;
        // D s_1_27: read-var H:u8
        let s_1_27: bool = fn_state.H;
        // D s_1_28: cast zx s_1_27 -> bv
        let s_1_28: Bits = Bits::new(s_1_27 as u128, 1u16);
        // D s_1_29: read-var L:u8
        let s_1_29: bool = fn_state.L;
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 1u16);
        // D s_1_31: cast reint s_1_28 -> u128
        let s_1_31: u128 = (s_1_28.value() as u128);
        // D s_1_32: size-of s_1_28
        let s_1_32: u16 = s_1_28.length();
        // D s_1_33: cast reint s_1_30 -> u128
        let s_1_33: u128 = (s_1_30.value() as u128);
        // D s_1_34: size-of s_1_30
        let s_1_34: u16 = s_1_30.length();
        // D s_1_35: lsl s_1_31 s_1_34
        let s_1_35: u128 = s_1_31 << s_1_34;
        // D s_1_36: or s_1_35 s_1_33
        let s_1_36: u128 = ((s_1_35) | (s_1_33));
        // D s_1_37: add s_1_32 s_1_34
        let s_1_37: u16 = (s_1_32 + s_1_34);
        // D s_1_38: create-bits s_1_36 s_1_37
        let s_1_38: Bits = Bits::new(s_1_36, s_1_37);
        // D s_1_39: cast reint s_1_38 -> u8
        let s_1_39: u8 = (s_1_38.value() as u8);
        // D s_1_40: cast zx s_1_39 -> bv
        let s_1_40: Bits = Bits::new(s_1_39 as u128, 2u16);
        // D s_1_41: cast zx s_1_40 -> i
        let s_1_41: i128 = (s_1_40.value() as i128);
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var i <= s_1_42
        fn_state.i = s_1_42;
        // D s_1_44: read-var Q:u8
        let s_1_44: bool = fn_state.Q;
        // D s_1_45: cast zx s_1_44 -> bv
        let s_1_45: Bits = Bits::new(s_1_44 as u128, 1u16);
        // C s_1_46: const #1u : u8
        let s_1_46: bool = true;
        // C s_1_47: cast zx s_1_46 -> bv
        let s_1_47: Bits = Bits::new(s_1_46 as u128, 1u16);
        // D s_1_48: cmp-eq s_1_45 s_1_47
        let s_1_48: bool = ((s_1_45) == (s_1_47));
        // N s_1_49: branch s_1_48 b4 b2
        if s_1_48 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: write-var ga#249828 <= s_2_0
        fn_state.ga_249828 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#249828:i64
        let s_3_0: i64 = fn_state.ga_249828;
        // C s_3_1: const #32s : i
        let s_3_1: i128 = 32;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: div s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) / (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var d:i64
        let s_3_7: i64 = fn_state.d;
        // D s_3_8: read-var i:i64
        let s_3_8: i64 = fn_state.i;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: read-var n:i64
        let s_3_10: i64 = fn_state.n;
        // D s_3_11: call execute_aarch64_instrs_vector_arithmetic_binary_element_bfdot(s_3_7, s_3_6, s_3_4, s_3_8, s_3_9, s_3_10)
        let s_3_11: () = execute_aarch64_instrs_vector_arithmetic_binary_element_bfdot(
            state,
            tracer,
            s_3_7,
            s_3_6,
            s_3_4,
            s_3_8,
            s_3_9,
            s_3_10,
        );
        // N s_3_12: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #128s : i64
        let s_4_0: i64 = 128;
        // D s_4_1: write-var ga#249828 <= s_4_0
        fn_state.ga_249828 = s_4_0;
        // N s_4_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
