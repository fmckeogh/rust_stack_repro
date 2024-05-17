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
use execute_aarch32_instrs_VINS_Op_A_txt::*;
use FPSCR_read__1::*;
use u_get_FPSCR_Type_length::*;
use ConditionPassed::*;
use HaveFP16Ext::*;
use u_get_FPSCR_Type_Stride::*;
use common::*;
pub fn decode_aarch32_instrs_VINS_A1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    D: bool,
    Vd: u8,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_312772: bool,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveFP16Ext(s_2_0)
        let s_2_1: bool = HaveFP16Ext(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b9 b3
        if s_2_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call FPSCR_read__1(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_3_0);
        // S s_3_2: call _get_FPSCR_Type_length(s_3_1)
        let s_3_2: u8 = u_get_FPSCR_Type_length(state, tracer, s_3_1);
        // S s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 3u16);
        // C s_3_4: const #0u : u8
        let s_3_4: u8 = 0;
        // C s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 3u16);
        // S s_3_6: cmp-ne s_3_3 s_3_5
        let s_3_6: bool = ((s_3_3) != (s_3_5));
        // N s_3_7: branch s_3_6 b8 b4
        if s_3_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call FPSCR_read__1(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_4_0);
        // S s_4_2: call _get_FPSCR_Type_Stride(s_4_1)
        let s_4_2: u8 = u_get_FPSCR_Type_Stride(state, tracer, s_4_1);
        // S s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 2u16);
        // C s_4_4: const #0u : u8
        let s_4_4: u8 = 0;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 2u16);
        // S s_4_6: cmp-ne s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) != (s_4_5));
        // D s_4_7: write-var gs#312772 <= s_4_6
        fn_state.gs_312772 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#312772:u8
        let s_5_0: bool = fn_state.gs_312772;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var Vd:u8
        let s_6_0: u8 = fn_state.Vd;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: read-var D:u8
        let s_6_2: bool = fn_state.D;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cast reint s_6_1 -> u128
        let s_6_4: u128 = (s_6_1.value() as u128);
        // D s_6_5: size-of s_6_1
        let s_6_5: u16 = s_6_1.length();
        // D s_6_6: cast reint s_6_3 -> u128
        let s_6_6: u128 = (s_6_3.value() as u128);
        // D s_6_7: size-of s_6_3
        let s_6_7: u16 = s_6_3.length();
        // D s_6_8: lsl s_6_4 s_6_7
        let s_6_8: u128 = s_6_4 << s_6_7;
        // D s_6_9: or s_6_8 s_6_6
        let s_6_9: u128 = ((s_6_8) | (s_6_6));
        // D s_6_10: add s_6_5 s_6_7
        let s_6_10: u16 = (s_6_5 + s_6_7);
        // D s_6_11: create-bits s_6_9 s_6_10
        let s_6_11: Bits = Bits::new(s_6_9, s_6_10);
        // D s_6_12: cast reint s_6_11 -> u8
        let s_6_12: u8 = (s_6_11.value() as u8);
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 5u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: read-var Vm:u8
        let s_6_16: u8 = fn_state.Vm;
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 4u16);
        // D s_6_18: read-var M:u8
        let s_6_18: bool = fn_state.M;
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 1u16);
        // D s_6_20: cast reint s_6_17 -> u128
        let s_6_20: u128 = (s_6_17.value() as u128);
        // D s_6_21: size-of s_6_17
        let s_6_21: u16 = s_6_17.length();
        // D s_6_22: cast reint s_6_19 -> u128
        let s_6_22: u128 = (s_6_19.value() as u128);
        // D s_6_23: size-of s_6_19
        let s_6_23: u16 = s_6_19.length();
        // D s_6_24: lsl s_6_20 s_6_23
        let s_6_24: u128 = s_6_20 << s_6_23;
        // D s_6_25: or s_6_24 s_6_22
        let s_6_25: u128 = ((s_6_24) | (s_6_22));
        // D s_6_26: add s_6_21 s_6_23
        let s_6_26: u16 = (s_6_21 + s_6_23);
        // D s_6_27: create-bits s_6_25 s_6_26
        let s_6_27: Bits = Bits::new(s_6_25, s_6_26);
        // D s_6_28: cast reint s_6_27 -> u8
        let s_6_28: u8 = (s_6_27.value() as u8);
        // D s_6_29: cast zx s_6_28 -> bv
        let s_6_29: Bits = Bits::new(s_6_28 as u128, 5u16);
        // D s_6_30: cast zx s_6_29 -> i
        let s_6_30: i128 = (s_6_29.value() as i128);
        // D s_6_31: cast reint s_6_30 -> i64
        let s_6_31: i64 = (s_6_30 as i64);
        // D s_6_32: call execute_aarch32_instrs_VINS_Op_A_txt(s_6_15, s_6_31)
        let s_6_32: () = execute_aarch32_instrs_VINS_Op_A_txt(
            state,
            tracer,
            s_6_15,
            s_6_31,
        );
        // N s_6_33: return
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
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#312772 <= s_8_0
        fn_state.gs_312772 = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
