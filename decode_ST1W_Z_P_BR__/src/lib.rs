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
use execute_ST1W_Z_P_BR__::*;
use HaveSME::*;
use common::*;
pub fn decode_ST1W_Z_P_BR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    sz: bool,
    Rm: u8,
    Pg: u8,
    Rn: u8,
    Zt: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        gs_242756: bool,
        VL: i64,
        esize: i64,
        n: i64,
        g: i64,
        sz: bool,
        Rm: u8,
        Pg: u8,
        Rn: u8,
        Zt: u8,
    }
    let fn_state = FunctionState {
        sz,
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#242756 <= s_1_0
        fn_state.gs_242756 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#242756:u8
        let s_2_0: bool = fn_state.gs_242756;
        // N s_2_1: branch s_2_0 b16 b3
        if s_2_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rm:u8
        let s_3_0: u8 = fn_state.Rm;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // C s_3_2: const #31u : u8
        let s_3_2: u8 = 31;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 5u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b15 b4
        if s_3_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var Zt:u8
        let s_4_0: u8 = fn_state.Zt;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 5u16);
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (s_4_1.value() as i128);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: write-var t <= s_4_3
        fn_state.t = s_4_3;
        // D s_4_5: read-var Rn:u8
        let s_4_5: u8 = fn_state.Rn;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 5u16);
        // D s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (s_4_6.value() as i128);
        // D s_4_8: cast reint s_4_7 -> i64
        let s_4_8: i64 = (s_4_7 as i64);
        // D s_4_9: write-var n <= s_4_8
        fn_state.n = s_4_8;
        // D s_4_10: read-var Rm:u8
        let s_4_10: u8 = fn_state.Rm;
        // D s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 5u16);
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (s_4_11.value() as i128);
        // D s_4_13: cast reint s_4_12 -> i64
        let s_4_13: i64 = (s_4_12 as i64);
        // D s_4_14: write-var m <= s_4_13
        fn_state.m = s_4_13;
        // D s_4_15: read-var Pg:u8
        let s_4_15: u8 = fn_state.Pg;
        // D s_4_16: cast zx s_4_15 -> bv
        let s_4_16: Bits = Bits::new(s_4_15 as u128, 3u16);
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (s_4_16.value() as i128);
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // D s_4_19: write-var g <= s_4_18
        fn_state.g = s_4_18;
        // D s_4_20: read-var sz:u8
        let s_4_20: bool = fn_state.sz;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 1u16);
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (s_4_21.value() as i128);
        // D s_4_23: cast reint s_4_22 -> i64
        let s_4_23: i64 = (s_4_22 as i64);
        // C s_4_24: const #32s : i64
        let s_4_24: i64 = 32;
        // D s_4_25: lsl s_4_24 s_4_23
        let s_4_25: i64 = s_4_24 << s_4_23;
        // D s_4_26: write-var esize <= s_4_25
        fn_state.esize = s_4_25;
        // D s_4_27: read-var VL:i64
        let s_4_27: i64 = fn_state.VL;
        // C s_4_28: const #128s : i
        let s_4_28: i128 = 128;
        // D s_4_29: cast zx s_4_27 -> i
        let s_4_29: i128 = (i128::try_from(s_4_27).unwrap());
        // D s_4_30: cmp-eq s_4_29 s_4_28
        let s_4_30: bool = ((s_4_29) == (s_4_28));
        // D s_4_31: not s_4_30
        let s_4_31: bool = !s_4_30;
        // N s_4_32: branch s_4_31 b6 b5
        if s_4_31 {
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
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: read-var esize:i64
        let s_5_1: i64 = fn_state.esize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #32s : i64
        let s_5_4: i64 = 32;
        // D s_5_5: read-var g:i64
        let s_5_5: i64 = fn_state.g;
        // D s_5_6: read-var m:i64
        let s_5_6: i64 = fn_state.m;
        // D s_5_7: read-var n:i64
        let s_5_7: i64 = fn_state.n;
        // D s_5_8: read-var t:i64
        let s_5_8: i64 = fn_state.t;
        // D s_5_9: call execute_ST1W_Z_P_BR__(s_5_0, s_5_3, s_5_5, s_5_6, s_5_4, s_5_7, s_5_8)
        let s_5_9: () = execute_ST1W_Z_P_BR__(
            state,
            tracer,
            s_5_0,
            s_5_3,
            s_5_5,
            s_5_6,
            s_5_4,
            s_5_7,
            s_5_8,
        );
        // N s_5_10: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var VL:i64
        let s_6_0: i64 = fn_state.VL;
        // C s_6_1: const #256s : i
        let s_6_1: i128 = 256;
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
        // C s_7_0: const #256s : i64
        let s_7_0: i64 = 256;
        // D s_7_1: read-var esize:i64
        let s_7_1: i64 = fn_state.esize;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #32s : i64
        let s_7_4: i64 = 32;
        // D s_7_5: read-var g:i64
        let s_7_5: i64 = fn_state.g;
        // D s_7_6: read-var m:i64
        let s_7_6: i64 = fn_state.m;
        // D s_7_7: read-var n:i64
        let s_7_7: i64 = fn_state.n;
        // D s_7_8: read-var t:i64
        let s_7_8: i64 = fn_state.t;
        // D s_7_9: call execute_ST1W_Z_P_BR__(s_7_0, s_7_3, s_7_5, s_7_6, s_7_4, s_7_7, s_7_8)
        let s_7_9: () = execute_ST1W_Z_P_BR__(
            state,
            tracer,
            s_7_0,
            s_7_3,
            s_7_5,
            s_7_6,
            s_7_4,
            s_7_7,
            s_7_8,
        );
        // N s_7_10: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var VL:i64
        let s_8_0: i64 = fn_state.VL;
        // C s_8_1: const #512s : i
        let s_8_1: i128 = 512;
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
        // C s_9_0: const #512s : i64
        let s_9_0: i64 = 512;
        // D s_9_1: read-var esize:i64
        let s_9_1: i64 = fn_state.esize;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #32s : i64
        let s_9_4: i64 = 32;
        // D s_9_5: read-var g:i64
        let s_9_5: i64 = fn_state.g;
        // D s_9_6: read-var m:i64
        let s_9_6: i64 = fn_state.m;
        // D s_9_7: read-var n:i64
        let s_9_7: i64 = fn_state.n;
        // D s_9_8: read-var t:i64
        let s_9_8: i64 = fn_state.t;
        // D s_9_9: call execute_ST1W_Z_P_BR__(s_9_0, s_9_3, s_9_5, s_9_6, s_9_4, s_9_7, s_9_8)
        let s_9_9: () = execute_ST1W_Z_P_BR__(
            state,
            tracer,
            s_9_0,
            s_9_3,
            s_9_5,
            s_9_6,
            s_9_4,
            s_9_7,
            s_9_8,
        );
        // N s_9_10: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VL:i64
        let s_10_0: i64 = fn_state.VL;
        // C s_10_1: const #1024s : i
        let s_10_1: i128 = 1024;
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
        // C s_11_0: const #1024s : i64
        let s_11_0: i64 = 1024;
        // D s_11_1: read-var esize:i64
        let s_11_1: i64 = fn_state.esize;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // C s_11_4: const #32s : i64
        let s_11_4: i64 = 32;
        // D s_11_5: read-var g:i64
        let s_11_5: i64 = fn_state.g;
        // D s_11_6: read-var m:i64
        let s_11_6: i64 = fn_state.m;
        // D s_11_7: read-var n:i64
        let s_11_7: i64 = fn_state.n;
        // D s_11_8: read-var t:i64
        let s_11_8: i64 = fn_state.t;
        // D s_11_9: call execute_ST1W_Z_P_BR__(s_11_0, s_11_3, s_11_5, s_11_6, s_11_4, s_11_7, s_11_8)
        let s_11_9: () = execute_ST1W_Z_P_BR__(
            state,
            tracer,
            s_11_0,
            s_11_3,
            s_11_5,
            s_11_6,
            s_11_4,
            s_11_7,
            s_11_8,
        );
        // N s_11_10: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var VL:i64
        let s_12_0: i64 = fn_state.VL;
        // C s_12_1: const #2048s : i
        let s_12_1: i128 = 2048;
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
        // C s_13_0: const #2048s : i64
        let s_13_0: i64 = 2048;
        // D s_13_1: read-var esize:i64
        let s_13_1: i64 = fn_state.esize;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: const #32s : i64
        let s_13_4: i64 = 32;
        // D s_13_5: read-var g:i64
        let s_13_5: i64 = fn_state.g;
        // D s_13_6: read-var m:i64
        let s_13_6: i64 = fn_state.m;
        // D s_13_7: read-var n:i64
        let s_13_7: i64 = fn_state.n;
        // D s_13_8: read-var t:i64
        let s_13_8: i64 = fn_state.t;
        // D s_13_9: call execute_ST1W_Z_P_BR__(s_13_0, s_13_3, s_13_5, s_13_6, s_13_4, s_13_7, s_13_8)
        let s_13_9: () = execute_ST1W_Z_P_BR__(
            state,
            tracer,
            s_13_0,
            s_13_3,
            s_13_5,
            s_13_6,
            s_13_4,
            s_13_7,
            s_13_8,
        );
        // N s_13_10: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
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
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call HaveSME(s_17_0)
        let s_17_1: bool = HaveSME(state, tracer, s_17_0);
        // S s_17_2: not s_17_1
        let s_17_2: bool = !s_17_1;
        // D s_17_3: write-var gs#242756 <= s_17_2
        fn_state.gs_242756 = s_17_2;
        // N s_17_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
