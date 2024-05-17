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
use HaveSVE::*;
use HaveSME::*;
use execute_SQINCH_R_RS_SX::*;
use common::*;
pub fn decode_SQINCH_R_RS_SX<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    sf: bool,
    imm4: u8,
    D: bool,
    U: bool,
    pattern: u8,
    Rdn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_191260: bool,
        size: u8,
        sf: bool,
        imm4: u8,
        D: bool,
        U: bool,
        pattern: u8,
        Rdn: u8,
    }
    let fn_state = FunctionState {
        size,
        sf,
        imm4,
        D,
        U,
        pattern,
        Rdn,
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
        // S s_0_1: call HaveSVE(s_0_0)
        let s_0_1: bool = HaveSVE(state, tracer, s_0_0);
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#191260 <= s_1_0
        fn_state.gs_191260 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#191260:u8
        let s_2_0: bool = fn_state.gs_191260;
        // N s_2_1: branch s_2_0 b4 b3
        if s_2_0 {
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
        // C s_3_0: const #16s : i64
        let s_3_0: i64 = 16;
        // D s_3_1: read-var Rdn:u8
        let s_3_1: u8 = fn_state.Rdn;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 5u16);
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (s_3_2.value() as i128);
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: read-var pattern:u8
        let s_3_5: u8 = fn_state.pattern;
        // D s_3_6: read-var imm4:u8
        let s_3_6: u8 = fn_state.imm4;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 4u16);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (s_3_7.value() as i128);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // D s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: add s_3_11 s_3_10
        let s_3_12: i128 = (s_3_11 + s_3_10);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // C s_3_14: const #0u : u8
        let s_3_14: bool = false;
        // C s_3_15: const #32s : i64
        let s_3_15: i64 = 32;
        // D s_3_16: call execute_SQINCH_R_RS_SX(s_3_4, s_3_0, s_3_13, s_3_5, s_3_15, s_3_14)
        let s_3_16: () = execute_SQINCH_R_RS_SX(
            state,
            tracer,
            s_3_4,
            s_3_0,
            s_3_13,
            s_3_5,
            s_3_15,
            s_3_14,
        );
        // N s_3_17: return
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
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call HaveSME(s_5_0)
        let s_5_1: bool = HaveSME(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // D s_5_3: write-var gs#191260 <= s_5_2
        fn_state.gs_191260 = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
