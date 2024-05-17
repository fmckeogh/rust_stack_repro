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
use MPAMisEnabled::*;
use u_get_MPAM2_EL2_Type_ALTSP_HFC::*;
use u_get_MPAM3_EL3_Type_ALTSP_HFC::*;
use EL2Enabled::*;
use MPAM3_EL3_read::*;
use u_get_MPAM3_EL3_Type_ALTSP_HEN::*;
use common::*;
pub fn UsePrimarySpaceEL10<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6923: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6926: bool,
        gs_6925: bool,
        return_value: bool,
        gs_6923: (),
    }
    let fn_state = FunctionState {
        gs_6923,
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
        // S s_0_1: call MPAM3_EL3_read(s_0_0)
        let s_0_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_0_0);
        // S s_0_2: call _get_MPAM3_EL3_Type_ALTSP_HEN(s_0_1)
        let s_0_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HEN(state, tracer, s_0_1);
        // S s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // S s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b9 b1
        if s_0_6 {
            return block_9(state, tracer, fn_state);
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
        // S s_1_1: call MPAMisEnabled(s_1_0)
        let s_1_1: bool = MPAMisEnabled(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b8 b2
        if s_1_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call EL2Enabled(s_2_0)
        let s_2_1: bool = EL2Enabled(state, tracer, s_2_0);
        // S s_2_2: not s_2_1
        let s_2_2: bool = !s_2_1;
        // D s_2_3: write-var gs#6925 <= s_2_2
        fn_state.gs_6925 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#6925:u8
        let s_3_0: bool = fn_state.gs_6925;
        // N s_3_1: branch s_3_0 b7 b4
        if s_3_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #90504u : u32
        let s_4_0: u32 = 90504;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_MPAM2_EL2_Type_ALTSP_HFC(s_4_1)
        let s_4_2: bool = u_get_MPAM2_EL2_Type_ALTSP_HFC(state, tracer, s_4_1);
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // C s_4_4: const #0u : u8
        let s_4_4: bool = false;
        // C s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // D s_4_6: cmp-eq s_4_3 s_4_5
        let s_4_6: bool = ((s_4_3) == (s_4_5));
        // D s_4_7: write-var gs#6926 <= s_4_6
        fn_state.gs_6926 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#6926:u8
        let s_5_0: bool = fn_state.gs_6926;
        // D s_5_1: write-var return_value <= s_5_0
        fn_state.return_value = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var return_value:u8
        let s_6_0: bool = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var gs#6926 <= s_7_0
        fn_state.gs_6926 = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#6925 <= s_8_0
        fn_state.gs_6925 = s_8_0;
        // N s_8_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call MPAM3_EL3_read(s_9_0)
        let s_9_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_9_0);
        // S s_9_2: call _get_MPAM3_EL3_Type_ALTSP_HFC(s_9_1)
        let s_9_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HFC(state, tracer, s_9_1);
        // S s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #0u : u8
        let s_9_4: bool = false;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // S s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var return_value <= s_9_6
        fn_state.return_value = s_9_6;
        // N s_9_8: jump b6
        return block_6(state, tracer, fn_state);
    }
}
