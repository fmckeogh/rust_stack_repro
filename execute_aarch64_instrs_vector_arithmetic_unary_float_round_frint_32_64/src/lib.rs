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
use FPRoundIntN::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use FPCR_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_float_round_frint_32_64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    intsize: i64,
    n: i64,
    rounding: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        ga_256183: i64,
        result: Bits,
        gs_154062: i64,
        ga_256184: Bits,
        esizeshadow_1427: i64,
        datasizeshadow_1428: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        intsize: i64,
        n: i64,
        rounding: u32,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        intsize,
        n,
        rounding,
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
        // D s_0_3: write-var esizeshadow#1427 <= s_0_2
        fn_state.esizeshadow_1427 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1428 <= s_0_6
        fn_state.datasizeshadow_1428 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1428:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1428;
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
        // D s_1_9: read-var elements:i
        let s_1_9: i128 = fn_state.elements;
        // D s_1_10: sub s_1_9 s_1_8
        let s_1_10: i128 = ((s_1_9) - (s_1_8));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var gs#154062 <= s_1_11
        fn_state.gs_154062 = s_1_11;
        // D s_1_13: write-var e <= s_1_7
        fn_state.e = s_1_7;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#154062:i64
        let s_2_1: i64 = fn_state.gs_154062;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var esizeshadow#1427:i64
        let s_3_0: i64 = fn_state.esizeshadow_1427;
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
        // D s_3_8: read-var esizeshadow#1427:i64
        let s_3_8: i64 = fn_state.esizeshadow_1427;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: write-var ga#256183 <= s_3_10
        fn_state.ga_256183 = s_3_10;
        // C s_3_12: const #() : ()
        let s_3_12: () = ();
        // S s_3_13: call FPCR_read(s_3_12)
        let s_3_13: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_12);
        // D s_3_14: read-var intsize:i64
        let s_3_14: i64 = fn_state.intsize;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: read-var rounding:u32
        let s_3_16: u32 = fn_state.rounding;
        // D s_3_17: call FPRoundIntN(s_3_7, s_3_13, s_3_16, s_3_15)
        let s_3_17: Bits = FPRoundIntN(state, tracer, s_3_7, s_3_13, s_3_16, s_3_15);
        // D s_3_18: write-var ga#256184 <= s_3_17
        fn_state.ga_256184 = s_3_17;
        // N s_3_19: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var ga#256183:i64
        let s_4_2: i64 = fn_state.ga_256183;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var result:bv
        let s_4_4: Bits = fn_state.result;
        // D s_4_5: read-var ga#256184:bv
        let s_4_5: Bits = fn_state.ga_256184;
        // D s_4_6: call Elem_set(s_4_4, s_4_1, s_4_3, s_4_5)
        let s_4_6: Bits = Elem_set(state, tracer, s_4_4, s_4_1, s_4_3, s_4_5);
        // D s_4_7: write-var result <= s_4_6
        fn_state.result = s_4_6;
        // D s_4_8: read-var e:i64
        let s_4_8: i64 = fn_state.e;
        // C s_4_9: const #1s : i64
        let s_4_9: i64 = 1;
        // D s_4_10: add s_4_8 s_4_9
        let s_4_10: i64 = (s_4_8 + s_4_9);
        // D s_4_11: write-var e <= s_4_10
        fn_state.e = s_4_10;
        // N s_4_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1428:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1428;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var result:bv
        let s_5_5: Bits = fn_state.result;
        // D s_5_6: call V_set(s_5_4, s_5_2, s_5_5)
        let s_5_6: () = V_set(state, tracer, s_5_4, s_5_2, s_5_5);
        // N s_5_7: return
        return;
    }
}
