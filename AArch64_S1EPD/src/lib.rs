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
use u_get_TCR_EL2_Type_EPD0::*;
use u_get_TCR_EL1_Type_EPD0::*;
use AArch64_GetVARange::*;
use u_get_TCR_EL1_Type_EPD1::*;
use u_get_TCR_EL2_Type_EPD1::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1EPD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    va: u64,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        ga_13213: bool,
        ga_13211: bool,
        ga_13214: bool,
        varange: u32,
        regime: u32,
        va: u64,
    }
    let fn_state = FunctionState {
        regime,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // D s_0_1: call HasUnprivileged(s_0_0)
        let s_0_1: bool = HasUnprivileged(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // D s_0_3: read-var va:u64
        let s_0_3: u64 = fn_state.va;
        // D s_0_4: call AArch64_GetVARange(s_0_3)
        let s_0_4: u32 = AArch64_GetVARange(state, tracer, s_0_3);
        // D s_0_5: write-var varange <= s_0_4
        fn_state.varange = s_0_4;
        // C s_0_6: const #3u : u32
        let s_0_6: u32 = 3;
        // D s_0_7: read-var regime:u32
        let s_0_7: u32 = fn_state.regime;
        // D s_0_8: cmp-eq s_0_6 s_0_7
        let s_0_8: bool = ((s_0_6) == (s_0_7));
        // D s_0_9: not s_0_8
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
        // D s_1_0: read-var varange:u32
        let s_1_0: u32 = fn_state.varange;
        // C s_1_1: const #0u : u32
        let s_1_1: u32 = 0;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b5 b2
        if s_1_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #12816u : u32
        let s_2_0: u32 = 12816;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_TCR_EL2_Type_EPD1(s_2_1)
        let s_2_2: bool = u_get_TCR_EL2_Type_EPD1(state, tracer, s_2_1);
        // D s_2_3: write-var ga#13211 <= s_2_2
        fn_state.ga_13211 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var ga#13211:u8
        let s_3_0: bool = fn_state.ga_13211;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var return_value:u8
        let s_4_0: bool = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #12816u : u32
        let s_5_0: u32 = 12816;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_TCR_EL2_Type_EPD0(s_5_1)
        let s_5_2: bool = u_get_TCR_EL2_Type_EPD0(state, tracer, s_5_1);
        // D s_5_3: write-var ga#13211 <= s_5_2
        fn_state.ga_13211 = s_5_2;
        // N s_5_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #4u : u32
        let s_6_0: u32 = 4;
        // D s_6_1: read-var regime:u32
        let s_6_1: u32 = fn_state.regime;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b11 b7
        if s_6_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var varange:u32
        let s_7_0: u32 = fn_state.varange;
        // C s_7_1: const #0u : u32
        let s_7_1: u32 = 0;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #22392u : u32
        let s_8_0: u32 = 22392;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_TCR_EL1_Type_EPD1(s_8_1)
        let s_8_2: bool = u_get_TCR_EL1_Type_EPD1(state, tracer, s_8_1);
        // D s_8_3: write-var ga#13213 <= s_8_2
        fn_state.ga_13213 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var ga#13213:u8
        let s_9_0: bool = fn_state.ga_13213;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #22392u : u32
        let s_10_0: u32 = 22392;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_TCR_EL1_Type_EPD0(s_10_1)
        let s_10_2: bool = u_get_TCR_EL1_Type_EPD0(state, tracer, s_10_1);
        // D s_10_3: write-var ga#13213 <= s_10_2
        fn_state.ga_13213 = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var ga#13214:u8
        let s_11_0: bool = fn_state.ga_13214;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
