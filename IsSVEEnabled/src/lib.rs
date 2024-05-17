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
use HaveSVE::*;
use IsSMEEnabled::*;
use IsOriginalSVEEnabled::*;
use HaveSME::*;
use common::*;
pub fn IsSVEEnabled<T: Tracer>(state: &mut State, tracer: &T, el: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_4123: bool,
        return_value: bool,
        el: u8,
    }
    let fn_state = FunctionState {
        el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveSME(s_0_0)
        let s_0_1: bool = HaveSME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b8 b1
        if s_0_1 {
            return block_8(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#4123 <= s_1_0
        fn_state.gs_4123 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#4123:u8
        let s_2_0: bool = fn_state.gs_4123;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveSVE(s_3_0)
        let s_3_1: bool = HaveSVE(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b6 b4
        if s_3_1 {
            return block_6(state, tracer, fn_state);
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
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var el:u8
        let s_6_0: u8 = fn_state.el;
        // D s_6_1: call IsOriginalSVEEnabled(s_6_0)
        let s_6_1: bool = IsOriginalSVEEnabled(state, tracer, s_6_0);
        // D s_6_2: write-var return_value <= s_6_1
        fn_state.return_value = s_6_1;
        // N s_6_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var el:u8
        let s_7_0: u8 = fn_state.el;
        // D s_7_1: call IsSMEEnabled(s_7_0)
        let s_7_1: bool = IsSMEEnabled(state, tracer, s_7_0);
        // D s_7_2: write-var return_value <= s_7_1
        fn_state.return_value = s_7_1;
        // N s_7_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #16989u : u32
        let s_8_0: u32 = 16989;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: bool = {
            let value = state.read_register::<bool>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 1u16);
        // C s_8_3: const #1u : u8
        let s_8_3: bool = true;
        // C s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: cmp-eq s_8_2 s_8_4
        let s_8_5: bool = ((s_8_2) == (s_8_4));
        // D s_8_6: write-var gs#4123 <= s_8_5
        fn_state.gs_4123 = s_8_5;
        // N s_8_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
