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
use CheckAdvSIMDEnabled::*;
use u_shl_int_general::*;
use integer_subrange::*;
use u_shr_int_general::*;
use asl_Int::*;
use D_set::*;
use Elem_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VSHL_r_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_319016: i64,
        shift: i128,
        d: i128,
        esizeshadow_7808: i64,
        result: i128,
        gs_319022: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        regs,
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
        // D s_0_3: write-var esizeshadow#7808 <= s_0_2
        fn_state.esizeshadow_7808 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#319016 <= s_1_5
        fn_state.gs_319016 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#319016:i64
        let s_2_1: i64 = fn_state.gs_319016;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#319022 <= s_3_4
        fn_state.gs_319022 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#319022:i64
        let s_4_1: i64 = fn_state.gs_319022;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7808:i64
        let s_5_8: i64 = fn_state.esizeshadow_7808;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // C s_5_17: const #1s : i64
        let s_5_17: i64 = 1;
        // C s_5_18: cast zx s_5_17 -> i
        let s_5_18: i128 = (i128::try_from(s_5_17).unwrap());
        // C s_5_19: const #7s : i
        let s_5_19: i128 = 7;
        // C s_5_20: add s_5_19 s_5_18
        let s_5_20: i128 = (s_5_19 + s_5_18);
        // D s_5_21: bit-extract s_5_15 s_5_16 s_5_20
        let s_5_21: Bits = (Bits::new(
            ((s_5_15) >> (s_5_16)).value(),
            u16::try_from(s_5_20).unwrap(),
        ));
        // D s_5_22: cast reint s_5_21 -> u8
        let s_5_22: u8 = (s_5_21.value() as u8);
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 8u16);
        // D s_5_24: cast sx s_5_23 -> i
        let s_5_24: i128 = {
            let sign_bit = s_5_23.length() - 1;
            let mut result = s_5_23.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_25: write-var shift <= s_5_24
        fn_state.shift = s_5_24;
        // C s_5_26: const #0s : i
        let s_5_26: i128 = 0;
        // D s_5_27: read-var shift:i
        let s_5_27: i128 = fn_state.shift;
        // D s_5_28: cmp-ge s_5_27 s_5_26
        let s_5_28: bool = ((s_5_27) >= (s_5_26));
        // N s_5_29: branch s_5_28 b8 b6
        if s_5_28 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var m:i64
        let s_6_0: i64 = fn_state.m;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: add s_6_1 s_6_3
        let s_6_4: i128 = (s_6_1 + s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: call D_read(s_6_6)
        let s_6_7: u64 = D_read(state, tracer, s_6_6);
        // D s_6_8: read-var esizeshadow#7808:i64
        let s_6_8: i64 = fn_state.esizeshadow_7808;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: cast zx s_6_7 -> bv
        let s_6_11: Bits = Bits::new(s_6_7 as u128, 64u16);
        // D s_6_12: read-var e:i64
        let s_6_12: i64 = fn_state.e;
        // D s_6_13: cast zx s_6_12 -> i
        let s_6_13: i128 = (i128::try_from(s_6_12).unwrap());
        // D s_6_14: cast zx s_6_10 -> i
        let s_6_14: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_15: call Elem_read(s_6_11, s_6_13, s_6_14)
        let s_6_15: Bits = Elem_read(state, tracer, s_6_11, s_6_13, s_6_14);
        // D s_6_16: read-var is_unsigned:u8
        let s_6_16: bool = fn_state.is_unsigned;
        // D s_6_17: call asl_Int(s_6_15, s_6_16)
        let s_6_17: i128 = asl_Int(state, tracer, s_6_15, s_6_16);
        // D s_6_18: read-var shift:i
        let s_6_18: i128 = fn_state.shift;
        // D s_6_19: neg s_6_18
        let s_6_19: i128 = -s_6_18;
        // D s_6_20: call _shr_int_general(s_6_17, s_6_19)
        let s_6_20: i128 = u_shr_int_general(state, tracer, s_6_17, s_6_19);
        // D s_6_21: write-var result <= s_6_20
        fn_state.result = s_6_20;
        // N s_6_22: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var result:i
        let s_7_0: i128 = fn_state.result;
        // D s_7_1: read-var r:i64
        let s_7_1: i64 = fn_state.r;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var d:i
        let s_7_3: i128 = fn_state.d;
        // D s_7_4: add s_7_3 s_7_2
        let s_7_4: i128 = (s_7_3 + s_7_2);
        // D s_7_5: read-var r:i64
        let s_7_5: i64 = fn_state.r;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var d:i
        let s_7_7: i128 = fn_state.d;
        // D s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: call D_read(s_7_8)
        let s_7_9: u64 = D_read(state, tracer, s_7_8);
        // D s_7_10: read-var esizeshadow#7808:i64
        let s_7_10: i64 = fn_state.esizeshadow_7808;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // C s_7_13: const #1s : i
        let s_7_13: i128 = 1;
        // D s_7_14: read-var esizeshadow#7808:i64
        let s_7_14: i64 = fn_state.esizeshadow_7808;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: sub s_7_15 s_7_13
        let s_7_16: i128 = ((s_7_15) - (s_7_13));
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // C s_7_18: const #0s : i
        let s_7_18: i128 = 0;
        // D s_7_19: cast zx s_7_17 -> i
        let s_7_19: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_20: call integer_subrange(s_7_0, s_7_19, s_7_18)
        let s_7_20: Bits = integer_subrange(state, tracer, s_7_0, s_7_19, s_7_18);
        // D s_7_21: cast zx s_7_9 -> bv
        let s_7_21: Bits = Bits::new(s_7_9 as u128, 64u16);
        // D s_7_22: read-var e:i64
        let s_7_22: i64 = fn_state.e;
        // D s_7_23: cast zx s_7_22 -> i
        let s_7_23: i128 = (i128::try_from(s_7_22).unwrap());
        // D s_7_24: cast zx s_7_12 -> i
        let s_7_24: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_25: call Elem_set(s_7_21, s_7_23, s_7_24, s_7_20)
        let s_7_25: Bits = Elem_set(state, tracer, s_7_21, s_7_23, s_7_24, s_7_20);
        // D s_7_26: cast reint s_7_25 -> u64
        let s_7_26: u64 = (s_7_25.value() as u64);
        // D s_7_27: call D_set(s_7_4, s_7_26)
        let s_7_27: () = D_set(state, tracer, s_7_4, s_7_26);
        // D s_7_28: read-var e:i64
        let s_7_28: i64 = fn_state.e;
        // C s_7_29: const #1s : i64
        let s_7_29: i64 = 1;
        // D s_7_30: add s_7_28 s_7_29
        let s_7_30: i64 = (s_7_28 + s_7_29);
        // D s_7_31: write-var e <= s_7_30
        fn_state.e = s_7_30;
        // N s_7_32: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var m:i64
        let s_8_0: i64 = fn_state.m;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var r:i64
        let s_8_2: i64 = fn_state.r;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: add s_8_1 s_8_3
        let s_8_4: i128 = (s_8_1 + s_8_3);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: call D_read(s_8_6)
        let s_8_7: u64 = D_read(state, tracer, s_8_6);
        // D s_8_8: read-var esizeshadow#7808:i64
        let s_8_8: i64 = fn_state.esizeshadow_7808;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: cast zx s_8_7 -> bv
        let s_8_11: Bits = Bits::new(s_8_7 as u128, 64u16);
        // D s_8_12: read-var e:i64
        let s_8_12: i64 = fn_state.e;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: cast zx s_8_10 -> i
        let s_8_14: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_15: call Elem_read(s_8_11, s_8_13, s_8_14)
        let s_8_15: Bits = Elem_read(state, tracer, s_8_11, s_8_13, s_8_14);
        // D s_8_16: read-var is_unsigned:u8
        let s_8_16: bool = fn_state.is_unsigned;
        // D s_8_17: call asl_Int(s_8_15, s_8_16)
        let s_8_17: i128 = asl_Int(state, tracer, s_8_15, s_8_16);
        // D s_8_18: read-var shift:i
        let s_8_18: i128 = fn_state.shift;
        // D s_8_19: call _shl_int_general(s_8_17, s_8_18)
        let s_8_19: i128 = u_shl_int_general(state, tracer, s_8_17, s_8_18);
        // D s_8_20: write-var result <= s_8_19
        fn_state.result = s_8_19;
        // N s_8_21: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var r <= s_9_2
        fn_state.r = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
