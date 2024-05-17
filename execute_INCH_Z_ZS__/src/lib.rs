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
use DecodePredCount::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Elem_set::*;
use Z_set::*;
use common::*;
pub fn execute_INCH_Z_ZS__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    imm: i64,
    pat: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        count: i128,
        gs_192100: i64,
        VLshadow_2747: i64,
        result: Bits,
        operand1: Bits,
        VLshadow_2746: i64,
        VL: i64,
        dn: i64,
        esize: i64,
        imm: i64,
        pat: u8,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        imm,
        pat,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2746 <= s_0_2
        fn_state.VLshadow_2746 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2746:i64
        let s_1_0: i64 = fn_state.VLshadow_2746;
        // D s_1_1: write-var VLshadow#2747 <= s_1_0
        fn_state.VLshadow_2747 = s_1_0;
        // D s_1_2: read-var VLshadow#2747:i64
        let s_1_2: i64 = fn_state.VLshadow_2747;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var esize:i64
        let s_1_8: i64 = fn_state.esize;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: read-var pat:u8
        let s_1_10: u8 = fn_state.pat;
        // D s_1_11: call DecodePredCount(s_1_10, s_1_9)
        let s_1_11: i128 = DecodePredCount(state, tracer, s_1_10, s_1_9);
        // D s_1_12: write-var count <= s_1_11
        fn_state.count = s_1_11;
        // D s_1_13: read-var VLshadow#2747:i64
        let s_1_13: i64 = fn_state.VLshadow_2747;
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var dn:i64
        let s_1_16: i64 = fn_state.dn;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call Z_read(s_1_17, s_1_18)
        let s_1_19: Bits = Z_read(state, tracer, s_1_17, s_1_18);
        // D s_1_20: write-var operand1 <= s_1_19
        fn_state.operand1 = s_1_19;
        // C s_1_21: const #0s : i64
        let s_1_21: i64 = 0;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: cast zx s_1_7 -> i
        let s_1_23: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_24: sub s_1_23 s_1_22
        let s_1_24: i128 = ((s_1_23) - (s_1_22));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var gs#192100 <= s_1_25
        fn_state.gs_192100 = s_1_25;
        // D s_1_27: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_28: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#192100:i64
        let s_2_1: i64 = fn_state.gs_192100;
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
        // D s_3_0: read-var esize:i64
        let s_3_0: i64 = fn_state.esize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var esize:i64
        let s_3_3: i64 = fn_state.esize;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var e:i64
        let s_3_6: i64 = fn_state.e;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast zx s_3_5 -> i
        let s_3_8: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_9: read-var operand1:bv
        let s_3_9: Bits = fn_state.operand1;
        // D s_3_10: call Elem_read(s_3_9, s_3_7, s_3_8)
        let s_3_10: Bits = Elem_read(state, tracer, s_3_9, s_3_7, s_3_8);
        // D s_3_11: cast reint s_3_10 -> u16
        let s_3_11: u16 = (s_3_10.value() as u16);
        // D s_3_12: read-var imm:i64
        let s_3_12: i64 = fn_state.imm;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: read-var count:i
        let s_3_14: i128 = fn_state.count;
        // D s_3_15: mul s_3_14 s_3_13
        let s_3_15: i128 = ((s_3_14) * (s_3_13));
        // D s_3_16: cast zx s_3_11 -> bv
        let s_3_16: Bits = Bits::new(s_3_11 as u128, 16u16);
        // D s_3_17: cast cvt s_3_15 -> bv
        let s_3_17: Bits = Bits::new(s_3_15 as u128, 128);
        // D s_3_18: add s_3_16 s_3_17
        let s_3_18: Bits = (s_3_16 + s_3_17);
        // D s_3_19: cast reint s_3_18 -> u16
        let s_3_19: u16 = (s_3_18.value() as u16);
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: cast zx s_3_2 -> i
        let s_3_22: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_23: cast zx s_3_19 -> bv
        let s_3_23: Bits = Bits::new(s_3_19 as u128, 16u16);
        // D s_3_24: read-var result:bv
        let s_3_24: Bits = fn_state.result;
        // D s_3_25: call Elem_set(s_3_24, s_3_21, s_3_22, s_3_23)
        let s_3_25: Bits = Elem_set(state, tracer, s_3_24, s_3_21, s_3_22, s_3_23);
        // D s_3_26: write-var result <= s_3_25
        fn_state.result = s_3_25;
        // D s_3_27: read-var e:i64
        let s_3_27: i64 = fn_state.e;
        // C s_3_28: const #1s : i64
        let s_3_28: i64 = 1;
        // D s_3_29: add s_3_27 s_3_28
        let s_3_29: i64 = (s_3_27 + s_3_28);
        // D s_3_30: write-var e <= s_3_29
        fn_state.e = s_3_29;
        // N s_3_31: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#2747:i64
        let s_4_0: i64 = fn_state.VLshadow_2747;
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
