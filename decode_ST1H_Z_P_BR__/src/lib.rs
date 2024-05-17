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
use execute_ST1H_Z_P_BR__::*;
use HaveSME::*;
use common::*;
pub fn decode_ST1H_Z_P_BR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    Rm: u8,
    Pg: u8,
    Rn: u8,
    Zt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        VL: i64,
        esize: i64,
        n: i64,
        gs_242685: bool,
        g: i64,
        size: u8,
        Rm: u8,
        Pg: u8,
        Rn: u8,
        Zt: u8,
    }
    let fn_state = FunctionState {
        size,
        Rm,
        Pg,
        Rn,
        Zt,
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
        // N s_0_6: branch s_0_5 b19 b1
        if s_0_5 {
            return block_19(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#242685 <= s_1_0
        fn_state.gs_242685 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#242685:u8
        let s_2_0: bool = fn_state.gs_242685;
        // N s_2_1: branch s_2_0 b18 b3
        if s_2_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var size:u8
        let s_3_0: u8 = fn_state.size;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #0u : u8
        let s_3_2: u8 = 0;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b17 b4
        if s_3_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Rm:u8
        let s_4_0: u8 = fn_state.Rm;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // C s_4_2: const #31u : u8
        let s_4_2: u8 = 31;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 5u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b16 b5
        if s_4_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var Zt:u8
        let s_5_0: u8 = fn_state.Zt;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 5u16);
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (s_5_1.value() as i128);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: write-var t <= s_5_3
        fn_state.t = s_5_3;
        // D s_5_5: read-var Rn:u8
        let s_5_5: u8 = fn_state.Rn;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 5u16);
        // D s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (s_5_6.value() as i128);
        // D s_5_8: cast reint s_5_7 -> i64
        let s_5_8: i64 = (s_5_7 as i64);
        // D s_5_9: write-var n <= s_5_8
        fn_state.n = s_5_8;
        // D s_5_10: read-var Rm:u8
        let s_5_10: u8 = fn_state.Rm;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 5u16);
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (s_5_11.value() as i128);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // D s_5_14: write-var m <= s_5_13
        fn_state.m = s_5_13;
        // D s_5_15: read-var Pg:u8
        let s_5_15: u8 = fn_state.Pg;
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 3u16);
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (s_5_16.value() as i128);
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: write-var g <= s_5_18
        fn_state.g = s_5_18;
        // D s_5_20: read-var size:u8
        let s_5_20: u8 = fn_state.size;
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 2u16);
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (s_5_21.value() as i128);
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // C s_5_24: const #8s : i64
        let s_5_24: i64 = 8;
        // D s_5_25: lsl s_5_24 s_5_23
        let s_5_25: i64 = s_5_24 << s_5_23;
        // D s_5_26: write-var esize <= s_5_25
        fn_state.esize = s_5_25;
        // D s_5_27: read-var VL:i64
        let s_5_27: i64 = fn_state.VL;
        // C s_5_28: const #128s : i
        let s_5_28: i128 = 128;
        // D s_5_29: cast zx s_5_27 -> i
        let s_5_29: i128 = (i128::try_from(s_5_27).unwrap());
        // D s_5_30: cmp-eq s_5_29 s_5_28
        let s_5_30: bool = ((s_5_29) == (s_5_28));
        // D s_5_31: not s_5_30
        let s_5_31: bool = !s_5_30;
        // N s_5_32: branch s_5_31 b7 b6
        if s_5_31 {
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
        // D s_6_1: read-var esize:i64
        let s_6_1: i64 = fn_state.esize;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #16s : i64
        let s_6_4: i64 = 16;
        // D s_6_5: read-var g:i64
        let s_6_5: i64 = fn_state.g;
        // D s_6_6: read-var m:i64
        let s_6_6: i64 = fn_state.m;
        // D s_6_7: read-var n:i64
        let s_6_7: i64 = fn_state.n;
        // D s_6_8: read-var t:i64
        let s_6_8: i64 = fn_state.t;
        // D s_6_9: call execute_ST1H_Z_P_BR__(s_6_0, s_6_3, s_6_5, s_6_6, s_6_4, s_6_7, s_6_8)
        let s_6_9: () = execute_ST1H_Z_P_BR__(
            state,
            tracer,
            s_6_0,
            s_6_3,
            s_6_5,
            s_6_6,
            s_6_4,
            s_6_7,
            s_6_8,
        );
        // N s_6_10: return
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
        // D s_8_1: read-var esize:i64
        let s_8_1: i64 = fn_state.esize;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #16s : i64
        let s_8_4: i64 = 16;
        // D s_8_5: read-var g:i64
        let s_8_5: i64 = fn_state.g;
        // D s_8_6: read-var m:i64
        let s_8_6: i64 = fn_state.m;
        // D s_8_7: read-var n:i64
        let s_8_7: i64 = fn_state.n;
        // D s_8_8: read-var t:i64
        let s_8_8: i64 = fn_state.t;
        // D s_8_9: call execute_ST1H_Z_P_BR__(s_8_0, s_8_3, s_8_5, s_8_6, s_8_4, s_8_7, s_8_8)
        let s_8_9: () = execute_ST1H_Z_P_BR__(
            state,
            tracer,
            s_8_0,
            s_8_3,
            s_8_5,
            s_8_6,
            s_8_4,
            s_8_7,
            s_8_8,
        );
        // N s_8_10: return
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
        // D s_10_1: read-var esize:i64
        let s_10_1: i64 = fn_state.esize;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #16s : i64
        let s_10_4: i64 = 16;
        // D s_10_5: read-var g:i64
        let s_10_5: i64 = fn_state.g;
        // D s_10_6: read-var m:i64
        let s_10_6: i64 = fn_state.m;
        // D s_10_7: read-var n:i64
        let s_10_7: i64 = fn_state.n;
        // D s_10_8: read-var t:i64
        let s_10_8: i64 = fn_state.t;
        // D s_10_9: call execute_ST1H_Z_P_BR__(s_10_0, s_10_3, s_10_5, s_10_6, s_10_4, s_10_7, s_10_8)
        let s_10_9: () = execute_ST1H_Z_P_BR__(
            state,
            tracer,
            s_10_0,
            s_10_3,
            s_10_5,
            s_10_6,
            s_10_4,
            s_10_7,
            s_10_8,
        );
        // N s_10_10: return
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
        // D s_12_1: read-var esize:i64
        let s_12_1: i64 = fn_state.esize;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #16s : i64
        let s_12_4: i64 = 16;
        // D s_12_5: read-var g:i64
        let s_12_5: i64 = fn_state.g;
        // D s_12_6: read-var m:i64
        let s_12_6: i64 = fn_state.m;
        // D s_12_7: read-var n:i64
        let s_12_7: i64 = fn_state.n;
        // D s_12_8: read-var t:i64
        let s_12_8: i64 = fn_state.t;
        // D s_12_9: call execute_ST1H_Z_P_BR__(s_12_0, s_12_3, s_12_5, s_12_6, s_12_4, s_12_7, s_12_8)
        let s_12_9: () = execute_ST1H_Z_P_BR__(
            state,
            tracer,
            s_12_0,
            s_12_3,
            s_12_5,
            s_12_6,
            s_12_4,
            s_12_7,
            s_12_8,
        );
        // N s_12_10: return
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
        // D s_14_1: read-var esize:i64
        let s_14_1: i64 = fn_state.esize;
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: const #16s : i64
        let s_14_4: i64 = 16;
        // D s_14_5: read-var g:i64
        let s_14_5: i64 = fn_state.g;
        // D s_14_6: read-var m:i64
        let s_14_6: i64 = fn_state.m;
        // D s_14_7: read-var n:i64
        let s_14_7: i64 = fn_state.n;
        // D s_14_8: read-var t:i64
        let s_14_8: i64 = fn_state.t;
        // D s_14_9: call execute_ST1H_Z_P_BR__(s_14_0, s_14_3, s_14_5, s_14_6, s_14_4, s_14_7, s_14_8)
        let s_14_9: () = execute_ST1H_Z_P_BR__(
            state,
            tracer,
            s_14_0,
            s_14_3,
            s_14_5,
            s_14_6,
            s_14_4,
            s_14_7,
            s_14_8,
        );
        // N s_14_10: return
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
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
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
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveSME(s_19_0)
        let s_19_1: bool = HaveSME(state, tracer, s_19_0);
        // S s_19_2: not s_19_1
        let s_19_2: bool = !s_19_1;
        // D s_19_3: write-var gs#242685 <= s_19_2
        fn_state.gs_242685 = s_19_2;
        // N s_19_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}