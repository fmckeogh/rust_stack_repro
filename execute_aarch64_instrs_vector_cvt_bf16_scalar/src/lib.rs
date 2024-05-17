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
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use V_set::*;
use IsMerging::*;
use Zeros::*;
use FPConvertBF__1::*;
use common::*;
pub fn execute_aarch64_instrs_vector_cvt_bf16_scalar<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: u32,
        gs_683660: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        result: u128,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
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
        // S s_0_1: call CheckFPEnabled64(s_0_0)
        let s_0_1: () = CheckFPEnabled64(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u32
        let s_1_4: u32 = (s_1_3.value() as u32);
        // D s_1_5: write-var operand <= s_1_4
        fn_state.operand = s_1_4;
        // C s_1_6: const #() : ()
        let s_1_6: () = ();
        // S s_1_7: call FPCR_read(s_1_6)
        let s_1_7: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_6);
        // D s_1_8: write-var fpcr <= s_1_7
        fn_state.fpcr = s_1_7;
        // D s_1_9: read-var fpcr:struct
        let s_1_9: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_1_10: call IsMerging(s_1_9)
        let s_1_10: bool = IsMerging(state, tracer, s_1_9);
        // N s_1_11: branch s_1_10 b5 b2
        if s_1_10 {
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
        // C s_2_0: const #128s : i
        let s_2_0: i128 = 128;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u128
        let s_2_2: u128 = (s_2_1.value() as u128);
        // D s_2_3: write-var result <= s_2_2
        fn_state.result = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var operand:u32
        let s_3_0: u32 = fn_state.operand;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 32u16);
        // D s_3_2: read-var fpcr:struct
        let s_3_2: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_3_3: call FPConvertBF__1(s_3_1, s_3_2)
        let s_3_3: Bits = FPConvertBF__1(state, tracer, s_3_1, s_3_2);
        // D s_3_4: write-var gs#683660 <= s_3_3
        fn_state.gs_683660 = s_3_3;
        // N s_3_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#683660:bv
        let s_4_0: Bits = fn_state.gs_683660;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // C s_4_2: const #0s : i
        let s_4_2: i128 = 0;
        // D s_4_3: read-var result:u128
        let s_4_3: u128 = fn_state.result;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 128u16);
        // C s_4_5: const #16s : i64
        let s_4_5: i64 = 16;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: cast zx s_4_1 -> bv
        let s_4_7: Bits = Bits::new(s_4_1 as u128, 16u16);
        // D s_4_8: call Elem_set(s_4_4, s_4_2, s_4_6, s_4_7)
        let s_4_8: Bits = Elem_set(state, tracer, s_4_4, s_4_2, s_4_6, s_4_7);
        // D s_4_9: cast reint s_4_8 -> u128
        let s_4_9: u128 = (s_4_8.value() as u128);
        // D s_4_10: write-var result <= s_4_9
        fn_state.result = s_4_9;
        // C s_4_11: const #128s : i64
        let s_4_11: i64 = 128;
        // D s_4_12: read-var d:i64
        let s_4_12: i64 = fn_state.d;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: read-var result:u128
        let s_4_14: u128 = fn_state.result;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 128u16);
        // D s_4_16: call V_set(s_4_13, s_4_11, s_4_15)
        let s_4_16: () = V_set(state, tracer, s_4_13, s_4_11, s_4_15);
        // N s_4_17: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
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
