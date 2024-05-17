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
use u__UNKNOWN_PASpace::*;
use Zeros::*;
use common::*;
pub fn ExceptionSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    exceptype: u32,
) -> ProductTypeb7f99f96751e17c4 {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypeb7f99f96751e17c4,
        exceptype: u32,
    }
    let fn_state = FunctionState {
        exceptype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_0_0: read-var exceptype:u32
        let s_0_0: u32 = fn_state.exceptype;
        // D s_0_1: write-var r.1 <= s_0_0
        fn_state.r._1 = s_0_0;
        // C s_0_2: const #25s : i
        let s_0_2: i128 = 25;
        // S s_0_3: call Zeros(s_0_2)
        let s_0_3: Bits = Zeros(state, tracer, s_0_2);
        // S s_0_4: cast reint s_0_3 -> u25
        let s_0_4: u32 = (s_0_3.value() as u32);
        // D s_0_5: write-var r.6 <= s_0_4
        fn_state.r._6 = s_0_4;
        // C s_0_6: const #24s : i
        let s_0_6: i128 = 24;
        // S s_0_7: call Zeros(s_0_6)
        let s_0_7: Bits = Zeros(state, tracer, s_0_6);
        // S s_0_8: cast reint s_0_7 -> u24
        let s_0_8: u32 = (s_0_7.value() as u32);
        // D s_0_9: write-var r.7 <= s_0_8
        fn_state.r._7 = s_0_8;
        // C s_0_10: const #64s : i
        let s_0_10: i128 = 64;
        // S s_0_11: call Zeros(s_0_10)
        let s_0_11: Bits = Zeros(state, tracer, s_0_10);
        // S s_0_12: cast reint s_0_11 -> u64
        let s_0_12: u64 = (s_0_11.value() as u64);
        // D s_0_13: write-var r.9 <= s_0_12
        fn_state.r._9 = s_0_12;
        // C s_0_14: const #0u : u8
        let s_0_14: bool = false;
        // D s_0_15: write-var r.3 <= s_0_14
        fn_state.r._3 = s_0_14;
        // C s_0_16: const #0u : u8
        let s_0_16: bool = false;
        // D s_0_17: write-var r.0 <= s_0_16
        fn_state.r._0 = s_0_16;
        // C s_0_18: const #56s : i
        let s_0_18: i128 = 56;
        // S s_0_19: call Zeros(s_0_18)
        let s_0_19: Bits = Zeros(state, tracer, s_0_18);
        // S s_0_20: cast reint s_0_19 -> u56
        let s_0_20: u64 = (s_0_19.value() as u64);
        // D s_0_21: write-var r.2 <= s_0_20
        fn_state.r._2 = s_0_20;
        // C s_0_22: const #() : ()
        let s_0_22: () = ();
        // S s_0_23: call __UNKNOWN_PASpace(s_0_22)
        let s_0_23: u32 = u__UNKNOWN_PASpace(state, tracer, s_0_22);
        // D s_0_24: write-var r.4.1 <= s_0_23
        fn_state.r._4._1 = s_0_23;
        // C s_0_25: const #56s : i64
        let s_0_25: i64 = 56;
        // C s_0_26: cast zx s_0_25 -> i
        let s_0_26: i128 = (i128::try_from(s_0_25).unwrap());
        // S s_0_27: call __UNKNOWN_bits(s_0_26)
        let s_0_27: Bits = u__UNKNOWN_bits(state, tracer, s_0_26);
        // S s_0_28: cast reint s_0_27 -> u56
        let s_0_28: u64 = (s_0_27.value() as u64);
        // D s_0_29: write-var r.4.0 <= s_0_28
        fn_state.r._4._0 = s_0_28;
        // C s_0_30: const #0u : u8
        let s_0_30: bool = false;
        // D s_0_31: write-var r.8 <= s_0_30
        fn_state.r._8 = s_0_30;
        // D s_0_32: read-var r:struct
        let s_0_32: ProductTypeb7f99f96751e17c4 = fn_state.r;
        // N s_0_33: return s_0_32
        return s_0_32;
    }
}
