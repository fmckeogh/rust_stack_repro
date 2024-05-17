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
use Elem_set::*;
use Elem_read::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use fdiv_int::*;
use common::*;
pub fn execute_aarch32_instrs_VPADD_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        dest: u64,
        esizeshadow_7638: i64,
        gs_314966: i64,
        h: i128,
        d: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        elements,
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
        // D s_0_3: write-var esizeshadow#7638 <= s_0_2
        fn_state.esizeshadow_7638 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckAdvSIMDEnabled(s_0_4)
        let s_0_5: () = CheckAdvSIMDEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
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
        // D s_1_6: read-var h:i
        let s_1_6: i128 = fn_state.h;
        // D s_1_7: sub s_1_6 s_1_5
        let s_1_7: i128 = ((s_1_6) - (s_1_5));
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var gs#314966 <= s_1_8
        fn_state.gs_314966 = s_1_8;
        // D s_1_10: write-var e <= s_1_4
        fn_state.e = s_1_4;
        // N s_1_11: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#314966:i64
        let s_2_1: i64 = fn_state.gs_314966;
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
        // D s_3_0: read-var esizeshadow#7638:i64
        let s_3_0: i64 = fn_state.esizeshadow_7638;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var n:i64
        let s_3_3: i64 = fn_state.n;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: call D_read(s_3_4)
        let s_3_5: u64 = D_read(state, tracer, s_3_4);
        // C s_3_6: const #2s : i
        let s_3_6: i128 = 2;
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: mul s_3_6 s_3_8
        let s_3_9: i128 = ((s_3_6) * (s_3_8));
        // D s_3_10: read-var esizeshadow#7638:i64
        let s_3_10: i64 = fn_state.esizeshadow_7638;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: cast zx s_3_5 -> bv
        let s_3_13: Bits = Bits::new(s_3_5 as u128, 64u16);
        // D s_3_14: cast zx s_3_12 -> i
        let s_3_14: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_15: call Elem_read(s_3_13, s_3_9, s_3_14)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_13, s_3_9, s_3_14);
        // D s_3_16: read-var n:i64
        let s_3_16: i64 = fn_state.n;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: call D_read(s_3_17)
        let s_3_18: u64 = D_read(state, tracer, s_3_17);
        // C s_3_19: const #2s : i
        let s_3_19: i128 = 2;
        // D s_3_20: read-var e:i64
        let s_3_20: i64 = fn_state.e;
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // D s_3_22: mul s_3_19 s_3_21
        let s_3_22: i128 = ((s_3_19) * (s_3_21));
        // C s_3_23: const #1s : i
        let s_3_23: i128 = 1;
        // D s_3_24: add s_3_22 s_3_23
        let s_3_24: i128 = (s_3_22 + s_3_23);
        // D s_3_25: read-var esizeshadow#7638:i64
        let s_3_25: i64 = fn_state.esizeshadow_7638;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: cast zx s_3_18 -> bv
        let s_3_28: Bits = Bits::new(s_3_18 as u128, 64u16);
        // D s_3_29: cast zx s_3_27 -> i
        let s_3_29: i128 = (i128::try_from(s_3_27).unwrap());
        // D s_3_30: call Elem_read(s_3_28, s_3_24, s_3_29)
        let s_3_30: Bits = Elem_read(state, tracer, s_3_28, s_3_24, s_3_29);
        // D s_3_31: add s_3_15 s_3_30
        let s_3_31: Bits = (s_3_15 + s_3_30);
        // D s_3_32: read-var dest:u64
        let s_3_32: u64 = fn_state.dest;
        // D s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 64u16);
        // D s_3_34: read-var e:i64
        let s_3_34: i64 = fn_state.e;
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_36: cast zx s_3_2 -> i
        let s_3_36: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_37: call Elem_set(s_3_33, s_3_35, s_3_36, s_3_31)
        let s_3_37: Bits = Elem_set(state, tracer, s_3_33, s_3_35, s_3_36, s_3_31);
        // D s_3_38: cast reint s_3_37 -> u64
        let s_3_38: u64 = (s_3_37.value() as u64);
        // D s_3_39: write-var dest <= s_3_38
        fn_state.dest = s_3_38;
        // D s_3_40: read-var e:i64
        let s_3_40: i64 = fn_state.e;
        // D s_3_41: cast zx s_3_40 -> i
        let s_3_41: i128 = (i128::try_from(s_3_40).unwrap());
        // D s_3_42: read-var h:i
        let s_3_42: i128 = fn_state.h;
        // D s_3_43: add s_3_41 s_3_42
        let s_3_43: i128 = (s_3_41 + s_3_42);
        // D s_3_44: read-var esizeshadow#7638:i64
        let s_3_44: i64 = fn_state.esizeshadow_7638;
        // D s_3_45: cast zx s_3_44 -> i
        let s_3_45: i128 = (i128::try_from(s_3_44).unwrap());
        // D s_3_46: cast reint s_3_45 -> i64
        let s_3_46: i64 = (s_3_45 as i64);
        // D s_3_47: read-var m:i64
        let s_3_47: i64 = fn_state.m;
        // D s_3_48: cast zx s_3_47 -> i
        let s_3_48: i128 = (i128::try_from(s_3_47).unwrap());
        // D s_3_49: call D_read(s_3_48)
        let s_3_49: u64 = D_read(state, tracer, s_3_48);
        // C s_3_50: const #2s : i
        let s_3_50: i128 = 2;
        // D s_3_51: read-var e:i64
        let s_3_51: i64 = fn_state.e;
        // D s_3_52: cast zx s_3_51 -> i
        let s_3_52: i128 = (i128::try_from(s_3_51).unwrap());
        // D s_3_53: mul s_3_50 s_3_52
        let s_3_53: i128 = ((s_3_50) * (s_3_52));
        // D s_3_54: read-var esizeshadow#7638:i64
        let s_3_54: i64 = fn_state.esizeshadow_7638;
        // D s_3_55: cast zx s_3_54 -> i
        let s_3_55: i128 = (i128::try_from(s_3_54).unwrap());
        // D s_3_56: cast reint s_3_55 -> i64
        let s_3_56: i64 = (s_3_55 as i64);
        // D s_3_57: cast zx s_3_49 -> bv
        let s_3_57: Bits = Bits::new(s_3_49 as u128, 64u16);
        // D s_3_58: cast zx s_3_56 -> i
        let s_3_58: i128 = (i128::try_from(s_3_56).unwrap());
        // D s_3_59: call Elem_read(s_3_57, s_3_53, s_3_58)
        let s_3_59: Bits = Elem_read(state, tracer, s_3_57, s_3_53, s_3_58);
        // D s_3_60: read-var m:i64
        let s_3_60: i64 = fn_state.m;
        // D s_3_61: cast zx s_3_60 -> i
        let s_3_61: i128 = (i128::try_from(s_3_60).unwrap());
        // D s_3_62: call D_read(s_3_61)
        let s_3_62: u64 = D_read(state, tracer, s_3_61);
        // C s_3_63: const #2s : i
        let s_3_63: i128 = 2;
        // D s_3_64: read-var e:i64
        let s_3_64: i64 = fn_state.e;
        // D s_3_65: cast zx s_3_64 -> i
        let s_3_65: i128 = (i128::try_from(s_3_64).unwrap());
        // D s_3_66: mul s_3_63 s_3_65
        let s_3_66: i128 = ((s_3_63) * (s_3_65));
        // C s_3_67: const #1s : i
        let s_3_67: i128 = 1;
        // D s_3_68: add s_3_66 s_3_67
        let s_3_68: i128 = (s_3_66 + s_3_67);
        // D s_3_69: read-var esizeshadow#7638:i64
        let s_3_69: i64 = fn_state.esizeshadow_7638;
        // D s_3_70: cast zx s_3_69 -> i
        let s_3_70: i128 = (i128::try_from(s_3_69).unwrap());
        // D s_3_71: cast reint s_3_70 -> i64
        let s_3_71: i64 = (s_3_70 as i64);
        // D s_3_72: cast zx s_3_62 -> bv
        let s_3_72: Bits = Bits::new(s_3_62 as u128, 64u16);
        // D s_3_73: cast zx s_3_71 -> i
        let s_3_73: i128 = (i128::try_from(s_3_71).unwrap());
        // D s_3_74: call Elem_read(s_3_72, s_3_68, s_3_73)
        let s_3_74: Bits = Elem_read(state, tracer, s_3_72, s_3_68, s_3_73);
        // D s_3_75: add s_3_59 s_3_74
        let s_3_75: Bits = (s_3_59 + s_3_74);
        // D s_3_76: read-var dest:u64
        let s_3_76: u64 = fn_state.dest;
        // D s_3_77: cast zx s_3_76 -> bv
        let s_3_77: Bits = Bits::new(s_3_76 as u128, 64u16);
        // D s_3_78: cast zx s_3_46 -> i
        let s_3_78: i128 = (i128::try_from(s_3_46).unwrap());
        // D s_3_79: call Elem_set(s_3_77, s_3_43, s_3_78, s_3_75)
        let s_3_79: Bits = Elem_set(state, tracer, s_3_77, s_3_43, s_3_78, s_3_75);
        // D s_3_80: cast reint s_3_79 -> u64
        let s_3_80: u64 = (s_3_79.value() as u64);
        // D s_3_81: write-var dest <= s_3_80
        fn_state.dest = s_3_80;
        // D s_3_82: read-var e:i64
        let s_3_82: i64 = fn_state.e;
        // C s_3_83: const #1s : i64
        let s_3_83: i64 = 1;
        // D s_3_84: add s_3_82 s_3_83
        let s_3_84: i64 = (s_3_82 + s_3_83);
        // D s_3_85: write-var e <= s_3_84
        fn_state.e = s_3_84;
        // N s_3_86: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var d:i64
        let s_4_0: i64 = fn_state.d;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var dest:u64
        let s_4_2: u64 = fn_state.dest;
        // D s_4_3: call D_set(s_4_1, s_4_2)
        let s_4_3: () = D_set(state, tracer, s_4_1, s_4_2);
        // N s_4_4: return
        return;
    }
}
