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
use u__IMPDEF_boolean::*;
use NewAccDesc::*;
use InStreamingMode::*;
use HaveTME::*;
use common::*;
pub fn CreateAccDescSVENF<T: Tracer>(
    state: &mut State,
    tracer: &T,
    contiguous: bool,
    tagchecked: bool,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_7357: bool,
        gs_7355: bool,
        contiguous: bool,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        contiguous,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_0_0: const #3u : u32
        let s_0_0: u32 = 3;
        // S s_0_1: call NewAccDesc(s_0_0)
        let s_0_1: ProductType9878976b5bcce9c9 = NewAccDesc(state, tracer, s_0_0);
        // D s_0_2: write-var accdesc <= s_0_1
        fn_state.accdesc = s_0_1;
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // D s_0_4: write-var accdesc.23 <= s_0_3
        fn_state.accdesc._23 = s_0_3;
        // C s_0_5: const #1u : u8
        let s_0_5: bool = true;
        // D s_0_6: write-var accdesc.20 <= s_0_5
        fn_state.accdesc._20 = s_0_5;
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // D s_0_8: write-var accdesc.17 <= s_0_7
        fn_state.accdesc._17 = s_0_7;
        // D s_0_9: read-var contiguous:u8
        let s_0_9: bool = fn_state.contiguous;
        // D s_0_10: write-var accdesc.7 <= s_0_9
        fn_state.accdesc._7 = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call InStreamingMode(s_0_11)
        let s_0_12: bool = InStreamingMode(state, tracer, s_0_11);
        // D s_0_13: write-var accdesc.26 <= s_0_12
        fn_state.accdesc._26 = s_0_12;
        // D s_0_14: read-var accdesc.26:struct
        let s_0_14: bool = fn_state.accdesc._26;
        // N s_0_15: branch s_0_14 b9 b1
        if s_0_14 {
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
        // D s_1_1: write-var gs#7355 <= s_1_0
        fn_state.gs_7355 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_2_0: read-var gs#7355:u8
        let s_2_0: bool = fn_state.gs_7355;
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
    ) -> ProductType9878976b5bcce9c9 {
        // D s_3_0: read-var tagchecked:u8
        let s_3_0: bool = fn_state.tagchecked;
        // D s_3_1: write-var accdesc.28 <= s_3_0
        fn_state.accdesc._28 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveTME(s_4_0)
        let s_4_1: bool = HaveTME(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b7 b5
        if s_4_1 {
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
        // D s_5_1: write-var gs#7357 <= s_5_0
        fn_state.gs_7357 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_6_0: read-var gs#7357:u8
        let s_6_0: bool = fn_state.gs_7357;
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
        // D s_7_4: write-var gs#7357 <= s_7_3
        fn_state.gs_7357 = s_7_3;
        // N s_7_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var accdesc.28 <= s_8_0
        fn_state.accdesc._28 = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_9_0: const #"No tag checking of SIMD&FP loads and stores in Streaming SVE mode" : str
        let s_9_0: &'static str = "No tag checking of SIMD&FP loads and stores in Streaming SVE mode";
        // S s_9_1: call __IMPDEF_boolean(s_9_0)
        let s_9_1: bool = u__IMPDEF_boolean(state, tracer, s_9_0);
        // D s_9_2: write-var gs#7355 <= s_9_1
        fn_state.gs_7355 = s_9_1;
        // N s_9_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
