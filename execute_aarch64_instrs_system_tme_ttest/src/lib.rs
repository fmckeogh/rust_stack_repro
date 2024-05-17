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
use integer_subrange::*;
use IsTMEEnabled::*;
use common::*;
pub fn execute_aarch64_instrs_system_tme_ttest<T: Tracer>(
    state: &mut State,
    tracer: &T,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
    }
    let fn_state = FunctionState {
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
        // S s_0_1: call IsTMEEnabled(s_0_0)
        let s_0_1: bool = IsTMEEnabled(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b2 b1
        if s_0_2 {
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
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // C s_1_1: const #100180u : u32
        let s_1_1: u32 = 100180;
        // D s_1_2: read-reg s_1_1:i
        let s_1_2: i128 = {
            let value = state.read_register::<i128>(s_1_1 as isize);
            tracer.read_register(s_1_1 as isize, value);
            value
        };
        // C s_1_3: const #63s : i
        let s_1_3: i128 = 63;
        // C s_1_4: const #0s : i
        let s_1_4: i128 = 0;
        // D s_1_5: call integer_subrange(s_1_2, s_1_3, s_1_4)
        let s_1_5: Bits = integer_subrange(state, tracer, s_1_2, s_1_3, s_1_4);
        // D s_1_6: cast reint s_1_5 -> u64
        let s_1_6: u64 = (s_1_5.value() as u64);
        // D s_1_7: read-var t:i64
        let s_1_7: i64 = fn_state.t;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast zx s_1_6 -> bv
        let s_1_9: Bits = Bits::new(s_1_6 as u128, 64u16);
        // D s_1_10: call X_set(s_1_8, s_1_0, s_1_9)
        let s_1_10: () = X_set(state, tracer, s_1_8, s_1_0, s_1_9);
        // N s_1_11: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
}
