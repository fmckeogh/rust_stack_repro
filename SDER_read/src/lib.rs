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
use SDER32_EL3_read::*;
use common::*;
pub fn SDER_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2458: (),
) -> ProductType700c18a878c5601b {
    #[derive(Default)]
    struct FunctionState {
        r: ProductType700c18a878c5601b,
        gs_2458: (),
    }
    let fn_state = FunctionState {
        gs_2458,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_0_0: const #102328u : u32
        let s_0_0: u32 = 102328;
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
        // C s_0_5: const #2u : u8
        let s_0_5: u8 = 2;
        // D s_0_6: cmp-lt s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) < (s_0_5));
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_1_0: read-var r:struct
        let s_1_0: ProductType700c18a878c5601b = fn_state.r;
        // D s_1_1: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // D s_2_0: read-var r:struct
        let s_2_0: ProductType700c18a878c5601b = fn_state.r;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType700c18a878c5601b {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call SDER32_EL3_read(s_3_0)
        let s_3_1: ProductType5c790c8ef59cc8b2 = SDER32_EL3_read(state, tracer, s_3_0);
        // D s_3_2: read-var r:struct
        let s_3_2: ProductType700c18a878c5601b = fn_state.r;
        // D s_3_3: write-var r <= s_3_2
        fn_state.r = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
