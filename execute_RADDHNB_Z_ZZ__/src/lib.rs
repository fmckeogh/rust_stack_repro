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
use Z_set::*;
use Elem_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use integer_subrange::*;
use Zeros::*;
use common::*;
pub fn execute_RADDHNB_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        halfesize: i64,
        gs_208858: i64,
        e: i64,
        esizeshadow_3802: i64,
        VLshadow_3804: i64,
        VLshadow_3803: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
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
        // D s_0_3: write-var esizeshadow#3802 <= s_0_2
        fn_state.esizeshadow_3802 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#3803 <= s_0_6
        fn_state.VLshadow_3803 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#3803:i64
        let s_1_0: i64 = fn_state.VLshadow_3803;
        // D s_1_1: write-var VLshadow#3804 <= s_1_0
        fn_state.VLshadow_3804 = s_1_0;
        // D s_1_2: read-var VLshadow#3804:i64
        let s_1_2: i64 = fn_state.VLshadow_3804;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#3802:i64
        let s_1_4: i64 = fn_state.esizeshadow_3802;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: read-var VLshadow#3804:i64
        let s_1_8: i64 = fn_state.VLshadow_3804;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var n:i64
        let s_1_11: i64 = fn_state.n;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast zx s_1_10 -> i
        let s_1_13: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_14: call Z_read(s_1_12, s_1_13)
        let s_1_14: Bits = Z_read(state, tracer, s_1_12, s_1_13);
        // D s_1_15: write-var operand1 <= s_1_14
        fn_state.operand1 = s_1_14;
        // D s_1_16: read-var VLshadow#3804:i64
        let s_1_16: i64 = fn_state.VLshadow_3804;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: read-var m:i64
        let s_1_19: i64 = fn_state.m;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast zx s_1_18 -> i
        let s_1_21: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_22: call Z_read(s_1_20, s_1_21)
        let s_1_22: Bits = Z_read(state, tracer, s_1_20, s_1_21);
        // D s_1_23: write-var operand2 <= s_1_22
        fn_state.operand2 = s_1_22;
        // C s_1_24: const #2s : i
        let s_1_24: i128 = 2;
        // D s_1_25: read-var esizeshadow#3802:i64
        let s_1_25: i64 = fn_state.esizeshadow_3802;
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (i128::try_from(s_1_25).unwrap());
        // D s_1_27: div s_1_26 s_1_24
        let s_1_27: i128 = ((s_1_26) / (s_1_24));
        // D s_1_28: cast reint s_1_27 -> i64
        let s_1_28: i64 = (s_1_27 as i64);
        // D s_1_29: write-var halfesize <= s_1_28
        fn_state.halfesize = s_1_28;
        // C s_1_30: const #0s : i64
        let s_1_30: i64 = 0;
        // C s_1_31: const #1s : i
        let s_1_31: i128 = 1;
        // D s_1_32: cast zx s_1_7 -> i
        let s_1_32: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_33: sub s_1_32 s_1_31
        let s_1_33: i128 = ((s_1_32) - (s_1_31));
        // D s_1_34: cast reint s_1_33 -> i64
        let s_1_34: i64 = (s_1_33 as i64);
        // D s_1_35: write-var gs#208858 <= s_1_34
        fn_state.gs_208858 = s_1_34;
        // D s_1_36: write-var e <= s_1_30
        fn_state.e = s_1_30;
        // N s_1_37: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#208858:i64
        let s_2_1: i64 = fn_state.gs_208858;
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
        // D s_3_0: read-var esizeshadow#3802:i64
        let s_3_0: i64 = fn_state.esizeshadow_3802;
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
        // D s_3_6: read-var operand1:bv
        let s_3_6: Bits = fn_state.operand1;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (s_3_7.value() as i128);
        // D s_3_9: read-var esizeshadow#3802:i64
        let s_3_9: i64 = fn_state.esizeshadow_3802;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: read-var e:i64
        let s_3_12: i64 = fn_state.e;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: cast zx s_3_11 -> i
        let s_3_14: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_15: read-var operand2:bv
        let s_3_15: Bits = fn_state.operand2;
        // D s_3_16: call Elem_read(s_3_15, s_3_13, s_3_14)
        let s_3_16: Bits = Elem_read(state, tracer, s_3_15, s_3_13, s_3_14);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (s_3_16.value() as i128);
        // D s_3_18: add s_3_8 s_3_17
        let s_3_18: i128 = (s_3_8 + s_3_17);
        // C s_3_19: const #1s : i
        let s_3_19: i128 = 1;
        // D s_3_20: read-var halfesize:i64
        let s_3_20: i64 = fn_state.halfesize;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: sub s_3_21 s_3_19
        let s_3_22: i128 = ((s_3_21) - (s_3_19));
        // D s_3_23: cast reint s_3_22 -> i64
        let s_3_23: i64 = (s_3_22 as i64);
        // C s_3_24: const #1s : i
        let s_3_24: i128 = 1;
        // D s_3_25: cast zx s_3_23 -> i
        let s_3_25: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_26: lsl s_3_24 s_3_25
        let s_3_26: i128 = s_3_24 << s_3_25;
        // D s_3_27: add s_3_18 s_3_26
        let s_3_27: i128 = (s_3_18 + s_3_26);
        // D s_3_28: read-var halfesize:i64
        let s_3_28: i64 = fn_state.halfesize;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: lsr s_3_27 s_3_29
        let s_3_30: i128 = s_3_27 >> s_3_29;
        // C s_3_31: const #2s : i
        let s_3_31: i128 = 2;
        // D s_3_32: read-var e:i64
        let s_3_32: i64 = fn_state.e;
        // D s_3_33: cast zx s_3_32 -> i
        let s_3_33: i128 = (i128::try_from(s_3_32).unwrap());
        // D s_3_34: mul s_3_31 s_3_33
        let s_3_34: i128 = ((s_3_31) * (s_3_33));
        // D s_3_35: cast reint s_3_34 -> i64
        let s_3_35: i64 = (s_3_34 as i64);
        // C s_3_36: const #0s : i
        let s_3_36: i128 = 0;
        // D s_3_37: cast zx s_3_35 -> i
        let s_3_37: i128 = (i128::try_from(s_3_35).unwrap());
        // D s_3_38: add s_3_37 s_3_36
        let s_3_38: i128 = (s_3_37 + s_3_36);
        // D s_3_39: cast reint s_3_38 -> i64
        let s_3_39: i64 = (s_3_38 as i64);
        // D s_3_40: read-var halfesize:i64
        let s_3_40: i64 = fn_state.halfesize;
        // D s_3_41: cast zx s_3_40 -> i
        let s_3_41: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_42: cast reint s_3_41 -> i64
        let s_3_42: i64 = (s_3_41 as i64);
        // C s_3_43: const #1s : i
        let s_3_43: i128 = 1;
        // D s_3_44: read-var halfesize:i64
        let s_3_44: i64 = fn_state.halfesize;
        // D s_3_45: cast zx s_3_44 -> i
        let s_3_45: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_46: sub s_3_45 s_3_43
        let s_3_46: i128 = ((s_3_45) - (s_3_43));
        // D s_3_47: cast reint s_3_46 -> i64
        let s_3_47: i64 = (s_3_46 as i64);
        // C s_3_48: const #0s : i
        let s_3_48: i128 = 0;
        // D s_3_49: cast zx s_3_47 -> i
        let s_3_49: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_50: call integer_subrange(s_3_30, s_3_49, s_3_48)
        let s_3_50: Bits = integer_subrange(state, tracer, s_3_30, s_3_49, s_3_48);
        // D s_3_51: cast zx s_3_39 -> i
        let s_3_51: i128 = (i128::try_from(s_3_39).unwrap());
        // D s_3_52: cast zx s_3_42 -> i
        let s_3_52: i128 = (i128::try_from(s_3_42).unwrap());
        // D s_3_53: read-var result:bv
        let s_3_53: Bits = fn_state.result;
        // D s_3_54: call Elem_set(s_3_53, s_3_51, s_3_52, s_3_50)
        let s_3_54: Bits = Elem_set(state, tracer, s_3_53, s_3_51, s_3_52, s_3_50);
        // D s_3_55: write-var result <= s_3_54
        fn_state.result = s_3_54;
        // C s_3_56: const #2s : i
        let s_3_56: i128 = 2;
        // D s_3_57: read-var e:i64
        let s_3_57: i64 = fn_state.e;
        // D s_3_58: cast zx s_3_57 -> i
        let s_3_58: i128 = (i128::try_from(s_3_57).unwrap());
        // D s_3_59: mul s_3_56 s_3_58
        let s_3_59: i128 = ((s_3_56) * (s_3_58));
        // D s_3_60: cast reint s_3_59 -> i64
        let s_3_60: i64 = (s_3_59 as i64);
        // C s_3_61: const #1s : i
        let s_3_61: i128 = 1;
        // D s_3_62: cast zx s_3_60 -> i
        let s_3_62: i128 = (i128::try_from(s_3_60).unwrap());
        // D s_3_63: add s_3_62 s_3_61
        let s_3_63: i128 = (s_3_62 + s_3_61);
        // D s_3_64: cast reint s_3_63 -> i64
        let s_3_64: i64 = (s_3_63 as i64);
        // D s_3_65: read-var halfesize:i64
        let s_3_65: i64 = fn_state.halfesize;
        // D s_3_66: cast zx s_3_65 -> i
        let s_3_66: i128 = (i128::try_from(s_3_65).unwrap());
        // D s_3_67: cast reint s_3_66 -> i64
        let s_3_67: i64 = (s_3_66 as i64);
        // D s_3_68: read-var halfesize:i64
        let s_3_68: i64 = fn_state.halfesize;
        // D s_3_69: cast zx s_3_68 -> i
        let s_3_69: i128 = (i128::try_from(s_3_68).unwrap());
        // D s_3_70: call Zeros(s_3_69)
        let s_3_70: Bits = Zeros(state, tracer, s_3_69);
        // D s_3_71: cast zx s_3_64 -> i
        let s_3_71: i128 = (i128::try_from(s_3_64).unwrap());
        // D s_3_72: cast zx s_3_67 -> i
        let s_3_72: i128 = (i128::try_from(s_3_67).unwrap());
        // D s_3_73: read-var result:bv
        let s_3_73: Bits = fn_state.result;
        // D s_3_74: call Elem_set(s_3_73, s_3_71, s_3_72, s_3_70)
        let s_3_74: Bits = Elem_set(state, tracer, s_3_73, s_3_71, s_3_72, s_3_70);
        // D s_3_75: write-var result <= s_3_74
        fn_state.result = s_3_74;
        // D s_3_76: read-var e:i64
        let s_3_76: i64 = fn_state.e;
        // C s_3_77: const #1s : i64
        let s_3_77: i64 = 1;
        // D s_3_78: add s_3_76 s_3_77
        let s_3_78: i64 = (s_3_76 + s_3_77);
        // D s_3_79: write-var e <= s_3_78
        fn_state.e = s_3_78;
        // N s_3_80: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VLshadow#3804:i64
        let s_4_0: i64 = fn_state.VLshadow_3804;
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
        // D s_4_7: call Z_set(s_4_4, s_4_5, s_4_6)
        let s_4_7: () = Z_set(state, tracer, s_4_4, s_4_5, s_4_6);
        // N s_4_8: return
        return;
    }
}
