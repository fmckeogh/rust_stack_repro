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
use NewAccDesc::*;
use HaveTME::*;
use common::*;
pub fn CreateAccDescExLDST<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memop: u32,
    acqrel: bool,
    tagchecked: bool,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_7194: bool,
        gs_7197: bool,
        gs_7195: bool,
        memop: u32,
        acqrel: bool,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        memop,
        acqrel,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_0_0: const #1u : u32
        let s_0_0: u32 = 1;
        // S s_0_1: call NewAccDesc(s_0_0)
        let s_0_1: ProductType9878976b5bcce9c9 = NewAccDesc(state, tracer, s_0_0);
        // D s_0_2: write-var accdesc <= s_0_1
        fn_state.accdesc = s_0_1;
        // D s_0_3: read-var acqrel:u8
        let s_0_3: bool = fn_state.acqrel;
        // N s_0_4: branch s_0_3 b9 b1
        if s_0_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#7194 <= s_1_0
        fn_state.gs_7194 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_2_0: read-var gs#7194:u8
        let s_2_0: bool = fn_state.gs_7194;
        // D s_2_1: write-var accdesc.3 <= s_2_0
        fn_state.accdesc._3 = s_2_0;
        // D s_2_2: read-var acqrel:u8
        let s_2_2: bool = fn_state.acqrel;
        // N s_2_3: branch s_2_2 b8 b3
        if s_2_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#7195 <= s_3_0
        fn_state.gs_7195 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_4_0: read-var gs#7195:u8
        let s_4_0: bool = fn_state.gs_7195;
        // D s_4_1: write-var accdesc.24 <= s_4_0
        fn_state.accdesc._24 = s_4_0;
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // D s_4_3: write-var accdesc.9 <= s_4_2
        fn_state.accdesc._9 = s_4_2;
        // D s_4_4: read-var memop:u32
        let s_4_4: u32 = fn_state.memop;
        // C s_4_5: const #0u : u32
        let s_4_5: u32 = 0;
        // D s_4_6: cmp-eq s_4_4 s_4_5
        let s_4_6: bool = ((s_4_4) == (s_4_5));
        // D s_4_7: write-var accdesc.23 <= s_4_6
        fn_state.accdesc._23 = s_4_6;
        // D s_4_8: read-var memop:u32
        let s_4_8: u32 = fn_state.memop;
        // C s_4_9: const #1u : u32
        let s_4_9: u32 = 1;
        // D s_4_10: cmp-eq s_4_8 s_4_9
        let s_4_10: bool = ((s_4_8) == (s_4_9));
        // D s_4_11: write-var accdesc.32 <= s_4_10
        fn_state.accdesc._32 = s_4_10;
        // C s_4_12: const #1u : u8
        let s_4_12: bool = true;
        // D s_4_13: write-var accdesc.20 <= s_4_12
        fn_state.accdesc._20 = s_4_12;
        // D s_4_14: read-var tagchecked:u8
        let s_4_14: bool = fn_state.tagchecked;
        // D s_4_15: write-var accdesc.28 <= s_4_14
        fn_state.accdesc._28 = s_4_14;
        // C s_4_16: const #() : ()
        let s_4_16: () = ();
        // S s_4_17: call HaveTME(s_4_16)
        let s_4_17: bool = HaveTME(state, tracer, s_4_16);
        // N s_4_18: branch s_4_17 b7 b5
        if s_4_17 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#7197 <= s_5_0
        fn_state.gs_7197 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_6_0: read-var gs#7197:u8
        let s_6_0: bool = fn_state.gs_7197;
        // D s_6_1: write-var accdesc.30 <= s_6_0
        fn_state.accdesc._30 = s_6_0;
        // D s_6_2: read-var accdesc:struct
        let s_6_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // N s_6_3: return s_6_2
        return s_6_2;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_7_0: const #100180u : u32
        let s_7_0: u32 = 100180;
        // D s_7_1: read-reg s_7_0:i
        let s_7_1: i128 = {
            let value = state.read_register::<i128>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #0s : i
        let s_7_2: i128 = 0;
        // D s_7_3: cmp-gt s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) > (s_7_2));
        // D s_7_4: write-var gs#7197 <= s_7_3
        fn_state.gs_7197 = s_7_3;
        // N s_7_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_8_0: read-var memop:u32
        let s_8_0: u32 = fn_state.memop;
        // C s_8_1: const #1u : u32
        let s_8_1: u32 = 1;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: write-var gs#7195 <= s_8_2
        fn_state.gs_7195 = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_9_0: read-var memop:u32
        let s_9_0: u32 = fn_state.memop;
        // C s_9_1: const #0u : u32
        let s_9_1: u32 = 0;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: write-var gs#7194 <= s_9_2
        fn_state.gs_7194 = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
