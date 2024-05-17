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
use Reduce::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_reduce_fp16_max_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    esize: i64,
    n: i64,
    op: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_255365: i64,
        esizeshadow_1389: i64,
        datasizeshadow_1390: i64,
        ga_255366: Bits,
        d: i64,
        datasize: i64,
        esize: i64,
        n: i64,
        op: u32,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        esize,
        n,
        op,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1389 <= s_0_2
        fn_state.esizeshadow_1389 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1390 <= s_0_6
        fn_state.datasizeshadow_1390 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1390:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1390;
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
        // D s_1_6: read-var datasizeshadow#1390:i64
        let s_1_6: i64 = fn_state.datasizeshadow_1390;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call __id(s_1_7)
        let s_1_8: i128 = u__id(state, tracer, s_1_7);
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var esizeshadow#1389:i64
        let s_1_10: i64 = fn_state.esizeshadow_1389;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call __id(s_1_11)
        let s_1_12: i128 = u__id(state, tracer, s_1_11);
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: cast zx s_1_9 -> i
        let s_1_14: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_15: cast zx s_1_13 -> i
        let s_1_15: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_16: cmp-ge s_1_14 s_1_15
        let s_1_16: bool = ((s_1_14) >= (s_1_15));
        // N s_1_17: assert s_1_16
        let s_1_17: () = assert!(s_1_16);
        // D s_1_18: read-var esizeshadow#1389:i64
        let s_1_18: i64 = fn_state.esizeshadow_1389;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var ga#255365 <= s_1_20
        fn_state.ga_255365 = s_1_20;
        // D s_1_22: read-var esizeshadow#1389:i64
        let s_1_22: i64 = fn_state.esizeshadow_1389;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var op:u32
        let s_1_25: u32 = fn_state.op;
        // D s_1_26: call Reduce(s_1_25, s_1_5, s_1_24)
        let s_1_26: Bits = Reduce(state, tracer, s_1_25, s_1_5, s_1_24);
        // D s_1_27: write-var ga#255366 <= s_1_26
        fn_state.ga_255366 = s_1_26;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var d:i64
        let s_2_0: i64 = fn_state.d;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var ga#255365:i64
        let s_2_2: i64 = fn_state.ga_255365;
        // D s_2_3: read-var ga#255366:bv
        let s_2_3: Bits = fn_state.ga_255366;
        // D s_2_4: call V_set(s_2_1, s_2_2, s_2_3)
        let s_2_4: () = V_set(state, tracer, s_2_1, s_2_2, s_2_3);
        // N s_2_5: return
        return;
    }
}
