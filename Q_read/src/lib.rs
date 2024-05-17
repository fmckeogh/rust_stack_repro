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
use V_read::*;
use common::*;
pub fn Q_read<T: Tracer>(state: &mut State, tracer: &T, n: i128) -> u128 {
    #[derive(Default)]
    struct FunctionState {
        gs_31266: bool,
        n: i128,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var n:i
        let s_0_1: i128 = fn_state.n;
        // D s_0_2: cmp-ge s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) >= (s_0_0));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#31266 <= s_1_0
        fn_state.gs_31266 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_2_0: read-var gs#31266:u8
        let s_2_0: bool = fn_state.gs_31266;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #128s : i64
        let s_2_2: i64 = 128;
        // D s_2_3: read-var n:i
        let s_2_3: i128 = fn_state.n;
        // D s_2_4: call V_read(s_2_3, s_2_2)
        let s_2_4: Bits = V_read(state, tracer, s_2_3, s_2_2);
        // D s_2_5: cast reint s_2_4 -> u128
        let s_2_5: u128 = (s_2_4.value() as u128);
        // N s_2_6: return s_2_5
        return s_2_5;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i
        let s_3_1: i128 = fn_state.n;
        // D s_3_2: cmp-le s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) <= (s_3_0));
        // D s_3_3: write-var gs#31266 <= s_3_2
        fn_state.gs_31266 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
