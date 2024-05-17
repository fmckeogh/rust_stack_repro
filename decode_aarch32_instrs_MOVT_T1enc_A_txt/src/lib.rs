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
use execute_aarch32_instrs_MOVT_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MOVT_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i: bool,
    imm4: u8,
    imm3: u8,
    Rd: u8,
    imm8: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        imm16: u16,
        d: i64,
        i: bool,
        imm4: u8,
        imm3: u8,
        Rd: u8,
        imm8: u8,
    }
    let fn_state = FunctionState {
        i,
        imm4,
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
        // D s_2_0: read-var Rd:u8
        let s_2_0: u8 = fn_state.Rd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var d <= s_2_3
        fn_state.d = s_2_3;
        // D s_2_5: read-var imm4:u8
        let s_2_5: u8 = fn_state.imm4;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 4u16);
        // D s_2_7: read-var i:u8
        let s_2_7: bool = fn_state.i;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cast reint s_2_6 -> u128
        let s_2_9: u128 = (s_2_6.value() as u128);
        // D s_2_10: size-of s_2_6
        let s_2_10: u16 = s_2_6.length();
        // D s_2_11: cast reint s_2_8 -> u128
        let s_2_11: u128 = (s_2_8.value() as u128);
        // D s_2_12: size-of s_2_8
        let s_2_12: u16 = s_2_8.length();
        // D s_2_13: lsl s_2_9 s_2_12
        let s_2_13: u128 = s_2_9 << s_2_12;
        // D s_2_14: or s_2_13 s_2_11
        let s_2_14: u128 = ((s_2_13) | (s_2_11));
        // D s_2_15: add s_2_10 s_2_12
        let s_2_15: u16 = (s_2_10 + s_2_12);
        // D s_2_16: create-bits s_2_14 s_2_15
        let s_2_16: Bits = Bits::new(s_2_14, s_2_15);
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: u8 = (s_2_16.value() as u8);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 5u16);
        // D s_2_19: read-var imm3:u8
        let s_2_19: u8 = fn_state.imm3;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 3u16);
        // D s_2_21: cast reint s_2_18 -> u128
        let s_2_21: u128 = (s_2_18.value() as u128);
        // D s_2_22: size-of s_2_18
        let s_2_22: u16 = s_2_18.length();
        // D s_2_23: cast reint s_2_20 -> u128
        let s_2_23: u128 = (s_2_20.value() as u128);
        // D s_2_24: size-of s_2_20
        let s_2_24: u16 = s_2_20.length();
        // D s_2_25: lsl s_2_21 s_2_24
        let s_2_25: u128 = s_2_21 << s_2_24;
        // D s_2_26: or s_2_25 s_2_23
        let s_2_26: u128 = ((s_2_25) | (s_2_23));
        // D s_2_27: add s_2_22 s_2_24
        let s_2_27: u16 = (s_2_22 + s_2_24);
        // D s_2_28: create-bits s_2_26 s_2_27
        let s_2_28: Bits = Bits::new(s_2_26, s_2_27);
        // D s_2_29: cast reint s_2_28 -> u8
        let s_2_29: u8 = (s_2_28.value() as u8);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 8u16);
        // D s_2_31: read-var imm8:u8
        let s_2_31: u8 = fn_state.imm8;
        // D s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 8u16);
        // D s_2_33: cast reint s_2_30 -> u128
        let s_2_33: u128 = (s_2_30.value() as u128);
        // D s_2_34: size-of s_2_30
        let s_2_34: u16 = s_2_30.length();
        // D s_2_35: cast reint s_2_32 -> u128
        let s_2_35: u128 = (s_2_32.value() as u128);
        // D s_2_36: size-of s_2_32
        let s_2_36: u16 = s_2_32.length();
        // D s_2_37: lsl s_2_33 s_2_36
        let s_2_37: u128 = s_2_33 << s_2_36;
        // D s_2_38: or s_2_37 s_2_35
        let s_2_38: u128 = ((s_2_37) | (s_2_35));
        // D s_2_39: add s_2_34 s_2_36
        let s_2_39: u16 = (s_2_34 + s_2_36);
        // D s_2_40: create-bits s_2_38 s_2_39
        let s_2_40: Bits = Bits::new(s_2_38, s_2_39);
        // D s_2_41: cast reint s_2_40 -> u16
        let s_2_41: u16 = (s_2_40.value() as u16);
        // D s_2_42: write-var imm16 <= s_2_41
        fn_state.imm16 = s_2_41;
        // C s_2_43: const #15s : i
        let s_2_43: i128 = 15;
        // D s_2_44: read-var d:i64
        let s_2_44: i64 = fn_state.d;
        // D s_2_45: cast zx s_2_44 -> i
        let s_2_45: i128 = (i128::try_from(s_2_44).unwrap());
        // D s_2_46: cmp-eq s_2_45 s_2_43
        let s_2_46: bool = ((s_2_45) == (s_2_43));
        // N s_2_47: branch s_2_46 b4 b3
        if s_2_46 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: read-var imm16:u16
        let s_3_1: u16 = fn_state.imm16;
        // D s_3_2: call execute_aarch32_instrs_MOVT_Op_A_txt(s_3_0, s_3_1)
        let s_3_2: () = execute_aarch32_instrs_MOVT_Op_A_txt(
            state,
            tracer,
            s_3_0,
            s_3_1,
        );
        // N s_3_3: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
