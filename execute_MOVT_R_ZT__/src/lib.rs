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
use X_set::*;
use CheckSMEEnabled::*;
use CheckSMEZT0Enabled::*;
use Elem_read::*;
use ZT0_read::*;
use common::*;
pub fn execute_MOVT_R_ZT__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    offset: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        offset: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        offset,
        t,
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
        // S s_0_1: call CheckSMEEnabled(s_0_0)
        let s_0_1: () = CheckSMEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckSMEZT0Enabled(s_1_0)
        let s_1_1: () = CheckSMEZT0Enabled(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #512s : i64
        let s_2_0: i64 = 512;
        // S s_2_1: call ZT0_read(s_2_0)
        let s_2_1: Bits = ZT0_read(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u512
        let s_2_2: u64 = (s_2_1.value() as u64);
        // C s_2_3: const #64s : i64
        let s_2_3: i64 = 64;
        // C s_2_4: const #64s : i64
        let s_2_4: i64 = 64;
        // S s_2_5: cast zx s_2_2 -> bv
        let s_2_5: Bits = Bits::new(s_2_2 as u128, 512u16);
        // D s_2_6: read-var offset:i64
        let s_2_6: i64 = fn_state.offset;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // C s_2_8: cast zx s_2_4 -> i
        let s_2_8: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_9: call Elem_read(s_2_5, s_2_7, s_2_8)
        let s_2_9: Bits = Elem_read(state, tracer, s_2_5, s_2_7, s_2_8);
        // D s_2_10: cast reint s_2_9 -> u64
        let s_2_10: u64 = (s_2_9.value() as u64);
        // D s_2_11: read-var t:i64
        let s_2_11: i64 = fn_state.t;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: cast zx s_2_10 -> bv
        let s_2_13: Bits = Bits::new(s_2_10 as u128, 64u16);
        // D s_2_14: call X_set(s_2_12, s_2_3, s_2_13)
        let s_2_14: () = X_set(state, tracer, s_2_12, s_2_3, s_2_13);
        // N s_2_15: return
        return;
    }
}
