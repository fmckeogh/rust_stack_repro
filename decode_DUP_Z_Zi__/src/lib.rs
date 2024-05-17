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
use HaveSVE::*;
use CurrentVL_read::*;
use execute_DUP_Z_Zi__::*;
use HaveSME::*;
use common::*;
pub fn decode_DUP_Z_Zi__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    imm2: u8,
    tsz: u8,
    Zn: u8,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        n: i64,
        index: i64,
        indexshadow_2986: i64,
        VL: i64,
        esizeshadow_2987: i64,
        d: i64,
        gs_196654: bool,
        imm: u8,
        imm2: u8,
        tsz: u8,
        Zn: u8,
        Zd: u8,
    }
    let fn_state = FunctionState {
        imm2,
        tsz,
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
        // S s_0_4: call HaveSVE(s_0_3)
        let s_0_4: bool = HaveSVE(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b26 b1
        if s_0_5 {
            return block_26(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#196654 <= s_1_0
        fn_state.gs_196654 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#196654:u8
        let s_2_0: bool = fn_state.gs_196654;
        // N s_2_1: branch s_2_0 b25 b3
        if s_2_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var imm2:u8
        let s_3_0: u8 = fn_state.imm2;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // D s_3_2: read-var tsz:u8
        let s_3_2: u8 = fn_state.tsz;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
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
        // D s_3_13: write-var imm <= s_3_12
        fn_state.imm = s_3_12;
        // C s_3_14: const #8s : i64
        let s_3_14: i64 = 8;
        // D s_3_15: write-var esize <= s_3_14
        fn_state.esize = s_3_14;
        // D s_3_16: read-var tsz:u8
        let s_3_16: u8 = fn_state.tsz;
        // D s_3_17: cast zx s_3_16 -> bv
        let s_3_17: Bits = Bits::new(s_3_16 as u128, 5u16);
        // C s_3_18: const #0u : u8
        let s_3_18: u8 = 0;
        // C s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 5u16);
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
        // D s_5_0: read-var tsz:u8
        let s_5_0: u8 = fn_state.tsz;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // C s_5_2: const #16u : u8
        let s_5_2: u8 = 16;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 5u16);
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
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: write-var esize <= s_6_0
        fn_state.esize = s_6_0;
        // C s_6_2: const #5s : i
        let s_6_2: i128 = 5;
        // D s_6_3: read-var imm:u8
        let s_6_3: u8 = fn_state.imm;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 7u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #1s : i
        let s_6_7: i128 = 1;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 2u16);
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (s_6_11.value() as i128);
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: write-var index <= s_6_13
        fn_state.index = s_6_13;
        // N s_6_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var index:i64
        let s_7_0: i64 = fn_state.index;
        // D s_7_1: write-var indexshadow#2986 <= s_7_0
        fn_state.indexshadow_2986 = s_7_0;
        // D s_7_2: read-var esize:i64
        let s_7_2: i64 = fn_state.esize;
        // D s_7_3: write-var esizeshadow#2987 <= s_7_2
        fn_state.esizeshadow_2987 = s_7_2;
        // D s_7_4: read-var Zn:u8
        let s_7_4: u8 = fn_state.Zn;
        // D s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 5u16);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (s_7_5.value() as i128);
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: write-var n <= s_7_7
        fn_state.n = s_7_7;
        // D s_7_9: read-var Zd:u8
        let s_7_9: u8 = fn_state.Zd;
        // D s_7_10: cast zx s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 5u16);
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (s_7_10.value() as i128);
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: write-var d <= s_7_12
        fn_state.d = s_7_12;
        // D s_7_14: read-var VL:i64
        let s_7_14: i64 = fn_state.VL;
        // C s_7_15: const #128s : i
        let s_7_15: i128 = 128;
        // D s_7_16: cast zx s_7_14 -> i
        let s_7_16: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_17: cmp-eq s_7_16 s_7_15
        let s_7_17: bool = ((s_7_16) == (s_7_15));
        // D s_7_18: not s_7_17
        let s_7_18: bool = !s_7_17;
        // N s_7_19: branch s_7_18 b9 b8
        if s_7_18 {
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
        // D s_8_1: read-var esizeshadow#2987:i64
        let s_8_1: i64 = fn_state.esizeshadow_2987;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var d:i64
        let s_8_4: i64 = fn_state.d;
        // D s_8_5: read-var indexshadow#2986:i64
        let s_8_5: i64 = fn_state.indexshadow_2986;
        // D s_8_6: read-var n:i64
        let s_8_6: i64 = fn_state.n;
        // D s_8_7: call execute_DUP_Z_Zi__(s_8_0, s_8_4, s_8_3, s_8_5, s_8_6)
        let s_8_7: () = execute_DUP_Z_Zi__(
            state,
            tracer,
            s_8_0,
            s_8_4,
            s_8_3,
            s_8_5,
            s_8_6,
        );
        // N s_8_8: return
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
        // D s_10_1: read-var esizeshadow#2987:i64
        let s_10_1: i64 = fn_state.esizeshadow_2987;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // D s_10_4: read-var d:i64
        let s_10_4: i64 = fn_state.d;
        // D s_10_5: read-var indexshadow#2986:i64
        let s_10_5: i64 = fn_state.indexshadow_2986;
        // D s_10_6: read-var n:i64
        let s_10_6: i64 = fn_state.n;
        // D s_10_7: call execute_DUP_Z_Zi__(s_10_0, s_10_4, s_10_3, s_10_5, s_10_6)
        let s_10_7: () = execute_DUP_Z_Zi__(
            state,
            tracer,
            s_10_0,
            s_10_4,
            s_10_3,
            s_10_5,
            s_10_6,
        );
        // N s_10_8: return
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
        // D s_12_1: read-var esizeshadow#2987:i64
        let s_12_1: i64 = fn_state.esizeshadow_2987;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var d:i64
        let s_12_4: i64 = fn_state.d;
        // D s_12_5: read-var indexshadow#2986:i64
        let s_12_5: i64 = fn_state.indexshadow_2986;
        // D s_12_6: read-var n:i64
        let s_12_6: i64 = fn_state.n;
        // D s_12_7: call execute_DUP_Z_Zi__(s_12_0, s_12_4, s_12_3, s_12_5, s_12_6)
        let s_12_7: () = execute_DUP_Z_Zi__(
            state,
            tracer,
            s_12_0,
            s_12_4,
            s_12_3,
            s_12_5,
            s_12_6,
        );
        // N s_12_8: return
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
        // D s_14_1: read-var esizeshadow#2987:i64
        let s_14_1: i64 = fn_state.esizeshadow_2987;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // D s_14_4: read-var d:i64
        let s_14_4: i64 = fn_state.d;
        // D s_14_5: read-var indexshadow#2986:i64
        let s_14_5: i64 = fn_state.indexshadow_2986;
        // D s_14_6: read-var n:i64
        let s_14_6: i64 = fn_state.n;
        // D s_14_7: call execute_DUP_Z_Zi__(s_14_0, s_14_4, s_14_3, s_14_5, s_14_6)
        let s_14_7: () = execute_DUP_Z_Zi__(
            state,
            tracer,
            s_14_0,
            s_14_4,
            s_14_3,
            s_14_5,
            s_14_6,
        );
        // N s_14_8: return
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
        // D s_16_1: read-var esizeshadow#2987:i64
        let s_16_1: i64 = fn_state.esizeshadow_2987;
        // D s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (i128::try_from(s_16_1).unwrap());
        // D s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // D s_16_4: read-var d:i64
        let s_16_4: i64 = fn_state.d;
        // D s_16_5: read-var indexshadow#2986:i64
        let s_16_5: i64 = fn_state.indexshadow_2986;
        // D s_16_6: read-var n:i64
        let s_16_6: i64 = fn_state.n;
        // D s_16_7: call execute_DUP_Z_Zi__(s_16_0, s_16_4, s_16_3, s_16_5, s_16_6)
        let s_16_7: () = execute_DUP_Z_Zi__(
            state,
            tracer,
            s_16_0,
            s_16_4,
            s_16_3,
            s_16_5,
            s_16_6,
        );
        // N s_16_8: return
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
        // D s_18_0: read-var tsz:u8
        let s_18_0: u8 = fn_state.tsz;
        // C s_18_1: const #0s : i
        let s_18_1: i128 = 0;
        // D s_18_2: cast zx s_18_0 -> bv
        let s_18_2: Bits = Bits::new(s_18_0 as u128, 5u16);
        // C s_18_3: const #1s : i64
        let s_18_3: i64 = 1;
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #3s : i
        let s_18_5: i128 = 3;
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
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 4u16);
        // C s_18_10: const #8u : u8
        let s_18_10: u8 = 8;
        // C s_18_11: cast zx s_18_10 -> bv
        let s_18_11: Bits = Bits::new(s_18_10 as u128, 4u16);
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
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: write-var esize <= s_19_0
        fn_state.esize = s_19_0;
        // C s_19_2: const #4s : i
        let s_19_2: i128 = 4;
        // D s_19_3: read-var imm:u8
        let s_19_3: u8 = fn_state.imm;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 7u16);
        // C s_19_5: const #1s : i64
        let s_19_5: i64 = 1;
        // C s_19_6: cast zx s_19_5 -> i
        let s_19_6: i128 = (i128::try_from(s_19_5).unwrap());
        // C s_19_7: const #2s : i
        let s_19_7: i128 = 2;
        // C s_19_8: add s_19_7 s_19_6
        let s_19_8: i128 = (s_19_7 + s_19_6);
        // D s_19_9: bit-extract s_19_4 s_19_2 s_19_8
        let s_19_9: Bits = (Bits::new(
            ((s_19_4) >> (s_19_2)).value(),
            u16::try_from(s_19_8).unwrap(),
        ));
        // D s_19_10: cast reint s_19_9 -> u8
        let s_19_10: u8 = (s_19_9.value() as u8);
        // D s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 3u16);
        // D s_19_12: cast zx s_19_11 -> i
        let s_19_12: i128 = (s_19_11.value() as i128);
        // D s_19_13: cast reint s_19_12 -> i64
        let s_19_13: i64 = (s_19_12 as i64);
        // D s_19_14: write-var index <= s_19_13
        fn_state.index = s_19_13;
        // N s_19_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var tsz:u8
        let s_20_0: u8 = fn_state.tsz;
        // C s_20_1: const #0s : i
        let s_20_1: i128 = 0;
        // D s_20_2: cast zx s_20_0 -> bv
        let s_20_2: Bits = Bits::new(s_20_0 as u128, 5u16);
        // C s_20_3: const #1s : i64
        let s_20_3: i64 = 1;
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #2s : i
        let s_20_5: i128 = 2;
        // C s_20_6: add s_20_5 s_20_4
        let s_20_6: i128 = (s_20_5 + s_20_4);
        // D s_20_7: bit-extract s_20_2 s_20_1 s_20_6
        let s_20_7: Bits = (Bits::new(
            ((s_20_2) >> (s_20_1)).value(),
            u16::try_from(s_20_6).unwrap(),
        ));
        // D s_20_8: cast reint s_20_7 -> u8
        let s_20_8: u8 = (s_20_7.value() as u8);
        // D s_20_9: cast zx s_20_8 -> bv
        let s_20_9: Bits = Bits::new(s_20_8 as u128, 3u16);
        // C s_20_10: const #4u : u8
        let s_20_10: u8 = 4;
        // C s_20_11: cast zx s_20_10 -> bv
        let s_20_11: Bits = Bits::new(s_20_10 as u128, 3u16);
        // D s_20_12: cmp-eq s_20_9 s_20_11
        let s_20_12: bool = ((s_20_9) == (s_20_11));
        // D s_20_13: not s_20_12
        let s_20_13: bool = !s_20_12;
        // N s_20_14: branch s_20_13 b22 b21
        if s_20_13 {
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
        // C s_21_0: const #32s : i64
        let s_21_0: i64 = 32;
        // D s_21_1: write-var esize <= s_21_0
        fn_state.esize = s_21_0;
        // C s_21_2: const #3s : i
        let s_21_2: i128 = 3;
        // D s_21_3: read-var imm:u8
        let s_21_3: u8 = fn_state.imm;
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 7u16);
        // C s_21_5: const #1s : i64
        let s_21_5: i64 = 1;
        // C s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // C s_21_7: const #3s : i
        let s_21_7: i128 = 3;
        // C s_21_8: add s_21_7 s_21_6
        let s_21_8: i128 = (s_21_7 + s_21_6);
        // D s_21_9: bit-extract s_21_4 s_21_2 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_4) >> (s_21_2)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: u8 = (s_21_9.value() as u8);
        // D s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 4u16);
        // D s_21_12: cast zx s_21_11 -> i
        let s_21_12: i128 = (s_21_11.value() as i128);
        // D s_21_13: cast reint s_21_12 -> i64
        let s_21_13: i64 = (s_21_12 as i64);
        // D s_21_14: write-var index <= s_21_13
        fn_state.index = s_21_13;
        // N s_21_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var tsz:u8
        let s_22_0: u8 = fn_state.tsz;
        // C s_22_1: const #0s : i
        let s_22_1: i128 = 0;
        // D s_22_2: cast zx s_22_0 -> bv
        let s_22_2: Bits = Bits::new(s_22_0 as u128, 5u16);
        // C s_22_3: const #1s : i64
        let s_22_3: i64 = 1;
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #1s : i
        let s_22_5: i128 = 1;
        // C s_22_6: add s_22_5 s_22_4
        let s_22_6: i128 = (s_22_5 + s_22_4);
        // D s_22_7: bit-extract s_22_2 s_22_1 s_22_6
        let s_22_7: Bits = (Bits::new(
            ((s_22_2) >> (s_22_1)).value(),
            u16::try_from(s_22_6).unwrap(),
        ));
        // D s_22_8: cast reint s_22_7 -> u8
        let s_22_8: u8 = (s_22_7.value() as u8);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 2u16);
        // C s_22_10: const #2u : u8
        let s_22_10: u8 = 2;
        // C s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 2u16);
        // D s_22_12: cmp-eq s_22_9 s_22_11
        let s_22_12: bool = ((s_22_9) == (s_22_11));
        // D s_22_13: not s_22_12
        let s_22_13: bool = !s_22_12;
        // N s_22_14: branch s_22_13 b24 b23
        if s_22_13 {
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
        // C s_23_0: const #16s : i64
        let s_23_0: i64 = 16;
        // D s_23_1: write-var esize <= s_23_0
        fn_state.esize = s_23_0;
        // C s_23_2: const #2s : i
        let s_23_2: i128 = 2;
        // D s_23_3: read-var imm:u8
        let s_23_3: u8 = fn_state.imm;
        // D s_23_4: cast zx s_23_3 -> bv
        let s_23_4: Bits = Bits::new(s_23_3 as u128, 7u16);
        // C s_23_5: const #1s : i64
        let s_23_5: i64 = 1;
        // C s_23_6: cast zx s_23_5 -> i
        let s_23_6: i128 = (i128::try_from(s_23_5).unwrap());
        // C s_23_7: const #4s : i
        let s_23_7: i128 = 4;
        // C s_23_8: add s_23_7 s_23_6
        let s_23_8: i128 = (s_23_7 + s_23_6);
        // D s_23_9: bit-extract s_23_4 s_23_2 s_23_8
        let s_23_9: Bits = (Bits::new(
            ((s_23_4) >> (s_23_2)).value(),
            u16::try_from(s_23_8).unwrap(),
        ));
        // D s_23_10: cast reint s_23_9 -> u8
        let s_23_10: u8 = (s_23_9.value() as u8);
        // D s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 5u16);
        // D s_23_12: cast zx s_23_11 -> i
        let s_23_12: i128 = (s_23_11.value() as i128);
        // D s_23_13: cast reint s_23_12 -> i64
        let s_23_13: i64 = (s_23_12 as i64);
        // D s_23_14: write-var index <= s_23_13
        fn_state.index = s_23_13;
        // N s_23_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #8s : i64
        let s_24_0: i64 = 8;
        // D s_24_1: write-var esize <= s_24_0
        fn_state.esize = s_24_0;
        // C s_24_2: const #1s : i
        let s_24_2: i128 = 1;
        // D s_24_3: read-var imm:u8
        let s_24_3: u8 = fn_state.imm;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 7u16);
        // C s_24_5: const #1s : i64
        let s_24_5: i64 = 1;
        // C s_24_6: cast zx s_24_5 -> i
        let s_24_6: i128 = (i128::try_from(s_24_5).unwrap());
        // C s_24_7: const #5s : i
        let s_24_7: i128 = 5;
        // C s_24_8: add s_24_7 s_24_6
        let s_24_8: i128 = (s_24_7 + s_24_6);
        // D s_24_9: bit-extract s_24_4 s_24_2 s_24_8
        let s_24_9: Bits = (Bits::new(
            ((s_24_4) >> (s_24_2)).value(),
            u16::try_from(s_24_8).unwrap(),
        ));
        // D s_24_10: cast reint s_24_9 -> u8
        let s_24_10: u8 = (s_24_9.value() as u8);
        // D s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 6u16);
        // D s_24_12: cast zx s_24_11 -> i
        let s_24_12: i128 = (s_24_11.value() as i128);
        // D s_24_13: cast reint s_24_12 -> i64
        let s_24_13: i64 = (s_24_12 as i64);
        // D s_24_14: write-var index <= s_24_13
        fn_state.index = s_24_13;
        // N s_24_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call HaveSME(s_26_0)
        let s_26_1: bool = HaveSME(state, tracer, s_26_0);
        // S s_26_2: not s_26_1
        let s_26_2: bool = !s_26_1;
        // D s_26_3: write-var gs#196654 <= s_26_2
        fn_state.gs_196654 = s_26_2;
        // N s_26_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
