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
use ConditionPassed::*;
use execute_aarch32_instrs_VHADD_Op_A_txt::*;
use common::*;
pub fn decode_aarch32_instrs_VHADD_T1enc_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    U: bool,
    D: bool,
    size: u8,
    Vn: u8,
    Vd: u8,
    op: bool,
    N: bool,
    Q: bool,
    M: bool,
    Vm: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        gs_309209: bool,
        esize: i64,
        n: i64,
        d: i64,
        add: bool,
        is_unsigned: bool,
        elements: i64,
        gs_309205: bool,
        gs_309208: bool,
        ga_353197: i64,
        U: bool,
        D: bool,
        size: u8,
        Vn: u8,
        Vd: u8,
        op: bool,
        N: bool,
        Q: bool,
        M: bool,
        Vm: u8,
    }
    let fn_state = FunctionState {
        U,
        D,
        size,
        Vn,
        Vd,
        op,
        N,
        Q,
        M,
        Vm,
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
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Q:u8
        let s_2_0: bool = fn_state.Q;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_4: cmp-eq s_2_1 s_2_3
        let s_2_4: bool = ((s_2_1) == (s_2_3));
        // N s_2_5: branch s_2_4 b12 b3
        if s_2_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#309209 <= s_3_0
        fn_state.gs_309209 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#309209:u8
        let s_4_0: bool = fn_state.gs_309209;
        // N s_4_1: branch s_4_0 b11 b5
        if s_4_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var size:u8
        let s_5_0: u8 = fn_state.size;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #3u : u8
        let s_5_2: u8 = 3;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b10 b6
        if s_5_4 {
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
        // D s_6_0: read-var op:u8
        let s_6_0: bool = fn_state.op;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: write-var add <= s_6_4
        fn_state.add = s_6_4;
        // D s_6_6: read-var U:u8
        let s_6_6: bool = fn_state.U;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 1u16);
        // C s_6_8: const #1u : u8
        let s_6_8: bool = true;
        // C s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 1u16);
        // D s_6_10: cmp-eq s_6_7 s_6_9
        let s_6_10: bool = ((s_6_7) == (s_6_9));
        // D s_6_11: write-var is_unsigned <= s_6_10
        fn_state.is_unsigned = s_6_10;
        // D s_6_12: read-var size:u8
        let s_6_12: u8 = fn_state.size;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 2u16);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (s_6_13.value() as i128);
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // C s_6_16: const #8s : i64
        let s_6_16: i64 = 8;
        // D s_6_17: lsl s_6_16 s_6_15
        let s_6_17: i64 = s_6_16 << s_6_15;
        // D s_6_18: write-var esize <= s_6_17
        fn_state.esize = s_6_17;
        // C s_6_19: const #64s : i
        let s_6_19: i128 = 64;
        // D s_6_20: read-var esize:i64
        let s_6_20: i64 = fn_state.esize;
        // D s_6_21: cast zx s_6_20 -> i
        let s_6_21: i128 = (i128::try_from(s_6_20).unwrap());
        // D s_6_22: div s_6_19 s_6_21
        let s_6_22: i128 = ((s_6_19) / (s_6_21));
        // D s_6_23: cast reint s_6_22 -> i64
        let s_6_23: i64 = (s_6_22 as i64);
        // D s_6_24: write-var elements <= s_6_23
        fn_state.elements = s_6_23;
        // D s_6_25: read-var D:u8
        let s_6_25: bool = fn_state.D;
        // D s_6_26: cast zx s_6_25 -> bv
        let s_6_26: Bits = Bits::new(s_6_25 as u128, 1u16);
        // D s_6_27: read-var Vd:u8
        let s_6_27: u8 = fn_state.Vd;
        // D s_6_28: cast zx s_6_27 -> bv
        let s_6_28: Bits = Bits::new(s_6_27 as u128, 4u16);
        // D s_6_29: cast reint s_6_26 -> u128
        let s_6_29: u128 = (s_6_26.value() as u128);
        // D s_6_30: size-of s_6_26
        let s_6_30: u16 = s_6_26.length();
        // D s_6_31: cast reint s_6_28 -> u128
        let s_6_31: u128 = (s_6_28.value() as u128);
        // D s_6_32: size-of s_6_28
        let s_6_32: u16 = s_6_28.length();
        // D s_6_33: lsl s_6_29 s_6_32
        let s_6_33: u128 = s_6_29 << s_6_32;
        // D s_6_34: or s_6_33 s_6_31
        let s_6_34: u128 = ((s_6_33) | (s_6_31));
        // D s_6_35: add s_6_30 s_6_32
        let s_6_35: u16 = (s_6_30 + s_6_32);
        // D s_6_36: create-bits s_6_34 s_6_35
        let s_6_36: Bits = Bits::new(s_6_34, s_6_35);
        // D s_6_37: cast reint s_6_36 -> u8
        let s_6_37: u8 = (s_6_36.value() as u8);
        // D s_6_38: cast zx s_6_37 -> bv
        let s_6_38: Bits = Bits::new(s_6_37 as u128, 5u16);
        // D s_6_39: cast zx s_6_38 -> i
        let s_6_39: i128 = (s_6_38.value() as i128);
        // D s_6_40: cast reint s_6_39 -> i64
        let s_6_40: i64 = (s_6_39 as i64);
        // D s_6_41: write-var d <= s_6_40
        fn_state.d = s_6_40;
        // D s_6_42: read-var N:u8
        let s_6_42: bool = fn_state.N;
        // D s_6_43: cast zx s_6_42 -> bv
        let s_6_43: Bits = Bits::new(s_6_42 as u128, 1u16);
        // D s_6_44: read-var Vn:u8
        let s_6_44: u8 = fn_state.Vn;
        // D s_6_45: cast zx s_6_44 -> bv
        let s_6_45: Bits = Bits::new(s_6_44 as u128, 4u16);
        // D s_6_46: cast reint s_6_43 -> u128
        let s_6_46: u128 = (s_6_43.value() as u128);
        // D s_6_47: size-of s_6_43
        let s_6_47: u16 = s_6_43.length();
        // D s_6_48: cast reint s_6_45 -> u128
        let s_6_48: u128 = (s_6_45.value() as u128);
        // D s_6_49: size-of s_6_45
        let s_6_49: u16 = s_6_45.length();
        // D s_6_50: lsl s_6_46 s_6_49
        let s_6_50: u128 = s_6_46 << s_6_49;
        // D s_6_51: or s_6_50 s_6_48
        let s_6_51: u128 = ((s_6_50) | (s_6_48));
        // D s_6_52: add s_6_47 s_6_49
        let s_6_52: u16 = (s_6_47 + s_6_49);
        // D s_6_53: create-bits s_6_51 s_6_52
        let s_6_53: Bits = Bits::new(s_6_51, s_6_52);
        // D s_6_54: cast reint s_6_53 -> u8
        let s_6_54: u8 = (s_6_53.value() as u8);
        // D s_6_55: cast zx s_6_54 -> bv
        let s_6_55: Bits = Bits::new(s_6_54 as u128, 5u16);
        // D s_6_56: cast zx s_6_55 -> i
        let s_6_56: i128 = (s_6_55.value() as i128);
        // D s_6_57: cast reint s_6_56 -> i64
        let s_6_57: i64 = (s_6_56 as i64);
        // D s_6_58: write-var n <= s_6_57
        fn_state.n = s_6_57;
        // D s_6_59: read-var M:u8
        let s_6_59: bool = fn_state.M;
        // D s_6_60: cast zx s_6_59 -> bv
        let s_6_60: Bits = Bits::new(s_6_59 as u128, 1u16);
        // D s_6_61: read-var Vm:u8
        let s_6_61: u8 = fn_state.Vm;
        // D s_6_62: cast zx s_6_61 -> bv
        let s_6_62: Bits = Bits::new(s_6_61 as u128, 4u16);
        // D s_6_63: cast reint s_6_60 -> u128
        let s_6_63: u128 = (s_6_60.value() as u128);
        // D s_6_64: size-of s_6_60
        let s_6_64: u16 = s_6_60.length();
        // D s_6_65: cast reint s_6_62 -> u128
        let s_6_65: u128 = (s_6_62.value() as u128);
        // D s_6_66: size-of s_6_62
        let s_6_66: u16 = s_6_62.length();
        // D s_6_67: lsl s_6_63 s_6_66
        let s_6_67: u128 = s_6_63 << s_6_66;
        // D s_6_68: or s_6_67 s_6_65
        let s_6_68: u128 = ((s_6_67) | (s_6_65));
        // D s_6_69: add s_6_64 s_6_66
        let s_6_69: u16 = (s_6_64 + s_6_66);
        // D s_6_70: create-bits s_6_68 s_6_69
        let s_6_70: Bits = Bits::new(s_6_68, s_6_69);
        // D s_6_71: cast reint s_6_70 -> u8
        let s_6_71: u8 = (s_6_70.value() as u8);
        // D s_6_72: cast zx s_6_71 -> bv
        let s_6_72: Bits = Bits::new(s_6_71 as u128, 5u16);
        // D s_6_73: cast zx s_6_72 -> i
        let s_6_73: i128 = (s_6_72.value() as i128);
        // D s_6_74: cast reint s_6_73 -> i64
        let s_6_74: i64 = (s_6_73 as i64);
        // D s_6_75: write-var m <= s_6_74
        fn_state.m = s_6_74;
        // D s_6_76: read-var Q:u8
        let s_6_76: bool = fn_state.Q;
        // D s_6_77: cast zx s_6_76 -> bv
        let s_6_77: Bits = Bits::new(s_6_76 as u128, 1u16);
        // C s_6_78: const #0u : u8
        let s_6_78: bool = false;
        // C s_6_79: cast zx s_6_78 -> bv
        let s_6_79: Bits = Bits::new(s_6_78 as u128, 1u16);
        // D s_6_80: cmp-eq s_6_77 s_6_79
        let s_6_80: bool = ((s_6_77) == (s_6_79));
        // N s_6_81: branch s_6_80 b9 b7
        if s_6_80 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i64
        let s_7_0: i64 = 2;
        // D s_7_1: write-var ga#353197 <= s_7_0
        fn_state.ga_353197 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#353197:i64
        let s_8_0: i64 = fn_state.ga_353197;
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var elements:i64
        let s_8_4: i64 = fn_state.elements;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var add:u8
        let s_8_6: bool = fn_state.add;
        // D s_8_7: read-var d:i64
        let s_8_7: i64 = fn_state.d;
        // D s_8_8: read-var m:i64
        let s_8_8: i64 = fn_state.m;
        // D s_8_9: read-var n:i64
        let s_8_9: i64 = fn_state.n;
        // D s_8_10: read-var is_unsigned:u8
        let s_8_10: bool = fn_state.is_unsigned;
        // D s_8_11: call execute_aarch32_instrs_VHADD_Op_A_txt(s_8_6, s_8_7, s_8_5, s_8_3, s_8_8, s_8_9, s_8_0, s_8_10)
        let s_8_11: () = execute_aarch32_instrs_VHADD_Op_A_txt(
            state,
            tracer,
            s_8_6,
            s_8_7,
            s_8_5,
            s_8_3,
            s_8_8,
            s_8_9,
            s_8_0,
            s_8_10,
        );
        // N s_8_12: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i64
        let s_9_0: i64 = 1;
        // D s_9_1: write-var ga#353197 <= s_9_0
        fn_state.ga_353197 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var Vd:u8
        let s_12_1: u8 = fn_state.Vd;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1u : u64
        let s_12_3: u64 = 1;
        // D s_12_4: bit-extract s_12_2 s_12_0 s_12_3
        let s_12_4: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_3).unwrap(),
        ));
        // D s_12_5: cast reint s_12_4 -> u8
        let s_12_5: bool = ((s_12_4.value()) != 0);
        // C s_12_6: const #0s : i
        let s_12_6: i128 = 0;
        // C s_12_7: const #0u : u64
        let s_12_7: u64 = 0;
        // D s_12_8: cast zx s_12_5 -> u64
        let s_12_8: u64 = (s_12_5 as u64);
        // C s_12_9: const #1u : u64
        let s_12_9: u64 = 1;
        // D s_12_10: and s_12_8 s_12_9
        let s_12_10: u64 = ((s_12_8) & (s_12_9));
        // D s_12_11: cmp-eq s_12_10 s_12_9
        let s_12_11: bool = ((s_12_10) == (s_12_9));
        // D s_12_12: lsl s_12_8 s_12_6
        let s_12_12: u64 = s_12_8 << s_12_6;
        // D s_12_13: or s_12_7 s_12_12
        let s_12_13: u64 = ((s_12_7) | (s_12_12));
        // D s_12_14: cmpl s_12_12
        let s_12_14: u64 = !s_12_12;
        // D s_12_15: and s_12_7 s_12_14
        let s_12_15: u64 = ((s_12_7) & (s_12_14));
        // D s_12_16: select s_12_11 s_12_13 s_12_15
        let s_12_16: u64 = if s_12_11 { s_12_13 } else { s_12_15 };
        // D s_12_17: cast trunc s_12_16 -> u8
        let s_12_17: bool = ((s_12_16) != 0);
        // D s_12_18: cast zx s_12_17 -> bv
        let s_12_18: Bits = Bits::new(s_12_17 as u128, 1u16);
        // C s_12_19: const #1u : u8
        let s_12_19: bool = true;
        // C s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // D s_12_21: cmp-eq s_12_18 s_12_20
        let s_12_21: bool = ((s_12_18) == (s_12_20));
        // N s_12_22: branch s_12_21 b18 b13
        if s_12_21 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var Vn:u8
        let s_13_1: u8 = fn_state.Vn;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 4u16);
        // C s_13_3: const #1u : u64
        let s_13_3: u64 = 1;
        // D s_13_4: bit-extract s_13_2 s_13_0 s_13_3
        let s_13_4: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_3).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: bool = ((s_13_4.value()) != 0);
        // C s_13_6: const #0s : i
        let s_13_6: i128 = 0;
        // C s_13_7: const #0u : u64
        let s_13_7: u64 = 0;
        // D s_13_8: cast zx s_13_5 -> u64
        let s_13_8: u64 = (s_13_5 as u64);
        // C s_13_9: const #1u : u64
        let s_13_9: u64 = 1;
        // D s_13_10: and s_13_8 s_13_9
        let s_13_10: u64 = ((s_13_8) & (s_13_9));
        // D s_13_11: cmp-eq s_13_10 s_13_9
        let s_13_11: bool = ((s_13_10) == (s_13_9));
        // D s_13_12: lsl s_13_8 s_13_6
        let s_13_12: u64 = s_13_8 << s_13_6;
        // D s_13_13: or s_13_7 s_13_12
        let s_13_13: u64 = ((s_13_7) | (s_13_12));
        // D s_13_14: cmpl s_13_12
        let s_13_14: u64 = !s_13_12;
        // D s_13_15: and s_13_7 s_13_14
        let s_13_15: u64 = ((s_13_7) & (s_13_14));
        // D s_13_16: select s_13_11 s_13_13 s_13_15
        let s_13_16: u64 = if s_13_11 { s_13_13 } else { s_13_15 };
        // D s_13_17: cast trunc s_13_16 -> u8
        let s_13_17: bool = ((s_13_16) != 0);
        // D s_13_18: cast zx s_13_17 -> bv
        let s_13_18: Bits = Bits::new(s_13_17 as u128, 1u16);
        // C s_13_19: const #1u : u8
        let s_13_19: bool = true;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 1u16);
        // D s_13_21: cmp-eq s_13_18 s_13_20
        let s_13_21: bool = ((s_13_18) == (s_13_20));
        // D s_13_22: write-var gs#309205 <= s_13_21
        fn_state.gs_309205 = s_13_21;
        // N s_13_23: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#309205:u8
        let s_14_0: bool = fn_state.gs_309205;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var Vm:u8
        let s_15_1: u8 = fn_state.Vm;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 4u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: cast zx s_15_17 -> bv
        let s_15_18: Bits = Bits::new(s_15_17 as u128, 1u16);
        // C s_15_19: const #1u : u8
        let s_15_19: bool = true;
        // C s_15_20: cast zx s_15_19 -> bv
        let s_15_20: Bits = Bits::new(s_15_19 as u128, 1u16);
        // D s_15_21: cmp-eq s_15_18 s_15_20
        let s_15_21: bool = ((s_15_18) == (s_15_20));
        // D s_15_22: write-var gs#309208 <= s_15_21
        fn_state.gs_309208 = s_15_21;
        // N s_15_23: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#309208:u8
        let s_16_0: bool = fn_state.gs_309208;
        // D s_16_1: write-var gs#309209 <= s_16_0
        fn_state.gs_309209 = s_16_0;
        // N s_16_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#309208 <= s_17_0
        fn_state.gs_309208 = s_17_0;
        // N s_17_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#309205 <= s_18_0
        fn_state.gs_309205 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
}
