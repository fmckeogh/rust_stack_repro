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
use HaveSMEF16F16::*;
use CurrentVL_read::*;
use execute_FCVT_MZ2_Z__::*;
use common::*;
pub fn decode_FCVT_MZ2_Z__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Zn: u8,
    Zd: u8,
    L: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        VL: i64,
        n: i64,
        d: i64,
        Zn: u8,
        Zd: u8,
        L: bool,
    }
    let fn_state = FunctionState {
        Zn,
        Zd,
        L,
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
        // S s_0_4: call HaveSMEF16F16(s_0_3)
        let s_0_4: bool = HaveSMEF16F16(state, tracer, s_0_3);
        // S s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b12 b1
        if s_0_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Zn:u8
        let s_1_0: u8 = fn_state.Zn;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var n <= s_1_3
        fn_state.n = s_1_3;
        // D s_1_5: read-var Zd:u8
        let s_1_5: u8 = fn_state.Zd;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 4u16);
        // C s_1_7: const #0u : u8
        let s_1_7: bool = false;
        // C s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 1u16);
        // D s_1_9: cast reint s_1_6 -> u128
        let s_1_9: u128 = (s_1_6.value() as u128);
        // D s_1_10: size-of s_1_6
        let s_1_10: u16 = s_1_6.length();
        // C s_1_11: cast reint s_1_8 -> u128
        let s_1_11: u128 = (s_1_8.value() as u128);
        // D s_1_12: size-of s_1_8
        let s_1_12: u16 = s_1_8.length();
        // D s_1_13: lsl s_1_9 s_1_12
        let s_1_13: u128 = s_1_9 << s_1_12;
        // D s_1_14: or s_1_13 s_1_11
        let s_1_14: u128 = ((s_1_13) | (s_1_11));
        // D s_1_15: add s_1_10 s_1_12
        let s_1_15: u16 = (s_1_10 + s_1_12);
        // D s_1_16: create-bits s_1_14 s_1_15
        let s_1_16: Bits = Bits::new(s_1_14, s_1_15);
        // D s_1_17: cast reint s_1_16 -> u8
        let s_1_17: u8 = (s_1_16.value() as u8);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 5u16);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (s_1_18.value() as i128);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var d <= s_1_20
        fn_state.d = s_1_20;
        // D s_1_22: read-var VL:i64
        let s_1_22: i64 = fn_state.VL;
        // C s_1_23: const #128s : i
        let s_1_23: i128 = 128;
        // D s_1_24: cast zx s_1_22 -> i
        let s_1_24: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_25: cmp-eq s_1_24 s_1_23
        let s_1_25: bool = ((s_1_24) == (s_1_23));
        // D s_1_26: not s_1_25
        let s_1_26: bool = !s_1_25;
        // N s_1_27: branch s_1_26 b3 b2
        if s_1_26 {
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
        // C s_2_0: const #128s : i64
        let s_2_0: i64 = 128;
        // D s_2_1: read-var d:i64
        let s_2_1: i64 = fn_state.d;
        // D s_2_2: read-var n:i64
        let s_2_2: i64 = fn_state.n;
        // D s_2_3: call execute_FCVT_MZ2_Z__(s_2_0, s_2_1, s_2_2)
        let s_2_3: () = execute_FCVT_MZ2_Z__(state, tracer, s_2_0, s_2_1, s_2_2);
        // N s_2_4: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VL:i64
        let s_3_0: i64 = fn_state.VL;
        // C s_3_1: const #256s : i
        let s_3_1: i128 = 256;
        // D s_3_2: cast zx s_3_0 -> i
        let s_3_2: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_1
        let s_3_3: bool = ((s_3_2) == (s_3_1));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
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
        // C s_4_0: const #256s : i64
        let s_4_0: i64 = 256;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: read-var n:i64
        let s_4_2: i64 = fn_state.n;
        // D s_4_3: call execute_FCVT_MZ2_Z__(s_4_0, s_4_1, s_4_2)
        let s_4_3: () = execute_FCVT_MZ2_Z__(state, tracer, s_4_0, s_4_1, s_4_2);
        // N s_4_4: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #512s : i
        let s_5_1: i128 = 512;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // C s_6_0: const #512s : i64
        let s_6_0: i64 = 512;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: read-var n:i64
        let s_6_2: i64 = fn_state.n;
        // D s_6_3: call execute_FCVT_MZ2_Z__(s_6_0, s_6_1, s_6_2)
        let s_6_3: () = execute_FCVT_MZ2_Z__(state, tracer, s_6_0, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #1024s : i
        let s_7_1: i128 = 1024;
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
        // C s_8_0: const #1024s : i64
        let s_8_0: i64 = 1024;
        // D s_8_1: read-var d:i64
        let s_8_1: i64 = fn_state.d;
        // D s_8_2: read-var n:i64
        let s_8_2: i64 = fn_state.n;
        // D s_8_3: call execute_FCVT_MZ2_Z__(s_8_0, s_8_1, s_8_2)
        let s_8_3: () = execute_FCVT_MZ2_Z__(state, tracer, s_8_0, s_8_1, s_8_2);
        // N s_8_4: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #2048s : i
        let s_9_1: i128 = 2048;
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
        // C s_10_0: const #2048s : i64
        let s_10_0: i64 = 2048;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: read-var n:i64
        let s_10_2: i64 = fn_state.n;
        // D s_10_3: call execute_FCVT_MZ2_Z__(s_10_0, s_10_1, s_10_2)
        let s_10_3: () = execute_FCVT_MZ2_Z__(state, tracer, s_10_0, s_10_1, s_10_2);
        // N s_10_4: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
}
