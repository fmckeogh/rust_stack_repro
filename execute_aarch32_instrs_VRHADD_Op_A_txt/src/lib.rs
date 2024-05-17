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
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_VRHADD_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_317831: i64,
        gs_317837: i64,
        d: i128,
        esizeshadow_7757: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
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
        // D s_0_3: write-var esizeshadow#7757 <= s_0_2
        fn_state.esizeshadow_7757 = s_0_2;
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
        // D s_1_6: write-var gs#317831 <= s_1_5
        fn_state.gs_317831 = s_1_5;
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
        // D s_2_1: read-var gs#317831:i64
        let s_2_1: i64 = fn_state.gs_317831;
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
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#317837 <= s_3_4
        fn_state.gs_317837 = s_3_4;
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
        // D s_4_1: read-var gs#317837:i64
        let s_4_1: i64 = fn_state.gs_317837;
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
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
        // D s_5_8: read-var esizeshadow#7757:i64
        let s_5_8: i64 = fn_state.esizeshadow_7757;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: read-var is_unsigned:u8
        let s_5_16: bool = fn_state.is_unsigned;
        // D s_5_17: call asl_Int(s_5_15, s_5_16)
        let s_5_17: i128 = asl_Int(state, tracer, s_5_15, s_5_16);
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
        // D s_5_26: read-var esizeshadow#7757:i64
        let s_5_26: i64 = fn_state.esizeshadow_7757;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_25 -> bv
        let s_5_29: Bits = Bits::new(s_5_25 as u128, 64u16);
        // D s_5_30: read-var e:i64
        let s_5_30: i64 = fn_state.e;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: cast zx s_5_28 -> i
        let s_5_32: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_33: call Elem_read(s_5_29, s_5_31, s_5_32)
        let s_5_33: Bits = Elem_read(state, tracer, s_5_29, s_5_31, s_5_32);
        // D s_5_34: read-var is_unsigned:u8
        let s_5_34: bool = fn_state.is_unsigned;
        // D s_5_35: call asl_Int(s_5_33, s_5_34)
        let s_5_35: i128 = asl_Int(state, tracer, s_5_33, s_5_34);
        // D s_5_36: add s_5_17 s_5_35
        let s_5_36: i128 = (s_5_17 + s_5_35);
        // C s_5_37: const #1s : i
        let s_5_37: i128 = 1;
        // D s_5_38: add s_5_36 s_5_37
        let s_5_38: i128 = (s_5_36 + s_5_37);
        // C s_5_39: const #1s : i
        let s_5_39: i128 = 1;
        // D s_5_40: lsr s_5_38 s_5_39
        let s_5_40: i128 = s_5_38 >> s_5_39;
        // D s_5_41: read-var r:i64
        let s_5_41: i64 = fn_state.r;
        // D s_5_42: cast zx s_5_41 -> i
        let s_5_42: i128 = (i128::try_from(s_5_41).unwrap());
        // D s_5_43: read-var d:i
        let s_5_43: i128 = fn_state.d;
        // D s_5_44: add s_5_43 s_5_42
        let s_5_44: i128 = (s_5_43 + s_5_42);
        // D s_5_45: read-var r:i64
        let s_5_45: i64 = fn_state.r;
        // D s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_47: read-var d:i
        let s_5_47: i128 = fn_state.d;
        // D s_5_48: add s_5_47 s_5_46
        let s_5_48: i128 = (s_5_47 + s_5_46);
        // D s_5_49: call D_read(s_5_48)
        let s_5_49: u64 = D_read(state, tracer, s_5_48);
        // D s_5_50: read-var esizeshadow#7757:i64
        let s_5_50: i64 = fn_state.esizeshadow_7757;
        // D s_5_51: cast zx s_5_50 -> i
        let s_5_51: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_52: cast reint s_5_51 -> i64
        let s_5_52: i64 = (s_5_51 as i64);
        // C s_5_53: const #1s : i
        let s_5_53: i128 = 1;
        // D s_5_54: read-var esizeshadow#7757:i64
        let s_5_54: i64 = fn_state.esizeshadow_7757;
        // D s_5_55: cast zx s_5_54 -> i
        let s_5_55: i128 = (i128::try_from(s_5_54).unwrap());
        // D s_5_56: sub s_5_55 s_5_53
        let s_5_56: i128 = ((s_5_55) - (s_5_53));
        // D s_5_57: cast reint s_5_56 -> i64
        let s_5_57: i64 = (s_5_56 as i64);
        // C s_5_58: const #0s : i
        let s_5_58: i128 = 0;
        // D s_5_59: cast zx s_5_57 -> i
        let s_5_59: i128 = (i128::try_from(s_5_57).unwrap());
        // D s_5_60: call integer_subrange(s_5_40, s_5_59, s_5_58)
        let s_5_60: Bits = integer_subrange(state, tracer, s_5_40, s_5_59, s_5_58);
        // D s_5_61: cast zx s_5_49 -> bv
        let s_5_61: Bits = Bits::new(s_5_49 as u128, 64u16);
        // D s_5_62: read-var e:i64
        let s_5_62: i64 = fn_state.e;
        // D s_5_63: cast zx s_5_62 -> i
        let s_5_63: i128 = (i128::try_from(s_5_62).unwrap());
        // D s_5_64: cast zx s_5_52 -> i
        let s_5_64: i128 = (i128::try_from(s_5_52).unwrap());
        // D s_5_65: call Elem_set(s_5_61, s_5_63, s_5_64, s_5_60)
        let s_5_65: Bits = Elem_set(state, tracer, s_5_61, s_5_63, s_5_64, s_5_60);
        // D s_5_66: cast reint s_5_65 -> u64
        let s_5_66: u64 = (s_5_65.value() as u64);
        // D s_5_67: call D_set(s_5_44, s_5_66)
        let s_5_67: () = D_set(state, tracer, s_5_44, s_5_66);
        // D s_5_68: read-var e:i64
        let s_5_68: i64 = fn_state.e;
        // C s_5_69: const #1s : i64
        let s_5_69: i64 = 1;
        // D s_5_70: add s_5_68 s_5_69
        let s_5_70: i64 = (s_5_68 + s_5_69);
        // D s_5_71: write-var e <= s_5_70
        fn_state.e = s_5_70;
        // N s_5_72: jump b4
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
