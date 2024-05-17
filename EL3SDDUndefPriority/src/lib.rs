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
use u_get_EDSCR_Type_SDD::*;
use u__IMPDEF_boolean::*;
use Halted::*;
use EDSCR_read::*;
use common::*;
pub fn EL3SDDUndefPriority<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6722: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6724: bool,
        gs_6723: bool,
        gs_6722: (),
    }
    let fn_state = FunctionState {
        gs_6722,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Halted(s_0_0)
        let s_0_1: bool = Halted(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b6 b1
        if s_0_1 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#6723 <= s_1_0
        fn_state.gs_6723 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#6723:u8
        let s_2_0: bool = fn_state.gs_6723;
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
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#6724 <= s_3_0
        fn_state.gs_6724 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#6724:u8
        let s_4_0: bool = fn_state.gs_6724;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_5_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_5_1: call __IMPDEF_boolean(s_5_0)
        let s_5_1: bool = u__IMPDEF_boolean(state, tracer, s_5_0);
        // D s_5_2: write-var gs#6724 <= s_5_1
        fn_state.gs_6724 = s_5_1;
        // N s_5_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EDSCR_read(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_6_0);
        // S s_6_2: call _get_EDSCR_Type_SDD(s_6_1)
        let s_6_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_6_1);
        // S s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // S s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // D s_6_7: write-var gs#6723 <= s_6_6
        fn_state.gs_6723 = s_6_6;
        // N s_6_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
