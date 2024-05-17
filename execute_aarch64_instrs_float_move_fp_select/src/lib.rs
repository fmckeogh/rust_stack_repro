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
use CheckFPEnabled64::*;
use V_set::*;
use V_read::*;
use ConditionHolds::*;
use common::*;
pub fn execute_aarch64_instrs_float_move_fp_select<T: Tracer>(
    state: &mut State,
    tracer: &T,
    condition: u8,
    d: i64,
    datasize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1288: i64,
        result: Bits,
        condition: u8,
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        condition,
        d,
        datasize,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1288 <= s_0_2
        fn_state.datasizeshadow_1288 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPEnabled64(s_0_4)
        let s_0_5: () = CheckFPEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var condition:u8
        let s_1_0: u8 = fn_state.condition;
        // D s_1_1: call ConditionHolds(s_1_0)
        let s_1_1: bool = ConditionHolds(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b4 b2
        if s_1_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1288:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1288;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call V_read(s_2_4, s_2_2)
        let s_2_5: Bits = V_read(state, tracer, s_2_4, s_2_2);
        // D s_2_6: write-var result <= s_2_5
        fn_state.result = s_2_5;
        // N s_2_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var datasizeshadow#1288:i64
        let s_3_0: i64 = fn_state.datasizeshadow_1288;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var result:bv
        let s_3_5: Bits = fn_state.result;
        // D s_3_6: call V_set(s_3_4, s_3_2, s_3_5)
        let s_3_6: () = V_set(state, tracer, s_3_4, s_3_2, s_3_5);
        // N s_3_7: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1288:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1288;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var n:i64
        let s_4_3: i64 = fn_state.n;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: call V_read(s_4_4, s_4_2)
        let s_4_5: Bits = V_read(state, tracer, s_4_4, s_4_2);
        // D s_4_6: write-var result <= s_4_5
        fn_state.result = s_4_5;
        // N s_4_7: jump b3
        return block_3(state, tracer, fn_state);
    }
}
