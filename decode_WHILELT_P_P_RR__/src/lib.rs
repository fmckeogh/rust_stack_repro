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
use execute_WHILELT_P_P_RR__::*;
use CurrentVL_read::*;
use HaveSME::*;
use common::*;
pub fn decode_WHILELT_P_P_RR__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    size: u8,
    Rm: u8,
    sf: bool,
    U: bool,
    lt: bool,
    Rn: u8,
    eq: bool,
    Pd: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        VL: i64,
        esize: i64,
        n: i64,
        d: i64,
        rsize: i64,
        gs_187512: bool,
        size: u8,
        Rm: u8,
        sf: bool,
        U: bool,
        lt: bool,
        Rn: u8,
        eq: bool,
        Pd: u8,
    }
    let fn_state = FunctionState {
        size,
        Rm,
        sf,
        U,
        lt,
        Rn,
        eq,
        Pd,
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
        // N s_0_6: branch s_0_5 b15 b1
        if s_0_5 {
            return block_15(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#187512 <= s_1_0
        fn_state.gs_187512 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#187512:u8
        let s_2_0: bool = fn_state.gs_187512;
        // N s_2_1: branch s_2_0 b14 b3
        if s_2_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #8s : i64
        let s_3_4: i64 = 8;
        // D s_3_5: lsl s_3_4 s_3_3
        let s_3_5: i64 = s_3_4 << s_3_3;
        // D s_3_6: write-var esize <= s_3_5
        fn_state.esize = s_3_5;
        // D s_3_7: read-var sf:u8
        let s_3_7: bool = fn_state.sf;
        // D s_3_8: cast zx s_3_7 -> bv
        let s_3_8: Bits = Bits::new(s_3_7 as u128, 1u16);
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (s_3_8.value() as i128);
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // C s_3_11: const #32s : i64
        let s_3_11: i64 = 32;
        // D s_3_12: lsl s_3_11 s_3_10
        let s_3_12: i64 = s_3_11 << s_3_10;
        // D s_3_13: write-var rsize <= s_3_12
        fn_state.rsize = s_3_12;
        // D s_3_14: read-var Rn:u8
        let s_3_14: u8 = fn_state.Rn;
        // D s_3_15: cast zx s_3_14 -> bv
        let s_3_15: Bits = Bits::new(s_3_14 as u128, 5u16);
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (s_3_15.value() as i128);
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: write-var n <= s_3_17
        fn_state.n = s_3_17;
        // D s_3_19: read-var Rm:u8
        let s_3_19: u8 = fn_state.Rm;
        // D s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 5u16);
        // D s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (s_3_20.value() as i128);
        // D s_3_22: cast reint s_3_21 -> i64
        let s_3_22: i64 = (s_3_21 as i64);
        // D s_3_23: write-var m <= s_3_22
        fn_state.m = s_3_22;
        // D s_3_24: read-var Pd:u8
        let s_3_24: u8 = fn_state.Pd;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 4u16);
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (s_3_25.value() as i128);
        // D s_3_27: cast reint s_3_26 -> i64
        let s_3_27: i64 = (s_3_26 as i64);
        // D s_3_28: write-var d <= s_3_27
        fn_state.d = s_3_27;
        // D s_3_29: read-var VL:i64
        let s_3_29: i64 = fn_state.VL;
        // C s_3_30: const #128s : i
        let s_3_30: i128 = 128;
        // D s_3_31: cast zx s_3_29 -> i
        let s_3_31: i128 = (i128::try_from(s_3_29).unwrap());
        // D s_3_32: cmp-eq s_3_31 s_3_30
        let s_3_32: bool = ((s_3_31) == (s_3_30));
        // D s_3_33: not s_3_32
        let s_3_33: bool = !s_3_32;
        // N s_3_34: branch s_3_33 b5 b4
        if s_3_33 {
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
        // D s_4_0: read-var rsize:i64
        let s_4_0: i64 = fn_state.rsize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // C s_4_3: const #128s : i64
        let s_4_3: i64 = 128;
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: read-var esize:i64
        let s_4_5: i64 = fn_state.esize;
        // D s_4_6: read-var m:i64
        let s_4_6: i64 = fn_state.m;
        // D s_4_7: read-var n:i64
        let s_4_7: i64 = fn_state.n;
        // C s_4_8: const #4u : u32
        let s_4_8: u32 = 4;
        // C s_4_9: const #0u : u8
        let s_4_9: bool = false;
        // D s_4_10: call execute_WHILELT_P_P_RR__(s_4_3, s_4_4, s_4_5, s_4_6, s_4_7, s_4_8, s_4_2, s_4_9)
        let s_4_10: () = execute_WHILELT_P_P_RR__(
            state,
            tracer,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_7,
            s_4_8,
            s_4_2,
            s_4_9,
        );
        // N s_4_11: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var VL:i64
        let s_5_0: i64 = fn_state.VL;
        // C s_5_1: const #256s : i
        let s_5_1: i128 = 256;
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
        // D s_6_0: read-var rsize:i64
        let s_6_0: i64 = fn_state.rsize;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // C s_6_3: const #256s : i64
        let s_6_3: i64 = 256;
        // D s_6_4: read-var d:i64
        let s_6_4: i64 = fn_state.d;
        // D s_6_5: read-var esize:i64
        let s_6_5: i64 = fn_state.esize;
        // D s_6_6: read-var m:i64
        let s_6_6: i64 = fn_state.m;
        // D s_6_7: read-var n:i64
        let s_6_7: i64 = fn_state.n;
        // C s_6_8: const #4u : u32
        let s_6_8: u32 = 4;
        // C s_6_9: const #0u : u8
        let s_6_9: bool = false;
        // D s_6_10: call execute_WHILELT_P_P_RR__(s_6_3, s_6_4, s_6_5, s_6_6, s_6_7, s_6_8, s_6_2, s_6_9)
        let s_6_10: () = execute_WHILELT_P_P_RR__(
            state,
            tracer,
            s_6_3,
            s_6_4,
            s_6_5,
            s_6_6,
            s_6_7,
            s_6_8,
            s_6_2,
            s_6_9,
        );
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var VL:i64
        let s_7_0: i64 = fn_state.VL;
        // C s_7_1: const #512s : i
        let s_7_1: i128 = 512;
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
        // D s_8_0: read-var rsize:i64
        let s_8_0: i64 = fn_state.rsize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // C s_8_3: const #512s : i64
        let s_8_3: i64 = 512;
        // D s_8_4: read-var d:i64
        let s_8_4: i64 = fn_state.d;
        // D s_8_5: read-var esize:i64
        let s_8_5: i64 = fn_state.esize;
        // D s_8_6: read-var m:i64
        let s_8_6: i64 = fn_state.m;
        // D s_8_7: read-var n:i64
        let s_8_7: i64 = fn_state.n;
        // C s_8_8: const #4u : u32
        let s_8_8: u32 = 4;
        // C s_8_9: const #0u : u8
        let s_8_9: bool = false;
        // D s_8_10: call execute_WHILELT_P_P_RR__(s_8_3, s_8_4, s_8_5, s_8_6, s_8_7, s_8_8, s_8_2, s_8_9)
        let s_8_10: () = execute_WHILELT_P_P_RR__(
            state,
            tracer,
            s_8_3,
            s_8_4,
            s_8_5,
            s_8_6,
            s_8_7,
            s_8_8,
            s_8_2,
            s_8_9,
        );
        // N s_8_11: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var VL:i64
        let s_9_0: i64 = fn_state.VL;
        // C s_9_1: const #1024s : i
        let s_9_1: i128 = 1024;
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
        // D s_10_0: read-var rsize:i64
        let s_10_0: i64 = fn_state.rsize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // C s_10_3: const #1024s : i64
        let s_10_3: i64 = 1024;
        // D s_10_4: read-var d:i64
        let s_10_4: i64 = fn_state.d;
        // D s_10_5: read-var esize:i64
        let s_10_5: i64 = fn_state.esize;
        // D s_10_6: read-var m:i64
        let s_10_6: i64 = fn_state.m;
        // D s_10_7: read-var n:i64
        let s_10_7: i64 = fn_state.n;
        // C s_10_8: const #4u : u32
        let s_10_8: u32 = 4;
        // C s_10_9: const #0u : u8
        let s_10_9: bool = false;
        // D s_10_10: call execute_WHILELT_P_P_RR__(s_10_3, s_10_4, s_10_5, s_10_6, s_10_7, s_10_8, s_10_2, s_10_9)
        let s_10_10: () = execute_WHILELT_P_P_RR__(
            state,
            tracer,
            s_10_3,
            s_10_4,
            s_10_5,
            s_10_6,
            s_10_7,
            s_10_8,
            s_10_2,
            s_10_9,
        );
        // N s_10_11: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VL:i64
        let s_11_0: i64 = fn_state.VL;
        // C s_11_1: const #2048s : i
        let s_11_1: i128 = 2048;
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
        // D s_12_0: read-var rsize:i64
        let s_12_0: i64 = fn_state.rsize;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // C s_12_3: const #2048s : i64
        let s_12_3: i64 = 2048;
        // D s_12_4: read-var d:i64
        let s_12_4: i64 = fn_state.d;
        // D s_12_5: read-var esize:i64
        let s_12_5: i64 = fn_state.esize;
        // D s_12_6: read-var m:i64
        let s_12_6: i64 = fn_state.m;
        // D s_12_7: read-var n:i64
        let s_12_7: i64 = fn_state.n;
        // C s_12_8: const #4u : u32
        let s_12_8: u32 = 4;
        // C s_12_9: const #0u : u8
        let s_12_9: bool = false;
        // D s_12_10: call execute_WHILELT_P_P_RR__(s_12_3, s_12_4, s_12_5, s_12_6, s_12_7, s_12_8, s_12_2, s_12_9)
        let s_12_10: () = execute_WHILELT_P_P_RR__(
            state,
            tracer,
            s_12_3,
            s_12_4,
            s_12_5,
            s_12_6,
            s_12_7,
            s_12_8,
            s_12_2,
            s_12_9,
        );
        // N s_12_11: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
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
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveSME(s_15_0)
        let s_15_1: bool = HaveSME(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // D s_15_3: write-var gs#187512 <= s_15_2
        fn_state.gs_187512 = s_15_2;
        // N s_15_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
