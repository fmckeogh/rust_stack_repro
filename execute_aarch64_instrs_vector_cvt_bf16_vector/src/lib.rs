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
use Vpart_set::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use FPCR_read::*;
use FPConvertBF__1::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_cvt_bf16_vector<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: u128,
        gs_143972: i64,
        gs_683680: Bits,
        e: i64,
        result: u64,
        d: i64,
        elements: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        d,
        elements,
        n,
        part,
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
        // S s_0_1: call CheckFPAdvSIMDEnabled64(s_0_0)
        let s_0_1: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // D s_1_5: write-var operand <= s_1_4
        fn_state.operand = s_1_4;
        // C s_1_6: const #0s : i64
        let s_1_6: i64 = 0;
        // C s_1_7: const #1s : i
        let s_1_7: i128 = 1;
        // D s_1_8: read-var elements:i64
        let s_1_8: i64 = fn_state.elements;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: sub s_1_9 s_1_7
        let s_1_10: i128 = ((s_1_9) - (s_1_7));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: write-var gs#143972 <= s_1_11
        fn_state.gs_143972 = s_1_11;
        // D s_1_13: write-var e <= s_1_6
        fn_state.e = s_1_6;
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
        // D s_2_1: read-var gs#143972:i64
        let s_2_1: i64 = fn_state.gs_143972;
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
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: read-var operand:u128
        let s_3_1: u128 = fn_state.operand;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 128u16);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: call Elem_read(s_3_2, s_3_4, s_3_5)
        let s_3_6: Bits = Elem_read(state, tracer, s_3_2, s_3_4, s_3_5);
        // D s_3_7: cast reint s_3_6 -> u32
        let s_3_7: u32 = (s_3_6.value() as u32);
        // C s_3_8: const #() : ()
        let s_3_8: () = ();
        // S s_3_9: call FPCR_read(s_3_8)
        let s_3_9: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_8);
        // D s_3_10: cast zx s_3_7 -> bv
        let s_3_10: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_11: call FPConvertBF__1(s_3_10, s_3_9)
        let s_3_11: Bits = FPConvertBF__1(state, tracer, s_3_10, s_3_9);
        // D s_3_12: write-var gs#683680 <= s_3_11
        fn_state.gs_683680 = s_3_11;
        // N s_3_13: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#683680:bv
        let s_4_0: Bits = fn_state.gs_683680;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // D s_4_2: read-var result:u64
        let s_4_2: u64 = fn_state.result;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_4: read-var e:i64
        let s_4_4: i64 = fn_state.e;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // C s_4_6: const #16s : i64
        let s_4_6: i64 = 16;
        // C s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: cast zx s_4_1 -> bv
        let s_4_8: Bits = Bits::new(s_4_1 as u128, 16u16);
        // D s_4_9: call Elem_set(s_4_3, s_4_5, s_4_7, s_4_8)
        let s_4_9: Bits = Elem_set(state, tracer, s_4_3, s_4_5, s_4_7, s_4_8);
        // D s_4_10: cast reint s_4_9 -> u64
        let s_4_10: u64 = (s_4_9.value() as u64);
        // D s_4_11: write-var result <= s_4_10
        fn_state.result = s_4_10;
        // D s_4_12: read-var e:i64
        let s_4_12: i64 = fn_state.e;
        // C s_4_13: const #1s : i64
        let s_4_13: i64 = 1;
        // D s_4_14: add s_4_12 s_4_13
        let s_4_14: i64 = (s_4_12 + s_4_13);
        // D s_4_15: write-var e <= s_4_14
        fn_state.e = s_4_14;
        // N s_4_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: read-var part:i64
        let s_5_3: i64 = fn_state.part;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: cast zx s_5_0 -> i
        let s_5_5: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_6: read-var result:u64
        let s_5_6: u64 = fn_state.result;
        // D s_5_7: cast zx s_5_6 -> bv
        let s_5_7: Bits = Bits::new(s_5_6 as u128, 64u16);
        // D s_5_8: call Vpart_set(s_5_2, s_5_4, s_5_5, s_5_7)
        let s_5_8: () = Vpart_set(state, tracer, s_5_2, s_5_4, s_5_5, s_5_7);
        // N s_5_9: return
        return;
    }
}
