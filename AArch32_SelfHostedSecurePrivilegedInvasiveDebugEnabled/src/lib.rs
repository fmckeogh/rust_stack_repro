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
use NonSecureOnlyImplementation::*;
use common::*;
pub fn AArch32_SelfHostedSecurePrivilegedInvasiveDebugEnabled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_4847: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_4850: bool,
        gs_4848: bool,
        return_value: bool,
        gs_4847: (),
    }
    let fn_state = FunctionState {
        gs_4847,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b9 b1
        if s_0_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#4848 <= s_1_0
        fn_state.gs_4848 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#4848:u8
        let s_2_0: bool = fn_state.gs_4848;
        // N s_2_1: branch s_2_0 b8 b3
        if s_2_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #20440u : u32
        let s_3_0: u32 = 20440;
        // D s_3_1: read-reg s_3_0:u32
        let s_3_1: u32 = {
            let value = state.read_register::<u32>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #1u : u32
        let s_3_2: u32 = 1;
        // D s_3_3: cmp-eq s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) == (s_3_2));
        // N s_3_4: branch s_3_3 b7 b4
        if s_3_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#4850 <= s_4_0
        fn_state.gs_4850 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#4850:u8
        let s_5_0: bool = fn_state.gs_4850;
        // D s_5_1: write-var return_value <= s_5_0
        fn_state.return_value = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var return_value:u8
        let s_6_0: bool = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #12136u : u32
        let s_7_0: u32 = 12136;
        // D s_7_1: read-reg s_7_0:u32
        let s_7_1: u32 = {
            let value = state.read_register::<u32>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #1u : u32
        let s_7_2: u32 = 1;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // D s_7_4: write-var gs#4850 <= s_7_3
        fn_state.gs_4850 = s_7_3;
        // N s_7_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call NonSecureOnlyImplementation(s_9_0)
        let s_9_1: bool = NonSecureOnlyImplementation(state, tracer, s_9_0);
        // D s_9_2: write-var gs#4848 <= s_9_1
        fn_state.gs_4848 = s_9_1;
        // N s_9_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
