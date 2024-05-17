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
use CheckAdvSIMDEnabled::*;
use D_set::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VBIC_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    imm64: u64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_306504: i64,
        d: i64,
        imm64: u64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d,
        imm64,
        regs,
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
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#306504 <= s_1_5
        fn_state.gs_306504 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#306504:i64
        let s_2_1: i64 = fn_state.gs_306504;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
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
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var d:i64
        let s_3_6: i64 = fn_state.d;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: read-var r:i64
        let s_3_8: i64 = fn_state.r;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: add s_3_7 s_3_9
        let s_3_10: i128 = (s_3_7 + s_3_9);
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: call D_read(s_3_12)
        let s_3_13: u64 = D_read(state, tracer, s_3_12);
        // D s_3_14: read-var imm64:u64
        let s_3_14: u64 = fn_state.imm64;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 64u16);
        // D s_3_16: not s_3_15
        let s_3_16: Bits = !s_3_15;
        // D s_3_17: cast reint s_3_16 -> u64
        let s_3_17: u64 = (s_3_16.value() as u64);
        // D s_3_18: cast zx s_3_13 -> bv
        let s_3_18: Bits = Bits::new(s_3_13 as u128, 64u16);
        // D s_3_19: cast zx s_3_17 -> bv
        let s_3_19: Bits = Bits::new(s_3_17 as u128, 64u16);
        // D s_3_20: and s_3_18 s_3_19
        let s_3_20: Bits = ((s_3_18) & (s_3_19));
        // D s_3_21: cast reint s_3_20 -> u64
        let s_3_21: u64 = (s_3_20.value() as u64);
        // D s_3_22: cast zx s_3_5 -> i
        let s_3_22: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_23: call D_set(s_3_22, s_3_21)
        let s_3_23: () = D_set(state, tracer, s_3_22, s_3_21);
        // D s_3_24: read-var r:i64
        let s_3_24: i64 = fn_state.r;
        // C s_3_25: const #1s : i64
        let s_3_25: i64 = 1;
        // D s_3_26: add s_3_24 s_3_25
        let s_3_26: i64 = (s_3_24 + s_3_25);
        // D s_3_27: write-var r <= s_3_26
        fn_state.r = s_3_26;
        // N s_3_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
}
