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
use execute_aarch32_instrs_VJCVT_Op_A_txt::*;
use HaveFJCVTZSExt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_VJCVT_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vd: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        D: bool,
        Vd: u8,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
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
        // S s_0_1: call HaveFJCVTZSExt(s_0_0)
        let s_0_1: bool = HaveFJCVTZSExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b4 b1
        if s_0_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call InITBlock(s_1_0)
        let s_1_1: bool = InITBlock(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b3 b2
        if s_1_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Vd:u8
        let s_2_0: u8 = fn_state.Vd;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: read-var D:u8
        let s_2_2: bool = fn_state.D;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cast reint s_2_1 -> u128
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
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 5u16);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (s_2_13.value() as i128);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: read-var M:u8
        let s_2_16: bool = fn_state.M;
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // D s_2_18: read-var Vm:u8
        let s_2_18: u8 = fn_state.Vm;
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 4u16);
        // D s_2_20: cast reint s_2_17 -> u128
        let s_2_20: u128 = (s_2_17.value() as u128);
        // D s_2_21: size-of s_2_17
        let s_2_21: u16 = s_2_17.length();
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // D s_2_24: lsl s_2_20 s_2_23
        let s_2_24: u128 = s_2_20 << s_2_23;
        // D s_2_25: or s_2_24 s_2_22
        let s_2_25: u128 = ((s_2_24) | (s_2_22));
        // D s_2_26: add s_2_21 s_2_23
        let s_2_26: u16 = (s_2_21 + s_2_23);
        // D s_2_27: create-bits s_2_25 s_2_26
        let s_2_27: Bits = Bits::new(s_2_25, s_2_26);
        // D s_2_28: cast reint s_2_27 -> u8
        let s_2_28: u8 = (s_2_27.value() as u8);
        // D s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 5u16);
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (s_2_29.value() as i128);
        // D s_2_31: cast reint s_2_30 -> i64
        let s_2_31: i64 = (s_2_30 as i64);
        // D s_2_32: call execute_aarch32_instrs_VJCVT_Op_A_txt(s_2_15, s_2_31)
        let s_2_32: () = execute_aarch32_instrs_VJCVT_Op_A_txt(
            state,
            tracer,
            s_2_15,
            s_2_31,
        );
        // N s_2_33: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: panic
        panic!("{:?}", ());
        // N s_3_1: return
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
