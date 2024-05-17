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
use X_set::*;
use CheckFPEnabled64::*;
use V_read::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_integer_move_signed<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    esize: i64,
    idxdsize: i64,
    index: i128,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_1885: i64,
        idxdsizeshadow_1884: i64,
        datasizeshadow_1886: i64,
        d: i64,
        datasize: i64,
        esize: i64,
        idxdsize: i64,
        index: i128,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        esize,
        idxdsize,
        index,
        n,
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
        // D s_0_3: write-var idxdsizeshadow#1884 <= s_0_2
        fn_state.idxdsizeshadow_1884 = s_0_2;
        // D s_0_4: read-var esize:i64
        let s_0_4: i64 = fn_state.esize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var esizeshadow#1885 <= s_0_6
        fn_state.esizeshadow_1885 = s_0_6;
        // D s_0_8: read-var datasize:i64
        let s_0_8: i64 = fn_state.datasize;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: cast reint s_0_9 -> i64
        let s_0_10: i64 = (s_0_9 as i64);
        // D s_0_11: write-var datasizeshadow#1886 <= s_0_10
        fn_state.datasizeshadow_1886 = s_0_10;
        // C s_0_12: const #0s : i
        let s_0_12: i128 = 0;
        // D s_0_13: read-var index:i
        let s_0_13: i128 = fn_state.index;
        // D s_0_14: cmp-eq s_0_13 s_0_12
        let s_0_14: bool = ((s_0_13) == (s_0_12));
        // N s_0_15: branch s_0_14 b3 b1
        if s_0_14 {
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
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call CheckFPAdvSIMDEnabled64(s_1_0)
        let s_1_1: () = CheckFPAdvSIMDEnabled64(state, tracer, s_1_0);
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var idxdsizeshadow#1884:i64
        let s_2_0: i64 = fn_state.idxdsizeshadow_1884;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var n:i64
        let s_2_3: i64 = fn_state.n;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call V_read(s_2_4, s_2_2)
        let s_2_5: Bits = V_read(state, tracer, s_2_4, s_2_2);
        // D s_2_6: read-var datasizeshadow#1886:i64
        let s_2_6: i64 = fn_state.datasizeshadow_1886;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: read-var esizeshadow#1885:i64
        let s_2_9: i64 = fn_state.esizeshadow_1885;
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: read-var index:i
        let s_2_13: i128 = fn_state.index;
        // D s_2_14: call Elem_read(s_2_5, s_2_13, s_2_12)
        let s_2_14: Bits = Elem_read(state, tracer, s_2_5, s_2_13, s_2_12);
        // D s_2_15: read-var datasizeshadow#1886:i64
        let s_2_15: i64 = fn_state.datasizeshadow_1886;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: bits-cast sx s_2_14 -> bv length s_2_16
        let s_2_17: Bits = s_2_14.sign_extend(s_2_16);
        // D s_2_18: read-var d:i64
        let s_2_18: i64 = fn_state.d;
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: call X_set(s_2_19, s_2_8, s_2_17)
        let s_2_20: () = X_set(state, tracer, s_2_19, s_2_8, s_2_17);
        // N s_2_21: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call CheckFPEnabled64(s_3_0)
        let s_3_1: () = CheckFPEnabled64(state, tracer, s_3_0);
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
