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
use HaveAArch32::*;
use HaveAArch64::*;
use common::*;
pub fn UsingAArch32<T: Tracer>(state: &mut State, tracer: &T, gs_2039: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        aarch32: bool,
        gs_2039: (),
    }
    let fn_state = FunctionState {
        gs_2039,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #16999u : u32
        let s_0_0: u32 = 16999;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 1u16);
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 1u16);
        // D s_0_5: cmp-eq s_0_2 s_0_4
        let s_0_5: bool = ((s_0_2) == (s_0_4));
        // D s_0_6: write-var aarch32 <= s_0_5
        fn_state.aarch32 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HaveAArch32(s_0_7)
        let s_0_8: bool = HaveAArch32(state, tracer, s_0_7);
        // S s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b6 b1
        if s_0_9 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveAArch64(s_2_0)
        let s_2_1: bool = HaveAArch64(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var aarch32:u8
        let s_4_0: bool = fn_state.aarch32;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var aarch32:u8
        let s_5_0: bool = fn_state.aarch32;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var aarch32:u8
        let s_6_0: bool = fn_state.aarch32;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // N s_6_2: assert s_6_1
        let s_6_2: () = assert!(s_6_1);
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
