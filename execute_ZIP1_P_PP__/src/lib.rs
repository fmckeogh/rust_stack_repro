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
use P_set::*;
use P_read::*;
use common::*;
pub fn execute_ZIP1_P_PP__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        p: i64,
        base: i64,
        gs_194968: i64,
        PL: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        m,
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
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // C s_1_6: const #2s : i
        let s_1_6: i128 = 2;
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: mul s_1_8 s_1_6
        let s_1_9: i128 = ((s_1_8) * (s_1_6));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: cast zx s_1_0 -> i
        let s_1_11: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_12: cast zx s_1_10 -> i
        let s_1_12: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_13: div s_1_11 s_1_12
        let s_1_13: i128 = ((s_1_11) / (s_1_12));
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var PL:i64
        let s_1_15: i64 = fn_state.PL;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var n:i64
        let s_1_18: i64 = fn_state.n;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call P_read(s_1_19, s_1_20)
        let s_1_21: Bits = P_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand1 <= s_1_21
        fn_state.operand1 = s_1_21;
        // D s_1_23: read-var PL:i64
        let s_1_23: i64 = fn_state.PL;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: read-var m:i64
        let s_1_26: i64 = fn_state.m;
        // D s_1_27: cast zx s_1_26 -> i
        let s_1_27: i128 = (i128::try_from(s_1_26).unwrap());
        // D s_1_28: cast zx s_1_25 -> i
        let s_1_28: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_29: call P_read(s_1_27, s_1_28)
        let s_1_29: Bits = P_read(state, tracer, s_1_27, s_1_28);
        // D s_1_30: write-var operand2 <= s_1_29
        fn_state.operand2 = s_1_29;
        // D s_1_31: read-var part:i64
        let s_1_31: i64 = fn_state.part;
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast zx s_1_14 -> i
        let s_1_33: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_34: mul s_1_32 s_1_33
        let s_1_34: i128 = ((s_1_32) * (s_1_33));
        // D s_1_35: cast reint s_1_34 -> i64
        let s_1_35: i64 = (s_1_34 as i64);
        // D s_1_36: write-var base <= s_1_35
        fn_state.base = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: cast zx s_1_14 -> i
        let s_1_39: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_40: sub s_1_39 s_1_38
        let s_1_40: i128 = ((s_1_39) - (s_1_38));
        // D s_1_41: cast reint s_1_40 -> i64
        let s_1_41: i64 = (s_1_40 as i64);
        // D s_1_42: write-var gs#194968 <= s_1_41
        fn_state.gs_194968 = s_1_41;
        // D s_1_43: write-var p <= s_1_37
        fn_state.p = s_1_37;
        // N s_1_44: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var p:i64
        let s_2_0: i64 = fn_state.p;
        // D s_2_1: read-var gs#194968:i64
        let s_2_1: i64 = fn_state.gs_194968;
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var p:i64
        let s_3_1: i64 = fn_state.p;
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
        // C s_3_9: const #8s : i
        let s_3_9: i128 = 8;
        // D s_3_10: read-var esize:i64
        let s_3_10: i64 = fn_state.esize;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: div s_3_11 s_3_9
        let s_3_12: i128 = ((s_3_11) / (s_3_9));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: cast reint s_3_14 -> i64
        let s_3_15: i64 = (s_3_14 as i64);
        // D s_3_16: read-var base:i64
        let s_3_16: i64 = fn_state.base;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: read-var p:i64
        let s_3_18: i64 = fn_state.p;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: add s_3_17 s_3_19
        let s_3_20: i128 = (s_3_17 + s_3_19);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // C s_3_22: const #8s : i
        let s_3_22: i128 = 8;
        // D s_3_23: read-var esize:i64
        let s_3_23: i64 = fn_state.esize;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: div s_3_24 s_3_22
        let s_3_25: i128 = ((s_3_24) / (s_3_22));
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: cast zx s_3_21 -> i
        let s_3_29: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_30: cast zx s_3_28 -> i
        let s_3_30: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_31: read-var operand1:bv
        let s_3_31: Bits = fn_state.operand1;
        // D s_3_32: call Elem_read(s_3_31, s_3_29, s_3_30)
        let s_3_32: Bits = Elem_read(state, tracer, s_3_31, s_3_29, s_3_30);
        // D s_3_33: cast zx s_3_8 -> i
        let s_3_33: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_34: cast zx s_3_15 -> i
        let s_3_34: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_35: read-var result:bv
        let s_3_35: Bits = fn_state.result;
        // D s_3_36: call Elem_set(s_3_35, s_3_33, s_3_34, s_3_32)
        let s_3_36: Bits = Elem_set(state, tracer, s_3_35, s_3_33, s_3_34, s_3_32);
        // D s_3_37: write-var result <= s_3_36
        fn_state.result = s_3_36;
        // C s_3_38: const #2s : i
        let s_3_38: i128 = 2;
        // D s_3_39: read-var p:i64
        let s_3_39: i64 = fn_state.p;
        // D s_3_40: cast zx s_3_39 -> i
        let s_3_40: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_41: mul s_3_38 s_3_40
        let s_3_41: i128 = ((s_3_38) * (s_3_40));
        // D s_3_42: cast reint s_3_41 -> i64
        let s_3_42: i64 = (s_3_41 as i64);
        // C s_3_43: const #1s : i
        let s_3_43: i128 = 1;
        // D s_3_44: cast zx s_3_42 -> i
        let s_3_44: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_45: add s_3_44 s_3_43
        let s_3_45: i128 = (s_3_44 + s_3_43);
        // D s_3_46: cast reint s_3_45 -> i64
        let s_3_46: i64 = (s_3_45 as i64);
        // C s_3_47: const #8s : i
        let s_3_47: i128 = 8;
        // D s_3_48: read-var esize:i64
        let s_3_48: i64 = fn_state.esize;
        // D s_3_49: cast zx s_3_48 -> i
        let s_3_49: i128 = (i128::try_from(s_3_48).unwrap());
        // D s_3_50: div s_3_49 s_3_47
        let s_3_50: i128 = ((s_3_49) / (s_3_47));
        // D s_3_51: cast reint s_3_50 -> i64
        let s_3_51: i64 = (s_3_50 as i64);
        // D s_3_52: cast zx s_3_51 -> i
        let s_3_52: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_53: cast reint s_3_52 -> i64
        let s_3_53: i64 = (s_3_52 as i64);
        // D s_3_54: read-var base:i64
        let s_3_54: i64 = fn_state.base;
        // D s_3_55: cast zx s_3_54 -> i
        let s_3_55: i128 = (i128::try_from(s_3_54).unwrap());
        // D s_3_56: read-var p:i64
        let s_3_56: i64 = fn_state.p;
        // D s_3_57: cast zx s_3_56 -> i
        let s_3_57: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_58: add s_3_55 s_3_57
        let s_3_58: i128 = (s_3_55 + s_3_57);
        // D s_3_59: cast reint s_3_58 -> i64
        let s_3_59: i64 = (s_3_58 as i64);
        // C s_3_60: const #8s : i
        let s_3_60: i128 = 8;
        // D s_3_61: read-var esize:i64
        let s_3_61: i64 = fn_state.esize;
        // D s_3_62: cast zx s_3_61 -> i
        let s_3_62: i128 = (i128::try_from(s_3_61).unwrap());
        // D s_3_63: div s_3_62 s_3_60
        let s_3_63: i128 = ((s_3_62) / (s_3_60));
        // D s_3_64: cast reint s_3_63 -> i64
        let s_3_64: i64 = (s_3_63 as i64);
        // D s_3_65: cast zx s_3_64 -> i
        let s_3_65: i128 = (i128::try_from(s_3_64).unwrap());
        // D s_3_66: cast reint s_3_65 -> i64
        let s_3_66: i64 = (s_3_65 as i64);
        // D s_3_67: cast zx s_3_59 -> i
        let s_3_67: i128 = (i128::try_from(s_3_59).unwrap());
        // D s_3_68: cast zx s_3_66 -> i
        let s_3_68: i128 = (i128::try_from(s_3_66).unwrap());
        // D s_3_69: read-var operand2:bv
        let s_3_69: Bits = fn_state.operand2;
        // D s_3_70: call Elem_read(s_3_69, s_3_67, s_3_68)
        let s_3_70: Bits = Elem_read(state, tracer, s_3_69, s_3_67, s_3_68);
        // D s_3_71: cast zx s_3_46 -> i
        let s_3_71: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_72: cast zx s_3_53 -> i
        let s_3_72: i128 = (i128::try_from(s_3_53).unwrap());
        // D s_3_73: read-var result:bv
        let s_3_73: Bits = fn_state.result;
        // D s_3_74: call Elem_set(s_3_73, s_3_71, s_3_72, s_3_70)
        let s_3_74: Bits = Elem_set(state, tracer, s_3_73, s_3_71, s_3_72, s_3_70);
        // D s_3_75: write-var result <= s_3_74
        fn_state.result = s_3_74;
        // D s_3_76: read-var p:i64
        let s_3_76: i64 = fn_state.p;
        // C s_3_77: const #1s : i64
        let s_3_77: i64 = 1;
        // D s_3_78: add s_3_76 s_3_77
        let s_3_78: i64 = (s_3_76 + s_3_77);
        // D s_3_79: write-var p <= s_3_78
        fn_state.p = s_3_78;
        // N s_3_80: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var PL:i64
        let s_4_0: i64 = fn_state.PL;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: read-var result:bv
        let s_4_6: Bits = fn_state.result;
        // D s_4_7: call P_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = P_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
