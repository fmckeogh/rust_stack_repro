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
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_vector_insert<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    dst_index: i128,
    esize: i64,
    idxdsize: i64,
    n: i64,
    src_index: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_1474: i64,
        idxdsizeshadow_1473: i64,
        d: i64,
        dst_index: i128,
        esize: i64,
        idxdsize: i64,
        n: i64,
        src_index: i128,
    }
    let fn_state = FunctionState {
        d,
        dst_index,
        esize,
        idxdsize,
        n,
        src_index,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var idxdsize:i64
        let s_0_0: i64 = fn_state.idxdsize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var idxdsizeshadow#1473 <= s_0_2
        fn_state.idxdsizeshadow_1473 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1474 <= s_0_6
        fn_state.esizeshadow_1474 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var idxdsizeshadow#1473:i64
        let s_1_0: i64 = fn_state.idxdsizeshadow_1473;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // C s_1_6: const #128s : i64
        let s_1_6: i64 = 128;
        // D s_1_7: read-var d:i64
        let s_1_7: i64 = fn_state.d;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: cast reint s_1_9 -> u128
        let s_1_10: u128 = (s_1_9.value() as u128);
        // D s_1_11: read-var esizeshadow#1474:i64
        let s_1_11: i64 = fn_state.esizeshadow_1474;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var esizeshadow#1474:i64
        let s_1_14: i64 = fn_state.esizeshadow_1474;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: read-var src_index:i
        let s_1_18: i128 = fn_state.src_index;
        // D s_1_19: call Elem_read(s_1_5, s_1_18, s_1_17)
        let s_1_19: Bits = Elem_read(state, tracer, s_1_5, s_1_18, s_1_17);
        // D s_1_20: cast zx s_1_10 -> bv
        let s_1_20: Bits = Bits::new(s_1_10 as u128, 128u16);
        // D s_1_21: cast zx s_1_13 -> i
        let s_1_21: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_22: read-var dst_index:i
        let s_1_22: i128 = fn_state.dst_index;
        // D s_1_23: call Elem_set(s_1_20, s_1_22, s_1_21, s_1_19)
        let s_1_23: Bits = Elem_set(state, tracer, s_1_20, s_1_22, s_1_21, s_1_19);
        // D s_1_24: cast reint s_1_23 -> u128
        let s_1_24: u128 = (s_1_23.value() as u128);
        // C s_1_25: const #128s : i64
        let s_1_25: i64 = 128;
        // D s_1_26: read-var d:i64
        let s_1_26: i64 = fn_state.d;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast zx s_1_24 -> bv
        let s_1_28: Bits = Bits::new(s_1_24 as u128, 128u16);
        // D s_1_29: call V_set(s_1_27, s_1_25, s_1_28)
        let s_1_29: () = V_set(state, tracer, s_1_27, s_1_25, s_1_28);
        // N s_1_30: return
        return;
    }
}
