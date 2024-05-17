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
use neq_int::*;
use set_R::*;
use common::*;
pub fn X_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i128,
    width: i64,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_4177: bool,
        widthshadow_49: i64,
        gs_4180: bool,
        n: i128,
        width: i64,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        n,
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var width:i64
        let s_0_0: i64 = fn_state.width;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var widthshadow#49 <= s_0_2
        fn_state.widthshadow_49 = s_0_2;
        // C s_0_4: const #0s : i
        let s_0_4: i128 = 0;
        // D s_0_5: read-var n:i
        let s_0_5: i128 = fn_state.n;
        // D s_0_6: cmp-ge s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) >= (s_0_4));
        // N s_0_7: branch s_0_6 b8 b1
        if s_0_6 {
            return block_8(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#4177 <= s_1_0
        fn_state.gs_4177 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#4177:u8
        let s_2_0: bool = fn_state.gs_4177;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #32s : i
        let s_2_2: i128 = 32;
        // D s_2_3: read-var widthshadow#49:i64
        let s_2_3: i64 = fn_state.widthshadow_49;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: cmp-eq s_2_4 s_2_2
        let s_2_5: bool = ((s_2_4) == (s_2_2));
        // N s_2_6: branch s_2_5 b7 b3
        if s_2_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // D s_3_1: read-var widthshadow#49:i64
        let s_3_1: i64 = fn_state.widthshadow_49;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // D s_3_4: write-var gs#4180 <= s_3_3
        fn_state.gs_4180 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#4180:u8
        let s_4_0: bool = fn_state.gs_4180;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // C s_4_2: const #31s : i
        let s_4_2: i128 = 31;
        // D s_4_3: read-var n:i
        let s_4_3: i128 = fn_state.n;
        // D s_4_4: call neq_int(s_4_3, s_4_2)
        let s_4_4: bool = neq_int(state, tracer, s_4_3, s_4_2);
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i
        let s_6_0: i128 = 64;
        // D s_6_1: read-var value_name:bv
        let s_6_1: Bits = fn_state.value_name;
        // D s_6_2: bits-cast zx s_6_1 -> bv length s_6_0
        let s_6_2: Bits = s_6_1.zero_extend(s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // D s_6_4: read-var n:i
        let s_6_4: i128 = fn_state.n;
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: call set_R(s_6_5, s_6_3)
        let s_6_6: () = set_R(state, tracer, s_6_5, s_6_3);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#4180 <= s_7_0
        fn_state.gs_4180 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #31s : i
        let s_8_0: i128 = 31;
        // D s_8_1: read-var n:i
        let s_8_1: i128 = fn_state.n;
        // D s_8_2: cmp-le s_8_1 s_8_0
        let s_8_2: bool = ((s_8_1) <= (s_8_0));
        // D s_8_3: write-var gs#4177 <= s_8_2
        fn_state.gs_4177 = s_8_2;
        // N s_8_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
