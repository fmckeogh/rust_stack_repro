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
use u__UNKNOWN_bits::*;
use common::*;
pub fn AArch32_ReportedOuterAttrs<T: Tracer>(
    state: &mut State,
    tracer: &T,
    attrs: u8,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        attrs: u8,
    }
    let fn_state = FunctionState {
        attrs,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #2s : i64
        let s_0_0: i64 = 2;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call __UNKNOWN_bits(s_0_1)
        let s_0_2: Bits = u__UNKNOWN_bits(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u8
        let s_0_3: u8 = (s_0_2.value() as u8);
        // N s_0_4: return s_0_3
        return s_0_3;
    }
}