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
use HaveSME2::*;
use execute_LUTI2_MZ4_ZTZ_1::*;
use common::*;
pub fn decode_LUTI2_MZ4_ZTZ_1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    i2: u8,
    size: u8,
    Zn: u8,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        n: i64,
        VL: i64,
        d: i64,
        imm: i64,
        i2: u8,
        size: u8,
        Zn: u8,
        Zd: u8,
    }
    let fn_state = FunctionState {
        i2,
        size,
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
        // S s_0_4: call HaveSME2(s_0_3)
        let s_0_4: bool = HaveSME2(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b14 b1
        if s_0_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #3u : u8
        let s_1_2: u8 = 3;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b13 b2
        if s_1_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #8s : i64
        let s_2_4: i64 = 8;
        // D s_2_5: lsl s_2_4 s_2_3
        let s_2_5: i64 = s_2_4 << s_2_3;
        // D s_2_6: write-var esize <= s_2_5
        fn_state.esize = s_2_5;
        // D s_2_7: read-var Zn:u8
        let s_2_7: u8 = fn_state.Zn;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 5u16);
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (s_2_8.value() as i128);
        // D s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // D s_2_11: write-var n <= s_2_10
        fn_state.n = s_2_10;
        // D s_2_12: read-var Zd:u8
        let s_2_12: u8 = fn_state.Zd;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 3u16);
        // C s_2_14: const #0u : u8
        let s_2_14: u8 = 0;
        // C s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 2u16);
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // C s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u8
        let s_2_24: u8 = (s_2_23.value() as u8);
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 5u16);
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (s_2_25.value() as i128);
        // D s_2_27: cast reint s_2_26 -> i64
        let s_2_27: i64 = (s_2_26 as i64);
        // D s_2_28: write-var d <= s_2_27
        fn_state.d = s_2_27;
        // D s_2_29: read-var i2:u8
        let s_2_29: u8 = fn_state.i2;
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 2u16);
        // D s_2_31: cast zx s_2_30 -> i
        let s_2_31: i128 = (s_2_30.value() as i128);
        // D s_2_32: cast reint s_2_31 -> i64
        let s_2_32: i64 = (s_2_31 as i64);
        // D s_2_33: write-var imm <= s_2_32
        fn_state.imm = s_2_32;
        // D s_2_34: read-var VL:i64
        let s_2_34: i64 = fn_state.VL;
        // C s_2_35: const #128s : i
        let s_2_35: i128 = 128;
        // D s_2_36: cast zx s_2_34 -> i
        let s_2_36: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_37: cmp-eq s_2_36 s_2_35
        let s_2_37: bool = ((s_2_36) == (s_2_35));
        // D s_2_38: not s_2_37
        let s_2_38: bool = !s_2_37;
        // N s_2_39: branch s_2_38 b4 b3
        if s_2_38 {
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
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: read-var esize:i64
        let s_3_1: i64 = fn_state.esize;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // C s_3_5: const #2s : i64
        let s_3_5: i64 = 2;
        // C s_3_6: const #4s : i64
        let s_3_6: i64 = 4;
        // D s_3_7: read-var d:i64
        let s_3_7: i64 = fn_state.d;
        // D s_3_8: read-var imm:i64
        let s_3_8: i64 = fn_state.imm;
        // D s_3_9: read-var n:i64
        let s_3_9: i64 = fn_state.n;
        // D s_3_10: call execute_LUTI2_MZ4_ZTZ_1(s_3_0, s_3_7, s_3_4, s_3_3, s_3_8, s_3_5, s_3_9, s_3_6)
        let s_3_10: () = execute_LUTI2_MZ4_ZTZ_1(
            state,
            tracer,
            s_3_0,
            s_3_7,
            s_3_4,
            s_3_3,
            s_3_8,
            s_3_5,
            s_3_9,
            s_3_6,
        );
        // N s_3_11: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var VL:i64
        let s_4_0: i64 = fn_state.VL;
        // C s_4_1: const #256s : i
        let s_4_1: i128 = 256;
        // D s_4_2: cast zx s_4_0 -> i
        let s_4_2: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_1
        let s_4_3: bool = ((s_4_2) == (s_4_1));
        // D s_4_4: not s_4_3
        let s_4_4: bool = !s_4_3;
        // N s_4_5: branch s_4_4 b6 b5
        if s_4_4 {
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
        // C s_5_0: const #256s : i64
        let s_5_0: i64 = 256;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #1s : i64
        let s_5_4: i64 = 1;
        // C s_5_5: const #2s : i64
        let s_5_5: i64 = 2;
        // C s_5_6: const #4s : i64
        let s_5_6: i64 = 4;
        // D s_5_7: read-var d:i64
        let s_5_7: i64 = fn_state.d;
        // D s_5_8: read-var imm:i64
        let s_5_8: i64 = fn_state.imm;
        // D s_5_9: read-var n:i64
        let s_5_9: i64 = fn_state.n;
        // D s_5_10: call execute_LUTI2_MZ4_ZTZ_1(s_5_0, s_5_7, s_5_4, s_5_3, s_5_8, s_5_5, s_5_9, s_5_6)
        let s_5_10: () = execute_LUTI2_MZ4_ZTZ_1(
            state,
            tracer,
            s_5_0,
            s_5_7,
            s_5_4,
            s_5_3,
            s_5_8,
            s_5_5,
            s_5_9,
            s_5_6,
        );
        // N s_5_11: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #512s : i
        let s_6_1: i128 = 512;
        // D s_6_2: cast zx s_6_0 -> i
        let s_6_2: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_1
        let s_6_3: bool = ((s_6_2) == (s_6_1));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #512s : i64
        let s_7_0: i64 = 512;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // C s_7_5: const #2s : i64
        let s_7_5: i64 = 2;
        // C s_7_6: const #4s : i64
        let s_7_6: i64 = 4;
        // D s_7_7: read-var d:i64
        let s_7_7: i64 = fn_state.d;
        // D s_7_8: read-var imm:i64
        let s_7_8: i64 = fn_state.imm;
        // D s_7_9: read-var n:i64
        let s_7_9: i64 = fn_state.n;
        // D s_7_10: call execute_LUTI2_MZ4_ZTZ_1(s_7_0, s_7_7, s_7_4, s_7_3, s_7_8, s_7_5, s_7_9, s_7_6)
        let s_7_10: () = execute_LUTI2_MZ4_ZTZ_1(
            state,
            tracer,
            s_7_0,
            s_7_7,
            s_7_4,
            s_7_3,
            s_7_8,
            s_7_5,
            s_7_9,
            s_7_6,
        );
        // N s_7_11: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #1024s : i
        let s_8_1: i128 = 1024;
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
        // C s_9_0: const #1024s : i64
        let s_9_0: i64 = 1024;
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #1s : i64
        let s_9_4: i64 = 1;
        // C s_9_5: const #2s : i64
        let s_9_5: i64 = 2;
        // C s_9_6: const #4s : i64
        let s_9_6: i64 = 4;
        // D s_9_7: read-var d:i64
        let s_9_7: i64 = fn_state.d;
        // D s_9_8: read-var imm:i64
        let s_9_8: i64 = fn_state.imm;
        // D s_9_9: read-var n:i64
        let s_9_9: i64 = fn_state.n;
        // D s_9_10: call execute_LUTI2_MZ4_ZTZ_1(s_9_0, s_9_7, s_9_4, s_9_3, s_9_8, s_9_5, s_9_9, s_9_6)
        let s_9_10: () = execute_LUTI2_MZ4_ZTZ_1(
            state,
            tracer,
            s_9_0,
            s_9_7,
            s_9_4,
            s_9_3,
            s_9_8,
            s_9_5,
            s_9_9,
            s_9_6,
        );
        // N s_9_11: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #2048s : i
        let s_10_1: i128 = 2048;
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
        // C s_11_0: const #2048s : i64
        let s_11_0: i64 = 2048;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #1s : i64
        let s_11_4: i64 = 1;
        // C s_11_5: const #2s : i64
        let s_11_5: i64 = 2;
        // C s_11_6: const #4s : i64
        let s_11_6: i64 = 4;
        // D s_11_7: read-var d:i64
        let s_11_7: i64 = fn_state.d;
        // D s_11_8: read-var imm:i64
        let s_11_8: i64 = fn_state.imm;
        // D s_11_9: read-var n:i64
        let s_11_9: i64 = fn_state.n;
        // D s_11_10: call execute_LUTI2_MZ4_ZTZ_1(s_11_0, s_11_7, s_11_4, s_11_3, s_11_8, s_11_5, s_11_9, s_11_6)
        let s_11_10: () = execute_LUTI2_MZ4_ZTZ_1(
            state,
            tracer,
            s_11_0,
            s_11_7,
            s_11_4,
            s_11_3,
            s_11_8,
            s_11_5,
            s_11_9,
            s_11_6,
        );
        // N s_11_11: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
}