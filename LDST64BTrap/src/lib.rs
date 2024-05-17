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
use ThisInstrAddr::*;
use AArch64_TakeException::*;
use ExceptionSyndrome::*;
use common::*;
pub fn LDST64BTrap<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_el: u8,
    iss: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        target_el: u8,
        iss: u32,
    }
    let fn_state = FunctionState {
        target_el,
        iss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // C s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // S s_0_2: call ThisInstrAddr(s_0_1)
        let s_0_2: Bits = ThisInstrAddr(state, tracer, s_0_1);
        // S s_0_3: cast reint s_0_2 -> u64
        let s_0_3: u64 = (s_0_2.value() as u64);
        // C s_0_4: const #0u : u8
        let s_0_4: u8 = 0;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 4u16);
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (s_0_5.value() as i128);
        // C s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // C s_0_8: const #9u : u32
        let s_0_8: u32 = 9;
        // S s_0_9: call ExceptionSyndrome(s_0_8)
        let s_0_9: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_0_8);
        // C s_0_10: cast zx s_0_7 -> i
        let s_0_10: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_11: read-var target_el:u8
        let s_0_11: u8 = fn_state.target_el;
        // D s_0_12: call AArch64_TakeException(s_0_11, s_0_9, s_0_3, s_0_10)
        let s_0_12: () = AArch64_TakeException(
            state,
            tracer,
            s_0_11,
            s_0_9,
            s_0_3,
            s_0_10,
        );
        // N s_0_13: return
        return;
    }
}
