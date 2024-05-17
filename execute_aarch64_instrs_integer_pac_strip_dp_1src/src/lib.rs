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
use HavePACExt::*;
use Strip::*;
use X_read::*;
use X_set::*;
use common::*;
pub fn execute_aarch64_instrs_integer_pac_strip_dp_1src<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    data: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        d: i64,
        data: bool,
    }
    let fn_state = FunctionState {
        d,
        data,
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
        // S s_0_1: call HavePACExt(s_0_0)
        let s_0_1: bool = HavePACExt(state, tracer, s_0_0);
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
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // C s_2_1: const #64s : i64
        let s_2_1: i64 = 64;
        // D s_2_2: read-var d:i64
        let s_2_2: i64 = fn_state.d;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: call X_read(s_2_3, s_2_1)
        let s_2_4: Bits = X_read(state, tracer, s_2_3, s_2_1);
        // D s_2_5: cast reint s_2_4 -> u64
        let s_2_5: u64 = (s_2_4.value() as u64);
        // D s_2_6: read-var data:u8
        let s_2_6: bool = fn_state.data;
        // D s_2_7: call Strip(s_2_5, s_2_6)
        let s_2_7: u64 = Strip(state, tracer, s_2_5, s_2_6);
        // D s_2_8: read-var d:i64
        let s_2_8: i64 = fn_state.d;
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_10: cast zx s_2_7 -> bv
        let s_2_10: Bits = Bits::new(s_2_7 as u128, 64u16);
        // D s_2_11: call X_set(s_2_9, s_2_0, s_2_10)
        let s_2_11: () = X_set(state, tracer, s_2_9, s_2_0, s_2_10);
        // N s_2_12: return
        return;
    }
}
