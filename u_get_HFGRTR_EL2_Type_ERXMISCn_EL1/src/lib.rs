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
use common::*;
pub fn u_get_HFGRTR_EL2_Type_ERXMISCn_EL1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    v: ProductType5c790c8ef59cc8b2,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        v: ProductType5c790c8ef59cc8b2,
    }
    let fn_state = FunctionState {
        v,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var v.0:struct
        let s_0_0: u64 = fn_state.v._0;
        // C s_0_1: const #45s : i
        let s_0_1: i128 = 45;
        // D s_0_2: cast zx s_0_0 -> bv
        let s_0_2: Bits = Bits::new(s_0_0 as u128, 64u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #0s : i
        let s_0_5: i128 = 0;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_1 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_1)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: bool = ((s_0_7.value()) != 0);
        // N s_0_9: return s_0_8
        return s_0_8;
    }
}
