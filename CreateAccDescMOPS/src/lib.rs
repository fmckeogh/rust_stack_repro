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
pub fn CreateAccDescMOPS<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memop: u32,
    privileged: bool,
    nontemporal: bool,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_7214: bool,
        memop: u32,
        privileged: bool,
        nontemporal: bool,
    }
    let fn_state = FunctionState {
        memop,
        privileged,
        nontemporal,
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
        // D s_0_3: read-var privileged:u8
        let s_0_3: bool = fn_state.privileged;
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b6 b1
        if s_0_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var accdesc.8 <= s_1_1
        fn_state.accdesc._8 = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_2_0: read-var nontemporal:u8
        let s_2_0: bool = fn_state.nontemporal;
        // D s_2_1: write-var accdesc.18 <= s_2_0
        fn_state.accdesc._18 = s_2_0;
        // D s_2_2: read-var memop:u32
        let s_2_2: u32 = fn_state.memop;
        // C s_2_3: const #0u : u32
        let s_2_3: u32 = 0;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // D s_2_5: write-var accdesc.23 <= s_2_4
        fn_state.accdesc._23 = s_2_4;
        // D s_2_6: read-var memop:u32
        let s_2_6: u32 = fn_state.memop;
        // C s_2_7: const #1u : u32
        let s_2_7: u32 = 1;
        // D s_2_8: cmp-eq s_2_6 s_2_7
        let s_2_8: bool = ((s_2_6) == (s_2_7));
        // D s_2_9: write-var accdesc.32 <= s_2_8
        fn_state.accdesc._32 = s_2_8;
        // C s_2_10: const #1u : u8
        let s_2_10: bool = true;
        // D s_2_11: write-var accdesc.20 <= s_2_10
        fn_state.accdesc._20 = s_2_10;
        // C s_2_12: const #1u : u8
        let s_2_12: bool = true;
        // D s_2_13: write-var accdesc.15 <= s_2_12
        fn_state.accdesc._15 = s_2_12;
        // C s_2_14: const #1u : u8
        let s_2_14: bool = true;
        // D s_2_15: write-var accdesc.28 <= s_2_14
        fn_state.accdesc._28 = s_2_14;
        // C s_2_16: const #() : ()
        let s_2_16: () = ();
        // S s_2_17: call HaveTME(s_2_16)
        let s_2_17: bool = HaveTME(state, tracer, s_2_16);
        // N s_2_18: branch s_2_17 b5 b3
        if s_2_17 {
            return block_5(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#7214 <= s_3_0
        fn_state.gs_7214 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_4_0: read-var gs#7214:u8
        let s_4_0: bool = fn_state.gs_7214;
        // D s_4_1: write-var accdesc.30 <= s_4_0
        fn_state.accdesc._30 = s_4_0;
        // D s_4_2: read-var accdesc:struct
        let s_4_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // N s_4_3: return s_4_2
        return s_4_2;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_5_0: const #100180u : u32
        let s_5_0: u32 = 100180;
        // D s_5_1: read-reg s_5_0:i
        let s_5_1: i128 = {
            let value = state.read_register::<i128>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: cmp-gt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) > (s_5_2));
        // D s_5_4: write-var gs#7214 <= s_5_3
        fn_state.gs_7214 = s_5_3;
        // N s_5_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_6_0: const #448u : u32
        let s_6_0: u32 = 448;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: write-var accdesc.8 <= s_6_1
        fn_state.accdesc._8 = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
