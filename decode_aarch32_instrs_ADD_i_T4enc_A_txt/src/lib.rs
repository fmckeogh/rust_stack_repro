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
use ConditionPassed::*;
use execute_aarch32_instrs_ADD_i_OpT_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_ADD_i_T4enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    Rn: u8,
    imm3: u8,
    Rd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm32: u32,
        n: i64,
        d: i64,
        i: bool,
        Rn: u8,
        imm3: u8,
        Rd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        Rn,
        imm3,
        Rd,
        imm8,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
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
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b8 b3
        if s_2_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rn:u8
        let s_3_0: u8 = fn_state.Rn;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // C s_3_2: const #13u : u8
        let s_3_2: u8 = 13;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 4u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b7 b4
        if s_3_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rd:u8
        let s_4_0: u8 = fn_state.Rd;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: write-var d <= s_4_3
        fn_state.d = s_4_3;
        // D s_4_5: read-var Rn:u8
        let s_4_5: u8 = fn_state.Rn;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 4u16);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (s_4_6.value() as i128);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: write-var n <= s_4_8
        fn_state.n = s_4_8;
        // D s_4_10: read-var i:u8
        let s_4_10: bool = fn_state.i;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 1u16);
        // D s_4_12: read-var imm3:u8
        let s_4_12: u8 = fn_state.imm3;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 3u16);
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // D s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: lsl s_4_14 s_4_17
        let s_4_18: u128 = s_4_14 << s_4_17;
        // D s_4_19: or s_4_18 s_4_16
        let s_4_19: u128 = ((s_4_18) | (s_4_16));
        // D s_4_20: add s_4_15 s_4_17
        let s_4_20: u16 = (s_4_15 + s_4_17);
        // D s_4_21: create-bits s_4_19 s_4_20
        let s_4_21: Bits = Bits::new(s_4_19, s_4_20);
        // D s_4_22: cast reint s_4_21 -> u8
        let s_4_22: u8 = (s_4_21.value() as u8);
        // D s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 4u16);
        // D s_4_24: read-var imm8:u8
        let s_4_24: u8 = fn_state.imm8;
        // D s_4_25: cast zx s_4_24 -> bv
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 8u16);
        // D s_4_26: cast reint s_4_23 -> u128
        let s_4_26: u128 = (s_4_23.value() as u128);
        // D s_4_27: size-of s_4_23
        let s_4_27: u16 = s_4_23.length();
        // D s_4_28: cast reint s_4_25 -> u128
        let s_4_28: u128 = (s_4_25.value() as u128);
        // D s_4_29: size-of s_4_25
        let s_4_29: u16 = s_4_25.length();
        // D s_4_30: lsl s_4_26 s_4_29
        let s_4_30: u128 = s_4_26 << s_4_29;
        // D s_4_31: or s_4_30 s_4_28
        let s_4_31: u128 = ((s_4_30) | (s_4_28));
        // D s_4_32: add s_4_27 s_4_29
        let s_4_32: u16 = (s_4_27 + s_4_29);
        // D s_4_33: create-bits s_4_31 s_4_32
        let s_4_33: Bits = Bits::new(s_4_31, s_4_32);
        // D s_4_34: cast reint s_4_33 -> u12
        let s_4_34: u16 = (s_4_33.value() as u16);
        // C s_4_35: const #32s : i
        let s_4_35: i128 = 32;
        // D s_4_36: cast zx s_4_34 -> bv
        let s_4_36: Bits = Bits::new(s_4_34 as u128, 12u16);
        // D s_4_37: bits-cast zx s_4_36 -> bv length s_4_35
        let s_4_37: Bits = s_4_36.zero_extend(s_4_35);
        // D s_4_38: cast reint s_4_37 -> u32
        let s_4_38: u32 = (s_4_37.value() as u32);
        // D s_4_39: write-var imm32 <= s_4_38
        fn_state.imm32 = s_4_38;
        // C s_4_40: const #15s : i
        let s_4_40: i128 = 15;
        // D s_4_41: read-var d:i64
        let s_4_41: i64 = fn_state.d;
        // D s_4_42: cast zx s_4_41 -> i
        let s_4_42: i128 = (i128::try_from(s_4_41).unwrap());
        // D s_4_43: cmp-eq s_4_42 s_4_40
        let s_4_43: bool = ((s_4_42) == (s_4_40));
        // N s_4_44: branch s_4_43 b6 b5
        if s_4_43 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var d:i64
        let s_5_0: i64 = fn_state.d;
        // D s_5_1: read-var imm32:u32
        let s_5_1: u32 = fn_state.imm32;
        // D s_5_2: read-var n:i64
        let s_5_2: i64 = fn_state.n;
        // C s_5_3: const #0u : u8
        let s_5_3: bool = false;
        // D s_5_4: call execute_aarch32_instrs_ADD_i_OpT_A_txt(s_5_0, s_5_1, s_5_2, s_5_3)
        let s_5_4: () = execute_aarch32_instrs_ADD_i_OpT_A_txt(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
            s_5_3,
        );
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
