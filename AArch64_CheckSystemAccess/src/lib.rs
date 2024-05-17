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
use BranchTargetCheck::*;
use FailTransaction::*;
use HaveBTIExt::*;
use HaveTME::*;
use CheckTransactionalSystemAccess::*;
use common::*;
pub fn AArch64_CheckSystemAccess<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: u8,
    op1: u8,
    crn: u8,
    crm: u8,
    op2: u8,
    rt: u8,
    read: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_15714: bool,
        gs_15715: bool,
        op0: u8,
        op1: u8,
        crn: u8,
        crm: u8,
        op2: u8,
        rt: u8,
        read: bool,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        crm,
        op2,
        rt,
        read,
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
        // S s_0_1: call HaveBTIExt(s_0_0)
        let s_0_1: bool = HaveBTIExt(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b11 b1
        if s_0_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
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
        // N s_2_2: branch s_2_1 b10 b3
        if s_2_1 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#15714 <= s_3_0
        fn_state.gs_15714 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#15714:u8
        let s_4_0: bool = fn_state.gs_15714;
        // N s_4_1: branch s_4_0 b9 b5
        if s_4_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#15715 <= s_5_0
        fn_state.gs_15715 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#15715:u8
        let s_6_0: bool = fn_state.gs_15715;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
        // C s_8_1: const #0u : u8
        let s_8_1: bool = false;
        // S s_8_2: call FailTransaction(s_8_0, s_8_1)
        let s_8_2: () = FailTransaction(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var op0:u8
        let s_9_0: u8 = fn_state.op0;
        // D s_9_1: read-var op1:u8
        let s_9_1: u8 = fn_state.op1;
        // D s_9_2: read-var crn:u8
        let s_9_2: u8 = fn_state.crn;
        // D s_9_3: read-var crm:u8
        let s_9_3: u8 = fn_state.crm;
        // D s_9_4: read-var op2:u8
        let s_9_4: u8 = fn_state.op2;
        // D s_9_5: read-var read:u8
        let s_9_5: bool = fn_state.read;
        // D s_9_6: call CheckTransactionalSystemAccess(s_9_0, s_9_1, s_9_2, s_9_3, s_9_4, s_9_5)
        let s_9_6: bool = CheckTransactionalSystemAccess(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
            s_9_4,
            s_9_5,
        );
        // D s_9_7: not s_9_6
        let s_9_7: bool = !s_9_6;
        // D s_9_8: write-var gs#15715 <= s_9_7
        fn_state.gs_15715 = s_9_7;
        // N s_9_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #100180u : u32
        let s_10_0: u32 = 100180;
        // D s_10_1: read-reg s_10_0:i
        let s_10_1: i128 = {
            let value = state.read_register::<i128>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #0s : i
        let s_10_2: i128 = 0;
        // D s_10_3: cmp-gt s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) > (s_10_2));
        // D s_10_4: write-var gs#15714 <= s_10_3
        fn_state.gs_15714 = s_10_3;
        // N s_10_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call BranchTargetCheck(s_11_0)
        let s_11_1: () = BranchTargetCheck(state, tracer, s_11_0);
        // N s_11_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
