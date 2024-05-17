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
use execute_aarch32_instrs_MOV_i_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_MOV_i_A2enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    imm4: u8,
    Rd: u8,
    imm12: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        carry: bool,
        imm32: u32,
        d: i64,
        cond: u8,
        imm4: u8,
        Rd: u8,
        imm12: u16,
    }
    let fn_state = FunctionState {
        cond,
        imm4,
        Rd,
        imm12,
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
        // D s_2_0: read-var cond:u8
        let s_2_0: u8 = fn_state.cond;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // C s_2_2: const #15u : u8
        let s_2_2: u8 = 15;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 4u16);
        // D s_2_4: cmp-ne s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) != (s_2_3));
        // N s_2_5: assert s_2_4
        let s_2_5: () = assert!(s_2_4);
        // D s_2_6: read-var Rd:u8
        let s_2_6: u8 = fn_state.Rd;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 4u16);
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (s_2_7.value() as i128);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: write-var d <= s_2_9
        fn_state.d = s_2_9;
        // D s_2_11: read-var imm4:u8
        let s_2_11: u8 = fn_state.imm4;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 4u16);
        // D s_2_13: read-var imm12:u12
        let s_2_13: u16 = fn_state.imm12;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 12u16);
        // D s_2_15: cast reint s_2_12 -> u128
        let s_2_15: u128 = (s_2_12.value() as u128);
        // D s_2_16: size-of s_2_12
        let s_2_16: u16 = s_2_12.length();
        // D s_2_17: cast reint s_2_14 -> u128
        let s_2_17: u128 = (s_2_14.value() as u128);
        // D s_2_18: size-of s_2_14
        let s_2_18: u16 = s_2_14.length();
        // D s_2_19: lsl s_2_15 s_2_18
        let s_2_19: u128 = s_2_15 << s_2_18;
        // D s_2_20: or s_2_19 s_2_17
        let s_2_20: u128 = ((s_2_19) | (s_2_17));
        // D s_2_21: add s_2_16 s_2_18
        let s_2_21: u16 = (s_2_16 + s_2_18);
        // D s_2_22: create-bits s_2_20 s_2_21
        let s_2_22: Bits = Bits::new(s_2_20, s_2_21);
        // D s_2_23: cast reint s_2_22 -> u16
        let s_2_23: u16 = (s_2_22.value() as u16);
        // C s_2_24: const #32s : i
        let s_2_24: i128 = 32;
        // D s_2_25: cast zx s_2_23 -> bv
        let s_2_25: Bits = Bits::new(s_2_23 as u128, 16u16);
        // D s_2_26: bits-cast zx s_2_25 -> bv length s_2_24
        let s_2_26: Bits = s_2_25.zero_extend(s_2_24);
        // D s_2_27: cast reint s_2_26 -> u32
        let s_2_27: u32 = (s_2_26.value() as u32);
        // D s_2_28: write-var imm32 <= s_2_27
        fn_state.imm32 = s_2_27;
        // C s_2_29: const #15s : i
        let s_2_29: i128 = 15;
        // D s_2_30: read-var d:i64
        let s_2_30: i64 = fn_state.d;
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (i128::try_from(s_2_30).unwrap());
        // D s_2_32: cmp-eq s_2_31 s_2_29
        let s_2_32: bool = ((s_2_31) == (s_2_29));
        // N s_2_33: branch s_2_32 b4 b3
        if s_2_32 {
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
        // D s_3_0: read-var carry:u8
        let s_3_0: bool = fn_state.carry;
        // D s_3_1: read-var d:i64
        let s_3_1: i64 = fn_state.d;
        // D s_3_2: read-var imm32:u32
        let s_3_2: u32 = fn_state.imm32;
        // C s_3_3: const #0u : u8
        let s_3_3: bool = false;
        // D s_3_4: call execute_aarch32_instrs_MOV_i_Op_A_txt(s_3_0, s_3_1, s_3_2, s_3_3)
        let s_3_4: () = execute_aarch32_instrs_MOV_i_Op_A_txt(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
        );
        // N s_3_5: return
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
