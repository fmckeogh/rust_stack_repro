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
use CheckFPEnabled64::*;
use FPDiv::*;
use V_read::*;
use FPCR_read::*;
use Elem_set::*;
use IsMerging::*;
use V_set::*;
use common::*;
pub fn execute_aarch64_instrs_float_arithmetic_div<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_1365: i64,
        result: u128,
        ga_254996: i64,
        operand1: Bits,
        ga_254997: Bits,
        operand2: Bits,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
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
        // D s_0_3: write-var esizeshadow#1365 <= s_0_2
        fn_state.esizeshadow_1365 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPEnabled64(s_0_4)
        let s_0_5: () = CheckFPEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esizeshadow#1365:i64
        let s_1_0: i64 = fn_state.esizeshadow_1365;
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
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var esizeshadow#1365:i64
        let s_1_7: i64 = fn_state.esizeshadow_1365;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // C s_1_14: const #() : ()
        let s_1_14: () = ();
        // S s_1_15: call FPCR_read(s_1_14)
        let s_1_15: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_14);
        // S s_1_16: call IsMerging(s_1_15)
        let s_1_16: bool = IsMerging(state, tracer, s_1_15);
        // N s_1_17: branch s_1_16 b5 b2
        if s_1_16 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: u8 = 0;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 0u16);
        // C s_2_2: const #0u : u64
        let s_2_2: u64 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 64u16);
        // C s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // C s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // C s_2_12: const #0u : u64
        let s_2_12: u64 = 0;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 64u16);
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // C s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: lsl s_2_14 s_2_17
        let s_2_18: u128 = s_2_14 << s_2_17;
        // D s_2_19: or s_2_18 s_2_16
        let s_2_19: u128 = ((s_2_18) | (s_2_16));
        // D s_2_20: add s_2_15 s_2_17
        let s_2_20: u16 = (s_2_15 + s_2_17);
        // D s_2_21: create-bits s_2_19 s_2_20
        let s_2_21: Bits = Bits::new(s_2_19, s_2_20);
        // D s_2_22: cast reint s_2_21 -> u128
        let s_2_22: u128 = (s_2_21.value() as u128);
        // D s_2_23: write-var result <= s_2_22
        fn_state.result = s_2_22;
        // N s_2_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1365:i64
        let s_3_0: i64 = fn_state.esizeshadow_1365;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: write-var ga#254996 <= s_3_2
        fn_state.ga_254996 = s_3_2;
        // C s_3_4: const #() : ()
        let s_3_4: () = ();
        // S s_3_5: call FPCR_read(s_3_4)
        let s_3_5: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_4);
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: read-var operand2:bv
        let s_3_7: Bits = fn_state.operand2;
        // D s_3_8: call FPDiv(s_3_6, s_3_7, s_3_5)
        let s_3_8: Bits = FPDiv(state, tracer, s_3_6, s_3_7, s_3_5);
        // D s_3_9: write-var ga#254997 <= s_3_8
        fn_state.ga_254997 = s_3_8;
        // N s_3_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var result:u128
        let s_4_1: u128 = fn_state.result;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 128u16);
        // D s_4_3: read-var ga#254996:i64
        let s_4_3: i64 = fn_state.ga_254996;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var ga#254997:bv
        let s_4_5: Bits = fn_state.ga_254997;
        // D s_4_6: call Elem_set(s_4_2, s_4_0, s_4_4, s_4_5)
        let s_4_6: Bits = Elem_set(state, tracer, s_4_2, s_4_0, s_4_4, s_4_5);
        // D s_4_7: cast reint s_4_6 -> u128
        let s_4_7: u128 = (s_4_6.value() as u128);
        // D s_4_8: write-var result <= s_4_7
        fn_state.result = s_4_7;
        // C s_4_9: const #128s : i64
        let s_4_9: i64 = 128;
        // D s_4_10: read-var d:i64
        let s_4_10: i64 = fn_state.d;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // D s_4_12: read-var result:u128
        let s_4_12: u128 = fn_state.result;
        // D s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 128u16);
        // D s_4_14: call V_set(s_4_11, s_4_9, s_4_13)
        let s_4_14: () = V_set(state, tracer, s_4_11, s_4_9, s_4_13);
        // N s_4_15: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var n:i64
        let s_5_1: i64 = fn_state.n;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: call V_read(s_5_2, s_5_0)
        let s_5_3: Bits = V_read(state, tracer, s_5_2, s_5_0);
        // D s_5_4: cast reint s_5_3 -> u128
        let s_5_4: u128 = (s_5_3.value() as u128);
        // D s_5_5: write-var result <= s_5_4
        fn_state.result = s_5_4;
        // N s_5_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
