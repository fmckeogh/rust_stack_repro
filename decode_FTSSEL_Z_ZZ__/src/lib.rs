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
use execute_FTSSEL_Z_ZZ__::*;
use common::*;
pub fn decode_FTSSEL_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    Zm: u8,
    Zn: u8,
    Zd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        VL: i64,
        esize: i64,
        n: i64,
        d: i64,
        size: u8,
        Zm: u8,
        Zn: u8,
        Zd: u8,
    }
    let fn_state = FunctionState {
        size,
        Zm,
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
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
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
        // D s_2_12: read-var Zm:u8
        let s_2_12: u8 = fn_state.Zm;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 5u16);
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (s_2_13.value() as i128);
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: write-var m <= s_2_15
        fn_state.m = s_2_15;
        // D s_2_17: read-var Zd:u8
        let s_2_17: u8 = fn_state.Zd;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 5u16);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (s_2_18.value() as i128);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var d <= s_2_20
        fn_state.d = s_2_20;
        // D s_2_22: read-var VL:i64
        let s_2_22: i64 = fn_state.VL;
        // C s_2_23: const #128s : i
        let s_2_23: i128 = 128;
        // D s_2_24: cast zx s_2_22 -> i
        let s_2_24: i128 = (i128::try_from(s_2_22).unwrap());
        // D s_2_25: cmp-eq s_2_24 s_2_23
        let s_2_25: bool = ((s_2_24) == (s_2_23));
        // D s_2_26: not s_2_25
        let s_2_26: bool = !s_2_25;
        // N s_2_27: branch s_2_26 b4 b3
        if s_2_26 {
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
        // D s_3_4: read-var d:i64
        let s_3_4: i64 = fn_state.d;
        // D s_3_5: read-var m:i64
        let s_3_5: i64 = fn_state.m;
        // D s_3_6: read-var n:i64
        let s_3_6: i64 = fn_state.n;
        // D s_3_7: call execute_FTSSEL_Z_ZZ__(s_3_0, s_3_4, s_3_3, s_3_5, s_3_6)
        let s_3_7: () = execute_FTSSEL_Z_ZZ__(
            state,
            tracer,
            s_3_0,
            s_3_4,
            s_3_3,
            s_3_5,
            s_3_6,
        );
        // N s_3_8: return
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
        // D s_5_4: read-var d:i64
        let s_5_4: i64 = fn_state.d;
        // D s_5_5: read-var m:i64
        let s_5_5: i64 = fn_state.m;
        // D s_5_6: read-var n:i64
        let s_5_6: i64 = fn_state.n;
        // D s_5_7: call execute_FTSSEL_Z_ZZ__(s_5_0, s_5_4, s_5_3, s_5_5, s_5_6)
        let s_5_7: () = execute_FTSSEL_Z_ZZ__(
            state,
            tracer,
            s_5_0,
            s_5_4,
            s_5_3,
            s_5_5,
            s_5_6,
        );
        // N s_5_8: return
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
        // D s_7_4: read-var d:i64
        let s_7_4: i64 = fn_state.d;
        // D s_7_5: read-var m:i64
        let s_7_5: i64 = fn_state.m;
        // D s_7_6: read-var n:i64
        let s_7_6: i64 = fn_state.n;
        // D s_7_7: call execute_FTSSEL_Z_ZZ__(s_7_0, s_7_4, s_7_3, s_7_5, s_7_6)
        let s_7_7: () = execute_FTSSEL_Z_ZZ__(
            state,
            tracer,
            s_7_0,
            s_7_4,
            s_7_3,
            s_7_5,
            s_7_6,
        );
        // N s_7_8: return
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
        // D s_9_4: read-var d:i64
        let s_9_4: i64 = fn_state.d;
        // D s_9_5: read-var m:i64
        let s_9_5: i64 = fn_state.m;
        // D s_9_6: read-var n:i64
        let s_9_6: i64 = fn_state.n;
        // D s_9_7: call execute_FTSSEL_Z_ZZ__(s_9_0, s_9_4, s_9_3, s_9_5, s_9_6)
        let s_9_7: () = execute_FTSSEL_Z_ZZ__(
            state,
            tracer,
            s_9_0,
            s_9_4,
            s_9_3,
            s_9_5,
            s_9_6,
        );
        // N s_9_8: return
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
        // D s_11_4: read-var d:i64
        let s_11_4: i64 = fn_state.d;
        // D s_11_5: read-var m:i64
        let s_11_5: i64 = fn_state.m;
        // D s_11_6: read-var n:i64
        let s_11_6: i64 = fn_state.n;
        // D s_11_7: call execute_FTSSEL_Z_ZZ__(s_11_0, s_11_4, s_11_3, s_11_5, s_11_6)
        let s_11_7: () = execute_FTSSEL_Z_ZZ__(
            state,
            tracer,
            s_11_0,
            s_11_4,
            s_11_3,
            s_11_5,
            s_11_6,
        );
        // N s_11_8: return
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
