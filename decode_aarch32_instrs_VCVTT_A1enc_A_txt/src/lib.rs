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
use execute_aarch32_instrs_VCVTT_Op_A_txt::*;
use HaveAArch32BF16Ext::*;
use common::*;
pub fn decode_aarch32_instrs_VCVTT_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    cond: u8,
    D: bool,
    Vd: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        cond: u8,
        D: bool,
        Vd: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        cond,
        D,
        Vd,
        M,
        Vm,
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
        // C s_2_6: const #() : ()
        let s_2_6: () = ();
        // S s_2_7: call HaveAArch32BF16Ext(s_2_6)
        let s_2_7: bool = HaveAArch32BF16Ext(state, tracer, s_2_6);
        // S s_2_8: not s_2_7
        let s_2_8: bool = !s_2_7;
        // N s_2_9: branch s_2_8 b4 b3
        if s_2_8 {
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
        // D s_3_0: read-var Vd:u8
        let s_3_0: u8 = fn_state.Vd;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 4u16);
        // D s_3_2: read-var D:u8
        let s_3_2: bool = fn_state.D;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: cast zx s_3_12 -> bv
        let s_3_13: Bits = Bits::new(s_3_12 as u128, 5u16);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (s_3_13.value() as i128);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var Vm:u8
        let s_3_16: u8 = fn_state.Vm;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 4u16);
        // D s_3_18: read-var M:u8
        let s_3_18: bool = fn_state.M;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 1u16);
        // D s_3_20: cast reint s_3_17 -> u128
        let s_3_20: u128 = (s_3_17.value() as u128);
        // D s_3_21: size-of s_3_17
        let s_3_21: u16 = s_3_17.length();
        // D s_3_22: cast reint s_3_19 -> u128
        let s_3_22: u128 = (s_3_19.value() as u128);
        // D s_3_23: size-of s_3_19
        let s_3_23: u16 = s_3_19.length();
        // D s_3_24: lsl s_3_20 s_3_23
        let s_3_24: u128 = s_3_20 << s_3_23;
        // D s_3_25: or s_3_24 s_3_22
        let s_3_25: u128 = ((s_3_24) | (s_3_22));
        // D s_3_26: add s_3_21 s_3_23
        let s_3_26: u16 = (s_3_21 + s_3_23);
        // D s_3_27: create-bits s_3_25 s_3_26
        let s_3_27: Bits = Bits::new(s_3_25, s_3_26);
        // D s_3_28: cast reint s_3_27 -> u8
        let s_3_28: u8 = (s_3_27.value() as u8);
        // D s_3_29: cast zx s_3_28 -> bv
        let s_3_29: Bits = Bits::new(s_3_28 as u128, 5u16);
        // D s_3_30: cast zx s_3_29 -> i
        let s_3_30: i128 = (s_3_29.value() as i128);
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: call execute_aarch32_instrs_VCVTT_Op_A_txt(s_3_15, s_3_31)
        let s_3_32: () = execute_aarch32_instrs_VCVTT_Op_A_txt(
            state,
            tracer,
            s_3_15,
            s_3_31,
        );
        // N s_3_33: return
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
