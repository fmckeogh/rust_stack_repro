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
use CheckSVEEnabled::*;
use Z_read::*;
use P_set::*;
use common::*;
pub fn execute_PMOV_P_ZI_H<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    imm: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        VLshadow_4406: i64,
        e: i64,
        psize: i64,
        elements: i64,
        gs_221440: i64,
        result: Bits,
        PL: i64,
        VL: i64,
        d: i64,
        esize: i64,
        imm: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        imm,
        n,
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
        // D s_0_3: write-var VLshadow#4406 <= s_0_2
        fn_state.VLshadow_4406 = s_0_2;
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
        // D s_1_0: read-var VLshadow#4406:i64
        let s_1_0: i64 = fn_state.VLshadow_4406;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // D s_1_6: cast zx s_1_0 -> i
        let s_1_6: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: div s_1_6 s_1_8
        let s_1_9: i128 = ((s_1_6) / (s_1_8));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: write-var elements <= s_1_10
        fn_state.elements = s_1_10;
        // D s_1_12: cast zx s_1_0 -> i
        let s_1_12: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var n:i64
        let s_1_14: i64 = fn_state.n;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast zx s_1_13 -> i
        let s_1_16: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_17: call Z_read(s_1_15, s_1_16)
        let s_1_17: Bits = Z_read(state, tracer, s_1_15, s_1_16);
        // D s_1_18: write-var operand <= s_1_17
        fn_state.operand = s_1_17;
        // C s_1_19: const #8s : i
        let s_1_19: i128 = 8;
        // D s_1_20: read-var esize:i64
        let s_1_20: i64 = fn_state.esize;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: div s_1_21 s_1_19
        let s_1_22: i128 = ((s_1_21) / (s_1_19));
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: write-var psize <= s_1_23
        fn_state.psize = s_1_23;
        // C s_1_25: const #0s : i64
        let s_1_25: i64 = 0;
        // C s_1_26: const #1s : i
        let s_1_26: i128 = 1;
        // D s_1_27: read-var elements:i64
        let s_1_27: i64 = fn_state.elements;
        // D s_1_28: cast zx s_1_27 -> i
        let s_1_28: i128 = (i128::try_from(s_1_27).unwrap());
        // D s_1_29: sub s_1_28 s_1_26
        let s_1_29: i128 = ((s_1_28) - (s_1_26));
        // D s_1_30: cast reint s_1_29 -> i64
        let s_1_30: i64 = (s_1_29 as i64);
        // D s_1_31: write-var gs#221440 <= s_1_30
        fn_state.gs_221440 = s_1_30;
        // D s_1_32: write-var e <= s_1_25
        fn_state.e = s_1_25;
        // N s_1_33: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#221440:i64
        let s_2_1: i64 = fn_state.gs_221440;
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
        // D s_3_0: read-var psize:i64
        let s_3_0: i64 = fn_state.psize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var elements:i64
        let s_3_3: i64 = fn_state.elements;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var imm:i64
        let s_3_5: i64 = fn_state.imm;
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: mul s_3_4 s_3_6
        let s_3_7: i128 = ((s_3_4) * (s_3_6));
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: read-var e:i64
        let s_3_10: i64 = fn_state.e;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: add s_3_9 s_3_11
        let s_3_12: i128 = (s_3_9 + s_3_11);
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var operand:bv
        let s_3_15: Bits = fn_state.operand;
        // C s_3_16: const #1u : u64
        let s_3_16: u64 = 1;
        // D s_3_17: bit-extract s_3_15 s_3_14 s_3_16
        let s_3_17: Bits = (Bits::new(
            ((s_3_15) >> (s_3_14)).value(),
            u16::try_from(s_3_16).unwrap(),
        ));
        // D s_3_18: cast reint s_3_17 -> u8
        let s_3_18: bool = ((s_3_17.value()) != 0);
        // C s_3_19: const #0s : i
        let s_3_19: i128 = 0;
        // C s_3_20: const #0u : u64
        let s_3_20: u64 = 0;
        // D s_3_21: cast zx s_3_18 -> u64
        let s_3_21: u64 = (s_3_18 as u64);
        // C s_3_22: const #1u : u64
        let s_3_22: u64 = 1;
        // D s_3_23: and s_3_21 s_3_22
        let s_3_23: u64 = ((s_3_21) & (s_3_22));
        // D s_3_24: cmp-eq s_3_23 s_3_22
        let s_3_24: bool = ((s_3_23) == (s_3_22));
        // D s_3_25: lsl s_3_21 s_3_19
        let s_3_25: u64 = s_3_21 << s_3_19;
        // D s_3_26: or s_3_20 s_3_25
        let s_3_26: u64 = ((s_3_20) | (s_3_25));
        // D s_3_27: cmpl s_3_25
        let s_3_27: u64 = !s_3_25;
        // D s_3_28: and s_3_20 s_3_27
        let s_3_28: u64 = ((s_3_20) & (s_3_27));
        // D s_3_29: select s_3_24 s_3_26 s_3_28
        let s_3_29: u64 = if s_3_24 { s_3_26 } else { s_3_28 };
        // D s_3_30: cast trunc s_3_29 -> u8
        let s_3_30: bool = ((s_3_29) != 0);
        // D s_3_31: cast zx s_3_30 -> bv
        let s_3_31: Bits = Bits::new(s_3_30 as u128, 1u16);
        // D s_3_32: read-var psize:i64
        let s_3_32: i64 = fn_state.psize;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_34: bits-cast zx s_3_31 -> bv length s_3_33
        let s_3_34: Bits = s_3_31.zero_extend(s_3_33);
        // D s_3_35: cast reint s_3_34 -> u8
        let s_3_35: u8 = (s_3_34.value() as u8);
        // D s_3_36: read-var e:i64
        let s_3_36: i64 = fn_state.e;
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (i128::try_from(s_3_36).unwrap());
        // D s_3_38: cast zx s_3_2 -> i
        let s_3_38: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_39: cast zx s_3_35 -> bv
        let s_3_39: Bits = Bits::new(s_3_35 as u128, 2u16);
        // D s_3_40: read-var result:bv
        let s_3_40: Bits = fn_state.result;
        // D s_3_41: call Elem_set(s_3_40, s_3_37, s_3_38, s_3_39)
        let s_3_41: Bits = Elem_set(state, tracer, s_3_40, s_3_37, s_3_38, s_3_39);
        // D s_3_42: write-var result <= s_3_41
        fn_state.result = s_3_41;
        // D s_3_43: read-var e:i64
        let s_3_43: i64 = fn_state.e;
        // C s_3_44: const #1s : i64
        let s_3_44: i64 = 1;
        // D s_3_45: add s_3_43 s_3_44
        let s_3_45: i64 = (s_3_43 + s_3_44);
        // D s_3_46: write-var e <= s_3_45
        fn_state.e = s_3_45;
        // N s_3_47: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var PL:i64
        let s_4_0: i64 = fn_state.PL;
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
        // D s_4_7: call P_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = P_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
