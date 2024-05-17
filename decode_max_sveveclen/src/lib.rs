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
use IsPow2::*;
use FloorPow2::*;
use common::*;
pub fn decode_max_sveveclen<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: i128,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        max_vlshadow_543: i128,
        max_vl: i128,
        return_value: i128,
        value_name: i128,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #64s : i
        let s_0_0: i128 = 64;
        // D s_0_1: read-var value_name:i
        let s_0_1: i128 = fn_state.value_name;
        // D s_0_2: mul s_0_1 s_0_0
        let s_0_2: i128 = ((s_0_1) * (s_0_0));
        // D s_0_3: write-var max_vl <= s_0_2
        fn_state.max_vl = s_0_2;
        // C s_0_4: const #2048s : i
        let s_0_4: i128 = 2048;
        // D s_0_5: read-var max_vl:i
        let s_0_5: i128 = fn_state.max_vl;
        // D s_0_6: cmp-gt s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) > (s_0_4));
        // N s_0_7: branch s_0_6 b6 b1
        if s_0_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var max_vl:i
        let s_2_0: i128 = fn_state.max_vl;
        // D s_2_1: write-var max_vlshadow#543 <= s_2_0
        fn_state.max_vlshadow_543 = s_2_0;
        // D s_2_2: read-var max_vlshadow#543:i
        let s_2_2: i128 = fn_state.max_vlshadow_543;
        // D s_2_3: call IsPow2(s_2_2)
        let s_2_3: bool = IsPow2(state, tracer, s_2_2);
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b5 b3
        if s_2_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_3_0: read-var max_vlshadow#543:i
        let s_3_0: i128 = fn_state.max_vlshadow_543;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var return_value:i
        let s_4_0: i128 = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var max_vlshadow#543:i
        let s_5_0: i128 = fn_state.max_vlshadow_543;
        // D s_5_1: call FloorPow2(s_5_0)
        let s_5_1: i128 = FloorPow2(state, tracer, s_5_0);
        // D s_5_2: write-var return_value <= s_5_1
        fn_state.return_value = s_5_1;
        // N s_5_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_6_0: const #2048s : i
        let s_6_0: i128 = 2048;
        // D s_6_1: write-var max_vl <= s_6_0
        fn_state.max_vl = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
