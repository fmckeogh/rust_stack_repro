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
use D_set::*;
use asl_Int::*;
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch32_instrs_VPADAL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        esizeshadow_7632: i64,
        gs_314823: i64,
        d: i128,
        gs_314829: i64,
        h: i128,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        regs,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#7632 <= s_0_2
        fn_state.esizeshadow_7632 = s_0_2;
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
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var elements:i
        let s_1_1: i128 = fn_state.elements;
        // D s_1_2: call fdiv_int(s_1_1, s_1_0)
        let s_1_2: i128 = fdiv_int(state, tracer, s_1_1, s_1_0);
        // D s_1_3: write-var h <= s_1_2
        fn_state.h = s_1_2;
        // C s_1_4: const #0s : i64
        let s_1_4: i64 = 0;
        // C s_1_5: const #1s : i
        let s_1_5: i128 = 1;
        // D s_1_6: read-var regs:i64
        let s_1_6: i64 = fn_state.regs;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: sub s_1_7 s_1_5
        let s_1_8: i128 = ((s_1_7) - (s_1_5));
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: write-var gs#314823 <= s_1_9
        fn_state.gs_314823 = s_1_9;
        // D s_1_11: write-var r <= s_1_4
        fn_state.r = s_1_4;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#314823:i64
        let s_2_1: i64 = fn_state.gs_314823;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var h:i
        let s_3_2: i128 = fn_state.h;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#314829 <= s_3_4
        fn_state.gs_314829 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#314829:i64
        let s_4_1: i64 = fn_state.gs_314829;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // C s_5_8: const #2s : i
        let s_5_8: i128 = 2;
        // D s_5_9: read-var e:i64
        let s_5_9: i64 = fn_state.e;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: mul s_5_8 s_5_10
        let s_5_11: i128 = ((s_5_8) * (s_5_10));
        // D s_5_12: read-var esizeshadow#7632:i64
        let s_5_12: i64 = fn_state.esizeshadow_7632;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast reint s_5_13 -> i64
        let s_5_14: i64 = (s_5_13 as i64);
        // D s_5_15: cast zx s_5_7 -> bv
        let s_5_15: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_16: cast zx s_5_14 -> i
        let s_5_16: i128 = (i128::try_from(s_5_14).unwrap());
        // D s_5_17: call Elem_read(s_5_15, s_5_11, s_5_16)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_15, s_5_11, s_5_16);
        // D s_5_18: read-var m:i64
        let s_5_18: i64 = fn_state.m;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: read-var r:i64
        let s_5_20: i64 = fn_state.r;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: add s_5_19 s_5_21
        let s_5_22: i128 = (s_5_19 + s_5_21);
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: call D_read(s_5_24)
        let s_5_25: u64 = D_read(state, tracer, s_5_24);
        // C s_5_26: const #2s : i
        let s_5_26: i128 = 2;
        // D s_5_27: read-var e:i64
        let s_5_27: i64 = fn_state.e;
        // D s_5_28: cast zx s_5_27 -> i
        let s_5_28: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_29: mul s_5_26 s_5_28
        let s_5_29: i128 = ((s_5_26) * (s_5_28));
        // C s_5_30: const #1s : i
        let s_5_30: i128 = 1;
        // D s_5_31: add s_5_29 s_5_30
        let s_5_31: i128 = (s_5_29 + s_5_30);
        // D s_5_32: read-var esizeshadow#7632:i64
        let s_5_32: i64 = fn_state.esizeshadow_7632;
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: cast zx s_5_25 -> bv
        let s_5_35: Bits = Bits::new(s_5_25 as u128, 64u16);
        // D s_5_36: cast zx s_5_34 -> i
        let s_5_36: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_37: call Elem_read(s_5_35, s_5_31, s_5_36)
        let s_5_37: Bits = Elem_read(state, tracer, s_5_35, s_5_31, s_5_36);
        // D s_5_38: read-var is_unsigned:u8
        let s_5_38: bool = fn_state.is_unsigned;
        // D s_5_39: call asl_Int(s_5_17, s_5_38)
        let s_5_39: i128 = asl_Int(state, tracer, s_5_17, s_5_38);
        // D s_5_40: read-var is_unsigned:u8
        let s_5_40: bool = fn_state.is_unsigned;
        // D s_5_41: call asl_Int(s_5_37, s_5_40)
        let s_5_41: i128 = asl_Int(state, tracer, s_5_37, s_5_40);
        // D s_5_42: add s_5_39 s_5_41
        let s_5_42: i128 = (s_5_39 + s_5_41);
        // D s_5_43: read-var r:i64
        let s_5_43: i64 = fn_state.r;
        // D s_5_44: cast zx s_5_43 -> i
        let s_5_44: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_45: read-var d:i
        let s_5_45: i128 = fn_state.d;
        // D s_5_46: add s_5_45 s_5_44
        let s_5_46: i128 = (s_5_45 + s_5_44);
        // D s_5_47: read-var r:i64
        let s_5_47: i64 = fn_state.r;
        // D s_5_48: cast zx s_5_47 -> i
        let s_5_48: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_49: read-var d:i
        let s_5_49: i128 = fn_state.d;
        // D s_5_50: add s_5_49 s_5_48
        let s_5_50: i128 = (s_5_49 + s_5_48);
        // D s_5_51: call D_read(s_5_50)
        let s_5_51: u64 = D_read(state, tracer, s_5_50);
        // C s_5_52: const #2s : i
        let s_5_52: i128 = 2;
        // D s_5_53: read-var esizeshadow#7632:i64
        let s_5_53: i64 = fn_state.esizeshadow_7632;
        // D s_5_54: cast zx s_5_53 -> i
        let s_5_54: i128 = (i128::try_from(s_5_53).unwrap());
        // D s_5_55: mul s_5_52 s_5_54
        let s_5_55: i128 = ((s_5_52) * (s_5_54));
        // D s_5_56: cast reint s_5_55 -> i64
        let s_5_56: i64 = (s_5_55 as i64);
        // D s_5_57: cast zx s_5_56 -> i
        let s_5_57: i128 = (i128::try_from(s_5_56).unwrap());
        // D s_5_58: cast reint s_5_57 -> i64
        let s_5_58: i64 = (s_5_57 as i64);
        // D s_5_59: read-var r:i64
        let s_5_59: i64 = fn_state.r;
        // D s_5_60: cast zx s_5_59 -> i
        let s_5_60: i128 = (i128::try_from(s_5_59).unwrap());
        // D s_5_61: read-var d:i
        let s_5_61: i128 = fn_state.d;
        // D s_5_62: add s_5_61 s_5_60
        let s_5_62: i128 = (s_5_61 + s_5_60);
        // D s_5_63: call D_read(s_5_62)
        let s_5_63: u64 = D_read(state, tracer, s_5_62);
        // C s_5_64: const #2s : i
        let s_5_64: i128 = 2;
        // D s_5_65: read-var esizeshadow#7632:i64
        let s_5_65: i64 = fn_state.esizeshadow_7632;
        // D s_5_66: cast zx s_5_65 -> i
        let s_5_66: i128 = (i128::try_from(s_5_65).unwrap());
        // D s_5_67: mul s_5_64 s_5_66
        let s_5_67: i128 = ((s_5_64) * (s_5_66));
        // D s_5_68: cast reint s_5_67 -> i64
        let s_5_68: i64 = (s_5_67 as i64);
        // D s_5_69: cast zx s_5_68 -> i
        let s_5_69: i128 = (i128::try_from(s_5_68).unwrap());
        // D s_5_70: cast reint s_5_69 -> i64
        let s_5_70: i64 = (s_5_69 as i64);
        // D s_5_71: cast zx s_5_63 -> bv
        let s_5_71: Bits = Bits::new(s_5_63 as u128, 64u16);
        // D s_5_72: read-var e:i64
        let s_5_72: i64 = fn_state.e;
        // D s_5_73: cast zx s_5_72 -> i
        let s_5_73: i128 = (i128::try_from(s_5_72).unwrap());
        // D s_5_74: cast zx s_5_70 -> i
        let s_5_74: i128 = (i128::try_from(s_5_70).unwrap());
        // D s_5_75: call Elem_read(s_5_71, s_5_73, s_5_74)
        let s_5_75: Bits = Elem_read(state, tracer, s_5_71, s_5_73, s_5_74);
        // D s_5_76: cast cvt s_5_42 -> bv
        let s_5_76: Bits = Bits::new(s_5_42 as u128, 128);
        // D s_5_77: add s_5_75 s_5_76
        let s_5_77: Bits = (s_5_75 + s_5_76);
        // D s_5_78: cast zx s_5_51 -> bv
        let s_5_78: Bits = Bits::new(s_5_51 as u128, 64u16);
        // D s_5_79: read-var e:i64
        let s_5_79: i64 = fn_state.e;
        // D s_5_80: cast zx s_5_79 -> i
        let s_5_80: i128 = (i128::try_from(s_5_79).unwrap());
        // D s_5_81: cast zx s_5_58 -> i
        let s_5_81: i128 = (i128::try_from(s_5_58).unwrap());
        // D s_5_82: call Elem_set(s_5_78, s_5_80, s_5_81, s_5_77)
        let s_5_82: Bits = Elem_set(state, tracer, s_5_78, s_5_80, s_5_81, s_5_77);
        // D s_5_83: cast reint s_5_82 -> u64
        let s_5_83: u64 = (s_5_82.value() as u64);
        // D s_5_84: call D_set(s_5_46, s_5_83)
        let s_5_84: () = D_set(state, tracer, s_5_46, s_5_83);
        // D s_5_85: read-var e:i64
        let s_5_85: i64 = fn_state.e;
        // C s_5_86: const #1s : i64
        let s_5_86: i64 = 1;
        // D s_5_87: add s_5_85 s_5_86
        let s_5_87: i64 = (s_5_85 + s_5_86);
        // D s_5_88: write-var e <= s_5_87
        fn_state.e = s_5_87;
        // N s_5_89: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var r <= s_6_2
        fn_state.r = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
