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
use V_set::*;
use V_read::*;
use u__id::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_vector_extract<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    m: i64,
    n: i64,
    position: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_148110: bool,
        concat: Bits,
        datasizeshadow_1187: i64,
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
        position: i128,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        m,
        n,
        position,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1187 <= s_0_2
        fn_state.datasizeshadow_1187 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1187:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1187;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: read-var datasizeshadow#1187:i64
        let s_1_6: i64 = fn_state.datasizeshadow_1187;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var n:i64
        let s_1_9: i64 = fn_state.n;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: call V_read(s_1_10, s_1_8)
        let s_1_11: Bits = V_read(state, tracer, s_1_10, s_1_8);
        // D s_1_12: cast reint s_1_5 -> u128
        let s_1_12: u128 = (s_1_5.value() as u128);
        // D s_1_13: size-of s_1_5
        let s_1_13: u16 = s_1_5.length();
        // D s_1_14: cast reint s_1_11 -> u128
        let s_1_14: u128 = (s_1_11.value() as u128);
        // D s_1_15: size-of s_1_11
        let s_1_15: u16 = s_1_11.length();
        // D s_1_16: lsl s_1_12 s_1_15
        let s_1_16: u128 = s_1_12 << s_1_15;
        // D s_1_17: or s_1_16 s_1_14
        let s_1_17: u128 = ((s_1_16) | (s_1_14));
        // D s_1_18: add s_1_13 s_1_15
        let s_1_18: u16 = (s_1_13 + s_1_15);
        // D s_1_19: create-bits s_1_17 s_1_18
        let s_1_19: Bits = Bits::new(s_1_17, s_1_18);
        // D s_1_20: write-var concat <= s_1_19
        fn_state.concat = s_1_19;
        // D s_1_21: read-var position:i
        let s_1_21: i128 = fn_state.position;
        // D s_1_22: call __id(s_1_21)
        let s_1_22: i128 = u__id(state, tracer, s_1_21);
        // C s_1_23: const #0s : i
        let s_1_23: i128 = 0;
        // D s_1_24: cmp-le s_1_23 s_1_22
        let s_1_24: bool = ((s_1_23) <= (s_1_22));
        // N s_1_25: branch s_1_24 b4 b2
        if s_1_24 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#148110 <= s_2_0
        fn_state.gs_148110 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#148110:u8
        let s_3_0: bool = fn_state.gs_148110;
        // N s_3_1: assert s_3_0
        let s_3_1: () = assert!(s_3_0);
        // D s_3_2: read-var datasizeshadow#1187:i64
        let s_3_2: i64 = fn_state.datasizeshadow_1187;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: read-var datasizeshadow#1187:i64
        let s_3_5: i64 = fn_state.datasizeshadow_1187;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: read-var position:i
        let s_3_7: i128 = fn_state.position;
        // D s_3_8: add s_3_7 s_3_6
        let s_3_8: i128 = (s_3_7 + s_3_6);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // D s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: sub s_3_11 s_3_10
        let s_3_12: i128 = ((s_3_11) - (s_3_10));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var concat:bv
        let s_3_15: Bits = fn_state.concat;
        // D s_3_16: read-var position:i
        let s_3_16: i128 = fn_state.position;
        // C s_3_17: const #1s : i64
        let s_3_17: i64 = 1;
        // C s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: sub s_3_14 s_3_16
        let s_3_19: i128 = ((s_3_14) - (s_3_16));
        // D s_3_20: add s_3_19 s_3_18
        let s_3_20: i128 = (s_3_19 + s_3_18);
        // D s_3_21: bit-extract s_3_15 s_3_16 s_3_20
        let s_3_21: Bits = (Bits::new(
            ((s_3_15) >> (s_3_16)).value(),
            u16::try_from(s_3_20).unwrap(),
        ));
        // D s_3_22: read-var d:i64
        let s_3_22: i64 = fn_state.d;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: call V_set(s_3_23, s_3_4, s_3_21)
        let s_3_24: () = V_set(state, tracer, s_3_23, s_3_4, s_3_21);
        // N s_3_25: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var position:i
        let s_4_0: i128 = fn_state.position;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // D s_4_2: read-var datasizeshadow#1187:i64
        let s_4_2: i64 = fn_state.datasizeshadow_1187;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: call __id(s_4_3)
        let s_4_4: i128 = u__id(state, tracer, s_4_3);
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: add s_4_1 s_4_6
        let s_4_7: i128 = (s_4_1 + s_4_6);
        // C s_4_8: const #1s : i
        let s_4_8: i128 = 1;
        // D s_4_9: sub s_4_7 s_4_8
        let s_4_9: i128 = ((s_4_7) - (s_4_8));
        // D s_4_10: read-var datasizeshadow#1187:i64
        let s_4_10: i64 = fn_state.datasizeshadow_1187;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: call __id(s_4_11)
        let s_4_12: i128 = u__id(state, tracer, s_4_11);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // C s_4_14: const #2s : i
        let s_4_14: i128 = 2;
        // D s_4_15: cast zx s_4_13 -> i
        let s_4_15: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_16: mul s_4_15 s_4_14
        let s_4_16: i128 = ((s_4_15) * (s_4_14));
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_19: cmp-lt s_4_9 s_4_18
        let s_4_19: bool = ((s_4_9) < (s_4_18));
        // D s_4_20: write-var gs#148110 <= s_4_19
        fn_state.gs_148110 = s_4_19;
        // N s_4_21: jump b3
        return block_3(state, tracer, fn_state);
    }
}
