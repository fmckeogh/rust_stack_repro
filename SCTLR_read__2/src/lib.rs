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
use Bit::*;
use CurrentSecurityState::*;
use SCTLR_NS_read::*;
use ELUsingAArch32::*;
use common::*;
pub fn SCTLR_read__2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_5490: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        gs_5491: bool,
        gs_5490: (),
    }
    let fn_state = FunctionState {
        gs_5490,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #424u : u32
        let s_0_0: u32 = 424;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: branch s_0_2 b6 b1
        if s_0_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#5491 <= s_1_0
        fn_state.gs_5491 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_2_0: read-var gs#5491:u8
        let s_2_0: bool = fn_state.gs_5491;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call SCTLR_NS_read(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_3_0);
        // D s_3_2: write-var r <= s_3_1
        fn_state.r = s_3_1;
        // C s_3_3: const #1u : u8
        let s_3_3: bool = true;
        // S s_3_4: call Bit(s_3_3)
        let s_3_4: bool = Bit(state, tracer, s_3_3);
        // D s_3_5: read-var r:struct
        let s_3_5: ProductType700c18a878c5601b = fn_state.r;
        // D s_3_6: write-var r <= s_3_5
        fn_state.r = s_3_5;
        // C s_3_7: const #1u : u8
        let s_3_7: bool = true;
        // S s_3_8: call Bit(s_3_7)
        let s_3_8: bool = Bit(state, tracer, s_3_7);
        // D s_3_9: read-var r:struct
        let s_3_9: ProductType700c18a878c5601b = fn_state.r;
        // D s_3_10: write-var r <= s_3_9
        fn_state.r = s_3_9;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_4_0: read-var r:struct
        let s_4_0: ProductType700c18a878c5601b = fn_state.r;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_5_0: const #16456u : u32
        let s_5_0: u32 = 16456;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: write-var r <= s_5_1
        fn_state.r = s_5_1;
        // C s_5_3: const #1u : u8
        let s_5_3: bool = true;
        // S s_5_4: call Bit(s_5_3)
        let s_5_4: bool = Bit(state, tracer, s_5_3);
        // D s_5_5: read-var r:struct
        let s_5_5: ProductType700c18a878c5601b = fn_state.r;
        // D s_5_6: write-var r <= s_5_5
        fn_state.r = s_5_5;
        // C s_5_7: const #1u : u8
        let s_5_7: bool = true;
        // S s_5_8: call Bit(s_5_7)
        let s_5_8: bool = Bit(state, tracer, s_5_7);
        // D s_5_9: read-var r:struct
        let s_5_9: ProductType700c18a878c5601b = fn_state.r;
        // D s_5_10: write-var r <= s_5_9
        fn_state.r = s_5_9;
        // N s_5_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CurrentSecurityState(s_6_0)
        let s_6_1: u32 = CurrentSecurityState(state, tracer, s_6_0);
        // C s_6_2: const #3u : u32
        let s_6_2: u32 = 3;
        // S s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#5491 <= s_6_3
        fn_state.gs_5491 = s_6_3;
        // N s_6_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
