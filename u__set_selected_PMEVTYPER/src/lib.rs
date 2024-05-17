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
use PMEVTYPER_set::*;
use u_get_PMSELR_Type_SEL::*;
use PMSELR_read::*;
use common::*;
pub fn u__set_selected_PMEVTYPER<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: ProductType700c18a878c5601b,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        value_name: ProductType700c18a878c5601b,
    }
    let fn_state = FunctionState {
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call PMSELR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = PMSELR_read(state, tracer, s_0_0);
        // S s_0_2: call _get_PMSELR_Type_SEL(s_0_1)
        let s_0_2: u8 = u_get_PMSELR_Type_SEL(state, tracer, s_0_1);
        // S s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 5u16);
        // S s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
        // S s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // C s_0_6: const #30s : i
        let s_0_6: i128 = 30;
        // S s_0_7: cast zx s_0_5 -> i
        let s_0_7: i128 = (i128::try_from(s_0_5).unwrap());
        // S s_0_8: cmp-le s_0_7 s_0_6
        let s_0_8: bool = ((s_0_7) <= (s_0_6));
        // N s_0_9: assert s_0_8
        let s_0_9: () = assert!(s_0_8);
        // D s_0_10: read-var value_name:struct
        let s_0_10: ProductType700c18a878c5601b = fn_state.value_name;
        // D s_0_11: call PMEVTYPER_set(s_0_5, s_0_10)
        let s_0_11: () = PMEVTYPER_set(state, tracer, s_0_5, s_0_10);
        // N s_0_12: return
        return;
    }
}
