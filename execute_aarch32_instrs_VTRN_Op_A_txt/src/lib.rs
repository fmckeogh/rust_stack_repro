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
use u__UNKNOWN_bits::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use D_set::*;
use Din_read::*;
use Elem_read::*;
use D_read::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch32_instrs_VTRN_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m__arg: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i128,
        r: i64,
        e: i64,
        d: i128,
        esizeshadow_7877: i64,
        gs_322831: i64,
        h: i128,
        gs_322838: i64,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m__arg: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m__arg,
        regs,
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
        // D s_0_3: write-var esizeshadow#7877 <= s_0_2
        fn_state.esizeshadow_7877 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // D s_0_7: read-var m__arg:i64
        let s_0_7: i64 = fn_state.m__arg;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: write-var m <= s_0_8
        fn_state.m = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call CheckAdvSIMDEnabled(s_0_10)
        let s_0_11: () = CheckAdvSIMDEnabled(state, tracer, s_0_10);
        // N s_0_12: jump b1
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
        // D s_1_10: write-var gs#322831 <= s_1_9
        fn_state.gs_322831 = s_1_9;
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
        // D s_2_1: read-var gs#322831:i64
        let s_2_1: i64 = fn_state.gs_322831;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i
        let s_3_0: i128 = fn_state.d;
        // D s_3_1: read-var m:i
        let s_3_1: i128 = fn_state.m;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b9 b4
        if s_3_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0s : i64
        let s_4_0: i64 = 0;
        // C s_4_1: const #1s : i
        let s_4_1: i128 = 1;
        // D s_4_2: read-var h:i
        let s_4_2: i128 = fn_state.h;
        // D s_4_3: sub s_4_2 s_4_1
        let s_4_3: i128 = ((s_4_2) - (s_4_1));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: write-var gs#322838 <= s_4_4
        fn_state.gs_322838 = s_4_4;
        // D s_4_6: write-var e <= s_4_0
        fn_state.e = s_4_0;
        // N s_4_7: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: read-var gs#322838:i64
        let s_5_1: i64 = fn_state.gs_322838;
        // D s_5_2: cmp-gt s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) > (s_5_1));
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: read-var d:i
        let s_6_2: i128 = fn_state.d;
        // D s_6_3: add s_6_2 s_6_1
        let s_6_3: i128 = (s_6_2 + s_6_1);
        // D s_6_4: read-var r:i64
        let s_6_4: i64 = fn_state.r;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var d:i
        let s_6_6: i128 = fn_state.d;
        // D s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: call D_read(s_6_7)
        let s_6_8: u64 = D_read(state, tracer, s_6_7);
        // C s_6_9: const #2s : i
        let s_6_9: i128 = 2;
        // D s_6_10: read-var e:i64
        let s_6_10: i64 = fn_state.e;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: mul s_6_9 s_6_11
        let s_6_12: i128 = ((s_6_9) * (s_6_11));
        // C s_6_13: const #1s : i
        let s_6_13: i128 = 1;
        // D s_6_14: add s_6_12 s_6_13
        let s_6_14: i128 = (s_6_12 + s_6_13);
        // D s_6_15: read-var esizeshadow#7877:i64
        let s_6_15: i64 = fn_state.esizeshadow_7877;
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: cast reint s_6_16 -> i64
        let s_6_17: i64 = (s_6_16 as i64);
        // D s_6_18: read-var r:i64
        let s_6_18: i64 = fn_state.r;
        // D s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // D s_6_20: read-var m:i
        let s_6_20: i128 = fn_state.m;
        // D s_6_21: add s_6_20 s_6_19
        let s_6_21: i128 = (s_6_20 + s_6_19);
        // D s_6_22: call Din_read(s_6_21)
        let s_6_22: u64 = Din_read(state, tracer, s_6_21);
        // C s_6_23: const #2s : i
        let s_6_23: i128 = 2;
        // D s_6_24: read-var e:i64
        let s_6_24: i64 = fn_state.e;
        // D s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (i128::try_from(s_6_24).unwrap());
        // D s_6_26: mul s_6_23 s_6_25
        let s_6_26: i128 = ((s_6_23) * (s_6_25));
        // D s_6_27: read-var esizeshadow#7877:i64
        let s_6_27: i64 = fn_state.esizeshadow_7877;
        // D s_6_28: cast zx s_6_27 -> i
        let s_6_28: i128 = (i128::try_from(s_6_27).unwrap());
        // D s_6_29: cast reint s_6_28 -> i64
        let s_6_29: i64 = (s_6_28 as i64);
        // D s_6_30: cast zx s_6_22 -> bv
        let s_6_30: Bits = Bits::new(s_6_22 as u128, 64u16);
        // D s_6_31: cast zx s_6_29 -> i
        let s_6_31: i128 = (i128::try_from(s_6_29).unwrap());
        // D s_6_32: call Elem_read(s_6_30, s_6_26, s_6_31)
        let s_6_32: Bits = Elem_read(state, tracer, s_6_30, s_6_26, s_6_31);
        // D s_6_33: cast zx s_6_8 -> bv
        let s_6_33: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_34: cast zx s_6_17 -> i
        let s_6_34: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_35: call Elem_set(s_6_33, s_6_14, s_6_34, s_6_32)
        let s_6_35: Bits = Elem_set(state, tracer, s_6_33, s_6_14, s_6_34, s_6_32);
        // D s_6_36: cast reint s_6_35 -> u64
        let s_6_36: u64 = (s_6_35.value() as u64);
        // D s_6_37: call D_set(s_6_3, s_6_36)
        let s_6_37: () = D_set(state, tracer, s_6_3, s_6_36);
        // D s_6_38: read-var r:i64
        let s_6_38: i64 = fn_state.r;
        // D s_6_39: cast zx s_6_38 -> i
        let s_6_39: i128 = (i128::try_from(s_6_38).unwrap());
        // D s_6_40: read-var m:i
        let s_6_40: i128 = fn_state.m;
        // D s_6_41: add s_6_40 s_6_39
        let s_6_41: i128 = (s_6_40 + s_6_39);
        // D s_6_42: read-var r:i64
        let s_6_42: i64 = fn_state.r;
        // D s_6_43: cast zx s_6_42 -> i
        let s_6_43: i128 = (i128::try_from(s_6_42).unwrap());
        // D s_6_44: read-var m:i
        let s_6_44: i128 = fn_state.m;
        // D s_6_45: add s_6_44 s_6_43
        let s_6_45: i128 = (s_6_44 + s_6_43);
        // D s_6_46: call D_read(s_6_45)
        let s_6_46: u64 = D_read(state, tracer, s_6_45);
        // C s_6_47: const #2s : i
        let s_6_47: i128 = 2;
        // D s_6_48: read-var e:i64
        let s_6_48: i64 = fn_state.e;
        // D s_6_49: cast zx s_6_48 -> i
        let s_6_49: i128 = (i128::try_from(s_6_48).unwrap());
        // D s_6_50: mul s_6_47 s_6_49
        let s_6_50: i128 = ((s_6_47) * (s_6_49));
        // D s_6_51: read-var esizeshadow#7877:i64
        let s_6_51: i64 = fn_state.esizeshadow_7877;
        // D s_6_52: cast zx s_6_51 -> i
        let s_6_52: i128 = (i128::try_from(s_6_51).unwrap());
        // D s_6_53: cast reint s_6_52 -> i64
        let s_6_53: i64 = (s_6_52 as i64);
        // D s_6_54: read-var r:i64
        let s_6_54: i64 = fn_state.r;
        // D s_6_55: cast zx s_6_54 -> i
        let s_6_55: i128 = (i128::try_from(s_6_54).unwrap());
        // D s_6_56: read-var d:i
        let s_6_56: i128 = fn_state.d;
        // D s_6_57: add s_6_56 s_6_55
        let s_6_57: i128 = (s_6_56 + s_6_55);
        // D s_6_58: call Din_read(s_6_57)
        let s_6_58: u64 = Din_read(state, tracer, s_6_57);
        // C s_6_59: const #2s : i
        let s_6_59: i128 = 2;
        // D s_6_60: read-var e:i64
        let s_6_60: i64 = fn_state.e;
        // D s_6_61: cast zx s_6_60 -> i
        let s_6_61: i128 = (i128::try_from(s_6_60).unwrap());
        // D s_6_62: mul s_6_59 s_6_61
        let s_6_62: i128 = ((s_6_59) * (s_6_61));
        // C s_6_63: const #1s : i
        let s_6_63: i128 = 1;
        // D s_6_64: add s_6_62 s_6_63
        let s_6_64: i128 = (s_6_62 + s_6_63);
        // D s_6_65: read-var esizeshadow#7877:i64
        let s_6_65: i64 = fn_state.esizeshadow_7877;
        // D s_6_66: cast zx s_6_65 -> i
        let s_6_66: i128 = (i128::try_from(s_6_65).unwrap());
        // D s_6_67: cast reint s_6_66 -> i64
        let s_6_67: i64 = (s_6_66 as i64);
        // D s_6_68: cast zx s_6_58 -> bv
        let s_6_68: Bits = Bits::new(s_6_58 as u128, 64u16);
        // D s_6_69: cast zx s_6_67 -> i
        let s_6_69: i128 = (i128::try_from(s_6_67).unwrap());
        // D s_6_70: call Elem_read(s_6_68, s_6_64, s_6_69)
        let s_6_70: Bits = Elem_read(state, tracer, s_6_68, s_6_64, s_6_69);
        // D s_6_71: cast zx s_6_46 -> bv
        let s_6_71: Bits = Bits::new(s_6_46 as u128, 64u16);
        // D s_6_72: cast zx s_6_53 -> i
        let s_6_72: i128 = (i128::try_from(s_6_53).unwrap());
        // D s_6_73: call Elem_set(s_6_71, s_6_50, s_6_72, s_6_70)
        let s_6_73: Bits = Elem_set(state, tracer, s_6_71, s_6_50, s_6_72, s_6_70);
        // D s_6_74: cast reint s_6_73 -> u64
        let s_6_74: u64 = (s_6_73.value() as u64);
        // D s_6_75: call D_set(s_6_41, s_6_74)
        let s_6_75: () = D_set(state, tracer, s_6_41, s_6_74);
        // D s_6_76: read-var e:i64
        let s_6_76: i64 = fn_state.e;
        // C s_6_77: const #1s : i64
        let s_6_77: i64 = 1;
        // D s_6_78: add s_6_76 s_6_77
        let s_6_78: i64 = (s_6_76 + s_6_77);
        // D s_6_79: write-var e <= s_6_78
        fn_state.e = s_6_78;
        // N s_6_80: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var r <= s_8_2
        fn_state.r = s_8_2;
        // N s_8_4: jump b2
        return block_2(state, tracer, fn_state);
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
        // C s_9_4: const #64s : i64
        let s_9_4: i64 = 64;
        // C s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // S s_9_6: call __UNKNOWN_bits(s_9_5)
        let s_9_6: Bits = u__UNKNOWN_bits(state, tracer, s_9_5);
        // S s_9_7: cast reint s_9_6 -> u64
        let s_9_7: u64 = (s_9_6.value() as u64);
        // D s_9_8: call D_set(s_9_3, s_9_7)
        let s_9_8: () = D_set(state, tracer, s_9_3, s_9_7);
        // N s_9_9: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
