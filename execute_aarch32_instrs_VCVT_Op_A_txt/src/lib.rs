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
use StandardFPSCRValue::*;
use Q_read::*;
use Elem_set::*;
use ConditionPassed::*;
use CheckAdvSIMDEnabled::*;
use D_set::*;
use Elem_read::*;
use EncodingSpecificOperations::*;
use FPConvertBF__1::*;
use common::*;
pub fn execute_aarch32_instrs_VCVT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_917393: Bits,
        e: i64,
        result: u64,
        operandshadow_7993: u128,
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call EncodingSpecificOperations(s_2_0)
        let s_2_1: () = EncodingSpecificOperations(state, tracer, s_2_0);
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call CheckAdvSIMDEnabled(s_2_2)
        let s_2_3: () = CheckAdvSIMDEnabled(state, tracer, s_2_2);
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #1s : i64
        let s_3_0: i64 = 1;
        // D s_3_1: read-var m:i64
        let s_3_1: i64 = fn_state.m;
        // D s_3_2: lsr s_3_1 s_3_0
        let s_3_2: i64 = s_3_1 >> s_3_0;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: call Q_read(s_3_3)
        let s_3_4: u128 = Q_read(state, tracer, s_3_3);
        // D s_3_5: write-var operandshadow#7993 <= s_3_4
        fn_state.operandshadow_7993 = s_3_4;
        // C s_3_6: const #0s : i64
        let s_3_6: i64 = 0;
        // D s_3_7: write-var e <= s_3_6
        fn_state.e = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // D s_5_1: read-var operandshadow#7993:u128
        let s_5_1: u128 = fn_state.operandshadow_7993;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 128u16);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: cast zx s_5_0 -> i
        let s_5_5: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_6: call Elem_read(s_5_2, s_5_4, s_5_5)
        let s_5_6: Bits = Elem_read(state, tracer, s_5_2, s_5_4, s_5_5);
        // D s_5_7: cast reint s_5_6 -> u32
        let s_5_7: u32 = (s_5_6.value() as u32);
        // C s_5_8: const #() : ()
        let s_5_8: () = ();
        // S s_5_9: call StandardFPSCRValue(s_5_8)
        let s_5_9: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_5_8,
        );
        // D s_5_10: cast zx s_5_7 -> bv
        let s_5_10: Bits = Bits::new(s_5_7 as u128, 32u16);
        // D s_5_11: call FPConvertBF__1(s_5_10, s_5_9)
        let s_5_11: Bits = FPConvertBF__1(state, tracer, s_5_10, s_5_9);
        // D s_5_12: write-var gs#917393 <= s_5_11
        fn_state.gs_917393 = s_5_11;
        // N s_5_13: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#917393:bv
        let s_6_0: Bits = fn_state.gs_917393;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // D s_6_2: read-var result:u64
        let s_6_2: u64 = fn_state.result;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 64u16);
        // D s_6_4: read-var e:i64
        let s_6_4: i64 = fn_state.e;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // C s_6_6: const #16s : i64
        let s_6_6: i64 = 16;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: cast zx s_6_1 -> bv
        let s_6_8: Bits = Bits::new(s_6_1 as u128, 16u16);
        // D s_6_9: call Elem_set(s_6_3, s_6_5, s_6_7, s_6_8)
        let s_6_9: Bits = Elem_set(state, tracer, s_6_3, s_6_5, s_6_7, s_6_8);
        // D s_6_10: cast reint s_6_9 -> u64
        let s_6_10: u64 = (s_6_9.value() as u64);
        // D s_6_11: write-var result <= s_6_10
        fn_state.result = s_6_10;
        // D s_6_12: read-var e:i64
        let s_6_12: i64 = fn_state.e;
        // C s_6_13: const #1s : i64
        let s_6_13: i64 = 1;
        // D s_6_14: add s_6_12 s_6_13
        let s_6_14: i64 = (s_6_12 + s_6_13);
        // D s_6_15: write-var e <= s_6_14
        fn_state.e = s_6_14;
        // N s_6_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i64
        let s_7_0: i64 = fn_state.d;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var result:u64
        let s_7_2: u64 = fn_state.result;
        // D s_7_3: call D_set(s_7_1, s_7_2)
        let s_7_3: () = D_set(state, tracer, s_7_1, s_7_2);
        // N s_7_4: return
        return;
    }
}
