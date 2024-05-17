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
use common::*;
pub fn decode_max_smeveclen<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: i128,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_29695: bool,
        bitpos: i128,
        svl: i128,
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
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var value_name:i
        let s_0_1: i128 = fn_state.value_name;
        // D s_0_2: lsr s_0_1 s_0_0
        let s_0_2: i128 = s_0_1 >> s_0_0;
        // D s_0_3: write-var svl <= s_0_2
        fn_state.svl = s_0_2;
        // C s_0_4: const #0s : i
        let s_0_4: i128 = 0;
        // D s_0_5: write-var bitpos <= s_0_4
        fn_state.bitpos = s_0_4;
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var svl:i
        let s_1_1: i128 = fn_state.svl;
        // D s_1_2: call neq_int(s_1_1, s_1_0)
        let s_1_2: bool = neq_int(state, tracer, s_1_1, s_1_0);
        // D s_1_3: not s_1_2
        let s_1_3: bool = !s_1_2;
        // N s_1_4: branch s_1_3 b3 b2
        if s_1_3 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var svl:i
        let s_2_1: i128 = fn_state.svl;
        // D s_2_2: lsr s_2_1 s_2_0
        let s_2_2: i128 = s_2_1 >> s_2_0;
        // D s_2_3: write-var svl <= s_2_2
        fn_state.svl = s_2_2;
        // C s_2_4: const #1s : i
        let s_2_4: i128 = 1;
        // D s_2_5: read-var bitpos:i
        let s_2_5: i128 = fn_state.bitpos;
        // D s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: write-var bitpos <= s_2_6
        fn_state.bitpos = s_2_6;
        // N s_2_8: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var value_name:i
        let s_3_1: i128 = fn_state.value_name;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // N s_3_3: branch s_3_2 b9 b4
        if s_3_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_4_0: const #4s : i
        let s_4_0: i128 = 4;
        // D s_4_1: read-var bitpos:i
        let s_4_1: i128 = fn_state.bitpos;
        // D s_4_2: cmp-gt s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) > (s_4_0));
        // D s_4_3: write-var gs#29695 <= s_4_2
        fn_state.gs_29695 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_5_0: read-var gs#29695:u8
        let s_5_0: bool = fn_state.gs_29695;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_7_0: read-var bitpos:i
        let s_7_0: i128 = fn_state.bitpos;
        // D s_7_1: pow2 s_7_0
        let s_7_1: i128 = (s_7_0).pow(2);
        // C s_7_2: const #128s : i
        let s_7_2: i128 = 128;
        // D s_7_3: mul s_7_2 s_7_1
        let s_7_3: i128 = ((s_7_2) * (s_7_1));
        // N s_7_4: return s_7_3
        return s_7_3;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // D s_8_1: write-var bitpos <= s_8_0
        fn_state.bitpos = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#29695 <= s_9_0
        fn_state.gs_29695 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
