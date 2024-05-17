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
use execute_aarch64_instrs_integer_conditional_compare_register::*;
use common::*;
pub fn decode_ccmn_reg_aarch64_instrs_integer_conditional_compare_register<T: Tracer>(
    state: &mut State,
    tracer: &T,
    nzcv: u8,
    Rn: u8,
    cond: u8,
    Rm: u8,
    op: bool,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        ga_251030: i64,
        n: i64,
        nzcv: u8,
        Rn: u8,
        cond: u8,
        Rm: u8,
        op: bool,
        sf: bool,
    }
    let fn_state = FunctionState {
        nzcv,
        Rn,
        cond,
        Rm,
        op,
        sf,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rm:u8
        let s_0_5: u8 = fn_state.Rm;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var m <= s_0_8
        fn_state.m = s_0_8;
        // D s_0_10: read-var sf:u8
        let s_0_10: bool = fn_state.sf;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b3 b1
        if s_0_14 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#251030 <= s_1_0
        fn_state.ga_251030 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#251030:i64
        let s_2_0: i64 = fn_state.ga_251030;
        // D s_2_1: read-var op:u8
        let s_2_1: bool = fn_state.op;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 1u16);
        // C s_2_3: const #1u : u8
        let s_2_3: bool = true;
        // C s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 1u16);
        // D s_2_5: cmp-eq s_2_2 s_2_4
        let s_2_5: bool = ((s_2_2) == (s_2_4));
        // D s_2_6: read-var cond:u8
        let s_2_6: u8 = fn_state.cond;
        // D s_2_7: read-var nzcv:u8
        let s_2_7: u8 = fn_state.nzcv;
        // D s_2_8: cast zx s_2_0 -> i
        let s_2_8: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var m:i64
        let s_2_10: i64 = fn_state.m;
        // D s_2_11: read-var n:i64
        let s_2_11: i64 = fn_state.n;
        // D s_2_12: call execute_aarch64_instrs_integer_conditional_compare_register(s_2_6, s_2_9, s_2_7, s_2_10, s_2_11, s_2_5)
        let s_2_12: () = execute_aarch64_instrs_integer_conditional_compare_register(
            state,
            tracer,
            s_2_6,
            s_2_9,
            s_2_7,
            s_2_10,
            s_2_11,
            s_2_5,
        );
        // N s_2_13: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#251030 <= s_3_0
        fn_state.ga_251030 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
