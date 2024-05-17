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
use asl_Int::*;
use SatQ::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Elem_set::*;
use Z_set::*;
use common::*;
pub fn execute_UQSUB_Z_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    imm: i128,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_201282: i64,
        e: i64,
        VLshadow_3308: i64,
        VLshadow_3309: i64,
        ga_286952: ProductTypef506aa96a892fe52,
        esizeshadow_3307: i64,
        result: Bits,
        operand1: Bits,
        VL: i64,
        dn: i64,
        esize: i64,
        imm: i128,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        imm,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#3307 <= s_0_2
        fn_state.esizeshadow_3307 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3308 <= s_0_6
        fn_state.VLshadow_3308 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#3308:i64
        let s_1_0: i64 = fn_state.VLshadow_3308;
        // D s_1_1: write-var VLshadow#3309 <= s_1_0
        fn_state.VLshadow_3309 = s_1_0;
        // D s_1_2: read-var VLshadow#3309:i64
        let s_1_2: i64 = fn_state.VLshadow_3309;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3307:i64
        let s_1_4: i64 = fn_state.esizeshadow_3307;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#3309:i64
        let s_1_8: i64 = fn_state.VLshadow_3309;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var dn:i64
        let s_1_11: i64 = fn_state.dn;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // C s_1_16: const #0s : i64
        let s_1_16: i64 = 0;
        // C s_1_17: const #1s : i
        let s_1_17: i128 = 1;
        // D s_1_18: cast zx s_1_7 -> i
        let s_1_18: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_19: sub s_1_18 s_1_17
        let s_1_19: i128 = ((s_1_18) - (s_1_17));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var gs#201282 <= s_1_20
        fn_state.gs_201282 = s_1_20;
        // D s_1_22: write-var e <= s_1_16
        fn_state.e = s_1_16;
        // N s_1_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#201282:i64
        let s_2_1: i64 = fn_state.gs_201282;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#3307:i64
        let s_3_0: i64 = fn_state.esizeshadow_3307;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: call asl_Int(s_3_7, s_3_8)
        let s_3_9: i128 = asl_Int(state, tracer, s_3_7, s_3_8);
        // D s_3_10: read-var imm:i
        let s_3_10: i128 = fn_state.imm;
        // D s_3_11: sub s_3_9 s_3_10
        let s_3_11: i128 = ((s_3_9) - (s_3_10));
        // D s_3_12: read-var esizeshadow#3307:i64
        let s_3_12: i64 = fn_state.esizeshadow_3307;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: read-var is_unsigned:u8
        let s_3_16: bool = fn_state.is_unsigned;
        // D s_3_17: call SatQ(s_3_11, s_3_15, s_3_16)
        let s_3_17: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_3_11,
            s_3_15,
            s_3_16,
        );
        // D s_3_18: write-var ga#286952 <= s_3_17
        fn_state.ga_286952 = s_3_17;
        // D s_3_19: read-var ga#286952.0:struct
        let s_3_19: Bits = fn_state.ga_286952._0;
        // D s_3_20: read-var esizeshadow#3307:i64
        let s_3_20: i64 = fn_state.esizeshadow_3307;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: read-var e:i64
        let s_3_23: i64 = fn_state.e;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: cast zx s_3_22 -> i
        let s_3_25: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_26: read-var result:bv
        let s_3_26: Bits = fn_state.result;
        // D s_3_27: call Elem_set(s_3_26, s_3_24, s_3_25, s_3_19)
        let s_3_27: Bits = Elem_set(state, tracer, s_3_26, s_3_24, s_3_25, s_3_19);
        // D s_3_28: write-var result <= s_3_27
        fn_state.result = s_3_27;
        // D s_3_29: read-var e:i64
        let s_3_29: i64 = fn_state.e;
        // C s_3_30: const #1s : i64
        let s_3_30: i64 = 1;
        // D s_3_31: add s_3_29 s_3_30
        let s_3_31: i64 = (s_3_29 + s_3_30);
        // D s_3_32: write-var e <= s_3_31
        fn_state.e = s_3_31;
        // N s_3_33: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3309:i64
        let s_4_0: i64 = fn_state.VLshadow_3309;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var dn:i64
        let s_4_3: i64 = fn_state.dn;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
