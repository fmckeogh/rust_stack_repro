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
use BFMulAddH::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use FPCR_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_element_mul_acc_bf16_long<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i64,
    index: i64,
    m: i64,
    n: i64,
    sel: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: u16,
        e: i64,
        operand3: u128,
        gs_684021: Bits,
        result: u128,
        operand1: u128,
        gs_144185: i64,
        d: i64,
        elements: i64,
        index: i64,
        m: i64,
        n: i64,
        sel: i64,
    }
    let fn_state = FunctionState {
        d,
        elements,
        index,
        m,
        n,
        sel,
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
        // D s_1_5: write-var operand1 <= s_1_4
        fn_state.operand1 = s_1_4;
        // C s_1_6: const #128s : i64
        let s_1_6: i64 = 128;
        // D s_1_7: read-var m:i64
        let s_1_7: i64 = fn_state.m;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: cast reint s_1_9 -> u128
        let s_1_10: u128 = (s_1_9.value() as u128);
        // C s_1_11: const #128s : i64
        let s_1_11: i64 = 128;
        // D s_1_12: read-var d:i64
        let s_1_12: i64 = fn_state.d;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: call V_read(s_1_13, s_1_11)
        let s_1_14: Bits = V_read(state, tracer, s_1_13, s_1_11);
        // D s_1_15: cast reint s_1_14 -> u128
        let s_1_15: u128 = (s_1_14.value() as u128);
        // D s_1_16: write-var operand3 <= s_1_15
        fn_state.operand3 = s_1_15;
        // C s_1_17: const #16s : i64
        let s_1_17: i64 = 16;
        // D s_1_18: cast zx s_1_10 -> bv
        let s_1_18: Bits = Bits::new(s_1_10 as u128, 128u16);
        // D s_1_19: read-var index:i64
        let s_1_19: i64 = fn_state.index;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // C s_1_21: cast zx s_1_17 -> i
        let s_1_21: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_22: call Elem_read(s_1_18, s_1_20, s_1_21)
        let s_1_22: Bits = Elem_read(state, tracer, s_1_18, s_1_20, s_1_21);
        // D s_1_23: cast reint s_1_22 -> u16
        let s_1_23: u16 = (s_1_22.value() as u16);
        // D s_1_24: write-var element2 <= s_1_23
        fn_state.element2 = s_1_23;
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
        // D s_1_31: write-var gs#144185 <= s_1_30
        fn_state.gs_144185 = s_1_30;
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
        // D s_2_1: read-var gs#144185:i64
        let s_2_1: i64 = fn_state.gs_144185;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: read-var sel:i64
        let s_3_6: i64 = fn_state.sel;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_5 s_3_7
        let s_3_8: i128 = (s_3_5 + s_3_7);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // C s_3_10: const #16s : i64
        let s_3_10: i64 = 16;
        // D s_3_11: read-var operand1:u128
        let s_3_11: u128 = fn_state.operand1;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 128u16);
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // C s_3_14: cast zx s_3_10 -> i
        let s_3_14: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_15: call Elem_read(s_3_12, s_3_13, s_3_14)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_12, s_3_13, s_3_14);
        // D s_3_16: cast reint s_3_15 -> u16
        let s_3_16: u16 = (s_3_15.value() as u16);
        // C s_3_17: const #32s : i64
        let s_3_17: i64 = 32;
        // D s_3_18: read-var operand3:u128
        let s_3_18: u128 = fn_state.operand3;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 128u16);
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // C s_3_22: cast zx s_3_17 -> i
        let s_3_22: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_23: call Elem_read(s_3_19, s_3_21, s_3_22)
        let s_3_23: Bits = Elem_read(state, tracer, s_3_19, s_3_21, s_3_22);
        // D s_3_24: cast reint s_3_23 -> u32
        let s_3_24: u32 = (s_3_23.value() as u32);
        // C s_3_25: const #() : ()
        let s_3_25: () = ();
        // S s_3_26: call FPCR_read(s_3_25)
        let s_3_26: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_25);
        // D s_3_27: cast zx s_3_24 -> bv
        let s_3_27: Bits = Bits::new(s_3_24 as u128, 32u16);
        // D s_3_28: cast zx s_3_16 -> bv
        let s_3_28: Bits = Bits::new(s_3_16 as u128, 16u16);
        // D s_3_29: read-var element2:u16
        let s_3_29: u16 = fn_state.element2;
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 16u16);
        // D s_3_31: call BFMulAddH(s_3_27, s_3_28, s_3_30, s_3_26)
        let s_3_31: Bits = BFMulAddH(state, tracer, s_3_27, s_3_28, s_3_30, s_3_26);
        // D s_3_32: write-var gs#684021 <= s_3_31
        fn_state.gs_684021 = s_3_31;
        // N s_3_33: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#684021:bv
        let s_4_0: Bits = fn_state.gs_684021;
        // D s_4_1: cast reint s_4_0 -> u32
        let s_4_1: u32 = (s_4_0.value() as u32);
        // D s_4_2: read-var result:u128
        let s_4_2: u128 = fn_state.result;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 128u16);
        // D s_4_4: read-var e:i64
        let s_4_4: i64 = fn_state.e;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // C s_4_6: const #32s : i64
        let s_4_6: i64 = 32;
        // C s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: cast zx s_4_1 -> bv
        let s_4_8: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_9: call Elem_set(s_4_3, s_4_5, s_4_7, s_4_8)
        let s_4_9: Bits = Elem_set(state, tracer, s_4_3, s_4_5, s_4_7, s_4_8);
        // D s_4_10: cast reint s_4_9 -> u128
        let s_4_10: u128 = (s_4_9.value() as u128);
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
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: read-var result:u128
        let s_5_3: u128 = fn_state.result;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 128u16);
        // D s_5_5: call V_set(s_5_2, s_5_0, s_5_4)
        let s_5_5: () = V_set(state, tracer, s_5_2, s_5_0, s_5_4);
        // N s_5_6: return
        return;
    }
}
