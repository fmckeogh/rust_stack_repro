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
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_LSR_Z_ZW__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_200239: i64,
        VLshadow_3242: i64,
        e: i64,
        esizeshadow_3240: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_3241: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#3240 <= s_0_2
        fn_state.esizeshadow_3240 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3241 <= s_0_6
        fn_state.VLshadow_3241 = s_0_6;
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
        // D s_1_0: read-var VLshadow#3241:i64
        let s_1_0: i64 = fn_state.VLshadow_3241;
        // D s_1_1: write-var VLshadow#3242 <= s_1_0
        fn_state.VLshadow_3242 = s_1_0;
        // D s_1_2: read-var VLshadow#3242:i64
        let s_1_2: i64 = fn_state.VLshadow_3242;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3240:i64
        let s_1_4: i64 = fn_state.esizeshadow_3240;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#3242:i64
        let s_1_8: i64 = fn_state.VLshadow_3242;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var VLshadow#3242:i64
        let s_1_16: i64 = fn_state.VLshadow_3242;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // C s_1_24: const #0s : i64
        let s_1_24: i64 = 0;
        // C s_1_25: const #1s : i
        let s_1_25: i128 = 1;
        // D s_1_26: cast zx s_1_7 -> i
        let s_1_26: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_27: sub s_1_26 s_1_25
        let s_1_27: i128 = ((s_1_26) - (s_1_25));
        // D s_1_28: cast reint s_1_27 -> i64
        let s_1_28: i64 = (s_1_27 as i64);
        // D s_1_29: write-var gs#200239 <= s_1_28
        fn_state.gs_200239 = s_1_28;
        // D s_1_30: write-var e <= s_1_24
        fn_state.e = s_1_24;
        // N s_1_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#200239:i64
        let s_2_1: i64 = fn_state.gs_200239;
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
        // D s_3_0: read-var esizeshadow#3240:i64
        let s_3_0: i64 = fn_state.esizeshadow_3240;
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
        // D s_3_8: read-var e:i64
        let s_3_8: i64 = fn_state.e;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: read-var esizeshadow#3240:i64
        let s_3_10: i64 = fn_state.esizeshadow_3240;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: mul s_3_9 s_3_11
        let s_3_12: i128 = ((s_3_9) * (s_3_11));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // C s_3_14: const #64s : i
        let s_3_14: i128 = 64;
        // D s_3_15: cast zx s_3_13 -> i
        let s_3_15: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_16: div s_3_15 s_3_14
        let s_3_16: i128 = ((s_3_15) / (s_3_14));
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // C s_3_18: const #64s : i64
        let s_3_18: i64 = 64;
        // D s_3_19: cast zx s_3_17 -> i
        let s_3_19: i128 = (i128::try_from(s_3_17).unwrap());
        // C s_3_20: cast zx s_3_18 -> i
        let s_3_20: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_21: read-var operand2:bv
        let s_3_21: Bits = fn_state.operand2;
        // D s_3_22: call Elem_read(s_3_21, s_3_19, s_3_20)
        let s_3_22: Bits = Elem_read(state, tracer, s_3_21, s_3_19, s_3_20);
        // D s_3_23: cast reint s_3_22 -> u64
        let s_3_23: u64 = (s_3_22.value() as u64);
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 64u16);
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (s_3_24.value() as i128);
        // D s_3_26: read-var esizeshadow#3240:i64
        let s_3_26: i64 = fn_state.esizeshadow_3240;
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cmp-lt s_3_25 s_3_27
        let s_3_28: bool = ((s_3_25) < (s_3_27));
        // D s_3_29: select s_3_28 s_3_25 s_3_27
        let s_3_29: i128 = if s_3_28 { s_3_25 } else { s_3_27 };
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: read-var esizeshadow#3240:i64
        let s_3_31: i64 = fn_state.esizeshadow_3240;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: cast reint s_3_32 -> i64
        let s_3_33: i64 = (s_3_32 as i64);
        // D s_3_34: cast zx s_3_30 -> i
        let s_3_34: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_35: lsr s_3_7 s_3_34
        let s_3_35: Bits = s_3_7 >> s_3_34;
        // D s_3_36: read-var e:i64
        let s_3_36: i64 = fn_state.e;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast zx s_3_33 -> i
        let s_3_38: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_39: read-var result:bv
        let s_3_39: Bits = fn_state.result;
        // D s_3_40: call Elem_set(s_3_39, s_3_37, s_3_38, s_3_35)
        let s_3_40: Bits = Elem_set(state, tracer, s_3_39, s_3_37, s_3_38, s_3_35);
        // D s_3_41: write-var result <= s_3_40
        fn_state.result = s_3_40;
        // D s_3_42: read-var e:i64
        let s_3_42: i64 = fn_state.e;
        // C s_3_43: const #1s : i64
        let s_3_43: i64 = 1;
        // D s_3_44: add s_3_42 s_3_43
        let s_3_44: i64 = (s_3_42 + s_3_43);
        // D s_3_45: write-var e <= s_3_44
        fn_state.e = s_3_44;
        // N s_3_46: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3242:i64
        let s_4_0: i64 = fn_state.VLshadow_3242;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
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
