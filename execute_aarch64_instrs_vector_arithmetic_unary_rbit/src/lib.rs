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
use Bit::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_rbit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i64,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        datasizeshadow_1784: i64,
        gs_165820: i64,
        rev: Bits,
        element: Bits,
        result: Bits,
        i: i64,
        gs_165826: i64,
        d: i64,
        datasize: i64,
        elements: i64,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
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
        // D s_0_3: write-var datasizeshadow#1784 <= s_0_2
        fn_state.datasizeshadow_1784 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#1784:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1784;
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
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // C s_1_7: const #0s : i64
        let s_1_7: i64 = 0;
        // C s_1_8: const #1s : i
        let s_1_8: i128 = 1;
        // D s_1_9: read-var elements:i64
        let s_1_9: i64 = fn_state.elements;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: sub s_1_10 s_1_8
        let s_1_11: i128 = ((s_1_10) - (s_1_8));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var gs#165820 <= s_1_12
        fn_state.gs_165820 = s_1_12;
        // D s_1_14: write-var e <= s_1_7
        fn_state.e = s_1_7;
        // N s_1_15: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#165820:i64
        let s_2_1: i64 = fn_state.gs_165820;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand:bv
        let s_3_6: Bits = fn_state.operand;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var element <= s_3_7
        fn_state.element = s_3_7;
        // C s_3_9: const #0s : i64
        let s_3_9: i64 = 0;
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // D s_3_11: read-var esize:i64
        let s_3_11: i64 = fn_state.esize;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: sub s_3_12 s_3_10
        let s_3_13: i128 = ((s_3_12) - (s_3_10));
        // D s_3_14: cast reint s_3_13 -> i64
        let s_3_14: i64 = (s_3_13 as i64);
        // D s_3_15: write-var gs#165826 <= s_3_14
        fn_state.gs_165826 = s_3_14;
        // D s_3_16: write-var i <= s_3_9
        fn_state.i = s_3_9;
        // N s_3_17: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // D s_4_1: read-var gs#165826:i64
        let s_4_1: i64 = fn_state.gs_165826;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: sub s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) - (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var i:i64
        let s_5_6: i64 = fn_state.i;
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: sub s_5_5 s_5_7
        let s_5_8: i128 = ((s_5_5) - (s_5_7));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: read-var i:i64
        let s_5_10: i64 = fn_state.i;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: read-var element:bv
        let s_5_12: Bits = fn_state.element;
        // C s_5_13: const #1u : u64
        let s_5_13: u64 = 1;
        // D s_5_14: bit-extract s_5_12 s_5_11 s_5_13
        let s_5_14: Bits = (Bits::new(
            ((s_5_12) >> (s_5_11)).value(),
            u16::try_from(s_5_13).unwrap(),
        ));
        // D s_5_15: cast reint s_5_14 -> u8
        let s_5_15: bool = ((s_5_14.value()) != 0);
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // C s_5_17: const #0u : u64
        let s_5_17: u64 = 0;
        // D s_5_18: cast zx s_5_15 -> u64
        let s_5_18: u64 = (s_5_15 as u64);
        // C s_5_19: const #1u : u64
        let s_5_19: u64 = 1;
        // D s_5_20: and s_5_18 s_5_19
        let s_5_20: u64 = ((s_5_18) & (s_5_19));
        // D s_5_21: cmp-eq s_5_20 s_5_19
        let s_5_21: bool = ((s_5_20) == (s_5_19));
        // D s_5_22: lsl s_5_18 s_5_16
        let s_5_22: u64 = s_5_18 << s_5_16;
        // D s_5_23: or s_5_17 s_5_22
        let s_5_23: u64 = ((s_5_17) | (s_5_22));
        // D s_5_24: cmpl s_5_22
        let s_5_24: u64 = !s_5_22;
        // D s_5_25: and s_5_17 s_5_24
        let s_5_25: u64 = ((s_5_17) & (s_5_24));
        // D s_5_26: select s_5_21 s_5_23 s_5_25
        let s_5_26: u64 = if s_5_21 { s_5_23 } else { s_5_25 };
        // D s_5_27: cast trunc s_5_26 -> u8
        let s_5_27: bool = ((s_5_26) != 0);
        // D s_5_28: call Bit(s_5_27)
        let s_5_28: bool = Bit(state, tracer, s_5_27);
        // D s_5_29: cast zx s_5_9 -> i
        let s_5_29: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_30: read-var rev:bv
        let s_5_30: Bits = fn_state.rev;
        // C s_5_31: const #1u : u64
        let s_5_31: u64 = 1;
        // D s_5_32: bit-insert s_5_30 s_5_30 s_5_29 s_5_31
        let s_5_32: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_5_31 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_5_30.length(),
            );
            (s_5_30 & mask) | (s_5_30 << s_5_29)
        };
        // D s_5_33: write-var rev <= s_5_32
        fn_state.rev = s_5_32;
        // D s_5_34: read-var i:i64
        let s_5_34: i64 = fn_state.i;
        // C s_5_35: const #1s : i64
        let s_5_35: i64 = 1;
        // D s_5_36: add s_5_34 s_5_35
        let s_5_36: i64 = (s_5_34 + s_5_35);
        // D s_5_37: write-var i <= s_5_36
        fn_state.i = s_5_36;
        // N s_5_38: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esize:i64
        let s_6_0: i64 = fn_state.esize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var e:i64
        let s_6_3: i64 = fn_state.e;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: cast zx s_6_2 -> i
        let s_6_5: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_6: read-var result:bv
        let s_6_6: Bits = fn_state.result;
        // D s_6_7: read-var rev:bv
        let s_6_7: Bits = fn_state.rev;
        // D s_6_8: call Elem_set(s_6_6, s_6_4, s_6_5, s_6_7)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_6, s_6_4, s_6_5, s_6_7);
        // D s_6_9: write-var result <= s_6_8
        fn_state.result = s_6_8;
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // C s_6_11: const #1s : i64
        let s_6_11: i64 = 1;
        // D s_6_12: add s_6_10 s_6_11
        let s_6_12: i64 = (s_6_10 + s_6_11);
        // D s_6_13: write-var e <= s_6_12
        fn_state.e = s_6_12;
        // N s_6_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1784:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1784;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call V_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = V_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
}
