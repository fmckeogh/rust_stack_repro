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
use BitCount::*;
use execute_aarch32_instrs_PUSH_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_PUSH_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    M: bool,
    register_list: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        registers: u16,
        M: bool,
        register_list: u8,
    }
    let fn_state = FunctionState {
        M,
        register_list,
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
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var M:u8
        let s_2_2: bool = fn_state.M;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 2u16);
        // C s_2_14: const #0u : u8
        let s_2_14: u8 = 0;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 6u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // C s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u8
        let s_2_24: u8 = (s_2_23.value() as u8);
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 8u16);
        // D s_2_26: read-var register_list:u8
        let s_2_26: u8 = fn_state.register_list;
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 8u16);
        // D s_2_28: cast reint s_2_25 -> u128
        let s_2_28: u128 = (s_2_25.value() as u128);
        // D s_2_29: size-of s_2_25
        let s_2_29: u16 = s_2_25.length();
        // D s_2_30: cast reint s_2_27 -> u128
        let s_2_30: u128 = (s_2_27.value() as u128);
        // D s_2_31: size-of s_2_27
        let s_2_31: u16 = s_2_27.length();
        // D s_2_32: lsl s_2_28 s_2_31
        let s_2_32: u128 = s_2_28 << s_2_31;
        // D s_2_33: or s_2_32 s_2_30
        let s_2_33: u128 = ((s_2_32) | (s_2_30));
        // D s_2_34: add s_2_29 s_2_31
        let s_2_34: u16 = (s_2_29 + s_2_31);
        // D s_2_35: create-bits s_2_33 s_2_34
        let s_2_35: Bits = Bits::new(s_2_33, s_2_34);
        // D s_2_36: cast reint s_2_35 -> u16
        let s_2_36: u16 = (s_2_35.value() as u16);
        // D s_2_37: write-var registers <= s_2_36
        fn_state.registers = s_2_36;
        // D s_2_38: read-var registers:u16
        let s_2_38: u16 = fn_state.registers;
        // D s_2_39: cast zx s_2_38 -> bv
        let s_2_39: Bits = Bits::new(s_2_38 as u128, 16u16);
        // D s_2_40: call BitCount(s_2_39)
        let s_2_40: i128 = BitCount(state, tracer, s_2_39);
        // C s_2_41: const #1s : i
        let s_2_41: i128 = 1;
        // D s_2_42: cmp-lt s_2_40 s_2_41
        let s_2_42: bool = ((s_2_40) < (s_2_41));
        // N s_2_43: branch s_2_42 b4 b3
        if s_2_42 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: read-var registers:u16
        let s_3_1: u16 = fn_state.registers;
        // D s_3_2: call execute_aarch32_instrs_PUSH_Op_A_txt(s_3_0, s_3_1)
        let s_3_2: () = execute_aarch32_instrs_PUSH_Op_A_txt(
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
