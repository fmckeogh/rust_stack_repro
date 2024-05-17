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
use Bit::*;
use X_read::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_rbit<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_165851: i64,
        result: Bits,
        i: i64,
        datasizeshadow_1785: i64,
        d: i64,
        datasize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
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
        // D s_0_3: write-var datasizeshadow#1785 <= s_0_2
        fn_state.datasizeshadow_1785 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1785:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1785;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: write-var operand <= s_0_9
        fn_state.operand = s_0_9;
        // C s_0_11: const #0s : i64
        let s_0_11: i64 = 0;
        // C s_0_12: const #1s : i
        let s_0_12: i128 = 1;
        // D s_0_13: read-var datasizeshadow#1785:i64
        let s_0_13: i64 = fn_state.datasizeshadow_1785;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: sub s_0_14 s_0_12
        let s_0_15: i128 = ((s_0_14) - (s_0_12));
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // D s_0_17: write-var gs#165851 <= s_0_16
        fn_state.gs_165851 = s_0_16;
        // D s_0_18: write-var i <= s_0_11
        fn_state.i = s_0_11;
        // N s_0_19: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // D s_1_1: read-var gs#165851:i64
        let s_1_1: i64 = fn_state.gs_165851;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var datasizeshadow#1785:i64
        let s_2_1: i64 = fn_state.datasizeshadow_1785;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: sub s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) - (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: read-var i:i64
        let s_2_6: i64 = fn_state.i;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: sub s_2_5 s_2_7
        let s_2_8: i128 = ((s_2_5) - (s_2_7));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: read-var i:i64
        let s_2_10: i64 = fn_state.i;
        // D s_2_11: cast zx s_2_10 -> i
        let s_2_11: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_12: read-var operand:bv
        let s_2_12: Bits = fn_state.operand;
        // C s_2_13: const #1u : u64
        let s_2_13: u64 = 1;
        // D s_2_14: bit-extract s_2_12 s_2_11 s_2_13
        let s_2_14: Bits = (Bits::new(
            ((s_2_12) >> (s_2_11)).value(),
            u16::try_from(s_2_13).unwrap(),
        ));
        // D s_2_15: cast reint s_2_14 -> u8
        let s_2_15: bool = ((s_2_14.value()) != 0);
        // C s_2_16: const #0s : i
        let s_2_16: i128 = 0;
        // C s_2_17: const #0u : u64
        let s_2_17: u64 = 0;
        // D s_2_18: cast zx s_2_15 -> u64
        let s_2_18: u64 = (s_2_15 as u64);
        // C s_2_19: const #1u : u64
        let s_2_19: u64 = 1;
        // D s_2_20: and s_2_18 s_2_19
        let s_2_20: u64 = ((s_2_18) & (s_2_19));
        // D s_2_21: cmp-eq s_2_20 s_2_19
        let s_2_21: bool = ((s_2_20) == (s_2_19));
        // D s_2_22: lsl s_2_18 s_2_16
        let s_2_22: u64 = s_2_18 << s_2_16;
        // D s_2_23: or s_2_17 s_2_22
        let s_2_23: u64 = ((s_2_17) | (s_2_22));
        // D s_2_24: cmpl s_2_22
        let s_2_24: u64 = !s_2_22;
        // D s_2_25: and s_2_17 s_2_24
        let s_2_25: u64 = ((s_2_17) & (s_2_24));
        // D s_2_26: select s_2_21 s_2_23 s_2_25
        let s_2_26: u64 = if s_2_21 { s_2_23 } else { s_2_25 };
        // D s_2_27: cast trunc s_2_26 -> u8
        let s_2_27: bool = ((s_2_26) != 0);
        // D s_2_28: call Bit(s_2_27)
        let s_2_28: bool = Bit(state, tracer, s_2_27);
        // D s_2_29: cast zx s_2_9 -> i
        let s_2_29: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_30: read-var result:bv
        let s_2_30: Bits = fn_state.result;
        // C s_2_31: const #1u : u64
        let s_2_31: u64 = 1;
        // D s_2_32: bit-insert s_2_30 s_2_30 s_2_29 s_2_31
        let s_2_32: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_31 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_30.length(),
            );
            (s_2_30 & mask) | (s_2_30 << s_2_29)
        };
        // D s_2_33: write-var result <= s_2_32
        fn_state.result = s_2_32;
        // D s_2_34: read-var i:i64
        let s_2_34: i64 = fn_state.i;
        // C s_2_35: const #1s : i64
        let s_2_35: i64 = 1;
        // D s_2_36: add s_2_34 s_2_35
        let s_2_36: i64 = (s_2_34 + s_2_35);
        // D s_2_37: write-var i <= s_2_36
        fn_state.i = s_2_36;
        // N s_2_38: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var datasizeshadow#1785:i64
        let s_3_0: i64 = fn_state.datasizeshadow_1785;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var result:bv
        let s_3_5: Bits = fn_state.result;
        // D s_3_6: call X_set(s_3_4, s_3_2, s_3_5)
        let s_3_6: () = X_set(state, tracer, s_3_4, s_3_2, s_3_5);
        // N s_3_7: return
        return;
    }
}
