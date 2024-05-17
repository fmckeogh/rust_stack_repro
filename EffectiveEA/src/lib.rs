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
use HaveAArch64::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_SCR_Type_EA::*;
use Halted::*;
use u_get_SCR_EL3_Type_EA::*;
use EDSCR_read::*;
use common::*;
pub fn EffectiveEA<T: Tracer>(state: &mut State, tracer: &T, gs_6720: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_4523: bool,
        gs_6721: bool,
        return_value: bool,
        gs_6720: (),
    }
    let fn_state = FunctionState {
        gs_6720,
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
        // N s_0_2: branch s_0_1 b9 b1
        if s_0_1 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#6721 <= s_1_0
        fn_state.gs_6721 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#6721:u8
        let s_2_0: bool = fn_state.gs_6721;
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
    ) -> bool {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveAArch64(s_3_0)
        let s_3_1: bool = HaveAArch64(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b7 b4
        if s_3_1 {
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
        // C s_4_0: const #20920u : u32
        let s_4_0: u32 = 20920;
        // D s_4_1: read-reg s_4_0:struct
        let s_4_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: call _get_SCR_Type_EA(s_4_1)
        let s_4_2: bool = u_get_SCR_Type_EA(state, tracer, s_4_1);
        // D s_4_3: write-var ga#4523 <= s_4_2
        fn_state.ga_4523 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var ga#4523:u8
        let s_5_0: bool = fn_state.ga_4523;
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
        // C s_7_0: const #90704u : u32
        let s_7_0: u32 = 90704;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_SCR_EL3_Type_EA(s_7_1)
        let s_7_2: bool = u_get_SCR_EL3_Type_EA(state, tracer, s_7_1);
        // D s_7_3: write-var ga#4523 <= s_7_2
        fn_state.ga_4523 = s_7_2;
        // N s_7_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EDSCR_read(s_9_0)
        let s_9_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_9_0);
        // S s_9_2: call _get_EDSCR_Type_SDD(s_9_1)
        let s_9_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_9_1);
        // S s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #0u : u8
        let s_9_4: bool = false;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // S s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // D s_9_7: write-var gs#6721 <= s_9_6
        fn_state.gs_6721 = s_9_6;
        // N s_9_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
