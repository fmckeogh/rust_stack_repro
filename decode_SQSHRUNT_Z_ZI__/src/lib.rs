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
use CurrentVL_read::*;
use execute_SQSHRUNT_Z_ZI__::*;
use HaveSVE2::*;
use HaveSME::*;
use common::*;
pub fn decode_SQSHRUNT_Z_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tszh: bool,
    tszl: u8,
    imm3: u8,
    U: bool,
    R: bool,
    T: bool,
    Zn: u8,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_204316: bool,
        esize: i64,
        shift: i64,
        n: i64,
        esizeshadow_3451: i64,
        tsize: u8,
        VL: i64,
        d: i64,
        tszh: bool,
        tszl: u8,
        imm3: u8,
        U: bool,
        R: bool,
        T: bool,
        Zn: u8,
        Zd: u8,
    }
    let fn_state = FunctionState {
        tszh,
        tszl,
        imm3,
        U,
        R,
        T,
        Zn,
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
        // S s_0_4: call HaveSVE2(s_0_3)
        let s_0_4: bool = HaveSVE2(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b22 b1
        if s_0_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#204316 <= s_1_0
        fn_state.gs_204316 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#204316:u8
        let s_2_0: bool = fn_state.gs_204316;
        // N s_2_1: branch s_2_0 b21 b3
        if s_2_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var tszh:u8
        let s_3_0: bool = fn_state.tszh;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var tszl:u8
        let s_3_2: u8 = fn_state.tszl;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: write-var tsize <= s_3_12
        fn_state.tsize = s_3_12;
        // C s_3_14: const #8s : i64
        let s_3_14: i64 = 8;
        // D s_3_15: write-var esize <= s_3_14
        fn_state.esize = s_3_14;
        // D s_3_16: read-var tsize:u8
        let s_3_16: u8 = fn_state.tsize;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 3u16);
        // C s_3_18: const #0u : u8
        let s_3_18: u8 = 0;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 3u16);
        // D s_3_20: cmp-eq s_3_17 s_3_19
        let s_3_20: bool = ((s_3_17) == (s_3_19));
        // D s_3_21: not s_3_20
        let s_3_21: bool = !s_3_20;
        // N s_3_22: branch s_3_21 b5 b4
        if s_3_21 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var tsize:u8
        let s_5_0: u8 = fn_state.tsize;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 3u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b18 b6
        if s_5_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #8s : i64
        let s_6_0: i64 = 8;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // D s_7_1: write-var esizeshadow#3451 <= s_7_0
        fn_state.esizeshadow_3451 = s_7_0;
        // D s_7_2: read-var Zn:u8
        let s_7_2: u8 = fn_state.Zn;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 5u16);
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (s_7_3.value() as i128);
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var n <= s_7_5
        fn_state.n = s_7_5;
        // D s_7_7: read-var Zd:u8
        let s_7_7: u8 = fn_state.Zd;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 5u16);
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (s_7_8.value() as i128);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: write-var d <= s_7_10
        fn_state.d = s_7_10;
        // C s_7_12: const #2s : i
        let s_7_12: i128 = 2;
        // D s_7_13: read-var esizeshadow#3451:i64
        let s_7_13: i64 = fn_state.esizeshadow_3451;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: mul s_7_12 s_7_14
        let s_7_15: i128 = ((s_7_12) * (s_7_14));
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: read-var tsize:u8
        let s_7_17: u8 = fn_state.tsize;
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 3u16);
        // D s_7_19: read-var imm3:u8
        let s_7_19: u8 = fn_state.imm3;
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 3u16);
        // D s_7_21: cast reint s_7_18 -> u128
        let s_7_21: u128 = (s_7_18.value() as u128);
        // D s_7_22: size-of s_7_18
        let s_7_22: u16 = s_7_18.length();
        // D s_7_23: cast reint s_7_20 -> u128
        let s_7_23: u128 = (s_7_20.value() as u128);
        // D s_7_24: size-of s_7_20
        let s_7_24: u16 = s_7_20.length();
        // D s_7_25: lsl s_7_21 s_7_24
        let s_7_25: u128 = s_7_21 << s_7_24;
        // D s_7_26: or s_7_25 s_7_23
        let s_7_26: u128 = ((s_7_25) | (s_7_23));
        // D s_7_27: add s_7_22 s_7_24
        let s_7_27: u16 = (s_7_22 + s_7_24);
        // D s_7_28: create-bits s_7_26 s_7_27
        let s_7_28: Bits = Bits::new(s_7_26, s_7_27);
        // D s_7_29: cast reint s_7_28 -> u8
        let s_7_29: u8 = (s_7_28.value() as u8);
        // D s_7_30: cast zx s_7_29 -> bv
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 6u16);
        // D s_7_31: cast zx s_7_30 -> i
        let s_7_31: i128 = (s_7_30.value() as i128);
        // D s_7_32: cast reint s_7_31 -> i64
        let s_7_32: i64 = (s_7_31 as i64);
        // D s_7_33: cast zx s_7_16 -> i
        let s_7_33: i128 = (i128::try_from(s_7_16).unwrap());
        // D s_7_34: cast zx s_7_32 -> i
        let s_7_34: i128 = (i128::try_from(s_7_32).unwrap());
        // D s_7_35: sub s_7_33 s_7_34
        let s_7_35: i128 = ((s_7_33) - (s_7_34));
        // D s_7_36: cast reint s_7_35 -> i64
        let s_7_36: i64 = (s_7_35 as i64);
        // D s_7_37: write-var shift <= s_7_36
        fn_state.shift = s_7_36;
        // D s_7_38: read-var VL:i64
        let s_7_38: i64 = fn_state.VL;
        // C s_7_39: const #128s : i
        let s_7_39: i128 = 128;
        // D s_7_40: cast zx s_7_38 -> i
        let s_7_40: i128 = (i128::try_from(s_7_38).unwrap());
        // D s_7_41: cmp-eq s_7_40 s_7_39
        let s_7_41: bool = ((s_7_40) == (s_7_39));
        // D s_7_42: not s_7_41
        let s_7_42: bool = !s_7_41;
        // N s_7_43: branch s_7_42 b9 b8
        if s_7_42 {
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
        // C s_8_0: const #128s : i64
        let s_8_0: i64 = 128;
        // D s_8_1: read-var esizeshadow#3451:i64
        let s_8_1: i64 = fn_state.esizeshadow_3451;
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
        // D s_8_8: call execute_SQSHRUNT_Z_ZI__(s_8_0, s_8_6, s_8_3, s_8_7, s_8_5)
        let s_8_8: () = execute_SQSHRUNT_Z_ZI__(
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
        // C s_9_1: const #256s : i
        let s_9_1: i128 = 256;
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
        // C s_10_0: const #256s : i64
        let s_10_0: i64 = 256;
        // D s_10_1: read-var esizeshadow#3451:i64
        let s_10_1: i64 = fn_state.esizeshadow_3451;
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
        // D s_10_8: call execute_SQSHRUNT_Z_ZI__(s_10_0, s_10_6, s_10_3, s_10_7, s_10_5)
        let s_10_8: () = execute_SQSHRUNT_Z_ZI__(
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
        // C s_11_1: const #512s : i
        let s_11_1: i128 = 512;
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
        // C s_12_0: const #512s : i64
        let s_12_0: i64 = 512;
        // D s_12_1: read-var esizeshadow#3451:i64
        let s_12_1: i64 = fn_state.esizeshadow_3451;
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
        // D s_12_8: call execute_SQSHRUNT_Z_ZI__(s_12_0, s_12_6, s_12_3, s_12_7, s_12_5)
        let s_12_8: () = execute_SQSHRUNT_Z_ZI__(
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
        // C s_13_1: const #1024s : i
        let s_13_1: i128 = 1024;
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
        // C s_14_0: const #1024s : i64
        let s_14_0: i64 = 1024;
        // D s_14_1: read-var esizeshadow#3451:i64
        let s_14_1: i64 = fn_state.esizeshadow_3451;
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
        // D s_14_8: call execute_SQSHRUNT_Z_ZI__(s_14_0, s_14_6, s_14_3, s_14_7, s_14_5)
        let s_14_8: () = execute_SQSHRUNT_Z_ZI__(
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
        // D s_15_0: read-var VL:i64
        let s_15_0: i64 = fn_state.VL;
        // C s_15_1: const #2048s : i
        let s_15_1: i128 = 2048;
        // D s_15_2: cast zx s_15_0 -> i
        let s_15_2: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_1
        let s_15_3: bool = ((s_15_2) == (s_15_1));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #2048s : i64
        let s_16_0: i64 = 2048;
        // D s_16_1: read-var esizeshadow#3451:i64
        let s_16_1: i64 = fn_state.esizeshadow_3451;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // D s_16_4: read-var shift:i64
        let s_16_4: i64 = fn_state.shift;
        // D s_16_5: cast zx s_16_4 -> i
        let s_16_5: i128 = (i128::try_from(s_16_4).unwrap());
        // D s_16_6: read-var d:i64
        let s_16_6: i64 = fn_state.d;
        // D s_16_7: read-var n:i64
        let s_16_7: i64 = fn_state.n;
        // D s_16_8: call execute_SQSHRUNT_Z_ZI__(s_16_0, s_16_6, s_16_3, s_16_7, s_16_5)
        let s_16_8: () = execute_SQSHRUNT_Z_ZI__(
            state,
            tracer,
            s_16_0,
            s_16_6,
            s_16_3,
            s_16_7,
            s_16_5,
        );
        // N s_16_9: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var tsize:u8
        let s_18_0: u8 = fn_state.tsize;
        // C s_18_1: const #1s : i
        let s_18_1: i128 = 1;
        // D s_18_2: cast zx s_18_0 -> bv
        let s_18_2: Bits = Bits::new(s_18_0 as u128, 3u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #1s : i
        let s_18_5: i128 = 1;
        // C s_18_6: add s_18_5 s_18_4
        let s_18_6: i128 = (s_18_5 + s_18_4);
        // D s_18_7: bit-extract s_18_2 s_18_1 s_18_6
        let s_18_7: Bits = (Bits::new(
            ((s_18_2) >> (s_18_1)).value(),
            u16::try_from(s_18_6).unwrap(),
        ));
        // D s_18_8: cast reint s_18_7 -> u8
        let s_18_8: u8 = (s_18_7.value() as u8);
        // D s_18_9: cast zx s_18_8 -> bv
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 2u16);
        // C s_18_10: const #1u : u8
        let s_18_10: u8 = 1;
        // C s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 2u16);
        // D s_18_12: cmp-eq s_18_9 s_18_11
        let s_18_12: bool = ((s_18_9) == (s_18_11));
        // D s_18_13: not s_18_12
        let s_18_13: bool = !s_18_12;
        // N s_18_14: branch s_18_13 b20 b19
        if s_18_13 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #16s : i64
        let s_19_0: i64 = 16;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // N s_19_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #32s : i64
        let s_20_0: i64 = 32;
        // D s_20_1: write-var esize <= s_20_0
        fn_state.esize = s_20_0;
        // N s_20_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaveSME(s_22_0)
        let s_22_1: bool = HaveSME(state, tracer, s_22_0);
        // S s_22_2: not s_22_1
        let s_22_2: bool = !s_22_1;
        // D s_22_3: write-var gs#204316 <= s_22_2
        fn_state.gs_204316 = s_22_2;
        // N s_22_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
