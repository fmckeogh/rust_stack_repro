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
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use D_set::*;
use Din_read::*;
use FPNeg::*;
use Elem_read::*;
use FPMulAdd::*;
use D_read::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch32_instrs_VCMLA_idx_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
    regs: i64,
    rot: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        result1: Bits,
        element4: Bits,
        element3shadow_7985: Bits,
        element2: Bits,
        element1: Bits,
        element4shadow_7984: Bits,
        operand1: u64,
        operand2: u64,
        result2: Bits,
        e: i64,
        element3: Bits,
        operand3: u64,
        gs_326352: i64,
        gs_326342: i64,
        d: i128,
        esizeshadow_7983: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
        regs: i64,
        rot: u8,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        index,
        m,
        n,
        regs,
        rot,
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
        // D s_0_3: write-var esizeshadow#7983 <= s_0_2
        fn_state.esizeshadow_7983 = s_0_2;
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
        // D s_1_6: write-var gs#326342 <= s_1_5
        fn_state.gs_326342 = s_1_5;
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
        // D s_2_1: read-var gs#326342:i64
        let s_2_1: i64 = fn_state.gs_326342;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b20 b3
        if s_2_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_7: call D_read(s_3_6)
        let s_3_7: u64 = D_read(state, tracer, s_3_6);
        // D s_3_8: write-var operand1 <= s_3_7
        fn_state.operand1 = s_3_7;
        // D s_3_9: read-var m:i64
        let s_3_9: i64 = fn_state.m;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: call Din_read(s_3_10)
        let s_3_11: u64 = Din_read(state, tracer, s_3_10);
        // D s_3_12: write-var operand2 <= s_3_11
        fn_state.operand2 = s_3_11;
        // D s_3_13: read-var r:i64
        let s_3_13: i64 = fn_state.r;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var d:i
        let s_3_15: i128 = fn_state.d;
        // D s_3_16: add s_3_15 s_3_14
        let s_3_16: i128 = (s_3_15 + s_3_14);
        // D s_3_17: call D_read(s_3_16)
        let s_3_17: u64 = D_read(state, tracer, s_3_16);
        // D s_3_18: write-var operand3 <= s_3_17
        fn_state.operand3 = s_3_17;
        // C s_3_19: const #0s : i64
        let s_3_19: i64 = 0;
        // C s_3_20: const #2s : i
        let s_3_20: i128 = 2;
        // D s_3_21: read-var elements:i
        let s_3_21: i128 = fn_state.elements;
        // D s_3_22: call fdiv_int(s_3_21, s_3_20)
        let s_3_22: i128 = fdiv_int(state, tracer, s_3_21, s_3_20);
        // C s_3_23: const #1s : i
        let s_3_23: i128 = 1;
        // D s_3_24: sub s_3_22 s_3_23
        let s_3_24: i128 = ((s_3_22) - (s_3_23));
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // D s_3_26: write-var gs#326352 <= s_3_25
        fn_state.gs_326352 = s_3_25;
        // D s_3_27: write-var e <= s_3_19
        fn_state.e = s_3_19;
        // N s_3_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#326352:i64
        let s_4_1: i64 = fn_state.gs_326352;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b19 b5
        if s_4_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rot:u8
        let s_5_0: u8 = fn_state.rot;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #0u : u8
        let s_5_2: u8 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b10 b6
        if s_5_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var index:i64
        let s_6_1: i64 = fn_state.index;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: mul s_6_2 s_6_0
        let s_6_3: i128 = ((s_6_2) * (s_6_0));
        // D s_6_4: cast reint s_6_3 -> i64
        let s_6_4: i64 = (s_6_3 as i64);
        // D s_6_5: read-var esizeshadow#7983:i64
        let s_6_5: i64 = fn_state.esizeshadow_7983;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // D s_6_8: read-var operand2:u64
        let s_6_8: u64 = fn_state.operand2;
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_10: cast zx s_6_4 -> i
        let s_6_10: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_11: cast zx s_6_7 -> i
        let s_6_11: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_12: call Elem_read(s_6_9, s_6_10, s_6_11)
        let s_6_12: Bits = Elem_read(state, tracer, s_6_9, s_6_10, s_6_11);
        // D s_6_13: write-var element1 <= s_6_12
        fn_state.element1 = s_6_12;
        // C s_6_14: const #2s : i
        let s_6_14: i128 = 2;
        // D s_6_15: read-var e:i64
        let s_6_15: i64 = fn_state.e;
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: mul s_6_16 s_6_14
        let s_6_17: i128 = ((s_6_16) * (s_6_14));
        // D s_6_18: read-var esizeshadow#7983:i64
        let s_6_18: i64 = fn_state.esizeshadow_7983;
        // D s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // D s_6_20: cast reint s_6_19 -> i64
        let s_6_20: i64 = (s_6_19 as i64);
        // D s_6_21: read-var operand1:u64
        let s_6_21: u64 = fn_state.operand1;
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 64u16);
        // D s_6_23: cast zx s_6_20 -> i
        let s_6_23: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_24: call Elem_read(s_6_22, s_6_17, s_6_23)
        let s_6_24: Bits = Elem_read(state, tracer, s_6_22, s_6_17, s_6_23);
        // D s_6_25: write-var element2 <= s_6_24
        fn_state.element2 = s_6_24;
        // C s_6_26: const #2s : i
        let s_6_26: i128 = 2;
        // D s_6_27: read-var index:i64
        let s_6_27: i64 = fn_state.index;
        // D s_6_28: cast zx s_6_27 -> i
        let s_6_28: i128 = (i128::try_from(s_6_27).unwrap());
        // D s_6_29: mul s_6_28 s_6_26
        let s_6_29: i128 = ((s_6_28) * (s_6_26));
        // D s_6_30: cast reint s_6_29 -> i64
        let s_6_30: i64 = (s_6_29 as i64);
        // C s_6_31: const #1s : i
        let s_6_31: i128 = 1;
        // D s_6_32: cast zx s_6_30 -> i
        let s_6_32: i128 = (i128::try_from(s_6_30).unwrap());
        // D s_6_33: add s_6_32 s_6_31
        let s_6_33: i128 = (s_6_32 + s_6_31);
        // D s_6_34: cast reint s_6_33 -> i64
        let s_6_34: i64 = (s_6_33 as i64);
        // D s_6_35: read-var esizeshadow#7983:i64
        let s_6_35: i64 = fn_state.esizeshadow_7983;
        // D s_6_36: cast zx s_6_35 -> i
        let s_6_36: i128 = (i128::try_from(s_6_35).unwrap());
        // D s_6_37: cast reint s_6_36 -> i64
        let s_6_37: i64 = (s_6_36 as i64);
        // D s_6_38: read-var operand2:u64
        let s_6_38: u64 = fn_state.operand2;
        // D s_6_39: cast zx s_6_38 -> bv
        let s_6_39: Bits = Bits::new(s_6_38 as u128, 64u16);
        // D s_6_40: cast zx s_6_34 -> i
        let s_6_40: i128 = (i128::try_from(s_6_34).unwrap());
        // D s_6_41: cast zx s_6_37 -> i
        let s_6_41: i128 = (i128::try_from(s_6_37).unwrap());
        // D s_6_42: call Elem_read(s_6_39, s_6_40, s_6_41)
        let s_6_42: Bits = Elem_read(state, tracer, s_6_39, s_6_40, s_6_41);
        // D s_6_43: write-var element3 <= s_6_42
        fn_state.element3 = s_6_42;
        // C s_6_44: const #2s : i
        let s_6_44: i128 = 2;
        // D s_6_45: read-var e:i64
        let s_6_45: i64 = fn_state.e;
        // D s_6_46: cast zx s_6_45 -> i
        let s_6_46: i128 = (i128::try_from(s_6_45).unwrap());
        // D s_6_47: mul s_6_46 s_6_44
        let s_6_47: i128 = ((s_6_46) * (s_6_44));
        // D s_6_48: read-var esizeshadow#7983:i64
        let s_6_48: i64 = fn_state.esizeshadow_7983;
        // D s_6_49: cast zx s_6_48 -> i
        let s_6_49: i128 = (i128::try_from(s_6_48).unwrap());
        // D s_6_50: cast reint s_6_49 -> i64
        let s_6_50: i64 = (s_6_49 as i64);
        // D s_6_51: read-var operand1:u64
        let s_6_51: u64 = fn_state.operand1;
        // D s_6_52: cast zx s_6_51 -> bv
        let s_6_52: Bits = Bits::new(s_6_51 as u128, 64u16);
        // D s_6_53: cast zx s_6_50 -> i
        let s_6_53: i128 = (i128::try_from(s_6_50).unwrap());
        // D s_6_54: call Elem_read(s_6_52, s_6_47, s_6_53)
        let s_6_54: Bits = Elem_read(state, tracer, s_6_52, s_6_47, s_6_53);
        // D s_6_55: write-var element4 <= s_6_54
        fn_state.element4 = s_6_54;
        // N s_6_56: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var element4:bv
        let s_7_0: Bits = fn_state.element4;
        // D s_7_1: write-var element4shadow#7984 <= s_7_0
        fn_state.element4shadow_7984 = s_7_0;
        // D s_7_2: read-var element3:bv
        let s_7_2: Bits = fn_state.element3;
        // D s_7_3: write-var element3shadow#7985 <= s_7_2
        fn_state.element3shadow_7985 = s_7_2;
        // D s_7_4: read-var element2:bv
        let s_7_4: Bits = fn_state.element2;
        // D s_7_5: read-var element1:bv
        let s_7_5: Bits = fn_state.element1;
        // C s_7_6: const #2s : i
        let s_7_6: i128 = 2;
        // D s_7_7: read-var e:i64
        let s_7_7: i64 = fn_state.e;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: mul s_7_8 s_7_6
        let s_7_9: i128 = ((s_7_8) * (s_7_6));
        // D s_7_10: read-var esizeshadow#7983:i64
        let s_7_10: i64 = fn_state.esizeshadow_7983;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: read-var operand3:u64
        let s_7_13: u64 = fn_state.operand3;
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 64u16);
        // D s_7_15: cast zx s_7_12 -> i
        let s_7_15: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_16: call Elem_read(s_7_14, s_7_9, s_7_15)
        let s_7_16: Bits = Elem_read(state, tracer, s_7_14, s_7_9, s_7_15);
        // C s_7_17: const #() : ()
        let s_7_17: () = ();
        // S s_7_18: call StandardFPSCRValue(s_7_17)
        let s_7_18: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_7_17,
        );
        // D s_7_19: call FPMulAdd(s_7_16, s_7_4, s_7_5, s_7_18)
        let s_7_19: Bits = FPMulAdd(state, tracer, s_7_16, s_7_4, s_7_5, s_7_18);
        // D s_7_20: write-var result1 <= s_7_19
        fn_state.result1 = s_7_19;
        // N s_7_21: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // D s_8_1: read-var e:i64
        let s_8_1: i64 = fn_state.e;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_2 s_8_0
        let s_8_3: i128 = ((s_8_2) * (s_8_0));
        // C s_8_4: const #1s : i
        let s_8_4: i128 = 1;
        // D s_8_5: add s_8_3 s_8_4
        let s_8_5: i128 = (s_8_3 + s_8_4);
        // D s_8_6: read-var esizeshadow#7983:i64
        let s_8_6: i64 = fn_state.esizeshadow_7983;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // D s_8_9: read-var operand3:u64
        let s_8_9: u64 = fn_state.operand3;
        // D s_8_10: cast zx s_8_9 -> bv
        let s_8_10: Bits = Bits::new(s_8_9 as u128, 64u16);
        // D s_8_11: cast zx s_8_8 -> i
        let s_8_11: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_12: call Elem_read(s_8_10, s_8_5, s_8_11)
        let s_8_12: Bits = Elem_read(state, tracer, s_8_10, s_8_5, s_8_11);
        // C s_8_13: const #() : ()
        let s_8_13: () = ();
        // S s_8_14: call StandardFPSCRValue(s_8_13)
        let s_8_14: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_8_13,
        );
        // D s_8_15: read-var element4shadow#7984:bv
        let s_8_15: Bits = fn_state.element4shadow_7984;
        // D s_8_16: read-var element3shadow#7985:bv
        let s_8_16: Bits = fn_state.element3shadow_7985;
        // D s_8_17: call FPMulAdd(s_8_12, s_8_15, s_8_16, s_8_14)
        let s_8_17: Bits = FPMulAdd(state, tracer, s_8_12, s_8_15, s_8_16, s_8_14);
        // D s_8_18: write-var result2 <= s_8_17
        fn_state.result2 = s_8_17;
        // N s_8_19: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var d:i
        let s_9_2: i128 = fn_state.d;
        // D s_9_3: add s_9_2 s_9_1
        let s_9_3: i128 = (s_9_2 + s_9_1);
        // D s_9_4: read-var r:i64
        let s_9_4: i64 = fn_state.r;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var d:i
        let s_9_6: i128 = fn_state.d;
        // D s_9_7: add s_9_6 s_9_5
        let s_9_7: i128 = (s_9_6 + s_9_5);
        // D s_9_8: call D_read(s_9_7)
        let s_9_8: u64 = D_read(state, tracer, s_9_7);
        // C s_9_9: const #2s : i
        let s_9_9: i128 = 2;
        // D s_9_10: read-var e:i64
        let s_9_10: i64 = fn_state.e;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: mul s_9_11 s_9_9
        let s_9_12: i128 = ((s_9_11) * (s_9_9));
        // D s_9_13: read-var esizeshadow#7983:i64
        let s_9_13: i64 = fn_state.esizeshadow_7983;
        // D s_9_14: cast zx s_9_13 -> i
        let s_9_14: i128 = (i128::try_from(s_9_13).unwrap());
        // D s_9_15: cast reint s_9_14 -> i64
        let s_9_15: i64 = (s_9_14 as i64);
        // D s_9_16: cast zx s_9_8 -> bv
        let s_9_16: Bits = Bits::new(s_9_8 as u128, 64u16);
        // D s_9_17: cast zx s_9_15 -> i
        let s_9_17: i128 = (i128::try_from(s_9_15).unwrap());
        // D s_9_18: read-var result1:bv
        let s_9_18: Bits = fn_state.result1;
        // D s_9_19: call Elem_set(s_9_16, s_9_12, s_9_17, s_9_18)
        let s_9_19: Bits = Elem_set(state, tracer, s_9_16, s_9_12, s_9_17, s_9_18);
        // D s_9_20: cast reint s_9_19 -> u64
        let s_9_20: u64 = (s_9_19.value() as u64);
        // D s_9_21: call D_set(s_9_3, s_9_20)
        let s_9_21: () = D_set(state, tracer, s_9_3, s_9_20);
        // D s_9_22: read-var r:i64
        let s_9_22: i64 = fn_state.r;
        // D s_9_23: cast zx s_9_22 -> i
        let s_9_23: i128 = (i128::try_from(s_9_22).unwrap());
        // D s_9_24: read-var d:i
        let s_9_24: i128 = fn_state.d;
        // D s_9_25: add s_9_24 s_9_23
        let s_9_25: i128 = (s_9_24 + s_9_23);
        // D s_9_26: read-var r:i64
        let s_9_26: i64 = fn_state.r;
        // D s_9_27: cast zx s_9_26 -> i
        let s_9_27: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_28: read-var d:i
        let s_9_28: i128 = fn_state.d;
        // D s_9_29: add s_9_28 s_9_27
        let s_9_29: i128 = (s_9_28 + s_9_27);
        // D s_9_30: call D_read(s_9_29)
        let s_9_30: u64 = D_read(state, tracer, s_9_29);
        // C s_9_31: const #2s : i
        let s_9_31: i128 = 2;
        // D s_9_32: read-var e:i64
        let s_9_32: i64 = fn_state.e;
        // D s_9_33: cast zx s_9_32 -> i
        let s_9_33: i128 = (i128::try_from(s_9_32).unwrap());
        // D s_9_34: mul s_9_33 s_9_31
        let s_9_34: i128 = ((s_9_33) * (s_9_31));
        // C s_9_35: const #1s : i
        let s_9_35: i128 = 1;
        // D s_9_36: add s_9_34 s_9_35
        let s_9_36: i128 = (s_9_34 + s_9_35);
        // D s_9_37: read-var esizeshadow#7983:i64
        let s_9_37: i64 = fn_state.esizeshadow_7983;
        // D s_9_38: cast zx s_9_37 -> i
        let s_9_38: i128 = (i128::try_from(s_9_37).unwrap());
        // D s_9_39: cast reint s_9_38 -> i64
        let s_9_39: i64 = (s_9_38 as i64);
        // D s_9_40: cast zx s_9_30 -> bv
        let s_9_40: Bits = Bits::new(s_9_30 as u128, 64u16);
        // D s_9_41: cast zx s_9_39 -> i
        let s_9_41: i128 = (i128::try_from(s_9_39).unwrap());
        // D s_9_42: read-var result2:bv
        let s_9_42: Bits = fn_state.result2;
        // D s_9_43: call Elem_set(s_9_40, s_9_36, s_9_41, s_9_42)
        let s_9_43: Bits = Elem_set(state, tracer, s_9_40, s_9_36, s_9_41, s_9_42);
        // D s_9_44: cast reint s_9_43 -> u64
        let s_9_44: u64 = (s_9_43.value() as u64);
        // D s_9_45: call D_set(s_9_25, s_9_44)
        let s_9_45: () = D_set(state, tracer, s_9_25, s_9_44);
        // D s_9_46: read-var e:i64
        let s_9_46: i64 = fn_state.e;
        // C s_9_47: const #1s : i64
        let s_9_47: i64 = 1;
        // D s_9_48: add s_9_46 s_9_47
        let s_9_48: i64 = (s_9_46 + s_9_47);
        // D s_9_49: write-var e <= s_9_48
        fn_state.e = s_9_48;
        // N s_9_50: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var rot:u8
        let s_10_0: u8 = fn_state.rot;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #1u : u8
        let s_10_2: u8 = 1;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b13 b11
        if s_10_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2s : i
        let s_11_0: i128 = 2;
        // D s_11_1: read-var index:i64
        let s_11_1: i64 = fn_state.index;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: mul s_11_2 s_11_0
        let s_11_3: i128 = ((s_11_2) * (s_11_0));
        // D s_11_4: cast reint s_11_3 -> i64
        let s_11_4: i64 = (s_11_3 as i64);
        // C s_11_5: const #1s : i
        let s_11_5: i128 = 1;
        // D s_11_6: cast zx s_11_4 -> i
        let s_11_6: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_7: add s_11_6 s_11_5
        let s_11_7: i128 = (s_11_6 + s_11_5);
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // D s_11_9: read-var esizeshadow#7983:i64
        let s_11_9: i64 = fn_state.esizeshadow_7983;
        // D s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_11: cast reint s_11_10 -> i64
        let s_11_11: i64 = (s_11_10 as i64);
        // D s_11_12: read-var operand2:u64
        let s_11_12: u64 = fn_state.operand2;
        // D s_11_13: cast zx s_11_12 -> bv
        let s_11_13: Bits = Bits::new(s_11_12 as u128, 64u16);
        // D s_11_14: cast zx s_11_8 -> i
        let s_11_14: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_15: cast zx s_11_11 -> i
        let s_11_15: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_16: call Elem_read(s_11_13, s_11_14, s_11_15)
        let s_11_16: Bits = Elem_read(state, tracer, s_11_13, s_11_14, s_11_15);
        // D s_11_17: call FPNeg(s_11_16)
        let s_11_17: Bits = FPNeg(state, tracer, s_11_16);
        // D s_11_18: write-var element1 <= s_11_17
        fn_state.element1 = s_11_17;
        // N s_11_19: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2s : i
        let s_12_0: i128 = 2;
        // D s_12_1: read-var e:i64
        let s_12_1: i64 = fn_state.e;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: mul s_12_2 s_12_0
        let s_12_3: i128 = ((s_12_2) * (s_12_0));
        // C s_12_4: const #1s : i
        let s_12_4: i128 = 1;
        // D s_12_5: add s_12_3 s_12_4
        let s_12_5: i128 = (s_12_3 + s_12_4);
        // D s_12_6: read-var esizeshadow#7983:i64
        let s_12_6: i64 = fn_state.esizeshadow_7983;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: read-var operand1:u64
        let s_12_9: u64 = fn_state.operand1;
        // D s_12_10: cast zx s_12_9 -> bv
        let s_12_10: Bits = Bits::new(s_12_9 as u128, 64u16);
        // D s_12_11: cast zx s_12_8 -> i
        let s_12_11: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_12: call Elem_read(s_12_10, s_12_5, s_12_11)
        let s_12_12: Bits = Elem_read(state, tracer, s_12_10, s_12_5, s_12_11);
        // D s_12_13: write-var element2 <= s_12_12
        fn_state.element2 = s_12_12;
        // C s_12_14: const #2s : i
        let s_12_14: i128 = 2;
        // D s_12_15: read-var index:i64
        let s_12_15: i64 = fn_state.index;
        // D s_12_16: cast zx s_12_15 -> i
        let s_12_16: i128 = (i128::try_from(s_12_15).unwrap());
        // D s_12_17: mul s_12_16 s_12_14
        let s_12_17: i128 = ((s_12_16) * (s_12_14));
        // D s_12_18: cast reint s_12_17 -> i64
        let s_12_18: i64 = (s_12_17 as i64);
        // D s_12_19: read-var esizeshadow#7983:i64
        let s_12_19: i64 = fn_state.esizeshadow_7983;
        // D s_12_20: cast zx s_12_19 -> i
        let s_12_20: i128 = (i128::try_from(s_12_19).unwrap());
        // D s_12_21: cast reint s_12_20 -> i64
        let s_12_21: i64 = (s_12_20 as i64);
        // D s_12_22: read-var operand2:u64
        let s_12_22: u64 = fn_state.operand2;
        // D s_12_23: cast zx s_12_22 -> bv
        let s_12_23: Bits = Bits::new(s_12_22 as u128, 64u16);
        // D s_12_24: cast zx s_12_18 -> i
        let s_12_24: i128 = (i128::try_from(s_12_18).unwrap());
        // D s_12_25: cast zx s_12_21 -> i
        let s_12_25: i128 = (i128::try_from(s_12_21).unwrap());
        // D s_12_26: call Elem_read(s_12_23, s_12_24, s_12_25)
        let s_12_26: Bits = Elem_read(state, tracer, s_12_23, s_12_24, s_12_25);
        // D s_12_27: write-var element3 <= s_12_26
        fn_state.element3 = s_12_26;
        // C s_12_28: const #2s : i
        let s_12_28: i128 = 2;
        // D s_12_29: read-var e:i64
        let s_12_29: i64 = fn_state.e;
        // D s_12_30: cast zx s_12_29 -> i
        let s_12_30: i128 = (i128::try_from(s_12_29).unwrap());
        // D s_12_31: mul s_12_30 s_12_28
        let s_12_31: i128 = ((s_12_30) * (s_12_28));
        // C s_12_32: const #1s : i
        let s_12_32: i128 = 1;
        // D s_12_33: add s_12_31 s_12_32
        let s_12_33: i128 = (s_12_31 + s_12_32);
        // D s_12_34: read-var esizeshadow#7983:i64
        let s_12_34: i64 = fn_state.esizeshadow_7983;
        // D s_12_35: cast zx s_12_34 -> i
        let s_12_35: i128 = (i128::try_from(s_12_34).unwrap());
        // D s_12_36: cast reint s_12_35 -> i64
        let s_12_36: i64 = (s_12_35 as i64);
        // D s_12_37: read-var operand1:u64
        let s_12_37: u64 = fn_state.operand1;
        // D s_12_38: cast zx s_12_37 -> bv
        let s_12_38: Bits = Bits::new(s_12_37 as u128, 64u16);
        // D s_12_39: cast zx s_12_36 -> i
        let s_12_39: i128 = (i128::try_from(s_12_36).unwrap());
        // D s_12_40: call Elem_read(s_12_38, s_12_33, s_12_39)
        let s_12_40: Bits = Elem_read(state, tracer, s_12_38, s_12_33, s_12_39);
        // D s_12_41: write-var element4 <= s_12_40
        fn_state.element4 = s_12_40;
        // N s_12_42: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var rot:u8
        let s_13_0: u8 = fn_state.rot;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b17 b14
        if s_13_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #2s : i
        let s_14_0: i128 = 2;
        // D s_14_1: read-var index:i64
        let s_14_1: i64 = fn_state.index;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: mul s_14_2 s_14_0
        let s_14_3: i128 = ((s_14_2) * (s_14_0));
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: read-var esizeshadow#7983:i64
        let s_14_5: i64 = fn_state.esizeshadow_7983;
        // D s_14_6: cast zx s_14_5 -> i
        let s_14_6: i128 = (i128::try_from(s_14_5).unwrap());
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // D s_14_8: read-var operand2:u64
        let s_14_8: u64 = fn_state.operand2;
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 64u16);
        // D s_14_10: cast zx s_14_4 -> i
        let s_14_10: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_11: cast zx s_14_7 -> i
        let s_14_11: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_12: call Elem_read(s_14_9, s_14_10, s_14_11)
        let s_14_12: Bits = Elem_read(state, tracer, s_14_9, s_14_10, s_14_11);
        // D s_14_13: call FPNeg(s_14_12)
        let s_14_13: Bits = FPNeg(state, tracer, s_14_12);
        // D s_14_14: write-var element1 <= s_14_13
        fn_state.element1 = s_14_13;
        // N s_14_15: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2s : i
        let s_15_0: i128 = 2;
        // D s_15_1: read-var e:i64
        let s_15_1: i64 = fn_state.e;
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: mul s_15_2 s_15_0
        let s_15_3: i128 = ((s_15_2) * (s_15_0));
        // D s_15_4: read-var esizeshadow#7983:i64
        let s_15_4: i64 = fn_state.esizeshadow_7983;
        // D s_15_5: cast zx s_15_4 -> i
        let s_15_5: i128 = (i128::try_from(s_15_4).unwrap());
        // D s_15_6: cast reint s_15_5 -> i64
        let s_15_6: i64 = (s_15_5 as i64);
        // D s_15_7: read-var operand1:u64
        let s_15_7: u64 = fn_state.operand1;
        // D s_15_8: cast zx s_15_7 -> bv
        let s_15_8: Bits = Bits::new(s_15_7 as u128, 64u16);
        // D s_15_9: cast zx s_15_6 -> i
        let s_15_9: i128 = (i128::try_from(s_15_6).unwrap());
        // D s_15_10: call Elem_read(s_15_8, s_15_3, s_15_9)
        let s_15_10: Bits = Elem_read(state, tracer, s_15_8, s_15_3, s_15_9);
        // D s_15_11: write-var element2 <= s_15_10
        fn_state.element2 = s_15_10;
        // C s_15_12: const #2s : i
        let s_15_12: i128 = 2;
        // D s_15_13: read-var index:i64
        let s_15_13: i64 = fn_state.index;
        // D s_15_14: cast zx s_15_13 -> i
        let s_15_14: i128 = (i128::try_from(s_15_13).unwrap());
        // D s_15_15: mul s_15_14 s_15_12
        let s_15_15: i128 = ((s_15_14) * (s_15_12));
        // D s_15_16: cast reint s_15_15 -> i64
        let s_15_16: i64 = (s_15_15 as i64);
        // C s_15_17: const #1s : i
        let s_15_17: i128 = 1;
        // D s_15_18: cast zx s_15_16 -> i
        let s_15_18: i128 = (i128::try_from(s_15_16).unwrap());
        // D s_15_19: add s_15_18 s_15_17
        let s_15_19: i128 = (s_15_18 + s_15_17);
        // D s_15_20: cast reint s_15_19 -> i64
        let s_15_20: i64 = (s_15_19 as i64);
        // D s_15_21: read-var esizeshadow#7983:i64
        let s_15_21: i64 = fn_state.esizeshadow_7983;
        // D s_15_22: cast zx s_15_21 -> i
        let s_15_22: i128 = (i128::try_from(s_15_21).unwrap());
        // D s_15_23: cast reint s_15_22 -> i64
        let s_15_23: i64 = (s_15_22 as i64);
        // D s_15_24: read-var operand2:u64
        let s_15_24: u64 = fn_state.operand2;
        // D s_15_25: cast zx s_15_24 -> bv
        let s_15_25: Bits = Bits::new(s_15_24 as u128, 64u16);
        // D s_15_26: cast zx s_15_20 -> i
        let s_15_26: i128 = (i128::try_from(s_15_20).unwrap());
        // D s_15_27: cast zx s_15_23 -> i
        let s_15_27: i128 = (i128::try_from(s_15_23).unwrap());
        // D s_15_28: call Elem_read(s_15_25, s_15_26, s_15_27)
        let s_15_28: Bits = Elem_read(state, tracer, s_15_25, s_15_26, s_15_27);
        // D s_15_29: call FPNeg(s_15_28)
        let s_15_29: Bits = FPNeg(state, tracer, s_15_28);
        // D s_15_30: write-var element3 <= s_15_29
        fn_state.element3 = s_15_29;
        // N s_15_31: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #2s : i
        let s_16_0: i128 = 2;
        // D s_16_1: read-var e:i64
        let s_16_1: i64 = fn_state.e;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: mul s_16_2 s_16_0
        let s_16_3: i128 = ((s_16_2) * (s_16_0));
        // D s_16_4: read-var esizeshadow#7983:i64
        let s_16_4: i64 = fn_state.esizeshadow_7983;
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: cast reint s_16_5 -> i64
        let s_16_6: i64 = (s_16_5 as i64);
        // D s_16_7: read-var operand1:u64
        let s_16_7: u64 = fn_state.operand1;
        // D s_16_8: cast zx s_16_7 -> bv
        let s_16_8: Bits = Bits::new(s_16_7 as u128, 64u16);
        // D s_16_9: cast zx s_16_6 -> i
        let s_16_9: i128 = (i128::try_from(s_16_6).unwrap());
        // D s_16_10: call Elem_read(s_16_8, s_16_3, s_16_9)
        let s_16_10: Bits = Elem_read(state, tracer, s_16_8, s_16_3, s_16_9);
        // D s_16_11: write-var element4 <= s_16_10
        fn_state.element4 = s_16_10;
        // N s_16_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #2s : i
        let s_17_0: i128 = 2;
        // D s_17_1: read-var index:i64
        let s_17_1: i64 = fn_state.index;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: mul s_17_2 s_17_0
        let s_17_3: i128 = ((s_17_2) * (s_17_0));
        // D s_17_4: cast reint s_17_3 -> i64
        let s_17_4: i64 = (s_17_3 as i64);
        // C s_17_5: const #1s : i
        let s_17_5: i128 = 1;
        // D s_17_6: cast zx s_17_4 -> i
        let s_17_6: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_7: add s_17_6 s_17_5
        let s_17_7: i128 = (s_17_6 + s_17_5);
        // D s_17_8: cast reint s_17_7 -> i64
        let s_17_8: i64 = (s_17_7 as i64);
        // D s_17_9: read-var esizeshadow#7983:i64
        let s_17_9: i64 = fn_state.esizeshadow_7983;
        // D s_17_10: cast zx s_17_9 -> i
        let s_17_10: i128 = (i128::try_from(s_17_9).unwrap());
        // D s_17_11: cast reint s_17_10 -> i64
        let s_17_11: i64 = (s_17_10 as i64);
        // D s_17_12: read-var operand2:u64
        let s_17_12: u64 = fn_state.operand2;
        // D s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 64u16);
        // D s_17_14: cast zx s_17_8 -> i
        let s_17_14: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_15: cast zx s_17_11 -> i
        let s_17_15: i128 = (i128::try_from(s_17_11).unwrap());
        // D s_17_16: call Elem_read(s_17_13, s_17_14, s_17_15)
        let s_17_16: Bits = Elem_read(state, tracer, s_17_13, s_17_14, s_17_15);
        // D s_17_17: write-var element1 <= s_17_16
        fn_state.element1 = s_17_16;
        // C s_17_18: const #2s : i
        let s_17_18: i128 = 2;
        // D s_17_19: read-var e:i64
        let s_17_19: i64 = fn_state.e;
        // D s_17_20: cast zx s_17_19 -> i
        let s_17_20: i128 = (i128::try_from(s_17_19).unwrap());
        // D s_17_21: mul s_17_20 s_17_18
        let s_17_21: i128 = ((s_17_20) * (s_17_18));
        // C s_17_22: const #1s : i
        let s_17_22: i128 = 1;
        // D s_17_23: add s_17_21 s_17_22
        let s_17_23: i128 = (s_17_21 + s_17_22);
        // D s_17_24: read-var esizeshadow#7983:i64
        let s_17_24: i64 = fn_state.esizeshadow_7983;
        // D s_17_25: cast zx s_17_24 -> i
        let s_17_25: i128 = (i128::try_from(s_17_24).unwrap());
        // D s_17_26: cast reint s_17_25 -> i64
        let s_17_26: i64 = (s_17_25 as i64);
        // D s_17_27: read-var operand1:u64
        let s_17_27: u64 = fn_state.operand1;
        // D s_17_28: cast zx s_17_27 -> bv
        let s_17_28: Bits = Bits::new(s_17_27 as u128, 64u16);
        // D s_17_29: cast zx s_17_26 -> i
        let s_17_29: i128 = (i128::try_from(s_17_26).unwrap());
        // D s_17_30: call Elem_read(s_17_28, s_17_23, s_17_29)
        let s_17_30: Bits = Elem_read(state, tracer, s_17_28, s_17_23, s_17_29);
        // D s_17_31: write-var element2 <= s_17_30
        fn_state.element2 = s_17_30;
        // C s_17_32: const #2s : i
        let s_17_32: i128 = 2;
        // D s_17_33: read-var index:i64
        let s_17_33: i64 = fn_state.index;
        // D s_17_34: cast zx s_17_33 -> i
        let s_17_34: i128 = (i128::try_from(s_17_33).unwrap());
        // D s_17_35: mul s_17_34 s_17_32
        let s_17_35: i128 = ((s_17_34) * (s_17_32));
        // D s_17_36: cast reint s_17_35 -> i64
        let s_17_36: i64 = (s_17_35 as i64);
        // D s_17_37: read-var esizeshadow#7983:i64
        let s_17_37: i64 = fn_state.esizeshadow_7983;
        // D s_17_38: cast zx s_17_37 -> i
        let s_17_38: i128 = (i128::try_from(s_17_37).unwrap());
        // D s_17_39: cast reint s_17_38 -> i64
        let s_17_39: i64 = (s_17_38 as i64);
        // D s_17_40: read-var operand2:u64
        let s_17_40: u64 = fn_state.operand2;
        // D s_17_41: cast zx s_17_40 -> bv
        let s_17_41: Bits = Bits::new(s_17_40 as u128, 64u16);
        // D s_17_42: cast zx s_17_36 -> i
        let s_17_42: i128 = (i128::try_from(s_17_36).unwrap());
        // D s_17_43: cast zx s_17_39 -> i
        let s_17_43: i128 = (i128::try_from(s_17_39).unwrap());
        // D s_17_44: call Elem_read(s_17_41, s_17_42, s_17_43)
        let s_17_44: Bits = Elem_read(state, tracer, s_17_41, s_17_42, s_17_43);
        // D s_17_45: call FPNeg(s_17_44)
        let s_17_45: Bits = FPNeg(state, tracer, s_17_44);
        // D s_17_46: write-var element3 <= s_17_45
        fn_state.element3 = s_17_45;
        // N s_17_47: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2s : i
        let s_18_0: i128 = 2;
        // D s_18_1: read-var e:i64
        let s_18_1: i64 = fn_state.e;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: mul s_18_2 s_18_0
        let s_18_3: i128 = ((s_18_2) * (s_18_0));
        // C s_18_4: const #1s : i
        let s_18_4: i128 = 1;
        // D s_18_5: add s_18_3 s_18_4
        let s_18_5: i128 = (s_18_3 + s_18_4);
        // D s_18_6: read-var esizeshadow#7983:i64
        let s_18_6: i64 = fn_state.esizeshadow_7983;
        // D s_18_7: cast zx s_18_6 -> i
        let s_18_7: i128 = (i128::try_from(s_18_6).unwrap());
        // D s_18_8: cast reint s_18_7 -> i64
        let s_18_8: i64 = (s_18_7 as i64);
        // D s_18_9: read-var operand1:u64
        let s_18_9: u64 = fn_state.operand1;
        // D s_18_10: cast zx s_18_9 -> bv
        let s_18_10: Bits = Bits::new(s_18_9 as u128, 64u16);
        // D s_18_11: cast zx s_18_8 -> i
        let s_18_11: i128 = (i128::try_from(s_18_8).unwrap());
        // D s_18_12: call Elem_read(s_18_10, s_18_5, s_18_11)
        let s_18_12: Bits = Elem_read(state, tracer, s_18_10, s_18_5, s_18_11);
        // D s_18_13: write-var element4 <= s_18_12
        fn_state.element4 = s_18_12;
        // N s_18_14: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var r:i64
        let s_19_0: i64 = fn_state.r;
        // C s_19_1: const #1s : i64
        let s_19_1: i64 = 1;
        // D s_19_2: add s_19_0 s_19_1
        let s_19_2: i64 = (s_19_0 + s_19_1);
        // D s_19_3: write-var r <= s_19_2
        fn_state.r = s_19_2;
        // N s_19_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
}
