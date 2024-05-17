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
use IsSecureEL2Enabled::*;
use SCR_GEN_read::*;
use u_get_SCRType_NS::*;
use common::*;
pub fn EL2Enabled<T: Tracer>(state: &mut State, tracer: &T, gs_1893: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_1894: bool,
        gs_1896: bool,
        gs_1895: bool,
        gs_1893: (),
    }
    let fn_state = FunctionState {
        gs_1893,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
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
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
            return block_3(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#1896 <= s_1_0
        fn_state.gs_1896 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#1896:u8
        let s_2_0: bool = fn_state.gs_1896;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #424u : u32
        let s_3_0: u32 = 424;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // D s_3_3: cmp-lt s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) < (s_3_2));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b9 b4
        if s_3_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SCR_GEN_read(s_4_0)
        let s_4_1: ProductType5c790c8ef59cc8b2 = SCR_GEN_read(state, tracer, s_4_0);
        // S s_4_2: call _get_SCRType_NS(s_4_1)
        let s_4_2: bool = u_get_SCRType_NS(state, tracer, s_4_1);
        // S s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #1u : u8
        let s_4_4: bool = true;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // S s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // D s_4_7: write-var gs#1894 <= s_4_6
        fn_state.gs_1894 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#1894:u8
        let s_5_0: bool = fn_state.gs_1894;
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
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call IsSecureEL2Enabled(s_6_0)
        let s_6_1: bool = IsSecureEL2Enabled(state, tracer, s_6_0);
        // D s_6_2: write-var gs#1895 <= s_6_1
        fn_state.gs_1895 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#1895:u8
        let s_7_0: bool = fn_state.gs_1895;
        // D s_7_1: write-var gs#1896 <= s_7_0
        fn_state.gs_1896 = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#1895 <= s_8_0
        fn_state.gs_1895 = s_8_0;
        // N s_8_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#1894 <= s_9_0
        fn_state.gs_1894 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
