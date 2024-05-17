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
use neq_int::*;
use execute_aarch64_instrs_memory_ordered_rcpc::*;
use common::*;
pub fn decode_ldaprb_aarch64_instrs_memory_ordered_rcpc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    Rn: u8,
    Rs: u8,
    size: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        t: i64,
        elsize: i64,
        n: i64,
        ga_258860: i64,
        Rt: u8,
        Rn: u8,
        Rs: u8,
        size: u8,
    }
    let fn_state = FunctionState {
        Rt,
        Rn,
        Rs,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rn:u8
        let s_0_0: u8 = fn_state.Rn;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var n <= s_0_3
        fn_state.n = s_0_3;
        // D s_0_5: read-var Rt:u8
        let s_0_5: u8 = fn_state.Rt;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var t <= s_0_8
        fn_state.t = s_0_8;
        // D s_0_10: read-var size:u8
        let s_0_10: u8 = fn_state.size;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // C s_0_14: const #8s : i64
        let s_0_14: i64 = 8;
        // D s_0_15: lsl s_0_14 s_0_13
        let s_0_15: i64 = s_0_14 << s_0_13;
        // D s_0_16: write-var elsize <= s_0_15
        fn_state.elsize = s_0_15;
        // C s_0_17: const #64s : i
        let s_0_17: i128 = 64;
        // D s_0_18: read-var elsize:i64
        let s_0_18: i64 = fn_state.elsize;
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // D s_0_20: cmp-eq s_0_19 s_0_17
        let s_0_20: bool = ((s_0_19) == (s_0_17));
        // N s_0_21: branch s_0_20 b3 b1
        if s_0_20 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#258860 <= s_1_0
        fn_state.ga_258860 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#258860:i64
        let s_2_0: i64 = fn_state.ga_258860;
        // D s_2_1: read-var elsize:i64
        let s_2_1: i64 = fn_state.elsize;
        // C s_2_2: const #31s : i
        let s_2_2: i128 = 31;
        // D s_2_3: read-var n:i64
        let s_2_3: i64 = fn_state.n;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call neq_int(s_2_4, s_2_2)
        let s_2_5: bool = neq_int(state, tracer, s_2_4, s_2_2);
        // D s_2_6: cast zx s_2_1 -> i
        let s_2_6: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: cast zx s_2_0 -> i
        let s_2_8: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #0s : i64
        let s_2_10: i64 = 0;
        // D s_2_11: read-var n:i64
        let s_2_11: i64 = fn_state.n;
        // D s_2_12: read-var t:i64
        let s_2_12: i64 = fn_state.t;
        // C s_2_13: const #0u : u8
        let s_2_13: bool = false;
        // C s_2_14: const #0u : u8
        let s_2_14: bool = false;
        // D s_2_15: call execute_aarch64_instrs_memory_ordered_rcpc(s_2_7, s_2_11, s_2_10, s_2_9, s_2_12, s_2_5, s_2_13, s_2_14)
        let s_2_15: () = execute_aarch64_instrs_memory_ordered_rcpc(
            state,
            tracer,
            s_2_7,
            s_2_11,
            s_2_10,
            s_2_9,
            s_2_12,
            s_2_5,
            s_2_13,
            s_2_14,
        );
        // N s_2_16: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#258860 <= s_3_0
        fn_state.ga_258860 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
