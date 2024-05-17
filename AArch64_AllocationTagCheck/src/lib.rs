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
use PhysMemTagRead::*;
use IsFault__2::*;
use HandleExternalReadAbort::*;
use common::*;
pub fn AArch64_AllocationTagCheck<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memaddrdesc: ProductTypece7c66ccb2cab13e,
    accdesc: ProductType9878976b5bcce9c9,
    ptag: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        readtag: u8,
        memstatus: ProductTypef8c3639b88223255,
        ga_11811: ProductType2743ddd4af418639,
        ga_11808: ProductTypef170cab34335b70c,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        accdesc: ProductType9878976b5bcce9c9,
        ptag: u8,
    }
    let fn_state = FunctionState {
        memaddrdesc,
        accdesc,
        ptag,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var memaddrdesc.2:struct
        let s_0_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_0_1: write-var ga#11808 <= s_0_0
        fn_state.ga_11808 = s_0_0;
        // D s_0_2: read-var ga#11808.6:struct
        let s_0_2: u32 = fn_state.ga_11808._6;
        // C s_0_3: const #1u : u32
        let s_0_3: u32 = 1;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
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
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var memaddrdesc:struct
        let s_3_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_3_1: read-var accdesc:struct
        let s_3_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_3_2: call PhysMemTagRead(s_3_0, s_3_1)
        let s_3_2: ProductType2743ddd4af418639 = PhysMemTagRead(
            state,
            tracer,
            s_3_0,
            s_3_1,
        );
        // D s_3_3: write-var ga#11811 <= s_3_2
        fn_state.ga_11811 = s_3_2;
        // D s_3_4: read-var ga#11811.0:struct
        let s_3_4: ProductTypef8c3639b88223255 = fn_state.ga_11811._0;
        // D s_3_5: read-var ga#11811.1:struct
        let s_3_5: u8 = fn_state.ga_11811._1;
        // D s_3_6: write-var memstatus <= s_3_4
        fn_state.memstatus = s_3_4;
        // D s_3_7: write-var readtag <= s_3_5
        fn_state.readtag = s_3_5;
        // D s_3_8: read-var memstatus:struct
        let s_3_8: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_3_9: call IsFault__2(s_3_8)
        let s_3_9: bool = IsFault__2(state, tracer, s_3_8);
        // N s_3_10: branch s_3_9 b6 b4
        if s_3_9 {
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
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var ptag:u8
        let s_5_0: u8 = fn_state.ptag;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // D s_5_2: read-var readtag:u8
        let s_5_2: u8 = fn_state.readtag;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: write-var return_value <= s_5_4
        fn_state.return_value = s_5_4;
        // N s_5_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #1s : i
        let s_6_0: i128 = 1;
        // D s_6_1: read-var memstatus:struct
        let s_6_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_6_2: read-var memaddrdesc:struct
        let s_6_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_6_3: read-var accdesc:struct
        let s_6_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_6_4: call HandleExternalReadAbort(s_6_1, s_6_2, s_6_0, s_6_3)
        let s_6_4: () = HandleExternalReadAbort(
            state,
            tracer,
            s_6_1,
            s_6_2,
            s_6_0,
            s_6_3,
        );
        // N s_6_5: jump b5
        return block_5(state, tracer, fn_state);
    }
}
