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
use ZT0_set::*;
use FailTransaction::*;
use CheckSMEEnabled::*;
use CheckSMEZT0Enabled::*;
use Zeros::*;
use HaveTME::*;
use common::*;
pub fn execute_ZERO_ZT_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_289955: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_289957: bool,
        gs_289955: (),
    }
    let fn_state = FunctionState {
        gs_289955,
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
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveTME(s_2_0)
        let s_2_1: bool = HaveTME(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b8 b3
        if s_2_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#289957 <= s_3_0
        fn_state.gs_289957 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#289957:u8
        let s_4_0: bool = fn_state.gs_289957;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #512s : i
        let s_6_0: i128 = 512;
        // S s_6_1: call Zeros(s_6_0)
        let s_6_1: Bits = Zeros(state, tracer, s_6_0);
        // S s_6_2: cast reint s_6_1 -> u512
        let s_6_2: u64 = (s_6_1.value() as u64);
        // C s_6_3: const #512s : i64
        let s_6_3: i64 = 512;
        // S s_6_4: cast zx s_6_2 -> bv
        let s_6_4: Bits = Bits::new(s_6_2 as u128, 512u16);
        // S s_6_5: call ZT0_set(s_6_3, s_6_4)
        let s_6_5: () = ZT0_set(state, tracer, s_6_3, s_6_4);
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2u : u32
        let s_7_0: u32 = 2;
        // C s_7_1: const #0u : u8
        let s_7_1: bool = false;
        // S s_7_2: call FailTransaction(s_7_0, s_7_1)
        let s_7_2: () = FailTransaction(state, tracer, s_7_0, s_7_1);
        // N s_7_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #100180u : u32
        let s_8_0: u32 = 100180;
        // D s_8_1: read-reg s_8_0:i
        let s_8_1: i128 = {
            let value = state.read_register::<i128>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: cmp-gt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) > (s_8_2));
        // D s_8_4: write-var gs#289957 <= s_8_3
        fn_state.gs_289957 = s_8_3;
        // N s_8_5: jump b4
        return block_4(state, tracer, fn_state);
    }
}
