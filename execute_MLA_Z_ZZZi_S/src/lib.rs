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
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use integer_subrange::*;
use Z_set::*;
use common::*;
pub fn execute_MLA_Z_ZZZi_S<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    da: i64,
    esize: i64,
    index: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: Bits,
        VLshadow_3894: i64,
        e: i64,
        VLshadow_3895: i64,
        gs_211091: i64,
        result: Bits,
        operand1: Bits,
        eltspersegment: i64,
        VL: i64,
        da: i64,
        esize: i64,
        index: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        da,
        esize,
        index,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#3894 <= s_0_2
        fn_state.VLshadow_3894 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckSVEEnabled(s_0_4)
        let s_0_5: () = CheckSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#3894:i64
        let s_1_0: i64 = fn_state.VLshadow_3894;
        // D s_1_1: write-var VLshadow#3895 <= s_1_0
        fn_state.VLshadow_3895 = s_1_0;
        // D s_1_2: read-var VLshadow#3895:i64
        let s_1_2: i64 = fn_state.VLshadow_3895;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esize:i64
        let s_1_4: i64 = fn_state.esize;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #128s : i
        let s_1_8: i128 = 128;
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var eltspersegment <= s_1_12
        fn_state.eltspersegment = s_1_12;
        // D s_1_14: read-var VLshadow#3895:i64
        let s_1_14: i64 = fn_state.VLshadow_3895;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var n:i64
        let s_1_17: i64 = fn_state.n;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: cast zx s_1_16 -> i
        let s_1_19: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_20: call Z_read(s_1_18, s_1_19)
        let s_1_20: Bits = Z_read(state, tracer, s_1_18, s_1_19);
        // D s_1_21: write-var operand1 <= s_1_20
        fn_state.operand1 = s_1_20;
        // D s_1_22: read-var VLshadow#3895:i64
        let s_1_22: i64 = fn_state.VLshadow_3895;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: read-var m:i64
        let s_1_25: i64 = fn_state.m;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: cast zx s_1_24 -> i
        let s_1_27: i128 = (i128::try_from(s_1_24).unwrap());
        // D s_1_28: call Z_read(s_1_26, s_1_27)
        let s_1_28: Bits = Z_read(state, tracer, s_1_26, s_1_27);
        // D s_1_29: write-var operand2 <= s_1_28
        fn_state.operand2 = s_1_28;
        // D s_1_30: read-var VLshadow#3895:i64
        let s_1_30: i64 = fn_state.VLshadow_3895;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: cast reint s_1_31 -> i64
        let s_1_32: i64 = (s_1_31 as i64);
        // D s_1_33: read-var da:i64
        let s_1_33: i64 = fn_state.da;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: cast zx s_1_32 -> i
        let s_1_35: i128 = (i128::try_from(s_1_32).unwrap());
        // D s_1_36: call Z_read(s_1_34, s_1_35)
        let s_1_36: Bits = Z_read(state, tracer, s_1_34, s_1_35);
        // D s_1_37: write-var result <= s_1_36
        fn_state.result = s_1_36;
        // C s_1_38: const #0s : i64
        let s_1_38: i64 = 0;
        // C s_1_39: const #1s : i
        let s_1_39: i128 = 1;
        // D s_1_40: cast zx s_1_7 -> i
        let s_1_40: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_41: sub s_1_40 s_1_39
        let s_1_41: i128 = ((s_1_40) - (s_1_39));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#211091 <= s_1_42
        fn_state.gs_211091 = s_1_42;
        // D s_1_44: write-var e <= s_1_38
        fn_state.e = s_1_38;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#211091:i64
        let s_2_1: i64 = fn_state.gs_211091;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var eltspersegment:i64
        let s_3_2: i64 = fn_state.eltspersegment;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: mod s_3_1 s_3_3
        let s_3_4: i128 = ((s_3_1) % (s_3_3));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var e:i64
        let s_3_6: i64 = fn_state.e;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast zx s_3_5 -> i
        let s_3_8: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_9: sub s_3_7 s_3_8
        let s_3_9: i128 = ((s_3_7) - (s_3_8));
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: read-var index:i64
        let s_3_12: i64 = fn_state.index;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: add s_3_11 s_3_13
        let s_3_14: i128 = (s_3_11 + s_3_13);
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var esize:i64
        let s_3_16: i64 = fn_state.esize;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var e:i64
        let s_3_19: i64 = fn_state.e;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast zx s_3_18 -> i
        let s_3_21: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_22: read-var operand1:bv
        let s_3_22: Bits = fn_state.operand1;
        // D s_3_23: call Elem_read(s_3_22, s_3_20, s_3_21)
        let s_3_23: Bits = Elem_read(state, tracer, s_3_22, s_3_20, s_3_21);
        // D s_3_24: cast reint s_3_23 -> u32
        let s_3_24: u32 = (s_3_23.value() as u32);
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 32u16);
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (s_3_25.value() as i128);
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: read-var esize:i64
        let s_3_28: i64 = fn_state.esize;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: cast reint s_3_29 -> i64
        let s_3_30: i64 = (s_3_29 as i64);
        // D s_3_31: cast zx s_3_15 -> i
        let s_3_31: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_32: cast zx s_3_30 -> i
        let s_3_32: i128 = (i128::try_from(s_3_30).unwrap());
        // D s_3_33: read-var operand2:bv
        let s_3_33: Bits = fn_state.operand2;
        // D s_3_34: call Elem_read(s_3_33, s_3_31, s_3_32)
        let s_3_34: Bits = Elem_read(state, tracer, s_3_33, s_3_31, s_3_32);
        // D s_3_35: cast reint s_3_34 -> u32
        let s_3_35: u32 = (s_3_34.value() as u32);
        // D s_3_36: cast zx s_3_35 -> bv
        let s_3_36: Bits = Bits::new(s_3_35 as u128, 32u16);
        // D s_3_37: cast zx s_3_36 -> i
        let s_3_37: i128 = (s_3_36.value() as i128);
        // D s_3_38: cast reint s_3_37 -> i64
        let s_3_38: i64 = (s_3_37 as i64);
        // D s_3_39: read-var esize:i64
        let s_3_39: i64 = fn_state.esize;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: cast reint s_3_40 -> i64
        let s_3_41: i64 = (s_3_40 as i64);
        // D s_3_42: read-var esize:i64
        let s_3_42: i64 = fn_state.esize;
        // D s_3_43: cast zx s_3_42 -> i
        let s_3_43: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_44: cast reint s_3_43 -> i64
        let s_3_44: i64 = (s_3_43 as i64);
        // D s_3_45: read-var e:i64
        let s_3_45: i64 = fn_state.e;
        // D s_3_46: cast zx s_3_45 -> i
        let s_3_46: i128 = (i128::try_from(s_3_45).unwrap());
        // D s_3_47: cast zx s_3_44 -> i
        let s_3_47: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_48: read-var result:bv
        let s_3_48: Bits = fn_state.result;
        // D s_3_49: call Elem_read(s_3_48, s_3_46, s_3_47)
        let s_3_49: Bits = Elem_read(state, tracer, s_3_48, s_3_46, s_3_47);
        // D s_3_50: cast reint s_3_49 -> u32
        let s_3_50: u32 = (s_3_49.value() as u32);
        // D s_3_51: cast zx s_3_27 -> i
        let s_3_51: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_52: cast zx s_3_38 -> i
        let s_3_52: i128 = (i128::try_from(s_3_38).unwrap());
        // D s_3_53: mul s_3_51 s_3_52
        let s_3_53: i128 = ((s_3_51) * (s_3_52));
        // C s_3_54: const #1s : i
        let s_3_54: i128 = 1;
        // D s_3_55: read-var esize:i64
        let s_3_55: i64 = fn_state.esize;
        // D s_3_56: cast zx s_3_55 -> i
        let s_3_56: i128 = (i128::try_from(s_3_55).unwrap());
        // D s_3_57: sub s_3_56 s_3_54
        let s_3_57: i128 = ((s_3_56) - (s_3_54));
        // D s_3_58: cast reint s_3_57 -> i64
        let s_3_58: i64 = (s_3_57 as i64);
        // C s_3_59: const #0s : i
        let s_3_59: i128 = 0;
        // D s_3_60: cast zx s_3_58 -> i
        let s_3_60: i128 = (i128::try_from(s_3_58).unwrap());
        // D s_3_61: call integer_subrange(s_3_53, s_3_60, s_3_59)
        let s_3_61: Bits = integer_subrange(state, tracer, s_3_53, s_3_60, s_3_59);
        // D s_3_62: cast zx s_3_50 -> bv
        let s_3_62: Bits = Bits::new(s_3_50 as u128, 32u16);
        // D s_3_63: add s_3_62 s_3_61
        let s_3_63: Bits = (s_3_62 + s_3_61);
        // D s_3_64: cast reint s_3_63 -> u32
        let s_3_64: u32 = (s_3_63.value() as u32);
        // D s_3_65: read-var e:i64
        let s_3_65: i64 = fn_state.e;
        // D s_3_66: cast zx s_3_65 -> i
        let s_3_66: i128 = (i128::try_from(s_3_65).unwrap());
        // D s_3_67: cast zx s_3_41 -> i
        let s_3_67: i128 = (i128::try_from(s_3_41).unwrap());
        // D s_3_68: cast zx s_3_64 -> bv
        let s_3_68: Bits = Bits::new(s_3_64 as u128, 32u16);
        // D s_3_69: read-var result:bv
        let s_3_69: Bits = fn_state.result;
        // D s_3_70: call Elem_set(s_3_69, s_3_66, s_3_67, s_3_68)
        let s_3_70: Bits = Elem_set(state, tracer, s_3_69, s_3_66, s_3_67, s_3_68);
        // D s_3_71: write-var result <= s_3_70
        fn_state.result = s_3_70;
        // D s_3_72: read-var e:i64
        let s_3_72: i64 = fn_state.e;
        // C s_3_73: const #1s : i64
        let s_3_73: i64 = 1;
        // D s_3_74: add s_3_72 s_3_73
        let s_3_74: i64 = (s_3_72 + s_3_73);
        // D s_3_75: write-var e <= s_3_74
        fn_state.e = s_3_74;
        // N s_3_76: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3895:i64
        let s_4_0: i64 = fn_state.VLshadow_3895;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var da:i64
        let s_4_3: i64 = fn_state.da;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
