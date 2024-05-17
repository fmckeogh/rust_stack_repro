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
use BFDotAdd::*;
use Elem_set::*;
use V_read::*;
use Elem_read::*;
use FPCR_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_mul_int_bfdot<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        operand3: Bits,
        gs_144035: i64,
        datasizeshadow_1074: i64,
        gs_683834: Bits,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
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
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1074 <= s_0_2
        fn_state.datasizeshadow_1074 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#1074:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1074;
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
        // D s_1_7: read-var datasizeshadow#1074:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1074;
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
        // D s_1_14: read-var datasizeshadow#1074:i64
        let s_1_14: i64 = fn_state.datasizeshadow_1074;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var d:i64
        let s_1_17: i64 = fn_state.d;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: call V_read(s_1_18, s_1_16)
        let s_1_19: Bits = V_read(state, tracer, s_1_18, s_1_16);
        // D s_1_20: write-var operand3 <= s_1_19
        fn_state.operand3 = s_1_19;
        // C s_1_21: const #0s : i64
        let s_1_21: i64 = 0;
        // C s_1_22: const #1s : i
        let s_1_22: i128 = 1;
        // D s_1_23: read-var elements:i64
        let s_1_23: i64 = fn_state.elements;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: sub s_1_24 s_1_22
        let s_1_25: i128 = ((s_1_24) - (s_1_22));
        // D s_1_26: cast reint s_1_25 -> i64
        let s_1_26: i64 = (s_1_25 as i64);
        // D s_1_27: write-var gs#144035 <= s_1_26
        fn_state.gs_144035 = s_1_26;
        // D s_1_28: write-var e <= s_1_21
        fn_state.e = s_1_21;
        // N s_1_29: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#144035:i64
        let s_2_1: i64 = fn_state.gs_144035;
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
        // C s_3_5: const #0s : i
        let s_3_5: i128 = 0;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #16s : i64
        let s_3_9: i64 = 16;
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_11: cast zx s_3_9 -> i
        let s_3_11: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_12: read-var operand1:bv
        let s_3_12: Bits = fn_state.operand1;
        // D s_3_13: call Elem_read(s_3_12, s_3_10, s_3_11)
        let s_3_13: Bits = Elem_read(state, tracer, s_3_12, s_3_10, s_3_11);
        // D s_3_14: cast reint s_3_13 -> u16
        let s_3_14: u16 = (s_3_13.value() as u16);
        // C s_3_15: const #2s : i
        let s_3_15: i128 = 2;
        // D s_3_16: read-var e:i64
        let s_3_16: i64 = fn_state.e;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: mul s_3_15 s_3_17
        let s_3_18: i128 = ((s_3_15) * (s_3_17));
        // D s_3_19: cast reint s_3_18 -> i64
        let s_3_19: i64 = (s_3_18 as i64);
        // C s_3_20: const #1s : i
        let s_3_20: i128 = 1;
        // D s_3_21: cast zx s_3_19 -> i
        let s_3_21: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_22: add s_3_21 s_3_20
        let s_3_22: i128 = (s_3_21 + s_3_20);
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #16s : i64
        let s_3_24: i64 = 16;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // C s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_27: read-var operand1:bv
        let s_3_27: Bits = fn_state.operand1;
        // D s_3_28: call Elem_read(s_3_27, s_3_25, s_3_26)
        let s_3_28: Bits = Elem_read(state, tracer, s_3_27, s_3_25, s_3_26);
        // D s_3_29: cast reint s_3_28 -> u16
        let s_3_29: u16 = (s_3_28.value() as u16);
        // C s_3_30: const #2s : i
        let s_3_30: i128 = 2;
        // D s_3_31: read-var e:i64
        let s_3_31: i64 = fn_state.e;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: mul s_3_30 s_3_32
        let s_3_33: i128 = ((s_3_30) * (s_3_32));
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // C s_3_35: const #0s : i
        let s_3_35: i128 = 0;
        // D s_3_36: cast zx s_3_34 -> i
        let s_3_36: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_37: add s_3_36 s_3_35
        let s_3_37: i128 = (s_3_36 + s_3_35);
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // C s_3_39: const #16s : i64
        let s_3_39: i64 = 16;
        // D s_3_40: cast zx s_3_38 -> i
        let s_3_40: i128 = (i128::try_from(s_3_38).unwrap());
        // C s_3_41: cast zx s_3_39 -> i
        let s_3_41: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_42: read-var operand2:bv
        let s_3_42: Bits = fn_state.operand2;
        // D s_3_43: call Elem_read(s_3_42, s_3_40, s_3_41)
        let s_3_43: Bits = Elem_read(state, tracer, s_3_42, s_3_40, s_3_41);
        // D s_3_44: cast reint s_3_43 -> u16
        let s_3_44: u16 = (s_3_43.value() as u16);
        // C s_3_45: const #2s : i
        let s_3_45: i128 = 2;
        // D s_3_46: read-var e:i64
        let s_3_46: i64 = fn_state.e;
        // D s_3_47: cast zx s_3_46 -> i
        let s_3_47: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_48: mul s_3_45 s_3_47
        let s_3_48: i128 = ((s_3_45) * (s_3_47));
        // D s_3_49: cast reint s_3_48 -> i64
        let s_3_49: i64 = (s_3_48 as i64);
        // C s_3_50: const #1s : i
        let s_3_50: i128 = 1;
        // D s_3_51: cast zx s_3_49 -> i
        let s_3_51: i128 = (i128::try_from(s_3_49).unwrap());
        // D s_3_52: add s_3_51 s_3_50
        let s_3_52: i128 = (s_3_51 + s_3_50);
        // D s_3_53: cast reint s_3_52 -> i64
        let s_3_53: i64 = (s_3_52 as i64);
        // C s_3_54: const #16s : i64
        let s_3_54: i64 = 16;
        // D s_3_55: cast zx s_3_53 -> i
        let s_3_55: i128 = (i128::try_from(s_3_53).unwrap());
        // C s_3_56: cast zx s_3_54 -> i
        let s_3_56: i128 = (i128::try_from(s_3_54).unwrap());
        // D s_3_57: read-var operand2:bv
        let s_3_57: Bits = fn_state.operand2;
        // D s_3_58: call Elem_read(s_3_57, s_3_55, s_3_56)
        let s_3_58: Bits = Elem_read(state, tracer, s_3_57, s_3_55, s_3_56);
        // D s_3_59: cast reint s_3_58 -> u16
        let s_3_59: u16 = (s_3_58.value() as u16);
        // C s_3_60: const #32s : i64
        let s_3_60: i64 = 32;
        // D s_3_61: read-var e:i64
        let s_3_61: i64 = fn_state.e;
        // D s_3_62: cast zx s_3_61 -> i
        let s_3_62: i128 = (i128::try_from(s_3_61).unwrap());
        // C s_3_63: cast zx s_3_60 -> i
        let s_3_63: i128 = (i128::try_from(s_3_60).unwrap());
        // D s_3_64: read-var operand3:bv
        let s_3_64: Bits = fn_state.operand3;
        // D s_3_65: call Elem_read(s_3_64, s_3_62, s_3_63)
        let s_3_65: Bits = Elem_read(state, tracer, s_3_64, s_3_62, s_3_63);
        // D s_3_66: cast reint s_3_65 -> u32
        let s_3_66: u32 = (s_3_65.value() as u32);
        // C s_3_67: const #() : ()
        let s_3_67: () = ();
        // S s_3_68: call FPCR_read(s_3_67)
        let s_3_68: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_3_67);
        // D s_3_69: cast zx s_3_66 -> bv
        let s_3_69: Bits = Bits::new(s_3_66 as u128, 32u16);
        // D s_3_70: cast zx s_3_14 -> bv
        let s_3_70: Bits = Bits::new(s_3_14 as u128, 16u16);
        // D s_3_71: cast zx s_3_29 -> bv
        let s_3_71: Bits = Bits::new(s_3_29 as u128, 16u16);
        // D s_3_72: cast zx s_3_44 -> bv
        let s_3_72: Bits = Bits::new(s_3_44 as u128, 16u16);
        // D s_3_73: cast zx s_3_59 -> bv
        let s_3_73: Bits = Bits::new(s_3_59 as u128, 16u16);
        // D s_3_74: call BFDotAdd(s_3_69, s_3_70, s_3_71, s_3_72, s_3_73, s_3_68)
        let s_3_74: Bits = BFDotAdd(
            state,
            tracer,
            s_3_69,
            s_3_70,
            s_3_71,
            s_3_72,
            s_3_73,
            s_3_68,
        );
        // D s_3_75: write-var gs#683834 <= s_3_74
        fn_state.gs_683834 = s_3_74;
        // N s_3_76: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#683834:bv
        let s_4_0: Bits = fn_state.gs_683834;
        // D s_4_1: cast reint s_4_0 -> u32
        let s_4_1: u32 = (s_4_0.value() as u32);
        // C s_4_2: const #32s : i64
        let s_4_2: i64 = 32;
        // D s_4_3: read-var e:i64
        let s_4_3: i64 = fn_state.e;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: cast zx s_4_1 -> bv
        let s_4_6: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_7: read-var result:bv
        let s_4_7: Bits = fn_state.result;
        // D s_4_8: call Elem_set(s_4_7, s_4_4, s_4_5, s_4_6)
        let s_4_8: Bits = Elem_set(state, tracer, s_4_7, s_4_4, s_4_5, s_4_6);
        // D s_4_9: write-var result <= s_4_8
        fn_state.result = s_4_8;
        // D s_4_10: read-var e:i64
        let s_4_10: i64 = fn_state.e;
        // C s_4_11: const #1s : i64
        let s_4_11: i64 = 1;
        // D s_4_12: add s_4_10 s_4_11
        let s_4_12: i64 = (s_4_10 + s_4_11);
        // D s_4_13: write-var e <= s_4_12
        fn_state.e = s_4_12;
        // N s_4_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1074:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1074;
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
