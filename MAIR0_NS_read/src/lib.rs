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
use ELUsingAArch32::*;
use PRRR_read::*;
use PRRR_NS_read::*;
use common::*;
pub fn MAIR0_NS_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_27469: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        gs_27471: bool,
        gs_27469: (),
    }
    let fn_state = FunctionState {
        gs_27469,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #11000u : u32
        let s_0_0: u32 = 11000;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var r <= s_0_1
        fn_state.r = s_0_1;
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call ELUsingAArch32(s_0_4)
        let s_0_5: bool = ELUsingAArch32(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b8 b1
        if s_0_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_1_0: const #424u : u32
        let s_1_0: u32 = 424;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // N s_1_5: branch s_1_4 b7 b2
        if s_1_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_2_0: const #424u : u32
        let s_2_0: u32 = 424;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call ELUsingAArch32(s_2_1)
        let s_2_2: bool = ELUsingAArch32(state, tracer, s_2_1);
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // D s_2_4: write-var gs#27471 <= s_2_3
        fn_state.gs_27471 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_3_0: read-var gs#27471:u8
        let s_3_0: bool = fn_state.gs_27471;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call PRRR_read(s_4_0)
        let s_4_1: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_4_0);
        // D s_4_2: read-var r:struct
        let s_4_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_4_3: write-var r <= s_4_2
        fn_state.r = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_5_0: read-var r:struct
        let s_5_0: ProductType700c18a878c5601b = fn_state.r;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_6_0: read-var r:struct
        let s_6_0: ProductType700c18a878c5601b = fn_state.r;
        // D s_6_1: write-var r <= s_6_0
        fn_state.r = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#27471 <= s_7_0
        fn_state.gs_27471 = s_7_0;
        // N s_7_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call PRRR_NS_read(s_8_0)
        let s_8_1: ProductType700c18a878c5601b = PRRR_NS_read(state, tracer, s_8_0);
        // D s_8_2: read-var r:struct
        let s_8_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_8_3: write-var r <= s_8_2
        fn_state.r = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
}
