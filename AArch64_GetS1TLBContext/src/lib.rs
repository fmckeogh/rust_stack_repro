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
use AArch64_TLBContextEL20::*;
use AArch64_TLBContextEL10::*;
use AArch64_TLBContextEL3::*;
use AArch64_TLBContextEL2::*;
use common::*;
pub fn AArch64_GetS1TLBContext<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    ss: u32,
    va: u64,
    tg: u32,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        regime: u32,
        ss: u32,
        va: u64,
        tg: u32,
    }
    let fn_state = FunctionState {
        regime,
        ss,
        va,
        tg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // D s_0_1: read-var regime:u32
        let s_0_1: u32 = fn_state.regime;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
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
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_1_0: read-var ss:u32
        let s_1_0: u32 = fn_state.ss;
        // D s_1_1: read-var va:u64
        let s_1_1: u64 = fn_state.va;
        // D s_1_2: read-var tg:u32
        let s_1_2: u32 = fn_state.tg;
        // D s_1_3: call AArch64_TLBContextEL3(s_1_0, s_1_1, s_1_2)
        let s_1_3: ProductTypec0d0fb0603850c4c = AArch64_TLBContextEL3(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
        );
        // D s_1_4: write-var tlbcontext <= s_1_3
        fn_state.tlbcontext = s_1_3;
        // N s_1_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // D s_2_1: write-var tlbcontext.4 <= s_2_0
        fn_state.tlbcontext._4 = s_2_0;
        // C s_2_2: const #0u : u8
        let s_2_2: bool = false;
        // D s_2_3: write-var tlbcontext.5 <= s_2_2
        fn_state.tlbcontext._5 = s_2_2;
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // D s_2_5: write-var tlbcontext.3 <= s_2_4
        fn_state.tlbcontext._3 = s_2_4;
        // D s_2_6: read-var tlbcontext:struct
        let s_2_6: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_2_7: return s_2_6
        return s_2_6;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: read-var regime:u32
        let s_3_1: u32 = fn_state.regime;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_4_0: read-var ss:u32
        let s_4_0: u32 = fn_state.ss;
        // D s_4_1: read-var va:u64
        let s_4_1: u64 = fn_state.va;
        // D s_4_2: read-var tg:u32
        let s_4_2: u32 = fn_state.tg;
        // D s_4_3: call AArch64_TLBContextEL2(s_4_0, s_4_1, s_4_2)
        let s_4_3: ProductTypec0d0fb0603850c4c = AArch64_TLBContextEL2(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
        );
        // D s_4_4: write-var tlbcontext <= s_4_3
        fn_state.tlbcontext = s_4_3;
        // N s_4_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_5_0: const #3u : u32
        let s_5_0: u32 = 3;
        // D s_5_1: read-var regime:u32
        let s_5_1: u32 = fn_state.regime;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_6_0: read-var ss:u32
        let s_6_0: u32 = fn_state.ss;
        // D s_6_1: read-var va:u64
        let s_6_1: u64 = fn_state.va;
        // D s_6_2: read-var tg:u32
        let s_6_2: u32 = fn_state.tg;
        // D s_6_3: call AArch64_TLBContextEL20(s_6_0, s_6_1, s_6_2)
        let s_6_3: ProductTypec0d0fb0603850c4c = AArch64_TLBContextEL20(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
        );
        // D s_6_4: write-var tlbcontext <= s_6_3
        fn_state.tlbcontext = s_6_3;
        // N s_6_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_7_0: const #4u : u32
        let s_7_0: u32 = 4;
        // D s_7_1: read-var regime:u32
        let s_7_1: u32 = fn_state.regime;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_8_0: read-var ss:u32
        let s_8_0: u32 = fn_state.ss;
        // D s_8_1: read-var va:u64
        let s_8_1: u64 = fn_state.va;
        // D s_8_2: read-var tg:u32
        let s_8_2: u32 = fn_state.tg;
        // D s_8_3: call AArch64_TLBContextEL10(s_8_0, s_8_1, s_8_2)
        let s_8_3: ProductTypec0d0fb0603850c4c = AArch64_TLBContextEL10(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
        );
        // D s_8_4: write-var tlbcontext <= s_8_3
        fn_state.tlbcontext = s_8_3;
        // N s_8_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // N s_9_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
