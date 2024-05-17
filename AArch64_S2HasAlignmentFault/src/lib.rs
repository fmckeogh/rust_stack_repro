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
use ConstrainUnpredictable::*;
use HaveMTEExt::*;
use common::*;
pub fn AArch64_S2HasAlignmentFault<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
    memattrs: ProductTypef170cab34335b70c,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_18888: bool,
        gs_18886: bool,
        gs_18887: bool,
        gs_18885: bool,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        memattrs: ProductTypef170cab34335b70c,
    }
    let fn_state = FunctionState {
        accdesc,
        aligned,
        memattrs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var accdesc.1:struct
        let s_0_0: u32 = fn_state.accdesc._1;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b19 b1
        if s_0_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveMTEExt(s_1_0)
        let s_1_1: bool = HaveMTEExt(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b18 b2
        if s_1_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#18885 <= s_2_0
        fn_state.gs_18885 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#18885:u8
        let s_3_0: bool = fn_state.gs_18885;
        // N s_3_1: branch s_3_0 b17 b4
        if s_3_0 {
            return block_17(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#18886 <= s_4_0
        fn_state.gs_18886 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#18886:u8
        let s_5_0: bool = fn_state.gs_18886;
        // N s_5_1: branch s_5_0 b13 b6
        if s_5_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var accdesc.1:struct
        let s_6_0: u32 = fn_state.accdesc._1;
        // C s_6_1: const #7u : u32
        let s_6_1: u32 = 7;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b12 b7
        if s_6_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var memattrs.2:struct
        let s_7_0: u32 = fn_state.memattrs._2;
        // C s_7_1: const #1u : u32
        let s_7_1: u32 = 1;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b11 b8
        if s_7_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#18887 <= s_8_0
        fn_state.gs_18887 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#18887:u8
        let s_9_0: bool = fn_state.gs_18887;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var return_value:u8
        let s_10_0: bool = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var aligned:u8
        let s_11_0: bool = fn_state.aligned;
        // D s_11_1: not s_11_0
        let s_11_1: bool = !s_11_0;
        // D s_11_2: write-var gs#18887 <= s_11_1
        fn_state.gs_18887 = s_11_1;
        // N s_11_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var memattrs.2:struct
        let s_12_0: u32 = fn_state.memattrs._2;
        // C s_12_1: const #1u : u32
        let s_12_1: u32 = 1;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: write-var return_value <= s_12_2
        fn_state.return_value = s_12_2;
        // N s_12_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var memattrs.2:struct
        let s_13_0: u32 = fn_state.memattrs._2;
        // C s_13_1: const #1u : u32
        let s_13_1: u32 = 1;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#18888 <= s_14_0
        fn_state.gs_18888 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var gs#18888:u8
        let s_15_0: bool = fn_state.gs_18888;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #18u : u32
        let s_16_0: u32 = 18;
        // S s_16_1: call ConstrainUnpredictable(s_16_0)
        let s_16_1: u32 = ConstrainUnpredictable(state, tracer, s_16_0);
        // C s_16_2: const #12u : u32
        let s_16_2: u32 = 12;
        // S s_16_3: cmp-eq s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) == (s_16_2));
        // D s_16_4: write-var gs#18888 <= s_16_3
        fn_state.gs_18888 = s_16_3;
        // N s_16_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var accdesc.32:struct
        let s_17_0: bool = fn_state.accdesc._32;
        // D s_17_1: write-var gs#18886 <= s_17_0
        fn_state.gs_18886 = s_17_0;
        // N s_17_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var accdesc.27:struct
        let s_18_0: bool = fn_state.accdesc._27;
        // D s_18_1: write-var gs#18885 <= s_18_0
        fn_state.gs_18885 = s_18_0;
        // N s_18_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b10
        return block_10(state, tracer, fn_state);
    }
}
