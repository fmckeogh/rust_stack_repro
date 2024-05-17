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
use execute_UQRSHR_Z_MZ4__::*;
use CurrentVL_read::*;
use HaveSME2::*;
use common::*;
pub fn decode_UQRSHR_Z_MZ4__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tsize: u8,
    imm5: u8,
    N: bool,
    Zn: u8,
    U: bool,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        shift: i64,
        n: i64,
        esizeshadow_6760: i64,
        VL: i64,
        d: i64,
        tsize: u8,
        imm5: u8,
        N: bool,
        Zn: u8,
        U: bool,
        Zd: u8,
    }
    let fn_state = FunctionState {
        tsize,
        imm5,
        N,
        Zn,
        U,
        Zd,
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
        // S s_0_1: call CurrentVL_read(s_0_0)
        let s_0_1: i64 = CurrentVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var VL <= s_0_1
        fn_state.VL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSME2(s_0_3)
        let s_0_4: bool = HaveSME2(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b17 b1
        if s_0_5 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #8s : i64
        let s_1_0: i64 = 8;
        // D s_1_1: write-var esize <= s_1_0
        fn_state.esize = s_1_0;
        // D s_1_2: read-var tsize:u8
        let s_1_2: u8 = fn_state.tsize;
        // D s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // C s_1_4: const #0u : u8
        let s_1_4: u8 = 0;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: not s_1_6
        let s_1_7: bool = !s_1_6;
        // N s_1_8: branch s_1_7 b3 b2
        if s_1_7 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: panic
        panic!("{:?}", ());
        // N s_2_1: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var tsize:u8
        let s_3_0: u8 = fn_state.tsize;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b16 b4
        if s_3_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #8s : i64
        let s_4_0: i64 = 8;
        // D s_4_1: write-var esize <= s_4_0
        fn_state.esize = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // D s_5_1: write-var esizeshadow#6760 <= s_5_0
        fn_state.esizeshadow_6760 = s_5_0;
        // D s_5_2: read-var Zn:u8
        let s_5_2: u8 = fn_state.Zn;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // C s_5_4: const #0u : u8
        let s_5_4: u8 = 0;
        // C s_5_5: cast zx s_5_4 -> bv
        let s_5_5: Bits = Bits::new(s_5_4 as u128, 2u16);
        // D s_5_6: cast reint s_5_3 -> u128
        let s_5_6: u128 = (s_5_3.value() as u128);
        // D s_5_7: size-of s_5_3
        let s_5_7: u16 = s_5_3.length();
        // C s_5_8: cast reint s_5_5 -> u128
        let s_5_8: u128 = (s_5_5.value() as u128);
        // D s_5_9: size-of s_5_5
        let s_5_9: u16 = s_5_5.length();
        // D s_5_10: lsl s_5_6 s_5_9
        let s_5_10: u128 = s_5_6 << s_5_9;
        // D s_5_11: or s_5_10 s_5_8
        let s_5_11: u128 = ((s_5_10) | (s_5_8));
        // D s_5_12: add s_5_7 s_5_9
        let s_5_12: u16 = (s_5_7 + s_5_9);
        // D s_5_13: create-bits s_5_11 s_5_12
        let s_5_13: Bits = Bits::new(s_5_11, s_5_12);
        // D s_5_14: cast reint s_5_13 -> u8
        let s_5_14: u8 = (s_5_13.value() as u8);
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 5u16);
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (s_5_15.value() as i128);
        // D s_5_17: cast reint s_5_16 -> i64
        let s_5_17: i64 = (s_5_16 as i64);
        // D s_5_18: write-var n <= s_5_17
        fn_state.n = s_5_17;
        // D s_5_19: read-var Zd:u8
        let s_5_19: u8 = fn_state.Zd;
        // D s_5_20: cast zx s_5_19 -> bv
        let s_5_20: Bits = Bits::new(s_5_19 as u128, 5u16);
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (s_5_20.value() as i128);
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: write-var d <= s_5_22
        fn_state.d = s_5_22;
        // C s_5_24: const #8s : i
        let s_5_24: i128 = 8;
        // D s_5_25: read-var esizeshadow#6760:i64
        let s_5_25: i64 = fn_state.esizeshadow_6760;
        // D s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // D s_5_27: mul s_5_24 s_5_26
        let s_5_27: i128 = ((s_5_24) * (s_5_26));
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: read-var tsize:u8
        let s_5_29: u8 = fn_state.tsize;
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 2u16);
        // D s_5_31: read-var imm5:u8
        let s_5_31: u8 = fn_state.imm5;
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 5u16);
        // D s_5_33: cast reint s_5_30 -> u128
        let s_5_33: u128 = (s_5_30.value() as u128);
        // D s_5_34: size-of s_5_30
        let s_5_34: u16 = s_5_30.length();
        // D s_5_35: cast reint s_5_32 -> u128
        let s_5_35: u128 = (s_5_32.value() as u128);
        // D s_5_36: size-of s_5_32
        let s_5_36: u16 = s_5_32.length();
        // D s_5_37: lsl s_5_33 s_5_36
        let s_5_37: u128 = s_5_33 << s_5_36;
        // D s_5_38: or s_5_37 s_5_35
        let s_5_38: u128 = ((s_5_37) | (s_5_35));
        // D s_5_39: add s_5_34 s_5_36
        let s_5_39: u16 = (s_5_34 + s_5_36);
        // D s_5_40: create-bits s_5_38 s_5_39
        let s_5_40: Bits = Bits::new(s_5_38, s_5_39);
        // D s_5_41: cast reint s_5_40 -> u8
        let s_5_41: u8 = (s_5_40.value() as u8);
        // D s_5_42: cast zx s_5_41 -> bv
        let s_5_42: Bits = Bits::new(s_5_41 as u128, 7u16);
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (s_5_42.value() as i128);
        // D s_5_44: cast reint s_5_43 -> i64
        let s_5_44: i64 = (s_5_43 as i64);
        // D s_5_45: cast zx s_5_28 -> i
        let s_5_45: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_46: cast zx s_5_44 -> i
        let s_5_46: i128 = (i128::try_from(s_5_44).unwrap());
        // D s_5_47: sub s_5_45 s_5_46
        let s_5_47: i128 = ((s_5_45) - (s_5_46));
        // D s_5_48: cast reint s_5_47 -> i64
        let s_5_48: i64 = (s_5_47 as i64);
        // D s_5_49: write-var shift <= s_5_48
        fn_state.shift = s_5_48;
        // D s_5_50: read-var VL:i64
        let s_5_50: i64 = fn_state.VL;
        // C s_5_51: const #128s : i
        let s_5_51: i128 = 128;
        // D s_5_52: cast zx s_5_50 -> i
        let s_5_52: i128 = (i128::try_from(s_5_50).unwrap());
        // D s_5_53: cmp-eq s_5_52 s_5_51
        let s_5_53: bool = ((s_5_52) == (s_5_51));
        // D s_5_54: not s_5_53
        let s_5_54: bool = !s_5_53;
        // N s_5_55: branch s_5_54 b7 b6
        if s_5_54 {
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
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: read-var esizeshadow#6760:i64
        let s_6_1: i64 = fn_state.esizeshadow_6760;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: read-var shift:i64
        let s_6_4: i64 = fn_state.shift;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var d:i64
        let s_6_6: i64 = fn_state.d;
        // D s_6_7: read-var n:i64
        let s_6_7: i64 = fn_state.n;
        // D s_6_8: call execute_UQRSHR_Z_MZ4__(s_6_0, s_6_6, s_6_3, s_6_7, s_6_5)
        let s_6_8: () = execute_UQRSHR_Z_MZ4__(
            state,
            tracer,
            s_6_0,
            s_6_6,
            s_6_3,
            s_6_7,
            s_6_5,
        );
        // N s_6_9: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #256s : i
        let s_7_1: i128 = 256;
        // D s_7_2: cast zx s_7_0 -> i
        let s_7_2: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_3: cmp-eq s_7_2 s_7_1
        let s_7_3: bool = ((s_7_2) == (s_7_1));
        // D s_7_4: not s_7_3
        let s_7_4: bool = !s_7_3;
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #256s : i64
        let s_8_0: i64 = 256;
        // D s_8_1: read-var esizeshadow#6760:i64
        let s_8_1: i64 = fn_state.esizeshadow_6760;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var shift:i64
        let s_8_4: i64 = fn_state.shift;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var d:i64
        let s_8_6: i64 = fn_state.d;
        // D s_8_7: read-var n:i64
        let s_8_7: i64 = fn_state.n;
        // D s_8_8: call execute_UQRSHR_Z_MZ4__(s_8_0, s_8_6, s_8_3, s_8_7, s_8_5)
        let s_8_8: () = execute_UQRSHR_Z_MZ4__(
            state,
            tracer,
            s_8_0,
            s_8_6,
            s_8_3,
            s_8_7,
            s_8_5,
        );
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #512s : i
        let s_9_1: i128 = 512;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b11 b10
        if s_9_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #512s : i64
        let s_10_0: i64 = 512;
        // D s_10_1: read-var esizeshadow#6760:i64
        let s_10_1: i64 = fn_state.esizeshadow_6760;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var shift:i64
        let s_10_4: i64 = fn_state.shift;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var d:i64
        let s_10_6: i64 = fn_state.d;
        // D s_10_7: read-var n:i64
        let s_10_7: i64 = fn_state.n;
        // D s_10_8: call execute_UQRSHR_Z_MZ4__(s_10_0, s_10_6, s_10_3, s_10_7, s_10_5)
        let s_10_8: () = execute_UQRSHR_Z_MZ4__(
            state,
            tracer,
            s_10_0,
            s_10_6,
            s_10_3,
            s_10_7,
            s_10_5,
        );
        // N s_10_9: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #1024s : i
        let s_11_1: i128 = 1024;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1024s : i64
        let s_12_0: i64 = 1024;
        // D s_12_1: read-var esizeshadow#6760:i64
        let s_12_1: i64 = fn_state.esizeshadow_6760;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var shift:i64
        let s_12_4: i64 = fn_state.shift;
        // D s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_6: read-var d:i64
        let s_12_6: i64 = fn_state.d;
        // D s_12_7: read-var n:i64
        let s_12_7: i64 = fn_state.n;
        // D s_12_8: call execute_UQRSHR_Z_MZ4__(s_12_0, s_12_6, s_12_3, s_12_7, s_12_5)
        let s_12_8: () = execute_UQRSHR_Z_MZ4__(
            state,
            tracer,
            s_12_0,
            s_12_6,
            s_12_3,
            s_12_7,
            s_12_5,
        );
        // N s_12_9: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var VL:i64
        let s_13_0: i64 = fn_state.VL;
        // C s_13_1: const #2048s : i
        let s_13_1: i128 = 2048;
        // D s_13_2: cast zx s_13_0 -> i
        let s_13_2: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_3: cmp-eq s_13_2 s_13_1
        let s_13_3: bool = ((s_13_2) == (s_13_1));
        // D s_13_4: not s_13_3
        let s_13_4: bool = !s_13_3;
        // N s_13_5: branch s_13_4 b15 b14
        if s_13_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #2048s : i64
        let s_14_0: i64 = 2048;
        // D s_14_1: read-var esizeshadow#6760:i64
        let s_14_1: i64 = fn_state.esizeshadow_6760;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // D s_14_4: read-var shift:i64
        let s_14_4: i64 = fn_state.shift;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: read-var d:i64
        let s_14_6: i64 = fn_state.d;
        // D s_14_7: read-var n:i64
        let s_14_7: i64 = fn_state.n;
        // D s_14_8: call execute_UQRSHR_Z_MZ4__(s_14_0, s_14_6, s_14_3, s_14_7, s_14_5)
        let s_14_8: () = execute_UQRSHR_Z_MZ4__(
            state,
            tracer,
            s_14_0,
            s_14_6,
            s_14_3,
            s_14_7,
            s_14_5,
        );
        // N s_14_9: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16s : i64
        let s_16_0: i64 = 16;
        // D s_16_1: write-var esize <= s_16_0
        fn_state.esize = s_16_0;
        // N s_16_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
}
