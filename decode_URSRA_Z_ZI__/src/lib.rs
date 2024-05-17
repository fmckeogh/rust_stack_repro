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
use execute_URSRA_Z_ZI__::*;
use HaveSVE2::*;
use HaveSME::*;
use common::*;
pub fn decode_URSRA_Z_ZI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tszh: u8,
    tszl: u8,
    imm3: u8,
    R: bool,
    U: bool,
    Zn: u8,
    Zda: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        shift: i64,
        tsize: u8,
        esizeshadow_4203: i64,
        da: i64,
        esize: i64,
        n: i64,
        VL: i64,
        gs_217518: bool,
        tszh: u8,
        tszl: u8,
        imm3: u8,
        R: bool,
        U: bool,
        Zn: u8,
        Zda: u8,
    }
    let fn_state = FunctionState {
        tszh,
        tszl,
        imm3,
        R,
        U,
        Zn,
        Zda,
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
        // N s_0_6: branch s_0_5 b62 b1
        if s_0_5 {
            return block_62(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#217518 <= s_1_0
        fn_state.gs_217518 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#217518:u8
        let s_2_0: bool = fn_state.gs_217518;
        // N s_2_1: branch s_2_0 b61 b3
        if s_2_0 {
            return block_61(state, tracer, fn_state);
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
        let s_3_0: u8 = fn_state.tszh;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
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
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 4u16);
        // C s_3_18: const #0u : u8
        let s_3_18: u8 = 0;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 4u16);
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
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b56 b6
        if s_5_5 {
            return block_56(state, tracer, fn_state);
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
        // D s_7_1: write-var esizeshadow#4203 <= s_7_0
        fn_state.esizeshadow_4203 = s_7_0;
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
        // D s_7_7: read-var Zda:u8
        let s_7_7: u8 = fn_state.Zda;
        // D s_7_8: cast zx s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 5u16);
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (s_7_8.value() as i128);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: write-var da <= s_7_10
        fn_state.da = s_7_10;
        // C s_7_12: const #2s : i
        let s_7_12: i128 = 2;
        // D s_7_13: read-var esizeshadow#4203:i64
        let s_7_13: i64 = fn_state.esizeshadow_4203;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: mul s_7_12 s_7_14
        let s_7_15: i128 = ((s_7_12) * (s_7_14));
        // D s_7_16: cast reint s_7_15 -> i64
        let s_7_16: i64 = (s_7_15 as i64);
        // D s_7_17: read-var tsize:u8
        let s_7_17: u8 = fn_state.tsize;
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 4u16);
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
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 7u16);
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
        // D s_7_38: read-var esizeshadow#4203:i64
        let s_7_38: i64 = fn_state.esizeshadow_4203;
        // C s_7_39: const #8s : i
        let s_7_39: i128 = 8;
        // D s_7_40: cast zx s_7_38 -> i
        let s_7_40: i128 = (i128::try_from(s_7_38).unwrap());
        // D s_7_41: cmp-eq s_7_40 s_7_39
        let s_7_41: bool = ((s_7_40) == (s_7_39));
        // D s_7_42: not s_7_41
        let s_7_42: bool = !s_7_41;
        // N s_7_43: branch s_7_42 b19 b8
        if s_7_42 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #128s : i
        let s_8_1: i128 = 128;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #128s : i64
        let s_9_0: i64 = 128;
        // C s_9_1: const #8s : i64
        let s_9_1: i64 = 8;
        // D s_9_2: read-var shift:i64
        let s_9_2: i64 = fn_state.shift;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var da:i64
        let s_9_4: i64 = fn_state.da;
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: call execute_URSRA_Z_ZI__(s_9_0, s_9_4, s_9_1, s_9_5, s_9_3)
        let s_9_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_9_0,
            s_9_4,
            s_9_1,
            s_9_5,
            s_9_3,
        );
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #256s : i
        let s_10_1: i128 = 256;
        // D s_10_2: cast zx s_10_0 -> i
        let s_10_2: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_3: cmp-eq s_10_2 s_10_1
        let s_10_3: bool = ((s_10_2) == (s_10_1));
        // D s_10_4: not s_10_3
        let s_10_4: bool = !s_10_3;
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #256s : i64
        let s_11_0: i64 = 256;
        // C s_11_1: const #8s : i64
        let s_11_1: i64 = 8;
        // D s_11_2: read-var shift:i64
        let s_11_2: i64 = fn_state.shift;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: read-var da:i64
        let s_11_4: i64 = fn_state.da;
        // D s_11_5: read-var n:i64
        let s_11_5: i64 = fn_state.n;
        // D s_11_6: call execute_URSRA_Z_ZI__(s_11_0, s_11_4, s_11_1, s_11_5, s_11_3)
        let s_11_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_11_0,
            s_11_4,
            s_11_1,
            s_11_5,
            s_11_3,
        );
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #512s : i
        let s_12_1: i128 = 512;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #512s : i64
        let s_13_0: i64 = 512;
        // C s_13_1: const #8s : i64
        let s_13_1: i64 = 8;
        // D s_13_2: read-var shift:i64
        let s_13_2: i64 = fn_state.shift;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: read-var da:i64
        let s_13_4: i64 = fn_state.da;
        // D s_13_5: read-var n:i64
        let s_13_5: i64 = fn_state.n;
        // D s_13_6: call execute_URSRA_Z_ZI__(s_13_0, s_13_4, s_13_1, s_13_5, s_13_3)
        let s_13_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_13_0,
            s_13_4,
            s_13_1,
            s_13_5,
            s_13_3,
        );
        // N s_13_7: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var VL:i64
        let s_14_0: i64 = fn_state.VL;
        // C s_14_1: const #1024s : i
        let s_14_1: i128 = 1024;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
        // N s_14_5: branch s_14_4 b16 b15
        if s_14_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #1024s : i64
        let s_15_0: i64 = 1024;
        // C s_15_1: const #8s : i64
        let s_15_1: i64 = 8;
        // D s_15_2: read-var shift:i64
        let s_15_2: i64 = fn_state.shift;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: read-var da:i64
        let s_15_4: i64 = fn_state.da;
        // D s_15_5: read-var n:i64
        let s_15_5: i64 = fn_state.n;
        // D s_15_6: call execute_URSRA_Z_ZI__(s_15_0, s_15_4, s_15_1, s_15_5, s_15_3)
        let s_15_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_15_0,
            s_15_4,
            s_15_1,
            s_15_5,
            s_15_3,
        );
        // N s_15_7: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var VL:i64
        let s_16_0: i64 = fn_state.VL;
        // C s_16_1: const #2048s : i
        let s_16_1: i128 = 2048;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #2048s : i64
        let s_17_0: i64 = 2048;
        // C s_17_1: const #8s : i64
        let s_17_1: i64 = 8;
        // D s_17_2: read-var shift:i64
        let s_17_2: i64 = fn_state.shift;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-var da:i64
        let s_17_4: i64 = fn_state.da;
        // D s_17_5: read-var n:i64
        let s_17_5: i64 = fn_state.n;
        // D s_17_6: call execute_URSRA_Z_ZI__(s_17_0, s_17_4, s_17_1, s_17_5, s_17_3)
        let s_17_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_17_0,
            s_17_4,
            s_17_1,
            s_17_5,
            s_17_3,
        );
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var esizeshadow#4203:i64
        let s_19_0: i64 = fn_state.esizeshadow_4203;
        // C s_19_1: const #16s : i
        let s_19_1: i128 = 16;
        // D s_19_2: cast zx s_19_0 -> i
        let s_19_2: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_3: cmp-eq s_19_2 s_19_1
        let s_19_3: bool = ((s_19_2) == (s_19_1));
        // D s_19_4: not s_19_3
        let s_19_4: bool = !s_19_3;
        // N s_19_5: branch s_19_4 b31 b20
        if s_19_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var VL:i64
        let s_20_0: i64 = fn_state.VL;
        // C s_20_1: const #128s : i
        let s_20_1: i128 = 128;
        // D s_20_2: cast zx s_20_0 -> i
        let s_20_2: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_3: cmp-eq s_20_2 s_20_1
        let s_20_3: bool = ((s_20_2) == (s_20_1));
        // D s_20_4: not s_20_3
        let s_20_4: bool = !s_20_3;
        // N s_20_5: branch s_20_4 b22 b21
        if s_20_4 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #128s : i64
        let s_21_0: i64 = 128;
        // C s_21_1: const #16s : i64
        let s_21_1: i64 = 16;
        // D s_21_2: read-var shift:i64
        let s_21_2: i64 = fn_state.shift;
        // D s_21_3: cast zx s_21_2 -> i
        let s_21_3: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_4: read-var da:i64
        let s_21_4: i64 = fn_state.da;
        // D s_21_5: read-var n:i64
        let s_21_5: i64 = fn_state.n;
        // D s_21_6: call execute_URSRA_Z_ZI__(s_21_0, s_21_4, s_21_1, s_21_5, s_21_3)
        let s_21_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_21_0,
            s_21_4,
            s_21_1,
            s_21_5,
            s_21_3,
        );
        // N s_21_7: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var VL:i64
        let s_22_0: i64 = fn_state.VL;
        // C s_22_1: const #256s : i
        let s_22_1: i128 = 256;
        // D s_22_2: cast zx s_22_0 -> i
        let s_22_2: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_3: cmp-eq s_22_2 s_22_1
        let s_22_3: bool = ((s_22_2) == (s_22_1));
        // D s_22_4: not s_22_3
        let s_22_4: bool = !s_22_3;
        // N s_22_5: branch s_22_4 b24 b23
        if s_22_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #256s : i64
        let s_23_0: i64 = 256;
        // C s_23_1: const #16s : i64
        let s_23_1: i64 = 16;
        // D s_23_2: read-var shift:i64
        let s_23_2: i64 = fn_state.shift;
        // D s_23_3: cast zx s_23_2 -> i
        let s_23_3: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_4: read-var da:i64
        let s_23_4: i64 = fn_state.da;
        // D s_23_5: read-var n:i64
        let s_23_5: i64 = fn_state.n;
        // D s_23_6: call execute_URSRA_Z_ZI__(s_23_0, s_23_4, s_23_1, s_23_5, s_23_3)
        let s_23_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_23_0,
            s_23_4,
            s_23_1,
            s_23_5,
            s_23_3,
        );
        // N s_23_7: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var VL:i64
        let s_24_0: i64 = fn_state.VL;
        // C s_24_1: const #512s : i
        let s_24_1: i128 = 512;
        // D s_24_2: cast zx s_24_0 -> i
        let s_24_2: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_3: cmp-eq s_24_2 s_24_1
        let s_24_3: bool = ((s_24_2) == (s_24_1));
        // D s_24_4: not s_24_3
        let s_24_4: bool = !s_24_3;
        // N s_24_5: branch s_24_4 b26 b25
        if s_24_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #512s : i64
        let s_25_0: i64 = 512;
        // C s_25_1: const #16s : i64
        let s_25_1: i64 = 16;
        // D s_25_2: read-var shift:i64
        let s_25_2: i64 = fn_state.shift;
        // D s_25_3: cast zx s_25_2 -> i
        let s_25_3: i128 = (i128::try_from(s_25_2).unwrap());
        // D s_25_4: read-var da:i64
        let s_25_4: i64 = fn_state.da;
        // D s_25_5: read-var n:i64
        let s_25_5: i64 = fn_state.n;
        // D s_25_6: call execute_URSRA_Z_ZI__(s_25_0, s_25_4, s_25_1, s_25_5, s_25_3)
        let s_25_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_25_0,
            s_25_4,
            s_25_1,
            s_25_5,
            s_25_3,
        );
        // N s_25_7: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var VL:i64
        let s_26_0: i64 = fn_state.VL;
        // C s_26_1: const #1024s : i
        let s_26_1: i128 = 1024;
        // D s_26_2: cast zx s_26_0 -> i
        let s_26_2: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_3: cmp-eq s_26_2 s_26_1
        let s_26_3: bool = ((s_26_2) == (s_26_1));
        // D s_26_4: not s_26_3
        let s_26_4: bool = !s_26_3;
        // N s_26_5: branch s_26_4 b28 b27
        if s_26_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1024s : i64
        let s_27_0: i64 = 1024;
        // C s_27_1: const #16s : i64
        let s_27_1: i64 = 16;
        // D s_27_2: read-var shift:i64
        let s_27_2: i64 = fn_state.shift;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: read-var da:i64
        let s_27_4: i64 = fn_state.da;
        // D s_27_5: read-var n:i64
        let s_27_5: i64 = fn_state.n;
        // D s_27_6: call execute_URSRA_Z_ZI__(s_27_0, s_27_4, s_27_1, s_27_5, s_27_3)
        let s_27_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_27_0,
            s_27_4,
            s_27_1,
            s_27_5,
            s_27_3,
        );
        // N s_27_7: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var VL:i64
        let s_28_0: i64 = fn_state.VL;
        // C s_28_1: const #2048s : i
        let s_28_1: i128 = 2048;
        // D s_28_2: cast zx s_28_0 -> i
        let s_28_2: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_3: cmp-eq s_28_2 s_28_1
        let s_28_3: bool = ((s_28_2) == (s_28_1));
        // D s_28_4: not s_28_3
        let s_28_4: bool = !s_28_3;
        // N s_28_5: branch s_28_4 b30 b29
        if s_28_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #2048s : i64
        let s_29_0: i64 = 2048;
        // C s_29_1: const #16s : i64
        let s_29_1: i64 = 16;
        // D s_29_2: read-var shift:i64
        let s_29_2: i64 = fn_state.shift;
        // D s_29_3: cast zx s_29_2 -> i
        let s_29_3: i128 = (i128::try_from(s_29_2).unwrap());
        // D s_29_4: read-var da:i64
        let s_29_4: i64 = fn_state.da;
        // D s_29_5: read-var n:i64
        let s_29_5: i64 = fn_state.n;
        // D s_29_6: call execute_URSRA_Z_ZI__(s_29_0, s_29_4, s_29_1, s_29_5, s_29_3)
        let s_29_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_29_0,
            s_29_4,
            s_29_1,
            s_29_5,
            s_29_3,
        );
        // N s_29_7: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esizeshadow#4203:i64
        let s_31_0: i64 = fn_state.esizeshadow_4203;
        // C s_31_1: const #32s : i
        let s_31_1: i128 = 32;
        // D s_31_2: cast zx s_31_0 -> i
        let s_31_2: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_3: cmp-eq s_31_2 s_31_1
        let s_31_3: bool = ((s_31_2) == (s_31_1));
        // D s_31_4: not s_31_3
        let s_31_4: bool = !s_31_3;
        // N s_31_5: branch s_31_4 b43 b32
        if s_31_4 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var VL:i64
        let s_32_0: i64 = fn_state.VL;
        // C s_32_1: const #128s : i
        let s_32_1: i128 = 128;
        // D s_32_2: cast zx s_32_0 -> i
        let s_32_2: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_3: cmp-eq s_32_2 s_32_1
        let s_32_3: bool = ((s_32_2) == (s_32_1));
        // D s_32_4: not s_32_3
        let s_32_4: bool = !s_32_3;
        // N s_32_5: branch s_32_4 b34 b33
        if s_32_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #128s : i64
        let s_33_0: i64 = 128;
        // C s_33_1: const #32s : i64
        let s_33_1: i64 = 32;
        // D s_33_2: read-var shift:i64
        let s_33_2: i64 = fn_state.shift;
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_4: read-var da:i64
        let s_33_4: i64 = fn_state.da;
        // D s_33_5: read-var n:i64
        let s_33_5: i64 = fn_state.n;
        // D s_33_6: call execute_URSRA_Z_ZI__(s_33_0, s_33_4, s_33_1, s_33_5, s_33_3)
        let s_33_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_33_0,
            s_33_4,
            s_33_1,
            s_33_5,
            s_33_3,
        );
        // N s_33_7: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var VL:i64
        let s_34_0: i64 = fn_state.VL;
        // C s_34_1: const #256s : i
        let s_34_1: i128 = 256;
        // D s_34_2: cast zx s_34_0 -> i
        let s_34_2: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_3: cmp-eq s_34_2 s_34_1
        let s_34_3: bool = ((s_34_2) == (s_34_1));
        // D s_34_4: not s_34_3
        let s_34_4: bool = !s_34_3;
        // N s_34_5: branch s_34_4 b36 b35
        if s_34_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #256s : i64
        let s_35_0: i64 = 256;
        // C s_35_1: const #32s : i64
        let s_35_1: i64 = 32;
        // D s_35_2: read-var shift:i64
        let s_35_2: i64 = fn_state.shift;
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // D s_35_4: read-var da:i64
        let s_35_4: i64 = fn_state.da;
        // D s_35_5: read-var n:i64
        let s_35_5: i64 = fn_state.n;
        // D s_35_6: call execute_URSRA_Z_ZI__(s_35_0, s_35_4, s_35_1, s_35_5, s_35_3)
        let s_35_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_35_0,
            s_35_4,
            s_35_1,
            s_35_5,
            s_35_3,
        );
        // N s_35_7: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var VL:i64
        let s_36_0: i64 = fn_state.VL;
        // C s_36_1: const #512s : i
        let s_36_1: i128 = 512;
        // D s_36_2: cast zx s_36_0 -> i
        let s_36_2: i128 = (i128::try_from(s_36_0).unwrap());
        // D s_36_3: cmp-eq s_36_2 s_36_1
        let s_36_3: bool = ((s_36_2) == (s_36_1));
        // D s_36_4: not s_36_3
        let s_36_4: bool = !s_36_3;
        // N s_36_5: branch s_36_4 b38 b37
        if s_36_4 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #512s : i64
        let s_37_0: i64 = 512;
        // C s_37_1: const #32s : i64
        let s_37_1: i64 = 32;
        // D s_37_2: read-var shift:i64
        let s_37_2: i64 = fn_state.shift;
        // D s_37_3: cast zx s_37_2 -> i
        let s_37_3: i128 = (i128::try_from(s_37_2).unwrap());
        // D s_37_4: read-var da:i64
        let s_37_4: i64 = fn_state.da;
        // D s_37_5: read-var n:i64
        let s_37_5: i64 = fn_state.n;
        // D s_37_6: call execute_URSRA_Z_ZI__(s_37_0, s_37_4, s_37_1, s_37_5, s_37_3)
        let s_37_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_37_0,
            s_37_4,
            s_37_1,
            s_37_5,
            s_37_3,
        );
        // N s_37_7: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var VL:i64
        let s_38_0: i64 = fn_state.VL;
        // C s_38_1: const #1024s : i
        let s_38_1: i128 = 1024;
        // D s_38_2: cast zx s_38_0 -> i
        let s_38_2: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_3: cmp-eq s_38_2 s_38_1
        let s_38_3: bool = ((s_38_2) == (s_38_1));
        // D s_38_4: not s_38_3
        let s_38_4: bool = !s_38_3;
        // N s_38_5: branch s_38_4 b40 b39
        if s_38_4 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1024s : i64
        let s_39_0: i64 = 1024;
        // C s_39_1: const #32s : i64
        let s_39_1: i64 = 32;
        // D s_39_2: read-var shift:i64
        let s_39_2: i64 = fn_state.shift;
        // D s_39_3: cast zx s_39_2 -> i
        let s_39_3: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_4: read-var da:i64
        let s_39_4: i64 = fn_state.da;
        // D s_39_5: read-var n:i64
        let s_39_5: i64 = fn_state.n;
        // D s_39_6: call execute_URSRA_Z_ZI__(s_39_0, s_39_4, s_39_1, s_39_5, s_39_3)
        let s_39_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_39_0,
            s_39_4,
            s_39_1,
            s_39_5,
            s_39_3,
        );
        // N s_39_7: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var VL:i64
        let s_40_0: i64 = fn_state.VL;
        // C s_40_1: const #2048s : i
        let s_40_1: i128 = 2048;
        // D s_40_2: cast zx s_40_0 -> i
        let s_40_2: i128 = (i128::try_from(s_40_0).unwrap());
        // D s_40_3: cmp-eq s_40_2 s_40_1
        let s_40_3: bool = ((s_40_2) == (s_40_1));
        // D s_40_4: not s_40_3
        let s_40_4: bool = !s_40_3;
        // N s_40_5: branch s_40_4 b42 b41
        if s_40_4 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #2048s : i64
        let s_41_0: i64 = 2048;
        // C s_41_1: const #32s : i64
        let s_41_1: i64 = 32;
        // D s_41_2: read-var shift:i64
        let s_41_2: i64 = fn_state.shift;
        // D s_41_3: cast zx s_41_2 -> i
        let s_41_3: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_4: read-var da:i64
        let s_41_4: i64 = fn_state.da;
        // D s_41_5: read-var n:i64
        let s_41_5: i64 = fn_state.n;
        // D s_41_6: call execute_URSRA_Z_ZI__(s_41_0, s_41_4, s_41_1, s_41_5, s_41_3)
        let s_41_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_41_0,
            s_41_4,
            s_41_1,
            s_41_5,
            s_41_3,
        );
        // N s_41_7: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var esizeshadow#4203:i64
        let s_43_0: i64 = fn_state.esizeshadow_4203;
        // C s_43_1: const #64s : i
        let s_43_1: i128 = 64;
        // D s_43_2: cast zx s_43_0 -> i
        let s_43_2: i128 = (i128::try_from(s_43_0).unwrap());
        // D s_43_3: cmp-eq s_43_2 s_43_1
        let s_43_3: bool = ((s_43_2) == (s_43_1));
        // D s_43_4: not s_43_3
        let s_43_4: bool = !s_43_3;
        // N s_43_5: branch s_43_4 b55 b44
        if s_43_4 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var VL:i64
        let s_44_0: i64 = fn_state.VL;
        // C s_44_1: const #128s : i
        let s_44_1: i128 = 128;
        // D s_44_2: cast zx s_44_0 -> i
        let s_44_2: i128 = (i128::try_from(s_44_0).unwrap());
        // D s_44_3: cmp-eq s_44_2 s_44_1
        let s_44_3: bool = ((s_44_2) == (s_44_1));
        // D s_44_4: not s_44_3
        let s_44_4: bool = !s_44_3;
        // N s_44_5: branch s_44_4 b46 b45
        if s_44_4 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #128s : i64
        let s_45_0: i64 = 128;
        // C s_45_1: const #64s : i64
        let s_45_1: i64 = 64;
        // D s_45_2: read-var shift:i64
        let s_45_2: i64 = fn_state.shift;
        // D s_45_3: cast zx s_45_2 -> i
        let s_45_3: i128 = (i128::try_from(s_45_2).unwrap());
        // D s_45_4: read-var da:i64
        let s_45_4: i64 = fn_state.da;
        // D s_45_5: read-var n:i64
        let s_45_5: i64 = fn_state.n;
        // D s_45_6: call execute_URSRA_Z_ZI__(s_45_0, s_45_4, s_45_1, s_45_5, s_45_3)
        let s_45_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_45_0,
            s_45_4,
            s_45_1,
            s_45_5,
            s_45_3,
        );
        // N s_45_7: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var VL:i64
        let s_46_0: i64 = fn_state.VL;
        // C s_46_1: const #256s : i
        let s_46_1: i128 = 256;
        // D s_46_2: cast zx s_46_0 -> i
        let s_46_2: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_3: cmp-eq s_46_2 s_46_1
        let s_46_3: bool = ((s_46_2) == (s_46_1));
        // D s_46_4: not s_46_3
        let s_46_4: bool = !s_46_3;
        // N s_46_5: branch s_46_4 b48 b47
        if s_46_4 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #256s : i64
        let s_47_0: i64 = 256;
        // C s_47_1: const #64s : i64
        let s_47_1: i64 = 64;
        // D s_47_2: read-var shift:i64
        let s_47_2: i64 = fn_state.shift;
        // D s_47_3: cast zx s_47_2 -> i
        let s_47_3: i128 = (i128::try_from(s_47_2).unwrap());
        // D s_47_4: read-var da:i64
        let s_47_4: i64 = fn_state.da;
        // D s_47_5: read-var n:i64
        let s_47_5: i64 = fn_state.n;
        // D s_47_6: call execute_URSRA_Z_ZI__(s_47_0, s_47_4, s_47_1, s_47_5, s_47_3)
        let s_47_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_47_0,
            s_47_4,
            s_47_1,
            s_47_5,
            s_47_3,
        );
        // N s_47_7: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var VL:i64
        let s_48_0: i64 = fn_state.VL;
        // C s_48_1: const #512s : i
        let s_48_1: i128 = 512;
        // D s_48_2: cast zx s_48_0 -> i
        let s_48_2: i128 = (i128::try_from(s_48_0).unwrap());
        // D s_48_3: cmp-eq s_48_2 s_48_1
        let s_48_3: bool = ((s_48_2) == (s_48_1));
        // D s_48_4: not s_48_3
        let s_48_4: bool = !s_48_3;
        // N s_48_5: branch s_48_4 b50 b49
        if s_48_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #512s : i64
        let s_49_0: i64 = 512;
        // C s_49_1: const #64s : i64
        let s_49_1: i64 = 64;
        // D s_49_2: read-var shift:i64
        let s_49_2: i64 = fn_state.shift;
        // D s_49_3: cast zx s_49_2 -> i
        let s_49_3: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_4: read-var da:i64
        let s_49_4: i64 = fn_state.da;
        // D s_49_5: read-var n:i64
        let s_49_5: i64 = fn_state.n;
        // D s_49_6: call execute_URSRA_Z_ZI__(s_49_0, s_49_4, s_49_1, s_49_5, s_49_3)
        let s_49_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_49_0,
            s_49_4,
            s_49_1,
            s_49_5,
            s_49_3,
        );
        // N s_49_7: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var VL:i64
        let s_50_0: i64 = fn_state.VL;
        // C s_50_1: const #1024s : i
        let s_50_1: i128 = 1024;
        // D s_50_2: cast zx s_50_0 -> i
        let s_50_2: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_3: cmp-eq s_50_2 s_50_1
        let s_50_3: bool = ((s_50_2) == (s_50_1));
        // D s_50_4: not s_50_3
        let s_50_4: bool = !s_50_3;
        // N s_50_5: branch s_50_4 b52 b51
        if s_50_4 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1024s : i64
        let s_51_0: i64 = 1024;
        // C s_51_1: const #64s : i64
        let s_51_1: i64 = 64;
        // D s_51_2: read-var shift:i64
        let s_51_2: i64 = fn_state.shift;
        // D s_51_3: cast zx s_51_2 -> i
        let s_51_3: i128 = (i128::try_from(s_51_2).unwrap());
        // D s_51_4: read-var da:i64
        let s_51_4: i64 = fn_state.da;
        // D s_51_5: read-var n:i64
        let s_51_5: i64 = fn_state.n;
        // D s_51_6: call execute_URSRA_Z_ZI__(s_51_0, s_51_4, s_51_1, s_51_5, s_51_3)
        let s_51_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_51_0,
            s_51_4,
            s_51_1,
            s_51_5,
            s_51_3,
        );
        // N s_51_7: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var VL:i64
        let s_52_0: i64 = fn_state.VL;
        // C s_52_1: const #2048s : i
        let s_52_1: i128 = 2048;
        // D s_52_2: cast zx s_52_0 -> i
        let s_52_2: i128 = (i128::try_from(s_52_0).unwrap());
        // D s_52_3: cmp-eq s_52_2 s_52_1
        let s_52_3: bool = ((s_52_2) == (s_52_1));
        // D s_52_4: not s_52_3
        let s_52_4: bool = !s_52_3;
        // N s_52_5: branch s_52_4 b54 b53
        if s_52_4 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #2048s : i64
        let s_53_0: i64 = 2048;
        // C s_53_1: const #64s : i64
        let s_53_1: i64 = 64;
        // D s_53_2: read-var shift:i64
        let s_53_2: i64 = fn_state.shift;
        // D s_53_3: cast zx s_53_2 -> i
        let s_53_3: i128 = (i128::try_from(s_53_2).unwrap());
        // D s_53_4: read-var da:i64
        let s_53_4: i64 = fn_state.da;
        // D s_53_5: read-var n:i64
        let s_53_5: i64 = fn_state.n;
        // D s_53_6: call execute_URSRA_Z_ZI__(s_53_0, s_53_4, s_53_1, s_53_5, s_53_3)
        let s_53_6: () = execute_URSRA_Z_ZI__(
            state,
            tracer,
            s_53_0,
            s_53_4,
            s_53_1,
            s_53_5,
            s_53_3,
        );
        // N s_53_7: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // N s_55_1: assert s_55_0
        let s_55_1: () = assert!(s_55_0);
        // N s_55_2: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var tsize:u8
        let s_56_0: u8 = fn_state.tsize;
        // C s_56_1: const #1s : i
        let s_56_1: i128 = 1;
        // D s_56_2: cast zx s_56_0 -> bv
        let s_56_2: Bits = Bits::new(s_56_0 as u128, 4u16);
        // C s_56_3: const #1s : i64
        let s_56_3: i64 = 1;
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #2s : i
        let s_56_5: i128 = 2;
        // C s_56_6: add s_56_5 s_56_4
        let s_56_6: i128 = (s_56_5 + s_56_4);
        // D s_56_7: bit-extract s_56_2 s_56_1 s_56_6
        let s_56_7: Bits = (Bits::new(
            ((s_56_2) >> (s_56_1)).value(),
            u16::try_from(s_56_6).unwrap(),
        ));
        // D s_56_8: cast reint s_56_7 -> u8
        let s_56_8: u8 = (s_56_7.value() as u8);
        // D s_56_9: cast zx s_56_8 -> bv
        let s_56_9: Bits = Bits::new(s_56_8 as u128, 3u16);
        // C s_56_10: const #1u : u8
        let s_56_10: u8 = 1;
        // C s_56_11: cast zx s_56_10 -> bv
        let s_56_11: Bits = Bits::new(s_56_10 as u128, 3u16);
        // D s_56_12: cmp-eq s_56_9 s_56_11
        let s_56_12: bool = ((s_56_9) == (s_56_11));
        // D s_56_13: not s_56_12
        let s_56_13: bool = !s_56_12;
        // N s_56_14: branch s_56_13 b58 b57
        if s_56_13 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #16s : i64
        let s_57_0: i64 = 16;
        // D s_57_1: write-var esize <= s_57_0
        fn_state.esize = s_57_0;
        // N s_57_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var tsize:u8
        let s_58_0: u8 = fn_state.tsize;
        // C s_58_1: const #2s : i
        let s_58_1: i128 = 2;
        // D s_58_2: cast zx s_58_0 -> bv
        let s_58_2: Bits = Bits::new(s_58_0 as u128, 4u16);
        // C s_58_3: const #1s : i64
        let s_58_3: i64 = 1;
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // C s_58_5: const #1s : i
        let s_58_5: i128 = 1;
        // C s_58_6: add s_58_5 s_58_4
        let s_58_6: i128 = (s_58_5 + s_58_4);
        // D s_58_7: bit-extract s_58_2 s_58_1 s_58_6
        let s_58_7: Bits = (Bits::new(
            ((s_58_2) >> (s_58_1)).value(),
            u16::try_from(s_58_6).unwrap(),
        ));
        // D s_58_8: cast reint s_58_7 -> u8
        let s_58_8: u8 = (s_58_7.value() as u8);
        // D s_58_9: cast zx s_58_8 -> bv
        let s_58_9: Bits = Bits::new(s_58_8 as u128, 2u16);
        // C s_58_10: const #1u : u8
        let s_58_10: u8 = 1;
        // C s_58_11: cast zx s_58_10 -> bv
        let s_58_11: Bits = Bits::new(s_58_10 as u128, 2u16);
        // D s_58_12: cmp-eq s_58_9 s_58_11
        let s_58_12: bool = ((s_58_9) == (s_58_11));
        // D s_58_13: not s_58_12
        let s_58_13: bool = !s_58_12;
        // N s_58_14: branch s_58_13 b60 b59
        if s_58_13 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #32s : i64
        let s_59_0: i64 = 32;
        // D s_59_1: write-var esize <= s_59_0
        fn_state.esize = s_59_0;
        // N s_59_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #64s : i64
        let s_60_0: i64 = 64;
        // D s_60_1: write-var esize <= s_60_0
        fn_state.esize = s_60_0;
        // N s_60_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: panic
        panic!("{:?}", ());
        // N s_61_1: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call HaveSME(s_62_0)
        let s_62_1: bool = HaveSME(state, tracer, s_62_0);
        // S s_62_2: not s_62_1
        let s_62_2: bool = !s_62_1;
        // D s_62_3: write-var gs#217518 <= s_62_2
        fn_state.gs_217518 = s_62_2;
        // N s_62_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
