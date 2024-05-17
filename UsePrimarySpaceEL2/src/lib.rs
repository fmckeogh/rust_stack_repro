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
use u_get_MPAM3_EL3_Type_ALTSP_HFC::*;
use u_get_MPAM2_EL2_Type_ALTSP_EL2::*;
use MPAM3_EL3_read::*;
use u_get_MPAM3_EL3_Type_ALTSP_HEN::*;
use common::*;
pub fn UsePrimarySpaceEL2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6927: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_6929: bool,
        gs_6927: (),
    }
    let fn_state = FunctionState {
        gs_6927,
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
        // N s_0_7: branch s_0_6 b6 b1
        if s_0_6 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call MPAMisEnabled(s_1_0)
        let s_1_1: bool = MPAMisEnabled(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
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
        // C s_2_0: const #90504u : u32
        let s_2_0: u32 = 90504;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_MPAM2_EL2_Type_ALTSP_EL2(s_2_1)
        let s_2_2: bool = u_get_MPAM2_EL2_Type_ALTSP_EL2(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // D s_2_7: write-var gs#6929 <= s_2_6
        fn_state.gs_6929 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#6929:u8
        let s_3_0: bool = fn_state.gs_6929;
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
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var gs#6929 <= s_5_0
        fn_state.gs_6929 = s_5_0;
        // N s_5_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call MPAM3_EL3_read(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = MPAM3_EL3_read(state, tracer, s_6_0);
        // S s_6_2: call _get_MPAM3_EL3_Type_ALTSP_HFC(s_6_1)
        let s_6_2: bool = u_get_MPAM3_EL3_Type_ALTSP_HFC(state, tracer, s_6_1);
        // S s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // S s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // D s_6_7: write-var return_value <= s_6_6
        fn_state.return_value = s_6_6;
        // N s_6_8: jump b4
        return block_4(state, tracer, fn_state);
    }
}
